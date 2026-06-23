//! Minimal UART clock-gate enable, ported from `cxd56-hal`'s clock subsystem
//! to the chiptool PAC.
//!
//! This is the *quickest* clock story for a blocking UART: it only gates the
//! peripheral on. The base-clock **frequency** used for the baud divisor is
//! supplied by the caller via [`crate::uart::Config::src_clk_hz`] — there is no
//! clock-tree reader here. For an accurate rate, read it from `cxd56-hal`'s
//! `Clocks` (e.g. `clocks.com` for UART1).
//!
//! - **UART1** (SYSIOP_SUB / COM domain): the COM/AHB bridge plus the
//!   enable→pulse→reset-release→enable dance on `SYSIOP_SUB_CKEN.CK_UART1`
//!   (mirrors `cxd56-hal/src/clocks/peripheral.rs::sysiop_sub_peripheral_enable`).
//! - **UART2** (IMG domain): PMU `AppSub` power-up, the IMG umbrella gate, then
//!   `GEAR_IMG_UART` (mirrors `img_uart_enable`).
//!
//! All functions are idempotent — they early-return if the gate is already on —
//! so the refcounting of the HAL versions is unnecessary here.

use crate::pac;

/// Assumed APP/CPU clock for the PMU `delay_us` kick timing. Matches the boot
/// ROM HP-mode value the HAL caches by default (`pmu.rs::APPSMP_HZ`). Used only
/// for the UART2 PMU power-on poll loop; UART1 needs no timed delays.
const CPU_HZ: u32 = 156_000_000;

// SYSIOP_SUB_CKEN bit layout (cxd56-hal/src/clocks/gate.rs).
const CK_AHB_BRG_COMIF: u32 = 1 << 0;
const CK_COM_BRG: u32 = 1 << 1;
const CK_UART1: u32 = 1 << 3;
/// `TOPREG.SWRESET_BUS` reset-release bit for UART1 (active-low: 1 = released).
const XRST_UART1: u32 = 1 << 5;

// IMG umbrella bits (cxd56-hal/src/clocks/gate.rs).
const CK_GATE_IMG: u32 = 1 << 4;
const XRS_IMG: u32 = 1 << 4;
/// PMU `AppSub` digital power domain (PWD_CTL/PWD_STAT bit 10).
const PWD_APPSUB: u32 = 1 << 10;
/// Default `GEAR_IMG_UART` value: N=1, M=4 → appsmp/4 ≈ 39 MHz (NuttX default).
const GEAR_IMG_UART_DEFAULT: u32 = 0x0001_0004;

/// Spin reading `CHIP_ID` to flush the SYSIOP bus path between gate writes.
/// Mirrors `pmu::busy_wait`.
#[inline]
fn busy_wait(cnt: u32) {
    for _ in 0..cnt {
        let _ = pac::TOPREG_SUB.CHIP_ID().read();
    }
}

#[inline]
fn delay_us(us: u32) {
    let cycles = ((CPU_HZ as u64) * us as u64 / 1_000_000) as u32;
    cortex_m::asm::delay(cycles.max(1));
}

#[inline]
fn sysiop_sub_cken_modify(set: u32, clr: u32) {
    pac::TOPREG_SUB
        .SYSIOP_SUB_CKEN()
        .modify(|w| w.0 = (w.0 & !clr) | set);
}

/// Enable the UART1 (COM domain) clock gate. Idempotent.
///
/// Mirrors `sysiop_sub_peripheral_enable(Uart1, ..)` in
/// `cxd56-hal/src/clocks/peripheral.rs`.
pub(crate) fn uart1_enable() {
    let cken = pac::TOPREG_SUB.SYSIOP_SUB_CKEN().read().0;
    if cken & CK_UART1 != 0 {
        return; // already enabled
    }
    // Acquire the shared COM/AHB bridges.
    sysiop_sub_cken_modify(CK_COM_BRG | CK_AHB_BRG_COMIF, 0);
    // enable → pulse → reset-release → enable.
    sysiop_sub_cken_modify(CK_UART1, 0);
    busy_wait(10);
    sysiop_sub_cken_modify(0, CK_UART1);
    pac::TOPREG.SWRESET_BUS().modify(|w| w.0 |= XRST_UART1);
    sysiop_sub_cken_modify(CK_UART1, 0);
}

/// Power up the PMU `AppSub` digital domain (idempotent). `AppSub` is not in the
/// HAL's `RESET_RELEASE_MASK` (0x3141), so no reset-release step is needed.
/// Mirrors `pmu::enable_domain(AppSub)`.
fn pmu_appsub_enable() {
    if pac::TOPREG.PWD_STAT().read().0 & PWD_APPSUB == PWD_APPSUB {
        return; // already powered
    }
    // PWD_CTL: write the set bit and its matching write-enable bit (mask << 16).
    pac::TOPREG
        .PWD_CTL()
        .write(|w| w.0 = PWD_APPSUB | (PWD_APPSUB << 16));
    // Kick PMU_PW_CTL and poll PWD_STAT until the domain reports powered.
    for _ in 0..20_000 {
        pac::TOPREG.PMU_PW_CTL().write(|w| w.set_POWER_CTRL_ON(true));
        delay_us(100);
        if pac::TOPREG.PWD_STAT().read().0 & PWD_APPSUB == PWD_APPSUB {
            delay_us(400);
            break;
        }
    }
}

/// Enable the IMG umbrella gate (idempotent). Mirrors `gate::img_acquire`.
fn img_acquire() {
    if pac::CRG.CK_GATE_AHB().read().0 & CK_GATE_IMG != 0 {
        return; // umbrella already on
    }
    pac::CRG.CK_GATE_AHB().modify(|w| w.0 |= CK_GATE_IMG);
    // Read-back drains the AHB write; settle before deasserting reset.
    let _ = pac::CRG.RESET().read();
    busy_wait(10);
    pac::CRG.RESET().modify(|w| w.0 |= XRS_IMG);
}

/// Enable the UART2 (IMG domain) clock gate. Idempotent.
///
/// Mirrors `img_uart_enable` in `cxd56-hal/src/clocks/peripheral.rs`.
pub(crate) fn uart2_enable() {
    pmu_appsub_enable();
    img_acquire();
    pac::CRG
        .GEAR_IMG_UART()
        .write(|w| w.0 = GEAR_IMG_UART_DEFAULT);
}
