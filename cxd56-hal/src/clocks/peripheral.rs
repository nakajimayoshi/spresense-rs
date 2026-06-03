//! Per-peripheral clock enable/disable + SPI gear adjust.
//!
//! Each [`PeripheralId`] dispatches to a bespoke sequence — there is no
//! uniform shape across the CXD5602's clock subsystem, so we keep the
//! per-peripheral logic explicit and short.

use crate::regs::{crg, topreg, topreg_sub};
use fugit::Hertz;

use super::gate::{self, ImgClient, SysiopBridgeClient, sysiop_sub_bits};
use super::pmu::{self, PmuDomain};
use super::reset::{self, ResetReg};

/// Errors surfaced from clock-enable/disable.
#[derive(Debug)]
pub enum ClockError {
    /// PMU power-domain never asserted/de-asserted within 20000 polls.
    PmuTimeout,
    /// This [`PeripheralId`] is not implemented in v1 of the HAL.
    Unimplemented,
}

/// Errors from [`PeripheralId::set_spi_gear`].
#[derive(Debug)]
pub enum GearError {
    /// Called on a peripheral that doesn't have a gear divider.
    NotASpi,
    /// `maxfreq` was zero.
    ZeroFreq,
    /// `appsmp` parent clock is zero (clock tree mis-sampled).
    ParentClockZero,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpiPort {
    Spi0,
    Spi3,
    Spi4,
    Spi5,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum I2cPort {
    I2c0,
    I2c1,
    I2c2,
    I2c4,
}

/// All clock-controlled peripherals on the CXD5602.
///
/// Variants marked `Unimplemented` return `ClockError::Unimplemented` until
/// a peripheral driver in this crate needs them. The PMU/gate/reset
/// infrastructure is in place — adding a variant is just writing the
/// per-peripheral sequence from `cxd56_clock.c`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PeripheralId {
    // APP_SUB / IMG block
    Spi4,
    Spi5,
    ImgUart, // UART2
    Usb,
    Sdio,
    Emmc,
    ImgCisif,
    ImgGe2d,

    // SYSIOP_SUB
    Uart1,
    Spim, // SPI0
    I2cm, // I2C2

    // SCU
    Scu,
    ScuSeq,
    I2c0,
    I2c1,
    HpAdc,
    LpAdc,

    // Other
    Audio,
    Udmac,
    Spif,
    Hostif,
    Hostseq,
    GnssRam,
}

impl PeripheralId {
    /// Enable the clock(s), bring up the PMU domain, and release reset.
    pub fn enable(self) -> Result<(), ClockError> {
        match self {
            PeripheralId::Spi4 => spi4_enable(),
            PeripheralId::Spi5 => spi5_enable(),
            PeripheralId::ImgUart => img_uart_enable(),
            PeripheralId::Usb => usb_enable(),
            PeripheralId::Sdio => sdio_enable(),
            PeripheralId::Uart1 => uart1_enable(),
            PeripheralId::Spim => spim_enable(),
            PeripheralId::I2cm => i2cm_enable(),
            PeripheralId::I2c0 => scu_i2c0_enable(),
            _ => Err(ClockError::Unimplemented),
        }
    }

    /// Tear down — reverses [`enable`](Self::enable). Idempotent.
    pub fn disable(self) -> Result<(), ClockError> {
        match self {
            PeripheralId::Spi4 => spi4_disable(),
            PeripheralId::Spi5 => spi5_disable(),
            PeripheralId::ImgUart => img_uart_disable(),
            PeripheralId::Usb => usb_disable(),
            PeripheralId::Sdio => sdio_disable(),
            PeripheralId::Uart1 => uart1_disable(),
            PeripheralId::Spim => spim_disable(),
            PeripheralId::I2cm => i2cm_disable(),
            PeripheralId::I2c0 => scu_i2c0_disable(),
            _ => Err(ClockError::Unimplemented),
        }
    }

