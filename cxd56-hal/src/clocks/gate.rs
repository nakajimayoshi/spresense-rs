//! Shared-bridge refcounting and umbrella clock gates.
//!
//! Two pieces of CXD5602 hardware are shared by multiple peripherals and
//! need software refcounts to know when the last user has left:
//!
//! - **SYSIOP bridges** (`CK_COM_BRG` | `CK_AHB_BRG_COMIF`) — shared by
//!   `Uart1`, `Spim`, `I2cm`. See `cxd56_spim_clock_disable` mask logic at
//!   `cxd56_clock.c:942-946`.
//! - **IMG block umbrella** (`CK_GATE_IMG` in `CK_GATE_AHB`, plus `XRS_IMG`
//!   in `RESET`) — shared by SPI4, SPI5, UART2, GE2D, CISIF, VSYNC encode.
//!   See `cxd56_img_clock_enable/disable` at `cxd56_clock.c:1797-1828`.

use crate::regs::{crg, topreg_sub};
use core::sync::atomic::{AtomicU32, Ordering};

// ---------- IMG umbrella ----------

/// Flags identifying the IMG-block clients. Mirrors `FLAG_IMG_*` in
/// `cxd56_clock.c`.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ImgClient {
    Cisif = 1 << 0,
    Ge2d = 1 << 1,
    Spi4 = 1 << 2,
    Spi5 = 1 << 3,
    Uart2 = 1 << 4,
    Vsync = 1 << 5,
}

static IMG_ACTIVE: AtomicU32 = AtomicU32::new(0);

const CK_GATE_IMG_BIT: u32 = 1 << 4;
const XRS_IMG_BIT: u32 = 1 << 4;

/// Enable the IMG umbrella for `client`. First caller flips the gate on.
pub(crate) fn img_acquire(client: ImgClient) {
    let prev = IMG_ACTIVE.fetch_or(client as u32, Ordering::Relaxed);
    if prev == 0 {
        let g = crg().ck_gate_ahb().read().bits();
        crg()
            .ck_gate_ahb()
            .write(|w| unsafe { w.bits(g | CK_GATE_IMG_BIT) });
        // Read-back to drain the AHB write and let the gate settle before
        // deasserting reset. Mirrors cxd56_clock.c:1808.
        let _ = crg().reset().read().bits();
        super::pmu::busy_wait(10);
        let r = crg().reset().read().bits();
        crg().reset().write(|w| unsafe { w.bits(r | XRS_IMG_BIT) });
    }
}

/// Release the IMG umbrella for `client`. Last caller flips the gate off.
pub(crate) fn img_release(client: ImgClient) {
    let prev = IMG_ACTIVE.fetch_and(!(client as u32), Ordering::Relaxed);
    let next = prev & !(client as u32);
    if next == 0 {
        let g = crg().ck_gate_ahb().read().bits();
        crg()
            .ck_gate_ahb()
            .write(|w| unsafe { w.bits(g & !CK_GATE_IMG_BIT) });
    }
}

// ---------- SYSIOP_SUB bridges ----------

/// Identifies the SYSIOP_SUB client requesting/releasing the COM bridges.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum SysiopBridgeClient {
    Uart1 = 1 << 0,
    Spim = 1 << 1,
    I2cm = 1 << 2,
}

static SYSIOP_BRIDGE_REFS: AtomicU32 = AtomicU32::new(0);

const CK_AHB_BRG_COMIF: u32 = 1 << 0;
const CK_COM_BRG: u32 = 1 << 1;

#[inline]
fn modify_sysiop_sub_cken(set: u32, clr: u32) {
    let v = topreg_sub().sysiop_sub_cken().read().bits();
    let new = (v & !clr) | set;
    topreg_sub()
        .sysiop_sub_cken()
        .write(|w| unsafe { w.bits(new) });
}

pub(crate) fn sysiop_bridge_acquire(client: SysiopBridgeClient) {
    let prev = SYSIOP_BRIDGE_REFS.fetch_or(client as u32, Ordering::Relaxed);
    if prev == 0 {
        modify_sysiop_sub_cken(CK_COM_BRG | CK_AHB_BRG_COMIF, 0);
    }
}

pub(crate) fn sysiop_bridge_release(client: SysiopBridgeClient) {
    let prev = SYSIOP_BRIDGE_REFS.fetch_and(!(client as u32), Ordering::Relaxed);
    let next = prev & !(client as u32);
    if next == 0 {
        modify_sysiop_sub_cken(0, CK_COM_BRG | CK_AHB_BRG_COMIF);
    }
}

/// Set/clear arbitrary bits in `SYSIOP_SUB_CKEN`. Re-exported for use by
/// peripheral.rs when toggling individual UART1/SPIM/I2CM gate bits.
pub(crate) fn sysiop_sub_cken_modify(set: u32, clr: u32) {
    modify_sysiop_sub_cken(set, clr);
}

/// Bit masks within `SYSIOP_SUB_CKEN` for the per-peripheral gates.
pub(crate) mod sysiop_sub_bits {
    pub const CK_UART1: u32 = 1 << 3;
    pub const CK_SPIM: u32 = 1 << 4;
    pub const CK_I2CM: u32 = 1 << 5;
}
