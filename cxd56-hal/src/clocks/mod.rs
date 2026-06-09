//! Clock subsystem.
//!
//! # Architecture
//!
//! The CXD5602 SYSIOP (Cortex-M0+) owns the PLL, root mux, and bus dividers.
//! The User Manual (§3.5.1 p.167, §3.13.3.3 p.974) explicitly states those
//! registers are "controlled by the API — use as RO registers."  User code:
//!
//! - **Reads** clock rates via [`Crg::freeze`] (a re-samplable snapshot).
//! - **Controls** per-peripheral gates, software resets, and the APP-local M/N
//!   "gear" dividers (UART2, USB, SDIO, SPI4/5) directly via the APP-local CRG
//!   at `0x0201_1000`. The gear divider sets a peripheral's base clock to
//!   `appsmp / M`; adjust it with [`PeripheralId::set_gear`] (raw divisor) or
//!   [`PeripheralId::set_spi_gear`] (target a max SPI frequency), and read it
//!   back with [`PeripheralId::gear_divisor`].
//! - **Requests** CPU/bus operating-point changes (HP ≈ 156 MHz / LP ≈ 39 MHz)
//!   from the SYSIOP over the ICC CPU-FIFO protocol via [`Clock::request_perf`].
//!
//! # Entry point
//!
//! ```ignore
//! let dp = pac::Peripherals::take().unwrap();
//! let mut crg = dp.crg.constrain(Config::default());
//! let clocks = crg.freeze();
//! PeripheralId::Uart1.enable()?;
//!
//! // Pick UART2's base clock before the driver brings the port up: appsmp/2
//! // (≈78 MHz at HP) instead of the default appsmp/4 (≈39 MHz).
//! PeripheralId::ImgUart.set_gear(2)?;
//! ```

use crate::pac;
use fugit::Hertz;

pub mod audio;
pub mod buses;
pub mod gate;
pub mod peripheral;
pub mod pm;
pub mod profile;
pub mod pmu;
pub mod reset;
pub mod sources;

pub use audio::{AudMclk, audio_clock_disable, audio_clock_enable};
pub use peripheral::{ClockError, GearError, I2cPort, PeripheralId, SpiPort};
pub use pm::{PmError, Perf};
pub use profile::{Clock, Dyn, Fixed};

/// Board-level clock configuration.
///
/// Only the XOSC frequency is configurable today — the rest of the clock
/// tree is set by boot ROM. Spresense main board uses a 26 MHz crystal.
#[derive(Copy, Clone, Debug)]
pub struct Config {
    /// XOSC frequency. Spresense main = 26 MHz.
    pub xosc: Hertz<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            xosc: Hertz::<u32>::MHz(26),
        }
    }
}

/// Extension trait — implemented for [`pac::Crg`].
pub trait RccExt {
    fn constrain(self, cfg: Config) -> Crg;
}

impl RccExt for pac::Crg {
    fn constrain(self, cfg: Config) -> Crg {
        Crg::new(self, cfg)
    }
}

/// Owns the CRG peripheral. Returned by [`RccExt::constrain`].
pub struct Crg {
    _crg: pac::Crg,
    cfg: Config,
}

impl Crg {
    pub(crate) fn new(crg: pac::Crg, cfg: Config) -> Self {
        sources::init_rcosc_cache();
        let appsmp = buses::appsmp_hz(cfg.xosc.to_Hz());
        pmu::set_appsmp_hz(appsmp);
        Self { _crg: crg, cfg }
    }

    /// Snapshot every readable clock. Cheap; call again after PMU changes
    /// that affect bus dividers.
    pub fn freeze(&self) -> Clocks {
        Clocks::sample(self.cfg)
    }

    /// Consume this `Crg` and return an owned [`Clock`] snapshot.
    ///
    /// Mirrors the `.constrain().freeze()` idiom for the profile-aware path.
    /// The returned [`Clock`] owns the `Crg` and exposes
    /// [`Clock::request_perf`] for runtime operating-point changes.
    pub fn into_clock(self) -> Clock {
        Clock::new(self)
    }

    /// Access the raw PMU sequencer (escape hatch for SCU firmware load etc.).
    pub fn pmu(&mut self) -> pmu::PmuCtl<'_> {
        pmu::PmuCtl::new(self)
    }
}

/// Snapshot of every readable clock on the chip.
///
/// All fields are derived from the current register state — caller can re-call
/// [`Crg::freeze`] after any operation that mutates a divider.
#[derive(Copy, Clone, Debug)]
pub struct Clocks {
    pub xosc: Hertz<u32>,
    pub rcosc: Hertz<u32>,
    pub rtc: Hertz<u32>,
    pub syspll: Hertz<u32>,