    /// Pulse the peripheral's software-reset line. Useful after a firmware
    /// load (SCU sequencer) or to recover from a wedged peripheral.
    pub fn reset(self) {
        match self {
            PeripheralId::Spi4
            | PeripheralId::Spi5
            | PeripheralId::ImgUart
            | PeripheralId::ImgCisif
            | PeripheralId::ImgGe2d => {
                reset::pulse(ResetReg::Crg, 1 << 4); // XRS_IMG
            }
            PeripheralId::Usb => reset::pulse(ResetReg::Crg, 1 << 8),
            PeripheralId::Sdio => reset::pulse(ResetReg::Crg, 1 << 9),
            PeripheralId::Emmc => reset::pulse(ResetReg::Crg, 1 << 10),
            PeripheralId::Uart1 => reset::pulse(ResetReg::SwresetBus, 1 << 5),
            PeripheralId::Spim => reset::pulse(ResetReg::SwresetBus, 1 << 0),
            PeripheralId::I2cm => reset::pulse(ResetReg::SwresetBus, 1 << 11),
            _ => {}
        }
    }

    /// Set a SPI port's clock gear so the resulting frequency is **at most**
    /// `maxfreq`. Valid for [`PeripheralId::Spi4`] and [`PeripheralId::Spi5`].
    /// Mirrors `cxd56_spi_clock_gear_adjust` (cxd56_clock.c:1133).
    pub fn set_spi_gear(self, appsmp: Hertz<u32>, maxfreq: Hertz<u32>) -> Result<(), GearError> {
        if maxfreq.to_Hz() == 0 {
            return Err(GearError::ZeroFreq);
        }
        let baseclk = appsmp.to_Hz();
        if baseclk == 0 {
            return Err(GearError::ParentClockZero);
        }
        let (max_divisor, write_gear): (u32, fn(u32)) = match self {
            PeripheralId::Spi4 => (0x7f, |gear| {
                crg().gear_img_spi().write(|w| unsafe { w.bits(gear) });
            }),
            PeripheralId::Spi5 => (0xf, |gear| {
                crg().gear_img_wspi().write(|w| unsafe { w.bits(gear) });
            }),
            _ => return Err(GearError::NotASpi),
        };
        // Ceiling-divide by (2 * maxfreq) — SPI hardware does an internal /2.
        let denom = maxfreq.to_Hz().saturating_mul(2);
        let divisor = baseclk.div_ceil(denom).min(max_divisor);
        write_gear(0x0001_0000 | divisor);
        Ok(())
    }
}

// ============================================================================
// APP_SUB / IMG-block peripherals
// ============================================================================

fn spi4_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    gate::img_acquire(ImgClient::Spi4);
    crg()
        .gear_img_spi()
        .write(|w| unsafe { w.bits(0x0001_0002) });
    Ok(())
}

fn spi4_disable() -> Result<(), ClockError> {
    crg().gear_img_spi().write(|w| unsafe { w.bits(0) });
    gate::img_release(ImgClient::Spi4);
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

fn spi5_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    gate::img_acquire(ImgClient::Spi5);
    crg()
        .gear_img_wspi()
        .write(|w| unsafe { w.bits(0x0001_0004) });
    Ok(())
}

fn spi5_disable() -> Result<(), ClockError> {
    crg().gear_img_wspi().write(|w| unsafe { w.bits(0) });
    gate::img_release(ImgClient::Spi5);
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

fn img_uart_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    gate::img_acquire(ImgClient::Uart2);
    // Default M=4 (apportions 156 MHz/4 ≈ 39 MHz, matches NuttX default).
    crg()
        .gear_img_uart()
        .write(|w| unsafe { w.bits(0x0001_0004) });
    Ok(())
}

