//! RTC0 alarm-0 embassy time driver.
//!
//! `now()` is the always-on 47-bit RTC counter (`(RTPOSTCNT<<15)|RTPRECNT`), which
//! at 32.768 kHz maps 1:1 onto embassy ticks under `tick-hz-32_768` — no rescale and
//! no overflow scheme (47 bits @ 32768 Hz wraps in ~136 years). One alarm channel
//! (ch0) drives an [`embassy_time_queue_utils`] queue: it is always armed for the
//! queue's earliest deadline, and its IRQ wakes expired timers and re-arms for the
//! next. Register MMIO + busy handshakes mirror the validated
//! [`async_delay`](crate::async_delay) RTC backing and NuttX `cxd56_rtc.c`.

use core::cell::RefCell;
use core::task::Waker;

use cortex_m::peripheral::NVIC;
use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

use crate::pac;

// The RTC ticks at exactly the embassy tick rate, so `now()` needs no rescaling.
const _: () = assert!(
    embassy_time_driver::TICK_HZ == 32_768,
    "time-driver-rtc requires the app's embassy-time tick rate to be tick-hz-32_768"
);

/// `SETALMPRECNT.ASET_BUSY` — a prior alarm-set is still reflecting.
const ASET_BUSY: u32 = 1 << 16;
/// `ALMOUTEN.ALM_BUSY` — a prior enable is still reflecting.
const ALM_BUSY: u32 = 1 << 8;
/// `ALMOUTEN.ALM_EN` — alarm match enable.
const ALM_EN: u32 = 1 << 0;
/// `ALMOUTEN.ALM_ERREN` — error-output enable: a deadline set in the past raises the
/// IRQ immediately (instead of being lost), so a just-scheduled past timer fires.
const ALM_ERREN: u32 = 1 << 16;
/// `ALMCLR`/`ALMFLG` channel-0 match + error bits.
const ALM0_FLAGS: u32 = (1 << 0) | (1 << 16);

struct RtcDriver {
    queue: Mutex<RefCell<Queue>>,
}

embassy_time_driver::time_driver_impl!(static DRIVER: RtcDriver = RtcDriver {
    queue: Mutex::new(RefCell::new(Queue::new())),
});

#[inline]
fn rtc() -> &'static pac::rtc0::RegisterBlock {
    // SAFETY: RTC0 is the always-on clock peripheral; we issue only single-register
    // reads/writes to its alarm-0 and counter registers.
    unsafe { &*pac::Rtc0::PTR }
}

/// Current monotonic 47-bit RTC tick count (== embassy ticks under `tick-hz-32_768`).
fn count() -> u64 {
    let rtc = rtc();
    // Re-read the high half for a consistent snapshot across a PRECNT wrap.
    loop {
        let hi = rtc.rtpostcnt().read().bits();
        let lo = rtc.rtprecnt().read().bits() & 0x7fff;
        if hi == rtc.rtpostcnt().read().bits() {
            return ((hi as u64) << 15) | lo as u64;
        }
    }
}

impl RtcDriver {
    /// Program alarm 0 to fire at absolute tick `at`, or disarm when the queue is
    /// empty (`u64::MAX`).
    fn set_alarm(&self, _cs: CriticalSection, at: u64) {
        let rtc = rtc();
        if at == u64::MAX {
            // SAFETY: fixed alarm-0 MMIO — clear flags and disable the channel.
            rtc.almclr().write(|w| unsafe { w.bits(ALM0_FLAGS) });
            rtc.almouten0().write(|w| unsafe { w.bits(0) });
            return;
        }
        // SAFETY (all writes): documented alarm-0 compare/flag/enable encodings.
        // Drop stale channel-0 flags, wait for any in-flight set, write both compare
        // halves, then enable so the comparator only starts once both are set.
        rtc.almclr().write(|w| unsafe { w.bits(ALM0_FLAGS) });
        while rtc.setalmprecnt0().read().bits() & ASET_BUSY != 0 {}
        rtc.setalmpostcnt0()
            .write(|w| unsafe { w.bits((at >> 15) as u32) });
        rtc.setalmprecnt0()
            .write(|w| unsafe { w.bits((at & 0x7fff) as u32) });
        while rtc.almouten0().read().bits() & ALM_BUSY != 0 {}
        rtc.almouten0()
            .write(|w| unsafe { w.bits(ALM_EN | ALM_ERREN) });
    }
}

impl Driver for RtcDriver {
    fn now(&self) -> u64 {
        count()
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            // Only reprogram the single channel when the earliest deadline changed.
            if queue.schedule_wake(at, waker) {
                let next = queue.next_expiration(count());
                self.set_alarm(cs, next);
            }
        });
    }
}

/// Open the INTC gate + unmask the NVIC line for the RTC alarm, and clear any stale
/// alarm a boot stage may have left. Idempotent; call once via [`super::init`].
pub(super) fn init() {
    crate::interrupt::enable(pac::Interrupt::RTC0_A0);
    // SAFETY: this HAL masks with critical-section (PRIMASK), not BASEPRI, so
    // unmasking a line cannot escape an in-progress critical section.
    unsafe { NVIC::unmask(pac::Interrupt::RTC0_A0) };
    let rtc = rtc();
    // SAFETY: fixed alarm-0 MMIO.
    rtc.almclr().write(|w| unsafe { w.bits(ALM0_FLAGS) });
    rtc.almouten0().write(|w| unsafe { w.bits(0) });
}

/// RTC alarm interrupt entry point — the app forwards `RTC0_A0` here. Clears the
/// alarm flag, wakes expired timers, and re-arms for the queue's next deadline.
pub fn on_interrupt() {
    critical_section::with(|cs| {
        // SAFETY: fixed alarm-0 MMIO — clear the match/error flags.
        rtc().almclr().write(|w| unsafe { w.bits(ALM0_FLAGS) });
        let mut queue = DRIVER.queue.borrow(cs).borrow_mut();
        let next = queue.next_expiration(count());
        DRIVER.set_alarm(cs, next);
    });
}
