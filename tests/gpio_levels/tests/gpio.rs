//! On-hardware GPIO level test (Option 2 — the `defmt-test` framework).
//!
//! This is a `harness = false` integration test: build & run it with
//! `cargo test --release --test gpio`. `defmt-test` only emits its entry point
//! under `cfg(test)`, which is why it lives here rather than in `src/`.
//!
//! Wire two JP2 header pins before running:
//!   * D21 / EMMC_DATA3 (`gp_emmc_data3`) → board **1.8V**  → must read **High**
//!   * D20 / EMMC_DATA2 (`gp_emmc_data2`) → board **GND**   → must read **Low**
//!
//! `defmt-test` prints `(n/2) running ...` per test and `all tests passed!` on
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
    use cxd56_hal::gpio::{pins, Input};
    use cxd56_hal::pac;
    use cxd56_hal::uart::{Uart1, UartConfig};

    /// Fixtures shared across tests: the two pins under test, held as inputs.
    struct State {
        high: Input<pac::topreg::GpEmmcData3>, // D21 — wired to 1.8V
        low: Input<pac::topreg::GpEmmcData2>,  // D20 — wired to GND
    }

    #[init]
    fn init() -> State {
        let pac = pac::Peripherals::take().unwrap();
        let clocks = pac.crg.constrain(Config::default()).freeze();

        // Install the defmt-over-UART1 logger before any test logs a frame.
        let uart =
            Uart1::new(pac.uart1, &clocks, UartConfig::default()).expect("uart1 init failed");
        defmt_serial::defmt_serial(crate::SERIAL.init(uart));

        // Reading a pin needs the pad's input buffer enabled (ENZI): `into_input()`
        // only flips DIR on the GP_* register, while ENZI + the pull bits live on
        // the separate IO_* pad register. Configure both pads as floating inputs
        // (PDN=PUN=1 → no pull, ENZI=1 → input buffer on), mirroring the UART/I2C
        // pinmux pattern in cxd56-hal::uart.
        let topreg = pac.topreg;
        topreg.io_emmc_data3().write(|w| {
            w.lowemi()
                .set_bit()
                .pdn()
                .set_bit()
                .pun()
                .set_bit()
                .enzi()
                .set_bit()
        });
        topreg.io_emmc_data2().write(|w| {
            w.lowemi()
                .set_bit()
                .pdn()
                .set_bit()
                .pun()
                .set_bit()
                .enzi()
                .set_bit()
        });

        let pins = pins::Parts::new(topreg);
        State {
            high: pins.gp_emmc_data3.into_input(),
            low: pins.gp_emmc_data2.into_input(),
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
}
