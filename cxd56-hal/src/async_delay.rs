//! Async one-shot tick delay backed by a hardware timer.
//!
//! This is the HAL's interrupt-driven async time source: a future that arms a
//! hardware compare/alarm, suspends, and is woken by that peripheral's interrupt
//! — so the core can `WFE`-sleep (and the executor run other tasks) for the whole
//! delay instead of busy-spinning a counter.
//!
//! # Swappable backing (`TickSource`)
//!
//! The timing peripheral sits behind the private [`TickSource`] seam and is
//! selected by a Cargo feature, mirroring embassy's `time-driver-*` convention:
//!
//! * **`async-delay-rtc`** (default) — RTC0 **alarm channel 0**. The RTC runs at a
//!   fixed 32.768 kHz and is **perf-invariant and always-on**, so a delay is the
//!   same real time at every operating point and survives a clock change. Tick
//!   granularity is ~30.5 µs. This is the right default for low-power work.
//!
//! A future `async-delay-timer` backing (SP804 `TIMER0/1`, see [`crate::timer`])
//! can drop in behind the same seam for sub-µs resolution at the cost of being
//! perf-dependent; nothing above [`TickSource`] changes.
//!
//! # The app must forward the source interrupt
//!
//! Like the GPIO [`Wait`](crate::gpio::Wait) API, a library cannot define the
//! interrupt vector itself (the PAC weakly binds it to `DefaultHandler`), so the
//! application owns the handler and forwards it to [`on_interrupt`]. With the
//! default RTC backing that is **`RTC0_A0`**:
//!
//! ```ignore
//! #[interrupt]
//! fn RTC0_A0() {
//!     cxd56_hal::async_delay::on_interrupt(cxd56_hal::pac::Interrupt::RTC0_A0);
//! }
//! ```
//!
//! This is required by anything that awaits a delay here — including the GPIO
//! edge-arm baseline settle (`wait_for_*_edge`, see [`crate::gpio`]). Without the
//! handler the alarm fires into `DefaultHandler` and the delay never completes.
//!
//! # Concurrency (v1)
//!
//! A single delay may be in flight at a time (one alarm channel, one waker). That
//! covers the edge-arm settle and simple apps; a multi-waiter deadline queue (or
//! the RTC's other two channels) is a future extension behind the same seam.

use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll, Waker};
use cortex_m::peripheral::NVIC;
use critical_section::Mutex;

use crate::clocks::Clock;
use crate::pac;

// =============================================================================
// Source seam
// =============================================================================

/// A hardware tick source that can wake the core after a one-shot interval.
///
/// Implemented by exactly one backing peripheral, chosen by the `async-delay-*`
/// feature and aliased to [`Source`]. All methods are static: the source is a
/// single global peripheral reached through its PAC pointer.
trait TickSource {
    /// The interrupt the app forwards to [`on_interrupt`].
    const INTERRUPT: pac::Interrupt;
    /// Tick rate of [`now`](TickSource::now).
    const TICK_HZ: u32;

    /// Current monotonic tick count.
    fn now() -> u64;

    /// Arm a one-shot wake `ticks` from now and return the absolute deadline
    /// (tick count) it will fire at. The baseline is sampled inside, *after* any
    /// programming handshake, so programming latency does not shorten the delay.
    fn arm(ticks: u64) -> u64;

    /// Clear the pending wake and disarm. Idempotent; called from the ISR and
    /// when a poll observes the deadline already passed.
    fn disarm();
}

// =============================================================================
// RTC backing (alarm channel 0)
// =============================================================================

#[cfg(feature = "async-delay-rtc")]
mod rtc {
    use super::TickSource;
    use crate::pac;

    /// `SETALMPRECNT.ASET_BUSY` — a prior alarm-set is still reflecting.
    const ASET_BUSY: u32 = 1 << 16;
    /// `ALMOUTEN.ALM_BUSY` — a prior enable is still reflecting.
    const ALM_BUSY: u32 = 1 << 8;
    /// `ALMOUTEN.ALM_EN` — alarm match enable.
    const ALM_EN: u32 = 1 << 0;
    /// `ALMOUTEN.ALM_ERREN` — alarm error-output enable. Enabling it means a
    /// deadline accidentally set in the past raises the IRQ immediately (an error
    /// flag) instead of being lost — a free safety net (mirrors NuttX).
    const ALM_ERREN: u32 = 1 << 16;
    /// `ALMCLR`/`ALMFLG` channel-0 match + error flag bits.
    const ALM0_FLAGS: u32 = (1 << 0) | (1 << 16);

