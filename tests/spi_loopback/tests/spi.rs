//! On-hardware SPI5 loopback test (defmt-test framework).
//!
//! This is a `harness = false` integration test: build and run it with
//! `cargo test --release --test spi`. `defmt-test` only emits its entry point
//! under `cfg(test)`, which is why it lives here rather than in `src/`.
//!
//! Two test cases:
//!
//!   [1/2] **internal_lbm** (no wiring): `SSPCR1.LBM = 1` connects TX to RX
//!         inside the PL022 SSP — the EMMC pads are never driven.
//!
//!   [2/2] **external_loopback** (feature-gated): `SSPCR1.LBM = 0`; bytes
//!         travel out MOSI (JP2-9 / D16) and back in via MISO (JP2-8 / D17)
//!         over a jumper wire. Build with `--features external-loopback`.
//!
//! # Wiring (external_loopback only)
//!
//! ```text
//! JP2 pin 9  (MOSI / EMMC_DATA0 / D16)  ─── jumper ───  JP2 pin 8  (MISO / EMMC_DATA1 / D17)
//! ```
//! ⚠️  Both pads are 1.8 V — never connect to 3.3 V or 5 V signals.
//! ⚠️  Do not use with an eMMC/SD add-on attached.

#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt_serial as _;
use panic_probe as _;
use static_cell::StaticCell;

use cxd56_hal::clocks::Clock;
use cxd56_hal::uart::Uart1;

static SERIAL: StaticCell<Uart1> = StaticCell::new();
static CLOCK: StaticCell<Clock> = StaticCell::new();

/// Pattern that exercises all-zeros, all-ones, alternating bits, and ASCII.
const PATTERN: [u8; 8] = [0xA5, 0x5A, 0x00, 0xFF, b'R', b'U', b'S', b'T'];

#[defmt_test::tests]
mod tests {
    use defmt::assert_eq;
    use embedded_hal::spi::SpiBus;

    use cxd56_hal::clocks::{Config, RccExt};
    use cxd56_hal::pac;
    use cxd56_hal::spi_alt::{Spi, SpiConfig};
    use cxd56_hal::uart::{Uart1, UartConfig};

    struct State {
        clock: &'static cxd56_hal::clocks::Clock,
    }

    #[init]
    fn init() -> State {
        let pac = pac::Peripherals::take().unwrap();
        let crg = pac.crg.constrain(Config::default());

        // freeze() borrows crg briefly and returns a Copy Clocks snapshot;
        // the borrow ends here so into_clock() can consume crg below.
        let clocks = crg.freeze();
        let uart =
            Uart1::new(pac.uart1, &clocks, UartConfig::default()).expect("uart1 init failed");
        defmt_serial::defmt_serial(crate::SERIAL.init(uart));

        let clock = crate::CLOCK.init(crg.into_clock());
        State { clock }
    }

    /// [1/2] Internal LBM — SSPCR1.LBM = 1, no external wiring required.
    #[test]
    fn internal_lbm(state: &mut State) {
        let spi5 = unsafe { pac::Peripherals::steal() }.spi5;
        let mut spi =
            Spi::new(spi5, SpiConfig { loopback: true, ..SpiConfig::default() }, state.clock)
                .expect("Spi::new failed");

        let tx = crate::PATTERN;
        let mut rx = crate::PATTERN;
        spi.transfer_in_place(&mut rx).expect("transfer failed");

        for i in 0..tx.len() {
            assert_eq!(rx[i], tx[i], "byte {} mismatch", i);
        }
    }

    /// [2/2] External pad loopback — SSPCR1.LBM = 0, MOSI ↔ MISO jumper required.
    /// Build with `--features external-loopback` and wire JP2-9 ↔ JP2-8.
    #[cfg(feature = "external-loopback")]
    #[test]
    fn external_loopback(state: &mut State) {
        let spi5 = unsafe { pac::Peripherals::steal() }.spi5;
        let mut spi =
            Spi::new(spi5, SpiConfig { loopback: false, ..SpiConfig::default() }, state.clock)
                .expect("Spi::new failed");

        let tx = crate::PATTERN;
        let mut rx = crate::PATTERN;
        spi.transfer_in_place(&mut rx).expect("transfer failed");

        for i in 0..tx.len() {
            assert_eq!(rx[i], tx[i], "byte {} mismatch", i);
        }
    }
}