fn img_uart_disable() -> Result<(), ClockError> {
    crg().gear_img_uart().write(|w| unsafe { w.bits(0) });
    gate::img_release(ImgClient::Uart2);
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

fn usb_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    let c = crg().ck_gate_ahb().read().bits();
    if c & (1 << 8) != 0 {
        return Ok(());
    }
    // Assert XRS_USB (=0), set CK_GATE_USB, wait, release reset, enable
    // USBPHY, set gear. Mirrors cxd56_clock.c:631-650.
    let r = crg().reset().read().bits();
    crg().reset().write(|w| unsafe { w.bits(r & !(1 << 8)) });
    crg()
        .ck_gate_ahb()
        .write(|w| unsafe { w.bits(c | (1 << 8)) });
    pmu::busy_wait(10);
    crg().reset().write(|w| unsafe { w.bits(r | (1 << 8)) });
    topreg().usbphy_cken().write(|w| unsafe { w.bits(1) });
    crg()
        .gear_per_usb()
        .write(|w| unsafe { w.bits(0x0001_0002) });
    Ok(())
}

fn usb_disable() -> Result<(), ClockError> {
    let c = crg().ck_gate_ahb().read().bits();
    if c & (1 << 8) != 0 {
        crg().gear_per_usb().write(|w| unsafe { w.bits(0) });
        topreg().usbphy_cken().write(|w| unsafe { w.bits(0) });
        crg()
            .ck_gate_ahb()
            .write(|w| unsafe { w.bits(c & !(1 << 8)) });
        let r = crg().reset().read().bits();
        crg().reset().write(|w| unsafe { w.bits(r & !(1 << 8)) });
    }
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

fn sdio_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    let c = crg().ck_gate_ahb().read().bits();
    if c & (1 << 9) != 0 {
        return Ok(());
    }
    let r = crg().reset().read().bits();
    crg().reset().write(|w| unsafe { w.bits(r & !(1 << 9)) });
    crg()
        .ck_gate_ahb()
        .write(|w| unsafe { w.bits(c | (1 << 9)) });
    crg()
        .gear_per_sdio()
        .write(|w| unsafe { w.bits(0x0001_0002) });
    pmu::busy_wait(10);
    crg().reset().write(|w| unsafe { w.bits(r | (1 << 9)) });
    Ok(())
}

