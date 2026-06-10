//! Bus-level clock readback (SYS/AHB/APB/SFC, APPSMP, SCU, COM, peripheral
//! gear dividers).
//!
//! Each function mirrors the equivalent `cxd56_get_*_baseclock` in
//! `nuttx/arch/arm/src/cxd56xx/cxd56_clock.c`. Cited line ranges in the doc
//! comments.

use crate::regs::{crg, topreg};

/// `APP_CKSEL` register address — lives in `topreg_sub` at offset 0x418.
/// `cxd56_clock.c:1636` reads it through `CXD56_TOPREG_APP_CKSEL`.
const APP_CKSEL_ADDR: usize = 0x0410_3418;

/// `GNSS_DIV` register address — `topreg_sub` offset 0x0c28.
const GNSS_DIV_ADDR: usize = 0x0410_3c28;

#[inline]
fn read_app_cksel() -> u32 {
    unsafe { core::ptr::read_volatile(APP_CKSEL_ADDR as *const u32) }
}

#[inline]
fn read_gnss_div() -> u32 {
    unsafe { core::ptr::read_volatile(GNSS_DIV_ADDR as *const u32) }
}

/// SYS/IOP root clock. Mirrors `cxd56_get_sys_baseclock` (cxd56_clock.c:1573).
pub fn sys_hz(rcosc: u32, rtc: u32, syspll: u32, xosc: u32) -> u32 {
    let val = topreg().cksel_root().read().bits();
    match (val >> 22) & 0x3 {
        0 => rcosc,
        1 => {
            let mut div = ((val >> 10) & 0x3) + 1;
            if div == 4 && (val & (1 << 2)) != 0 {
                div = 5;
            }
            syspll / div
        }
        2 => xosc,
        3 => rtc,
        _ => 0,
    }
}

/// AHB-side clock. `cxd56_clock.c:2265`.
pub fn sys_ahb_hz(sys: u32) -> u32 {
    let bus = topreg().ckdiv_cpu_dsp_bus().read().bits();
    let ahb = 1u32 << ((bus >> 16) & 0x7);
    sys / ahb
}

/// APB-side clock. `cxd56_clock.c:2275`.
pub fn sys_apb_hz(sys_ahb: u32) -> u32 {
    let bus = topreg().ckdiv_cpu_dsp_bus().read().bits();
    let apb = 1u32 << ((bus >> 24) & 0x3);
    sys_ahb / apb
}

/// SPI-flash controller clock. `cxd56_clock.c:2285`.
pub fn sys_sfc_hz(sys: u32) -> u32 {
    let bus = topreg().ckdiv_cpu_dsp_bus().read().bits();
    let sfc = (bus >> 28) & 0xf;
    if sfc <= 9 {
        sys / (sfc * 2 + 2)
    } else {
        sys / ((1u32 << (sfc - 10)) * 32)
    }
}

/// APP Cortex-M4 clock. `cxd56_clock.c:1634`. Reads `APP_CKSEL` (topreg_sub
/// offset 0x418) since the PAC doesn't expose individual fields for it.
pub fn appsmp_hz(xosc: u32) -> u32 {
    use super::sources::{rcosc_hz, rtc_hz, syspll_hz};
    let val = read_app_cksel();
    match (val >> 8) & 0x3 {
        0 => rcosc_hz(),
        1 => {
            let mut div = ((val >> 10) & 0x3) + 1;
            if div == 4 && (val & (1 << 7)) != 0 {
                div = 5;
            }
            syspll_hz(xosc) / div
        }
        2 => xosc,
        3 => rtc_hz(),
        _ => 0,
    }
}

/// SCU base clock. `cxd56_clock.c:1606`.
pub fn scu_hz(rcosc: u32, rtc: u32, xosc: u32) -> u32 {
    let val = topreg().cksel_scu().read().bits();
    match val & 0x3 {
        0 => rcosc,
        1 => xosc / (((val >> 8) & 0x3) + 1),
        2 => {
            if val & (1 << 4) != 0 {
                rtc
            } else {
                rcosc / 250
            }
        }
        _ => 0,
    }
}

/// COM-bus clock (SPI0/I2C2/UART1). `cxd56_clock.c:1665`.
pub fn com_hz(sys: u32) -> u32 {
    let val = topreg().ckdiv_com().read().bits();
    sys / ((val & 0x1f) + 1)
}

/// PMU I2C (I2C4) base clock. `cxd56_clock.c:2334`.
pub fn pmui2c_hz(sys_apb: u32, rtc: u32, rcosc: u32) -> u32 {
    let val = topreg().cksel_pmu().read().bits();
    match val & 0x3 {
        0 => sys_apb,
        1 => rtc,
        2 => rcosc,
        _ => 0,
    }
}

