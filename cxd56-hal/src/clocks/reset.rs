//! Software-reset helpers.
//!
//! CXD5602 resets are *active-low*: the bit named `XRST_*` is `0` while the
//! peripheral is held in reset, `1` once released. We expose two small
//! primitives — `pulse` (assert then release) and `set` (raw bit-set / -clear) —
//! that the per-peripheral enable sequences in [`super::peripheral`] use.

use crate::regs::{crg, topreg};

/// Which CRG software-reset register to touch.
#[derive(Copy, Clone, Debug)]
pub enum ResetReg {
    /// `CRG.RESET` — APP-domain blocks (img, usb, sdio, mmc, dsp, ...).
    Crg,
    /// `TOPREG.SWRESET_BUS` — SYSIOP-domain blocks (uart1, spim, i2cm, ...).
    SwresetBus,
    /// `TOPREG.SWRESET_SCU` — SCU peripherals (i2c, spi, adc, ...).
    SwresetScu,
}

#[inline]
fn read(reg: ResetReg) -> u32 {
    match reg {
        ResetReg::Crg => crg().reset().read().bits(),
        ResetReg::SwresetBus => topreg().swreset_bus().read().bits(),
        ResetReg::SwresetScu => topreg().swreset_scu().read().bits(),
    }
}

#[inline]
fn write(reg: ResetReg, val: u32) {
    match reg {
        ResetReg::Crg => crg().reset().write(|w| unsafe { w.bits(val) }),
        ResetReg::SwresetBus => topreg().swreset_bus().write(|w| unsafe { w.bits(val) }),
        ResetReg::SwresetScu => topreg().swreset_scu().write(|w| unsafe { w.bits(val) }),
    };
}

/// Pulse the named reset bit: assert (clear), busy-wait, release (set).
pub(crate) fn pulse(reg: ResetReg, mask: u32) {
    let v = read(reg);
    write(reg, v & !mask);
    super::pmu::busy_wait(10);
    write(reg, v | mask);
}

/// Hold the named reset bit asserted (active-low ⇒ clear bit).
pub(crate) fn assert_reset(reg: ResetReg, mask: u32) {
    let v = read(reg);
    write(reg, v & !mask);
}

/// Release reset (set bit).
pub(crate) fn release(reg: ResetReg, mask: u32) {
    let v = read(reg);
    write(reg, v | mask);
}