fn sdio_disable() -> Result<(), ClockError> {
    let c = crg().ck_gate_ahb().read().bits();
    if c & (1 << 9) != 0 {
        crg().gear_per_sdio().write(|w| unsafe { w.bits(0) });
        crg()
            .ck_gate_ahb()
            .write(|w| unsafe { w.bits(c & !(1 << 9)) });
        let r = crg().reset().read().bits();
        crg().reset().write(|w| unsafe { w.bits(r & !(1 << 9)) });
    }
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

// ============================================================================
// SYSIOP_SUB peripherals — share COM_BRG / AHB_BRG_COMIF bridges, all need
// the enable-reset-enable dance (cxd56_clock.c:898-951).
// ============================================================================

fn sysiop_sub_peripheral_enable(client: SysiopBridgeClient, cken_bit: u32, xrst_bit: u32) {
    let v = topreg_sub().sysiop_sub_cken().read().bits();
    if v & cken_bit != 0 {
        return;
    }
    gate::sysiop_bridge_acquire(client);
    // Step 1: set CKEN bit.
    gate::sysiop_sub_cken_modify(cken_bit, 0);
    pmu::busy_wait(10);
    // Step 2: clear CKEN bit.
    gate::sysiop_sub_cken_modify(0, cken_bit);
    // Step 3: pulse reset (release).
    reset::release(ResetReg::SwresetBus, xrst_bit);
    // Step 4: set CKEN bit again.
    gate::sysiop_sub_cken_modify(cken_bit, 0);
}

fn sysiop_sub_peripheral_disable(client: SysiopBridgeClient, cken_bit: u32, xrst_bit: u32) {
    let v = topreg_sub().sysiop_sub_cken().read().bits();
    if v & cken_bit == 0 {
        return;
    }
    gate::sysiop_sub_cken_modify(0, cken_bit);
    reset::assert_reset(ResetReg::SwresetBus, xrst_bit);
    gate::sysiop_bridge_release(client);
}

fn uart1_enable() -> Result<(), ClockError> {
    sysiop_sub_peripheral_enable(
        SysiopBridgeClient::Uart1,
        sysiop_sub_bits::CK_UART1,
        1 << 5, // XRST_UART1
    );
    Ok(())
}

fn uart1_disable() -> Result<(), ClockError> {
    sysiop_sub_peripheral_disable(SysiopBridgeClient::Uart1, sysiop_sub_bits::CK_UART1, 1 << 5);
    Ok(())
}

fn spim_enable() -> Result<(), ClockError> {
    sysiop_sub_peripheral_enable(
        SysiopBridgeClient::Spim,
        sysiop_sub_bits::CK_SPIM,
        1 << 0, // XRST_SPIM
    );
    Ok(())
}

fn spim_disable() -> Result<(), ClockError> {
    sysiop_sub_peripheral_disable(SysiopBridgeClient::Spim, sysiop_sub_bits::CK_SPIM, 1 << 0);
    Ok(())
}

fn i2cm_enable() -> Result<(), ClockError> {
    sysiop_sub_peripheral_enable(
        SysiopBridgeClient::I2cm,
        sysiop_sub_bits::CK_I2CM,
        1 << 11, // XRST_I2CM
    );
    Ok(())
}

fn i2cm_disable() -> Result<(), ClockError> {
    sysiop_sub_peripheral_disable(SysiopBridgeClient::I2cm, sysiop_sub_bits::CK_I2CM, 1 << 11);
    Ok(())
}

// ============================================================================
// SCU-domain peripherals (I2C0, I2C1, SPI3/SCU, ADCs)
// ============================================================================
//
// The SCU has its own clock domain gated by SYSIOP_CKEN.CKEN_BRG_SCU (bit 14)
// and its own umbrella blocks in SCU_CKEN. Per-peripheral enable is a three-step
// sequence: set cken → pulse off → release reset → set cken again. Each step
// polls CRG_INT_STAT_RAW0 for the corresponding ready bit.
// Mirrors cxd56_scu_clock_enable + cxd56_scu_peri_clock_enable in cxd56_clock.c.

// SYSIOP_CKEN bit 14 — SCU bridge enable.
const CKEN_BRG_SCU: u32 = 1 << 14;

// SCU_CKEN bits for the SCU umbrella blocks.
const SCU_SCU: u32 = 1 << 0; // SCU core
const SCU_SEQ: u32 = 1 << 4; // SCU sequencer
const SCU_32K: u32 = 1 << 5; // SCU 32 K block
const SCU_SC: u32 = 1 << 8; // SCU SC block

// CRG_INT_STAT_RAW0 / CRG_INT_CLR0 ready bits for the SCU umbrella.
const CRG_CK_SCU: u32 = 1 << 9;
const CRG_CK_BRG_SCU: u32 = 1 << 8;
const CRG_CK_SCU_SEQ: u32 = 1 << 13;
const CRG_CK_SCU_SC: u32 = 1 << 14;
const CRG_CK_32K: u32 = 1 << 15;

// CRG_INT_STAT_RAW0 bit 11 — I2C0 clock ready.
const CRG_CK_I2C0: u32 = 1 << 11;

// SCU_CKEN bit 1 — I2C0 enable.
const SCU_I2C0: u32 = 1 << 1;

// SWRESET_SCU bit 5 — I2C0 reset release.
const XRST_I2C0: u32 = 1 << 5;

/// Enable the SCU umbrella clock (bridge + core blocks). Idempotent.
/// Mirrors `cxd56_scu_clock_enable` (cxd56_clock.c:1876).
fn scu_clock_enable() {
    if topreg().sysiop_cken().read().bits() & CKEN_BRG_SCU != 0 {
        return;
    }

    // Default SCU clock source: RCOSC (sel = 0).
    topreg().cksel_scu().write(|w| unsafe { w.bits(0) });

    topreg()
        .crg_int_clr0()
        .write(|w| unsafe { w.bits(0xffff_ffff) });

    // Enable SCU bridge.
    let v = topreg().sysiop_cken().read().bits();
    topreg()
        .sysiop_cken()
        .write(|w| unsafe { w.bits(v | CKEN_BRG_SCU) });

    // Enable SCU umbrella blocks.
    let v = topreg().scu_cken().read().bits();
    topreg()
        .scu_cken()
        .write(|w| unsafe { w.bits(v | SCU_SCU | SCU_SC | SCU_32K | SCU_SEQ) });

    // Poll for all blocks ready.
    const INTR_MASK: u32 =
        CRG_CK_SCU | CRG_CK_SCU_SC | CRG_CK_BRG_SCU | CRG_CK_32K | CRG_CK_SCU_SEQ;
    let mut retry = 1000i32;
    loop {
        if topreg().crg_int_stat_raw0().read().bits() & INTR_MASK == INTR_MASK {
            break;
        }
        pmu::busy_wait(1000);
        retry -= 1;
        if retry <= 0 {
            break;
        }
    }

    topreg()
        .crg_int_clr0()
        .write(|w| unsafe { w.bits(0xffff_ffff) });
}

/// Set or clear `block` bits in SCU_CKEN, polling `intr` in CRG_INT_STAT_RAW0.
/// Mirrors `cxd56_scu_clock_ctrl` (cxd56_clock.c:1834).
fn scu_clock_ctrl(block: u32, intr: u32, on: bool) {
    topreg()
        .crg_int_clr0()
        .write(|w| unsafe { w.bits(0xffff_ffff) });

    let val = topreg().scu_cken().read().bits();
    if on {
        if val & block == block {
            return; // already on
        }
        topreg()
            .scu_cken()
            .write(|w| unsafe { w.bits(val | block) });
    } else {
        if val & block == 0 {
            return; // already off
        }
        topreg()
            .scu_cken()
            .write(|w| unsafe { w.bits(val & !block) });
    }

    let mut retry = 10_000i32;
    loop {
        if topreg().crg_int_stat_raw0().read().bits() & intr != 0 {
            break;
        }
        pmu::busy_wait(1000);
        retry -= 1;
        if retry <= 0 {
            break;
        }
    }

    topreg()
        .crg_int_clr0()
        .write(|w| unsafe { w.bits(0xffff_ffff) });
}

/// Enable I2C0 in the SCU domain. Mirrors `cxd56_scu_peri_clock_enable(&g_scui2c0)`.
fn scu_i2c0_enable() -> Result<(), ClockError> {
    pmu::enable_domain(pmu::PmuDomain::Scu)?;
    scu_clock_enable();

    if topreg().scu_cken().read().bits() & SCU_I2C0 != 0 {
        return Ok(()); // already enabled
    }

    scu_clock_ctrl(SCU_I2C0, CRG_CK_I2C0, true);
    scu_clock_ctrl(SCU_I2C0, CRG_CK_I2C0, false); // reset pulse
    let rst = topreg().swreset_scu().read().bits();
    topreg()
        .swreset_scu()
        .write(|w| unsafe { w.bits(rst | XRST_I2C0) }); // release reset
    scu_clock_ctrl(SCU_I2C0, CRG_CK_I2C0, true);

    Ok(())
}

/// Disable I2C0 in the SCU domain.
fn scu_i2c0_disable() -> Result<(), ClockError> {
    scu_clock_ctrl(SCU_I2C0, CRG_CK_I2C0, false);
    let rst = topreg().swreset_scu().read().bits();
    topreg()
        .swreset_scu()
        .write(|w| unsafe { w.bits(rst & !XRST_I2C0) });
    pmu::disable_domain(pmu::PmuDomain::Scu)?;
    Ok(())
}
