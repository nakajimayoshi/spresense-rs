use super::pm::{self, Perf, PmError};
use super::{Clocks, Crg, pmu};
use fugit::Hertz;

/// A perf-independent clock sample. `Copy` — safe to hold after the
/// originating [`Clock`] is dropped.
#[derive(Copy, Clone)]
pub struct Fixed(pub Hertz<u32>);

impl Fixed {
    pub fn hz(&self) -> Hertz<u32> {
        self.0
    }
}

/// A perf-dependent clock sample. Not `Copy`; no public constructor.
/// Must be borrowed from a live [`Clock`], keeping the `Clock` borrow
/// intact.
pub struct Dyn(Hertz<u32>);

impl Dyn {
    pub fn hz(&self) -> Hertz<u32> {
        self.0
    }
}

/// Owned typed clock snapshot. Consumes the [`Crg`] peripheral.
///
/// Obtained via [`Crg::into_clock`]. While this value (or any peripheral
/// borrowing a [`Dyn`] field from it) is alive, the `Clock` cannot be
/// borrowed mutably, preventing [`request_perf`](Clock::request_perf) from
/// silently invalidating a peripheral's baud/gear configuration.
///
/// Fixed fields are `pub` and `Copy` — they may be held freely. Dynamic
/// fields are only accessible by reference via the accessor methods, which
/// ties their lifetime to `&self`, preserving the dependency chain.
pub struct Clock {
    crg: Crg,
    // Perf-independent — safe to copy out.
    pub xosc: Fixed,
    pub rcosc: Fixed,
    pub rtc: Fixed,
    pub syspll: Fixed,
    pub sys: Fixed,
    pub sys_ahb: Fixed,
    pub sys_apb: Fixed,
    pub sys_sfc: Fixed,
    pub scu: Fixed,
    pub com: Fixed,
    pub pmui2c: Fixed,
    pub hpadc: Fixed,
    pub lpadc: Fixed,
    pub gps_cpu: Fixed,
    pub gps_ahb: Fixed,
    // Perf-dependent — private to prevent move-out that would decouple the
    // borrow from the owning `Clock`. Access via `&self` methods below.
    appsmp: Dyn,
    usb: Dyn,
    sdio: Dyn,
    img_uart: Dyn,
    img_spi: Dyn,
    img_wspi: Dyn,
    img_vsync: Dyn,
}

impl Clock {
    /// Consume `crg` and sample all clocks.
    pub fn new(crg: Crg) -> Self {
        let c = Clocks::sample(crg.cfg);
        Self {
            crg,
            xosc: Fixed(c.xosc),
            rcosc: Fixed(c.rcosc),
            rtc: Fixed(c.rtc),
            syspll: Fixed(c.syspll),
            sys: Fixed(c.sys),
            sys_ahb: Fixed(c.sys_ahb),
            sys_apb: Fixed(c.sys_apb),
            sys_sfc: Fixed(c.sys_sfc),
            scu: Fixed(c.scu),
            com: Fixed(c.com),
            pmui2c: Fixed(c.pmui2c),
            hpadc: Fixed(c.hpadc),
            lpadc: Fixed(c.lpadc),
            gps_cpu: Fixed(c.gps_cpu),
            gps_ahb: Fixed(c.gps_ahb),
            appsmp: Dyn(c.appsmp),
            usb: Dyn(c.usb),
            sdio: Dyn(c.sdio),
            img_uart: Dyn(c.img_uart),
            img_spi: Dyn(c.img_spi),
            img_wspi: Dyn(c.img_wspi),
            img_vsync: Dyn(c.img_vsync),
        }
    }

    /// Request a CPU/bus operating-point change from the SYSIOP loader firmware.
    ///
    /// Drives the ICC `FREQLOCK` → `CLK_CHG_START` / `CLK_CHG_END` handshake
    /// and updates the dynamic clock fields once the new operating point is
    /// stable.
    ///
    /// Operating points (XOSC = 26 MHz, User Manual Table APP-807/808):
    /// - [`Perf::Hp`]: APP CPU ~156 MHz, VDD_CORE = 1.0 V
    /// - [`Perf::Lp`]: APP CPU  ~39 MHz, VDD_CORE = 0.7 V
    ///
    /// This call **blocks** (polls the CPU FIFO) until the SYSIOP confirms the
    /// transition. While any peripheral holds a borrow of a [`Dyn`] field from
    /// this `Clock`, the borrow checker prevents calling `request_perf` (which
    /// requires `&mut self`).
    pub fn request_perf(&mut self, perf: Perf) -> Result<(), PmError> {
        pm::request_perf(perf)?;
        let c = Clocks::sample(self.crg.cfg);
        self.appsmp    = Dyn(c.appsmp);
        self.usb       = Dyn(c.usb);
        self.sdio      = Dyn(c.sdio);
        self.img_uart  = Dyn(c.img_uart);
        self.img_spi   = Dyn(c.img_spi);
        self.img_wspi  = Dyn(c.img_wspi);
        self.img_vsync = Dyn(c.img_vsync);
        Ok(())
    }

    /// Snapshot every readable clock. Cheap; delegates to the owned `Crg`.
    pub fn freeze(&self) -> Clocks {
        self.crg.freeze()
    }

    /// Access the raw PMU sequencer (escape hatch for SCU firmware load etc.).
    pub fn pmu(&mut self) -> pmu::PmuCtl<'_> {
        self.crg.pmu()
    }

    pub fn appsmp(&self) -> &Dyn {
        &self.appsmp
    }
    pub fn usb(&self) -> &Dyn {
        &self.usb
    }
    pub fn sdio(&self) -> &Dyn {
        &self.sdio
    }
    pub fn img_uart(&self) -> &Dyn {
        &self.img_uart
    }
    pub fn img_spi(&self) -> &Dyn {
        &self.img_spi
    }
    pub fn img_wspi(&self) -> &Dyn {
        &self.img_wspi
    }
    pub fn img_vsync(&self) -> &Dyn {
        &self.img_vsync
    }
}