    pub(super) struct RtcTick;

    impl RtcTick {
        #[inline]
        fn rtc() -> &'static pac::rtc0::RegisterBlock {
            // SAFETY: RTC0 is the always-on clock peripheral; we issue only
            // single-register reads/writes to its alarm-0 and counter registers.
            unsafe { &*pac::Rtc0::PTR }
        }
    }

    impl TickSource for RtcTick {
        const INTERRUPT: pac::Interrupt = pac::Interrupt::RTC0_A0;
        const TICK_HZ: u32 = 32_768;

        fn now() -> u64 {
            let rtc = Self::rtc();
            // Re-read the high half for a consistent (POSTCNT<<15)|PRECNT snapshot
            // across a PRECNT wrap; the 47-bit count is monotonic.
            loop {
                let hi = rtc.rtpostcnt().read().bits();
                let lo = rtc.rtprecnt().read().bits() & 0x7fff;
                if hi == rtc.rtpostcnt().read().bits() {
                    return ((hi as u64) << 15) | lo as u64;
                }
            }
        }

        fn arm(ticks: u64) -> u64 {
            let rtc = Self::rtc();
            // SAFETY (all writes below): fixed alarm-0 MMIO; the values are the
            // documented compare/flag/enable encodings.
            // Drop any stale channel-0 flags, then wait for a previous set to land.
            rtc.almclr().write(|w| unsafe { w.bits(ALM0_FLAGS) });
            while rtc.setalmprecnt0().read().bits() & ASET_BUSY != 0 {}
            // Sample the baseline *here* (post-handshake) so the realized delay
            // counts the full `ticks` from now; write both compare halves, then
            // enable so the comparator only starts once both are set.
            let deadline = Self::now().wrapping_add(ticks);
            rtc.setalmpostcnt0()
                .write(|w| unsafe { w.bits((deadline >> 15) as u32) });
            rtc.setalmprecnt0()
                .write(|w| unsafe { w.bits((deadline & 0x7fff) as u32) });
            while rtc.almouten0().read().bits() & ALM_BUSY != 0 {}
            rtc.almouten0()
                .write(|w| unsafe { w.bits(ALM_EN | ALM_ERREN) });
            deadline
        }

        fn disarm() {
            let rtc = Self::rtc();
            // SAFETY: fixed alarm-0 MMIO; clear the flags and disable the channel.
            rtc.almclr().write(|w| unsafe { w.bits(ALM0_FLAGS) });
            rtc.almouten0().write(|w| unsafe { w.bits(0) });
        }
    }
}

/// The active tick source selected by the `async-delay-*` feature.
#[cfg(feature = "async-delay-rtc")]
type Source = rtc::RtcTick;

#[cfg(not(feature = "async-delay-rtc"))]
compile_error!("enable exactly one async-delay source feature (async-delay-rtc)");

// =============================================================================
// Waker + one-time INTC/NVIC setup
// =============================================================================

/// The single in-flight delay's waker, shared with the alarm ISR.
static WAKER: Mutex<RefCell<Option<Waker>>> = Mutex::new(RefCell::new(None));

fn register_waker(waker: &Waker) {
    critical_section::with(|cs| {
        let mut slot = WAKER.borrow(cs).borrow_mut();
        match &*slot {
            Some(existing) if existing.will_wake(waker) => {}
            _ => *slot = Some(waker.clone()),
        }
    });
}

fn wake() {
    let waker = critical_section::with(|cs| WAKER.borrow(cs).borrow_mut().take());
    if let Some(waker) = waker {
        waker.wake();
    }
}

