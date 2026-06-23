//! On-hardware GPIO level test (Option 2 — the `defmt-test` framework).
//!
//! This is a `harness = false` integration test: build & run it with
//! `cargo test --release --test gpio`. `defmt-test` only emits its entry point
//! under `cfg(test)`, which is why it lives here rather than in `src/`.
//!
//! Wire the header pins before running:
//!   * D21 / EMMC_DATA3 (`gp_emmc_data3`) → board **1.8V**  → must read **High**
//!   * D20 / EMMC_DATA2 (`gp_emmc_data2`) → board **GND**   → must read **Low**
//!   * D28 / UART2_RTS (`gp_uart2_rts`) ↔ D27 / UART2_CTS (`gp_uart2_cts`) on JP1:
//!     short the two pins together — D28 drives the line, D27 reads it back, so
//!     driving D28 High/Low must make D27 read High/Low.
//!   * D22 / SEN_IRQ (`gp_sen_irq_in`, JP1 pin 12) → leave **unconnected**:
//!     the internal pull-up/pull-down tests drive this pin from its own pad
//!     resistors, so any external connection (or an add-on board on JP1) would
//!     fight the weak pull and is not allowed.
//!
//! `defmt-test` prints `(n/12) running ...` per test and `all tests passed!` on
//! success, which `cargo-spresense-flash --test` matches for the verdict. (On
//! bare metal defmt-test's final semihosting exit faults harmlessly *after* that
//! line has already been clocked out of the UART, so the host still sees it.)
//!
//! CXD5602 GPIO is 1.8 V — wire to the board's 1.8V rail only, never 3.3/5 V.

#![no_std]
#![no_main]

use cortex_m_rt as _; // reset handler that calls the defmt-test-generated `main`
use defmt_serial as _;
use panic_probe as _;
use static_cell::StaticCell;

use cxd56_hal::gpio;
use cxd56_hal::pac::{Interrupt, interrupt};
use cxd56_hal::uart::Uart1;

// `defmt-test` owns the module body, so keep the logger cell at crate root and
// reach it via `crate::SERIAL`.
static SERIAL: StaticCell<Uart1> = StaticCell::new();

/// D27 (CTS, pin 69) → first free APP slot 6 → `EXDEVICE_6`. Forward the vector to
/// the HAL so the async `wait_for_*` futures wake.
#[interrupt]
fn EXDEVICE_6() {
    gpio::on_interrupt(Interrupt::EXDEVICE_6);
}

/// Poll the PMU edge latch *continuously* for up to ~1M iterations, returning
/// whether the edge was caught.
///
/// The latch is brief and phase-dependent: edge detection samples the pin on the
/// ~32 kHz PMU clock ("two consecutive sampling" per the user manual), so the
/// latch appears ~7–15k CPU cycles after the edge and can self-clear ~1–2 RTC
/// periods later. Sampling `is_pending` once after a fixed delay therefore races
/// the latch; a tight poll (the same thing `wait_for_*edge` does via WFE) reliably
/// catches it. Measured ~7.7k cycles to appear, so 1M iterations is ample margin.
fn wait_pending(
    sense: &cxd56_hal::gpio::InterruptInput<cxd56_hal::pac::topreg::GpUart2Cts>,
) -> bool {
    for _ in 0..1_000_000 {
        if sense.is_pending() {
            return true;
        }
    }
    false
}