    /// SYS/IOP root bus.
    pub sys: Hertz<u32>,
    pub sys_ahb: Hertz<u32>,
    pub sys_apb: Hertz<u32>,
    pub sys_sfc: Hertz<u32>,

    /// APP CPU (Cortex-M4 user core).
    pub appsmp: Hertz<u32>,

    pub scu: Hertz<u32>,
    pub com: Hertz<u32>,
    pub pmui2c: Hertz<u32>,
    pub usb: Hertz<u32>,
    pub sdio: Hertz<u32>,
    pub hpadc: Hertz<u32>,
    pub lpadc: Hertz<u32>,

    /// IMG-block peripherals on APP_SUB.
    pub img_uart: Hertz<u32>,
    pub img_spi: Hertz<u32>,
    pub img_wspi: Hertz<u32>,
    pub img_vsync: Hertz<u32>,

    pub gps_cpu: Hertz<u32>,
    pub gps_ahb: Hertz<u32>,
}

impl Clocks {
    pub(crate) fn sample(cfg: Config) -> Self {
        let xosc = cfg.xosc.to_Hz();
        let rcosc = sources::rcosc_hz();
        let rtc = sources::rtc_hz();
        let syspll = sources::syspll_hz(xosc);
        let sys = buses::sys_hz(rcosc, rtc, syspll, xosc);
        let sys_ahb = buses::sys_ahb_hz(sys);
        let sys_apb = buses::sys_apb_hz(sys_ahb);
        let sys_sfc = buses::sys_sfc_hz(sys);
        let appsmp = buses::appsmp_hz(xosc);
        let scu = buses::scu_hz(rcosc, rtc, xosc);
        let com = buses::com_hz(sys);
        let pmui2c = buses::pmui2c_hz(sys_apb, rtc, rcosc);
        let usb = buses::usb_hz(appsmp);
        let sdio = buses::sdio_hz(appsmp);
        let hpadc = buses::hpadc_hz(rcosc, rtc);
        let lpadc = buses::lpadc_hz(rcosc, rtc);
        let img_uart = buses::img_uart_hz(appsmp);
        let img_spi = buses::img_spi_hz(appsmp);
        let img_wspi = buses::img_wspi_hz(appsmp);
        let img_vsync = buses::img_vsync_hz(appsmp);
        let gps_cpu = buses::gps_cpu_hz(sys);
        let gps_ahb = buses::gps_ahb_hz(sys);

        Self {
            xosc: Hertz::<u32>::Hz(xosc),
            rcosc: Hertz::<u32>::Hz(rcosc),
            rtc: Hertz::<u32>::Hz(rtc),
            syspll: Hertz::<u32>::Hz(syspll),
            sys: Hertz::<u32>::Hz(sys),
            sys_ahb: Hertz::<u32>::Hz(sys_ahb),
            sys_apb: Hertz::<u32>::Hz(sys_apb),
            sys_sfc: Hertz::<u32>::Hz(sys_sfc),
            appsmp: Hertz::<u32>::Hz(appsmp),
            scu: Hertz::<u32>::Hz(scu),
            com: Hertz::<u32>::Hz(com),
            pmui2c: Hertz::<u32>::Hz(pmui2c),
            usb: Hertz::<u32>::Hz(usb),
            sdio: Hertz::<u32>::Hz(sdio),
            hpadc: Hertz::<u32>::Hz(hpadc),
            lpadc: Hertz::<u32>::Hz(lpadc),
            img_uart: Hertz::<u32>::Hz(img_uart),
            img_spi: Hertz::<u32>::Hz(img_spi),
            img_wspi: Hertz::<u32>::Hz(img_wspi),
            img_vsync: Hertz::<u32>::Hz(img_vsync),
            gps_cpu: Hertz::<u32>::Hz(gps_cpu),
            gps_ahb: Hertz::<u32>::Hz(gps_ahb),
        }
    }

    /// SPI port base clock. Mirrors `cxd56_get_spi_baseclock` (cxd56_clock.c:1733).
    pub fn spi(&self, port: SpiPort) -> Hertz<u32> {
        match port {
            SpiPort::Spi0 => self.com,
            SpiPort::Spi3 => self.scu,
            SpiPort::Spi4 => self.img_spi,
            SpiPort::Spi5 => self.img_wspi,
        }
    }

    /// I2C port base clock. Mirrors `cxd56_get_i2c_baseclock` (cxd56_clock.c:1766).
    pub fn i2c(&self, port: I2cPort) -> Hertz<u32> {
        match port {
            I2cPort::I2c0 | I2cPort::I2c1 => self.scu,
            I2cPort::I2c2 => self.com,
            I2cPort::I2c4 => self.pmui2c,
        }
    }
}
