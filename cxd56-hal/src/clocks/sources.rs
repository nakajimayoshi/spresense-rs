//! Source-clock readback (XOSC, RCOSC, RTC, SYSPLL).
//!
//! Mirrors `cxd56_get_clock` in `nuttx/arch/arm/src/cxd56xx/cxd56_clock.c:1513-1571`.

use crate::pac;
use core::sync::atomic::{AtomicU32, Ordering};

/// RTC clock — 32.768 kHz, externally supplied.
pub(crate) const RTC_HZ: u32 = 32_768;

/// Calibrated RCOSC frequency. Boot ROM stores the post-calibration value
/// in the first word of BackUp SRAM. Until populated, falls back to the
/// nominal 8.192 MHz.
static RCOSC_HZ: AtomicU32 = AtomicU32::new(8_192_000);

/// BackUp SRAM base address. `BKUP->rcosc_clock` is the first u32.
/// `nuttx/arch/arm/src/cxd56xx/hardware/cxd5602_backupmem.h:41` places the
/// region at 0x04400000. Verify on hardware — if the region is power-gated
/// from APP, this read may need a PMU-mailbox path.
const BKUP_RCOSC_ADDR: usize = 0x0440_0000;

/// Latch the calibrated RCOSC value into the cache. Called once from
/// [`Crg::new`](super::Crg::new).
pub(crate) fn init_rcosc_cache() {
    let raw = unsafe { core::ptr::read_volatile(BKUP_RCOSC_ADDR as *const u32) };
    if raw != 0 {
        RCOSC_HZ.store(raw, Ordering::Relaxed);
    }
}

/// Cached RCOSC frequency in Hz.
pub fn rcosc_hz() -> u32 {
    RCOSC_HZ.load(Ordering::Relaxed)
}

/// RTC frequency in Hz (constant).
pub fn rtc_hz() -> u32 {
    RTC_HZ
}

/// SYSPLL output in Hz.
///
/// Per manual Table 53 (p. 171), output = `xosc * FBDIV / RCDIV` where
/// `SYS_PLL_CTRL2.ISP_LV_SELFBDIV[2:0]` is the FBDIV code (0→10, 1→12, 2→15)
/// and `ISP_LV_SELRCDIV[1:0]` is the RCDIV code (0→1, 1→2, 3→4).
/// Mirrors `cxd56_clock.c:1530-1567`.
pub fn syspll_hz(xosc: u32) -> u32 {
    let ctrl = unsafe { (*pac::Topreg::PTR).sys_pll_ctrl2().read().bits() };
    let rc = match ctrl >> 30 {
        0 => 1,
        1 => 2,
        3 => 4,
        _ => return 0,
    };
    let fb = match (ctrl >> 27) & 0x7 {
        0 => 10,
        1 => 12,
        2 => 15,
        _ => return 0,
    };
    ((xosc as u64) * fb as u64 / rc as u64) as u32
}