/// Generic M/N gear divider, output = appsmp * n / m.
/// Returns 0 if either operand is zero (gating).
#[inline]
fn gear_apply(appsmp: u32, n: u32, m: u32) -> u32 {
    if n != 0 && m != 0 {
        ((appsmp as u64) * n as u64 / m as u64) as u32
    } else {
        0
    }
}

/// USB clock. `cxd56_clock.c:2372`.
pub fn usb_hz(appsmp: u32) -> u32 {
    let val = crg().gear_per_usb().read().bits();
    let n = (val >> 16) & 1;
    let m = val & 0x3;
    gear_apply(appsmp, n, m)
}

/// SDIO clock. `cxd56_clock.c:1673`.
pub fn sdio_hz(appsmp: u32) -> u32 {
    let val = crg().gear_per_sdio().read().bits();
    let n = (val >> 16) & 1;
    let m = val & 0x3;
    gear_apply(appsmp, n, m)
}

/// IMG-SPI clock (SPI4). `cxd56_clock.c:1693`.
pub fn img_spi_hz(appsmp: u32) -> u32 {
    let val = crg().gear_img_spi().read().bits();
    let n = (val >> 16) & 1;
    let m = val & 0x7f;
    gear_apply(appsmp, n, m)
}

/// IMG-WSPI clock (SPI5). `cxd56_clock.c:1713`.
pub fn img_wspi_hz(appsmp: u32) -> u32 {
    let val = crg().gear_img_wspi().read().bits();
    let n = (val >> 16) & 1;
    let m = val & 0xf;
    gear_apply(appsmp, n, m)
}

/// IMG-UART clock (UART2). `cxd56_clock.c:1365`.
pub fn img_uart_hz(appsmp: u32) -> u32 {
    let val = crg().gear_img_uart().read().bits();
    let n = (val >> 16) & 1;
    let m = val & 0x7f;
    gear_apply(appsmp, n, m)
}

/// IMG VSYNC clock. `cxd56_clock.c:2392`.
pub fn img_vsync_hz(appsmp: u32) -> u32 {
    let n = crg().gear_n_img_venb().read().bits();
    let m = crg().gear_m_img_venb().read().bits();
    if n != 0 && m != 0 {
        ((appsmp as u64) * n as u64 / m as u64) as u32
    } else {
        0
    }
}

/// CPU/AHB base clock — the watchdog (SP805) timer clock.
/// Mirrors `cxd56_get_cpu_baseclk` (`cxd56_clock.c:506`): `appsmp * n / m`
/// where `n`/`m` are the AHB gear ratio in `CRG.GEAR_AHB` (offset 0x00).
pub fn cpu_baseclk_hz(appsmp: u32) -> u32 {
    let val = crg().gear_ahb().read().bits();
    let n = (val >> 16) & 0x7f;
    let m = val & 0x7f;
    gear_apply(appsmp, n, m)
}

/// HPADC clock. `cxd56_clock.c:2318`.
pub fn hpadc_hz(rcosc: u32, rtc: u32) -> u32 {
    let scu_src = suc32k_hz(rcosc, rtc);
    let div = topreg().ckdiv_scu().read().bits();
    scu_src / (1u32 << ((div >> 4) & 0xf))
}

/// LPADC clock. `cxd56_clock.c:2326`.
pub fn lpadc_hz(rcosc: u32, rtc: u32) -> u32 {
    let scu_src = suc32k_hz(rcosc, rtc);
    let div = topreg().ckdiv_scu().read().bits();
    scu_src / (1u32 << (div & 0xf))
}

/// SCU 32k source — RCOSC/250 or RTC depending on `CKSEL_SCU[4]`.
/// `cxd56_clock.c:2302`.
fn suc32k_hz(rcosc: u32, rtc: u32) -> u32 {
    let ckscu = topreg().cksel_scu().read().bits();
    if (ckscu >> 4) & 0x1 == 0 {
        rcosc / 250
    } else {
        rtc
    }
}

/// GPS CPU clock. `cxd56_clock.c:2356`.
pub fn gps_cpu_hz(sys: u32) -> u32 {
    let val = read_gnss_div();
    sys / ((val & 0x1f) + 1)
}

/// GPS AHB clock. `cxd56_clock.c:2364`.
pub fn gps_ahb_hz(sys: u32) -> u32 {
    let val = read_gnss_div();
    sys / (((val >> 16) & 0x1f) + 1)
}