/// Minimal in-file async runtime (`block_on` sleeps the core in `WFE`; `join` /
/// `yield_now` combinators) used to drive the async `wait_for_*` futures from the
/// synchronous `defmt-test` bodies. No executor crate — `cxd56-hal` implements the
/// standard `embedded-hal-async` `Wait` trait, so an Embassy executor would work too.
mod rt {
    use core::future::{Future, poll_fn};
    use core::pin::pin;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop_noop);
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(core::ptr::null(), &VTABLE)
    }
    fn wake(_: *const ()) {
        cortex_m::asm::sev();
    }
    fn drop_noop(_: *const ()) {}

    fn make_waker() -> Waker {
        // Safety: the vtable's functions ignore the data pointer entirely.
        unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) }
    }

    /// Drive `fut` to completion, sleeping in `WFE` between polls.
    pub fn block_on<F: Future>(fut: F) -> F::Output {
        let mut fut = pin!(fut);
        let waker = make_waker();
        let mut cx = Context::from_waker(&waker);
        loop {
            if let Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                return val;
            }
            cortex_m::asm::wfe();
        }
    }

    /// Yield once so a sibling future can run before we re-poll.
    pub async fn yield_now() {
        let mut yielded = false;
        poll_fn(|cx| {
            if yielded {
                Poll::Ready(())
            } else {
                yielded = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await
    }

    /// Poll two futures concurrently until both complete.
    pub async fn join<A: Future, B: Future>(a: A, b: B) -> (A::Output, B::Output) {
        let mut a = pin!(a);
        let mut b = pin!(b);
        let mut ao: Option<A::Output> = None;
        let mut bo: Option<B::Output> = None;
        poll_fn(|cx| {
            if ao.is_none() {
                if let Poll::Ready(v) = a.as_mut().poll(cx) {
                    ao = Some(v);
                }
            }
            if bo.is_none() {
                if let Poll::Ready(v) = b.as_mut().poll(cx) {
                    bo = Some(v);
                }
            }
            if ao.is_some() && bo.is_some() {
                Poll::Ready((ao.take().unwrap(), bo.take().unwrap()))
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

#[defmt_test::tests]
mod tests {
    use defmt::assert;

    use cxd56_hal::clocks::{Config, RccExt};
    use cxd56_hal::gpio::{pins, Input, InterruptInput, Level, Output, Pull, Trigger, Wait};
    use cxd56_hal::pac;
    use cxd56_hal::uart::{Uart1, UartConfig};

    /// Fixtures shared across tests.
    struct State {
        high: Input<pac::topreg::GpEmmcData3>, // D21 — wired to 1.8V
        low: Input<pac::topreg::GpEmmcData2>,  // D20 — wired to GND
        // Loopback pair on JP1, D27 ↔ D28 shorted together:
        out: Output<pac::topreg::GpUart2Rts>, // D28 — drives the line
        // D27 — floating, reads the line and is wired to an EXDEVICE interrupt.
        sense: InterruptInput<pac::topreg::GpUart2Cts>,
        // D22 — unconnected; driven only by its own internal pull resistors.
        pull: Input<pac::topreg::GpSenIrqIn>,
    }

    #[init]
    fn init() -> State {
        let pac = pac::Peripherals::take().unwrap();
        let clocks = pac.crg.constrain(Config::default()).freeze();

        // Install the defmt-over-UART1 logger before any test logs a frame.
        let uart =
            Uart1::new(pac.uart1, &clocks, UartConfig::default()).expect("uart1 init failed");
        defmt_serial::defmt_serial(crate::SERIAL.init(uart));

        // `into_floating_input()` now enables the pad input buffer (ENZI) and
        // sets the pull, so the inputs need no manual IO_* writes here. The
        // loopback output pad (D28) needs none either — `into_output()` drops
        // DIR to 0, enabling its driver. D22 starts floating; the pull tests
        // switch it to pull-up / pull-down at runtime via `set_pull`.
        let pins = pins::Parts::new(pac.topreg);
        State {
            high: pins.gp_emmc_data3.into_floating_input(),
            low: pins.gp_emmc_data2.into_floating_input(),
            out: pins.gp_uart2_rts.into_output(Level::Low),
            // D27 is pin 69 (APP domain) → first free APP slot 6 → EXDEVICE_6.
            sense: pins
                .gp_uart2_cts
                .into_floating_input()
                .into_interrupt(Trigger::RisingEdge, false)
                .expect("no free EXDEVICE slot"),
            pull: pins.gp_sen_irq_in.into_floating_input(),
        }
    }

    #[test]
    fn data3_high_reads_high(state: &mut State) {
        assert!(
            state.high.is_high(),
            "D21 (EMMC_DATA3) tied to 1.8V should read High"
        );
    }

    #[test]
    fn data2_low_reads_low(state: &mut State) {
        assert!(
            state.low.is_low(),
            "D20 (EMMC_DATA2) tied to GND should read Low"
        );
    }

    #[test]
    fn output_high_reads_high(state: &mut State) {
        state.out.set_high();
        cortex_m::asm::delay(1_000); // let the pad/line settle before sampling
        assert!(
            state.sense.is_high(),
            "D28 (UART2_RTS) driven High should make shorted D27 (UART2_CTS) read High"
        );
    }

    #[test]
    fn output_low_reads_low(state: &mut State) {
        state.out.set_low();
        cortex_m::asm::delay(1_000);
        assert!(
            state.sense.is_low(),
            "D28 (UART2_RTS) driven Low should make shorted D27 (UART2_CTS) read Low"
        );
    }

    // The internal-pull tests drive the unconnected D22 pin from its own pad
    // resistor. A weak (~100 kΩ) pull into the pin/trace capacitance settles far
    // slower than the actively driven loopback above, so wait ~100k cycles
    // (≈0.6 ms at 156 MHz, ≫ 5·RC) before sampling. Running pull-up then
    // pull-down means the second test swings the full rail through the resistor —
    // the worst case this delay must cover.
    #[test]
    fn pull_up_reads_high(state: &mut State) {
        state.pull.set_pull(Pull::Up);
        cortex_m::asm::delay(100_000);
        assert!(
            state.pull.is_high(),
            "D22 (SEN_IRQ, unconnected) with internal pull-up should read High"
        );
    }

    #[test]
    fn pull_down_reads_low(state: &mut State) {
        state.pull.set_pull(Pull::Down);
        cortex_m::asm::delay(100_000);
        assert!(
            state.pull.is_low(),
            "D22 (SEN_IRQ, unconnected) with internal pull-down should read Low"
        );
    }

    // --- EXDEVICE interrupt tests (same D27↔D28 jumper) ---
    //
    // The async `wait_for_*` futures are interrupt-driven: they unmask the NVIC line
    // and sleep until the `EXDEVICE_6` handler (forwarding to `gpio::on_interrupt`)
    // masks the line and wakes the task. The NVIC pending bit captures the edge in
    // hardware, so the brief, phase-dependent PMU latch is never polled. The edge
    // tests `join` the wait with a task that drives the loopback edge *after* the
    // wait has armed (a `yield_now` lets it poll first); reaching the end means the
    // ISR woke the future, a miss would hang → harness timeout. Level waits use an
    // already-true level and return at once. `is_pending_tracks_and_clears` still
    // covers the polled latch directly via [`wait_pending`]. Each test sets its own
    // trigger, so order does not matter.

    #[test]
    fn wait_for_high_returns_when_high(state: &mut State) {
        state.out.set_high();
        cortex_m::asm::delay(1_000);
        let _ = crate::rt::block_on(state.sense.wait_for_high()); // already high → at once
        assert!(state.sense.is_high(), "wait_for_high should leave D27 High");
    }

    #[test]
    fn wait_for_low_returns_when_low(state: &mut State) {
        state.out.set_low();
        cortex_m::asm::delay(1_000);
        let _ = crate::rt::block_on(state.sense.wait_for_low()); // already low → at once
        assert!(state.sense.is_low(), "wait_for_low should leave D27 Low");
    }

    #[test]
    fn rising_edge_latches_and_waits(state: &mut State) {
        // Hold the low baseline so the PMU detector samples it, then drive the
        // rising edge once `wait_for_rising_edge` has armed. Reaching the end means
        // the ISR woke the future; a miss would hang → harness timeout = FAIL.
        state.out.set_low();
        cortex_m::asm::delay(400_000);

        let sense = &mut state.sense;
        let out = &mut state.out;
        let _ = crate::rt::block_on(crate::rt::join(
            sense.wait_for_rising_edge(),
            async move {
                crate::rt::yield_now().await;
                out.set_high(); // rising edge on the shorted line
            },
        ));
    }

    #[test]
    fn falling_edge_latches_and_waits(state: &mut State) {
        state.out.set_high();
        cortex_m::asm::delay(400_000); // hold the high baseline

        let sense = &mut state.sense;
        let out = &mut state.out;
        let _ = crate::rt::block_on(crate::rt::join(
            sense.wait_for_falling_edge(), // switches trigger → settle samples high
            async move {
                crate::rt::yield_now().await;
                out.set_low(); // falling edge on the shorted line
            },
        ));
    }

    #[test]
    fn any_edge_latches_and_waits(state: &mut State) {
        state.out.set_low();
        cortex_m::asm::delay(400_000); // hold the low baseline

        let sense = &mut state.sense;
        let out = &mut state.out;
        let _ = crate::rt::block_on(crate::rt::join(
            sense.wait_for_any_edge(), // switches trigger → settle samples low
            async move {
                crate::rt::yield_now().await;
                out.set_high(); // a transition (here, rising) under AnyEdge
            },
        ));
    }

    #[test]
    fn is_pending_tracks_and_clears(state: &mut State) {
        // Exercises the raw `is_pending` query directly. `clear_pending` leaves it
        // not-pending (no live edge), and a continuous poll catches the brief latch
        // a real edge sets.
        state.out.set_low();
        cortex_m::asm::delay(2_000);
        state.sense.set_trigger(Trigger::RisingEdge);
        state.sense.clear_pending();
        assert!(!state.sense.is_pending(), "cleared latch reads not-pending");

        state.out.set_high(); // rising edge
        assert!(
            crate::wait_pending(&state.sense),
            "is_pending should catch the latched edge"
        );
    }
}
