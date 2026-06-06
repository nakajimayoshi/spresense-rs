use super::{Clocks, Crg};
use core::marker::PhantomData;
use fugit::Hertz;

/// Clock source whose rate is independent of the APP operating point.
pub trait FixedClock {
    fn hz(&self) -> Hertz<u32>;
}

/// Clock source whose rate tracks the APP operating point (`request_perf`).
pub trait DynClock {
    fn hz(&self) -> Hertz<u32>;
}

/// A perf-independent clock sample. `Copy` — safe to hold after the
/// originating [`Clock`] is dropped.
#[derive(Copy, Clone)]
pub struct Fixed(pub Hertz<u32>);

impl FixedClock for Fixed {
    fn hz(&self) -> Hertz<u32> {
        self.0
    }
}

/// A perf-dependent clock sample. Not `Copy`; no public constructor.
/// Must be borrowed from a live [`Clock`], keeping the `Crg` borrow intact.
pub struct Dyn(Hertz<u32>);

impl DynClock for Dyn {
    fn hz(&self) -> Hertz<u32> {
        self.0
    }
}

/// Typed clock snapshot that holds [`Crg`] borrowed for `'a`.
///
/// Obtained via [`Crg::clock`] or [`Crg::request_perf`]. While this value
/// (or any peripheral borrowing a [`Dyn`] field from it) is alive, the
/// originating `Crg` cannot be borrowed mutably, preventing `request_perf`
/// from silently invalidating a peripheral's baud/gear configuration.
///
/// Fixed fields are `pub` and `Copy` — they may be held freely. Dynamic
/// fields are only accessible by reference via the accessor methods, which
/// ties their lifetime to `&self`, preserving the dependency chain.
pub struct Clock<'a> {
    _crg: PhantomData<&'a Crg>,
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
    // lifetime from the `Crg` borrow. Access via &self methods below.
    appsmp: Dyn,
    usb: Dyn,
    sdio: Dyn,
    img_uart: Dyn,
    img_spi: Dyn,
    img_wspi: Dyn,
    img_vsync: Dyn,
}

impl<'a> Clock<'a> {
    /// Sample all clocks, tying the result's lifetime to `crg`.
    pub(crate) fn sample(crg: &'a Crg) -> Self {
        let c = Clocks::sample(crg.cfg);
        Self {
            _crg: PhantomData,
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
