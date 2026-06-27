use super::peripheral::{GearError, PeripheralId};
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
/// [`Fixed`] fields are `pub` and `Copy` (no borrow); [`Dyn`] fields are only
/// accessible by reference via the accessor methods, tying their lifetime to
/// `&self` so the borrow checker blocks [`request_perf`](Clock::request_perf)
/// while a peripheral depends on one.
///
/// **Caveat:** several `Fixed` fields — the SYSIOP-tree `syspll`/`sys`/`sys_ahb`/
/// `sys_apb`/`sys_sfc`/`com`/`gps_cpu`/`gps_ahb` (and `pmui2c` when sourced from
/// `sys_apb`) — are nonetheless **perf-dependent** (User Manual SYSIOP-825/826,
/// UART-791/792). They are refreshed on each [`request_perf`](Clock::request_perf)
/// (via the private `resample_dyn`), but a copy taken *before* a perf
/// change goes stale — read them again afterwards / rebuild the peripheral.
pub struct Clock {
    crg: Crg,
    // `Copy` snapshots (no borrow). Perf-INVARIANT: xosc/rcosc/rtc/scu/hpadc/
    // lpadc. Perf-DEPENDENT (refreshed by `request_perf`): syspll/sys/sys_ahb/
    // sys_apb/sys_sfc/com/pmui2c/gps_cpu/gps_ahb.
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
        self.resample_dyn();
        Ok(())
    }

    /// Set the APP-local gear divider for `id`, making its base clock
    /// `appsmp / divisor`, and re-sample the dynamic clock fields.
    ///
    /// Valid for [`PeripheralId::ImgUart`] (UART2), [`PeripheralId::Spi4`],
    /// [`PeripheralId::Spi5`], [`PeripheralId::Usb`], and
    /// [`PeripheralId::Sdio`]; `divisor` must lie in `1..=max` (`0x7f` for
    /// `ImgUart`/`Spi4`, `0xf` for `Spi5`, `0x3` for `Usb`/`Sdio`). The
    /// initial divisors come from [`Config::gear`](super::Config::gear).
    ///
    /// Requires `&mut self`: while any peripheral driver borrows a [`Dyn`]
    /// field from this `Clock` (e.g. a live UART2 or SPI5), the borrow
    /// checker rejects this call — the same protection that keeps
    /// [`request_perf`](Clock::request_perf) from invalidating a driver's
    /// baud/divisor math. Reconstruct the driver afterwards so it computes
    /// its divisors from the new rate.
    ///
    /// # Caveats
    /// `Usb` requires a specific base clock to function and `Sdio`'s divider
    /// bounds the card clock — override their defaults only if you know the
    /// resulting rate is valid.
    pub fn set_gear(&mut self, id: PeripheralId, divisor: u32) -> Result<(), GearError> {
        id.set_gear(divisor)?;
        self.resample_dyn();
        Ok(())
    }

    /// Set an SPI port's gear divider so the resulting SCK frequency is **at
    /// most** `maxfreq`, and re-sample the dynamic clock fields. Valid for
    /// [`PeripheralId::Spi4`] and [`PeripheralId::Spi5`]. Mirrors NuttX's
    /// `cxd56_spi_clock_gear_adjust`.
    ///
    /// Requires `&mut self` — see [`set_gear`](Clock::set_gear) for the
    /// borrow-checker protection this provides.
    pub fn set_spi_gear(&mut self, port: PeripheralId, maxfreq: Hertz<u32>) -> Result<(), GearError> {
        let appsmp = self.appsmp.hz();
        port.set_spi_gear(appsmp, maxfreq)?;
        self.resample_dyn();
        Ok(())
    }

    /// Re-sample the perf-dependent fields after an operation that changes
    /// them (operating-point change, gear rewrite).
    ///
    /// Besides the [`Dyn`] APP-domain clocks, the SYSIOP-tree clocks move with
    /// the voltage mode too — the operating point reconfigures SYSPLL and the
    /// SYS dividers (User Manual SYSIOP-825/826 & UART-791/792: COM 48.75 MHz HP
    /// → 32.5 MHz LP). They are typed [`Fixed`] for ergonomics but are *not*
    /// perf-invariant, so refresh their cached snapshots here; otherwise a
    /// freshly-built COM-bus peripheral (e.g. an `uart_alt` UART1, whose baud
    /// divisor is computed from `self.com`) would use the stale boot rate after
    /// a perf change. The always-on/sensor clocks (`xosc`/`rcosc`/`rtc`/`scu`/
    /// `hpadc`/`lpadc`) are genuinely perf-invariant and need no refresh.
    fn resample_dyn(&mut self) {
        let c = Clocks::sample(self.crg.cfg);
        // Perf-dependent APP-domain clocks.
        self.appsmp    = Dyn(c.appsmp);
        self.usb       = Dyn(c.usb);
        self.sdio      = Dyn(c.sdio);
        self.img_uart  = Dyn(c.img_uart);
        self.img_spi   = Dyn(c.img_spi);
        self.img_wspi  = Dyn(c.img_wspi);
        self.img_vsync = Dyn(c.img_vsync);
        // Perf-dependent SYSIOP-tree clocks (typed Fixed, but they track the
        // operating point — `gps_*` derive from `sys`, `pmui2c` from `sys_apb`).
        self.syspll  = Fixed(c.syspll);
        self.sys     = Fixed(c.sys);
        self.sys_ahb = Fixed(c.sys_ahb);
        self.sys_apb = Fixed(c.sys_apb);
        self.sys_sfc = Fixed(c.sys_sfc);
        self.com     = Fixed(c.com);
        self.pmui2c  = Fixed(c.pmui2c);
        self.gps_cpu = Fixed(c.gps_cpu);
        self.gps_ahb = Fixed(c.gps_ahb);
    }

    /// Refresh the perf-dependent cached fields from live registers after the
    /// SYSIOP changed the operating point **out-of-band** — i.e. through a path
    /// other than [`request_perf`](Clock::request_perf) / [`set_gear`].
    ///
    /// The motivating case is GNSS bring-up: starting the `gnssfw` DSP makes the
    /// loader raise the SYSIOP clock tree (the GPS CPU is clocked from `sys`),
    /// which shifts COM (48.75 MHz HP ↔ 32.5 MHz LP) without any
    /// [`request_perf`] call of ours. A COM-bus peripheral built before that
    /// (an `uart_alt` UART1) is then mis-divided. Call this once the external
    /// change has settled, then rebuild the affected peripheral so it recomputes
    /// its divisors from the refreshed [`com`](Clock::com).
    pub fn resample(&mut self) {
        self.resample_dyn();
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

    /// CPU/AHB base clock — the watchdog (SP805) timer's clock source.
    ///
    /// Derived from the perf-dependent [`appsmp`](Self::appsmp) clock via the
    /// AHB gear ratio (mirrors `cxd56_get_cpu_baseclk`). Returns a `Copy`
    /// [`Hertz`]; callers that need this value to stay valid across an
    /// operating-point change should hold the `Clock` borrow — see
    /// [`watchdog::Watchdog`](crate::watchdog::Watchdog).
    pub fn cpu_baseclk(&self) -> Hertz<u32> {
        Hertz::<u32>::Hz(super::buses::cpu_baseclk_hz(self.appsmp.hz().to_Hz()))
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