/// Open the INTC gate and unmask the NVIC line for [`Source::INTERRUPT`] once.
///
/// Idempotent: the INTC set is a guarded read-modify-write of one bit and the
/// NVIC unmask is idempotent, so a redundant call is harmless. Called from both
/// [`Delay::new`] and the first poll of a [`TickDelay`], so a delay works whether
/// or not the app constructed a [`Delay`] handle (it still must forward the IRQ).
fn ensure_init() {
    static INITED: AtomicBool = AtomicBool::new(false);
    if INITED.load(Ordering::Acquire) {
        return;
    }
    crate::interrupt::enable(Source::INTERRUPT);
    // SAFETY: this HAL masks with `critical-section` (PRIMASK), not BASEPRI, so
    // unmasking a line cannot escape an in-progress critical section.
    unsafe { NVIC::unmask(Source::INTERRUPT) };
    INITED.store(true, Ordering::Release);
}

// =============================================================================
// Future + public API
// =============================================================================

/// Future that resolves after at least its requested number of source ticks.
///
/// Created by [`after_ticks`] / [`Delay::after_ticks`] / [`Delay`]'s
/// [`DelayNs`](embedded_hal_async::delay::DelayNs) impl. On the first poll it arms
/// the source alarm; thereafter it is woken by the alarm ISR (or any re-poll) and
/// completes once the deadline tick count has passed.
#[must_use = "a TickDelay does nothing unless awaited"]
pub struct TickDelay {
    ticks: u64,
    /// Absolute deadline tick count, set when the alarm is armed on first poll.
    deadline: Option<u64>,
}

impl TickDelay {
    fn new(ticks: u64) -> Self {
        Self {
            ticks,
            deadline: None,
        }
    }
}

impl Future for TickDelay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        match this.deadline {
            // Armed: complete once the deadline passes, else stay asleep.
            Some(deadline) => {
                if Source::now() >= deadline {
                    Source::disarm();
                    Poll::Ready(())
                } else {
                    register_waker(cx.waker());
                    Poll::Pending
                }
            }
            // First poll: open the gate, register the waker, then arm. Re-check
            // after arming so a deadline that elapsed during programming does not
            // sleep waiting for an alarm that already passed.
            None => {
                ensure_init();
                register_waker(cx.waker());
                let deadline = Source::arm(this.ticks);
                this.deadline = Some(deadline);
                if Source::now() >= deadline {
                    Source::disarm();
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

/// Await a one-shot delay of at least `ticks` source ticks (with the default RTC
/// backing, ticks of the perf-invariant 32.768 kHz clock — ~30.5 µs each).
///
/// A free function so callers that do not want to dedicate the RTC peripheral to a
/// [`Delay`] handle (e.g. the GPIO edge-arm settle) can still use it. The app must
/// still forward the source IRQ to [`on_interrupt`] — see the [module docs](self).
pub fn after_ticks(ticks: u32) -> TickDelay {
    TickDelay::new(ticks as u64)
}

/// Async delay handle bound to the RTC.
///
/// Owns [`pac::Rtc0`] so the timer source is clearly reserved, and implements the
/// async [`DelayNs`](embedded_hal_async::delay::DelayNs) trait. The [`Clock`]
/// borrow is for API symmetry with [`Timer`](crate::timer::Timer) and the future
/// perf-dependent `TIMER` backing — the RTC backing itself is perf-invariant.
pub struct Delay {
    _rtc: pac::Rtc0,
}

impl Delay {
    /// Reserve the RTC as an async delay source and open its interrupt path.
    pub fn new(rtc: pac::Rtc0, _clock: &Clock) -> Self {
        ensure_init();
        Self { _rtc: rtc }
    }

    /// Await a delay of at least `ticks` source ticks. See [`after_ticks`].
    pub fn after_ticks(&mut self, ticks: u32) -> TickDelay {
        TickDelay::new(ticks as u64)
    }
}

impl embedded_hal_async::delay::DelayNs for Delay {
    async fn delay_ns(&mut self, ns: u32) {
        // Ceiling-divide ns→ticks: DelayNs contracts a delay of *at least* `ns`.
        let ticks =
            ((ns as u64) * Source::TICK_HZ as u64).div_ceil(1_000_000_000);
        TickDelay::new(ticks).await
    }
}

/// Source-interrupt entry point — forward the active source's vector here (with
/// the default `async-delay-rtc` backing, `RTC0_A0`). See the [module docs](self)
/// for the one-line `#[interrupt]` handler. No-op for any other interrupt.
pub fn on_interrupt(interrupt: pac::Interrupt) {
    if interrupt == Source::INTERRUPT {
        Source::disarm();
        wake();
    }
}
