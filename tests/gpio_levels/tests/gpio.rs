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

use cxd56_hal::uart::Uart1;

// `defmt-test` owns the module body, so keep the logger cell at crate root and
// reach it via `crate::SERIAL`.
static SERIAL: StaticCell<Uart1> = StaticCell::new();

#[defmt_test::tests]
mod tests {
    use defmt::assert;

    use cxd56_hal::clocks::{Config, RccExt};
    use cxd56_hal::gpio::{pins, Input, InterruptInput, Level, Output, Pull, Trigger};
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
    // These are single-threaded, so they never depend on the WFE sleep actually
    // firing: level waits use an already-true level and edge waits pre-latch the
    // edge before calling `wait_*`, so each call's condition is met on entry and
    // it returns at once. They exercise slot config, the PMU edge latch and the
    // clear path — independent of the INTC/NVIC layer (`is_pending` reads the raw
    // PMU latch). Each test sets its own trigger so order does not matter.

    #[test]
    fn wait_for_high_returns_when_high(state: &mut State) {
        state.out.set_high();
        cortex_m::asm::delay(1_000);
        state.sense.wait_for_high(); // pin already high → returns immediately
        assert!(state.sense.is_high(), "wait_for_high should leave D27 High");
    }

    #[test]
    fn wait_for_low_returns_when_low(state: &mut State) {
        state.out.set_low();
        cortex_m::asm::delay(1_000);
        state.sense.wait_for_low(); // pin already low → returns immediately
        assert!(state.sense.is_low(), "wait_for_low should leave D27 Low");
    }

    #[test]
    fn rising_edge_latches_and_waits(state: &mut State) {
        state.out.set_low();
        cortex_m::asm::delay(1_000);
        state.sense.set_trigger(Trigger::RisingEdge);
        state.sense.clear_pending();
        assert!(
            !state.sense.is_pending(),
            "no edge should be latched while D28 holds Low"
        );

        state.out.set_high(); // rising edge on the shorted line
        cortex_m::asm::delay(1_000);
        assert!(
            state.sense.is_pending(),
            "the rising edge should be latched in the PMU"
        );

        state.sense.wait_for_rising_edge(); // already latched → returns at once
        assert!(
            !state.sense.is_pending(),
            "the latch should be cleared after the wait"
        );
    }

    #[test]
    fn falling_edge_latches_and_waits(state: &mut State) {
        state.out.set_high();
        cortex_m::asm::delay(1_000);
        state.sense.set_trigger(Trigger::FallingEdge);
        state.sense.clear_pending();
        assert!(
            !state.sense.is_pending(),
            "no edge should be latched while D28 holds High"
        );

        state.out.set_low(); // falling edge on the shorted line
        cortex_m::asm::delay(1_000);
        assert!(
            state.sense.is_pending(),
            "the falling edge should be latched in the PMU"
        );

        state.sense.wait_for_falling_edge();
        assert!(
            !state.sense.is_pending(),
            "the latch should be cleared after the wait"
        );
    }

    #[test]
    fn any_edge_latches_and_waits(state: &mut State) {
        state.out.set_low();
        cortex_m::asm::delay(1_000);
        state.sense.set_trigger(Trigger::AnyEdge);
        state.sense.clear_pending();
        assert!(!state.sense.is_pending(), "no edge latched on a stable level");

        state.out.set_high(); // a transition (here, rising) under AnyEdge
        cortex_m::asm::delay(1_000);
        assert!(
            state.sense.is_pending(),
            "AnyEdge should latch either transition"
        );

        state.sense.wait_for_any_edge();
        assert!(!state.sense.is_pending(), "latch cleared after the wait");
    }

    #[test]
    fn is_pending_tracks_and_clears(state: &mut State) {
        state.out.set_low();
        cortex_m::asm::delay(1_000);
        state.sense.set_trigger(Trigger::RisingEdge);
        state.sense.clear_pending();
        assert!(!state.sense.is_pending(), "cleared latch reads not-pending");

        state.out.set_high();
        cortex_m::asm::delay(1_000);
        assert!(state.sense.is_pending(), "edge sets the latch");

        state.sense.clear_pending();
        assert!(
            !state.sense.is_pending(),
            "clear_pending re-arms (latch reads not-pending)"
        );
    }
}
