//! Per-peripheral clock enable/disable + SPI gear adjust.
//!
//! Each [`PeripheralId`] dispatches to a bespoke sequence — there is no
//! uniform shape across the CXD5602's clock subsystem, so we keep the
//! per-peripheral logic explicit and short.

use crate::pac;
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
                let crg = unsafe { &*pac::Crg::PTR };
                crg.gear_img_spi().write(|w| unsafe { w.bits(gear) });
            }),
            PeripheralId::Spi5 => (0xf, |gear| {
                let crg = unsafe { &*pac::Crg::PTR };
                crg.gear_img_wspi().write(|w| unsafe { w.bits(gear) });
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
    let crg = unsafe { &*pac::Crg::PTR };
    crg.gear_img_spi().write(|w| unsafe { w.bits(0x0001_0002) });
    Ok(())
}

fn spi4_disable() -> Result<(), ClockError> {
    let crg = unsafe { &*pac::Crg::PTR };
    crg.gear_img_spi().write(|w| unsafe { w.bits(0) });
    gate::img_release(ImgClient::Spi4);
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

fn spi5_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    gate::img_acquire(ImgClient::Spi5);
    let crg = unsafe { &*pac::Crg::PTR };
    crg.gear_img_wspi()
        .write(|w| unsafe { w.bits(0x0001_0004) });
    Ok(())
}

fn spi5_disable() -> Result<(), ClockError> {
    let crg = unsafe { &*pac::Crg::PTR };
    crg.gear_img_wspi().write(|w| unsafe { w.bits(0) });
    gate::img_release(ImgClient::Spi5);
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

fn img_uart_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    gate::img_acquire(ImgClient::Uart2);
    let crg = unsafe { &*pac::Crg::PTR };
    // Default M=4 (apportions 156 MHz/4 ≈ 39 MHz, matches NuttX default).
    crg.gear_img_uart()
        .write(|w| unsafe { w.bits(0x0001_0004) });
    Ok(())
}

fn img_uart_disable() -> Result<(), ClockError> {
    let crg = unsafe { &*pac::Crg::PTR };
    crg.gear_img_uart().write(|w| unsafe { w.bits(0) });
    gate::img_release(ImgClient::Uart2);
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

fn usb_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    let crg = unsafe { &*pac::Crg::PTR };
    let topreg = unsafe { &*pac::Topreg::PTR };

    let c = crg.ck_gate_ahb().read().bits();
    if c & (1 << 8) != 0 {
        return Ok(());
    }
    // Assert XRS_USB (=0), set CK_GATE_USB, wait, release reset, enable
    // USBPHY, set gear. Mirrors cxd56_clock.c:631-650.
    let r = crg.reset().read().bits();
    crg.reset().write(|w| unsafe { w.bits(r & !(1 << 8)) });
    crg.ck_gate_ahb().write(|w| unsafe { w.bits(c | (1 << 8)) });
    pmu::busy_wait(10);
    crg.reset().write(|w| unsafe { w.bits(r | (1 << 8)) });
    topreg.usbphy_cken().write(|w| unsafe { w.bits(1) });
    crg.gear_per_usb().write(|w| unsafe { w.bits(0x0001_0002) });
    Ok(())
}

fn usb_disable() -> Result<(), ClockError> {
    let crg = unsafe { &*pac::Crg::PTR };
    let topreg = unsafe { &*pac::Topreg::PTR };
    let c = crg.ck_gate_ahb().read().bits();
    if c & (1 << 8) != 0 {
        crg.gear_per_usb().write(|w| unsafe { w.bits(0) });
        topreg.usbphy_cken().write(|w| unsafe { w.bits(0) });
        crg.ck_gate_ahb()
            .write(|w| unsafe { w.bits(c & !(1 << 8)) });
        let r = crg.reset().read().bits();
        crg.reset().write(|w| unsafe { w.bits(r & !(1 << 8)) });
    }
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

fn sdio_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    let crg = unsafe { &*pac::Crg::PTR };
    let c = crg.ck_gate_ahb().read().bits();
    if c & (1 << 9) != 0 {
        return Ok(());
    }
    let r = crg.reset().read().bits();
    crg.reset().write(|w| unsafe { w.bits(r & !(1 << 9)) });
    crg.ck_gate_ahb().write(|w| unsafe { w.bits(c | (1 << 9)) });
    crg.gear_per_sdio()
        .write(|w| unsafe { w.bits(0x0001_0002) });
    pmu::busy_wait(10);
    crg.reset().write(|w| unsafe { w.bits(r | (1 << 9)) });
    Ok(())
}

fn sdio_disable() -> Result<(), ClockError> {
    let crg = unsafe { &*pac::Crg::PTR };
    let c = crg.ck_gate_ahb().read().bits();
    if c & (1 << 9) != 0 {
        crg.gear_per_sdio().write(|w| unsafe { w.bits(0) });
        crg.ck_gate_ahb()
            .write(|w| unsafe { w.bits(c & !(1 << 9)) });
        let r = crg.reset().read().bits();
        crg.reset().write(|w| unsafe { w.bits(r & !(1 << 9)) });
    }
    pmu::disable_domain(PmuDomain::AppSub)?;
    Ok(())
}

// ============================================================================
// SYSIOP_SUB peripherals — share COM_BRG / AHB_BRG_COMIF bridges, all need
// the enable-reset-enable dance (cxd56_clock.c:898-951).
// ============================================================================

fn sysiop_sub_peripheral_enable(client: SysiopBridgeClient, cken_bit: u32, xrst_bit: u32) {
    let topreg = unsafe { &*pac::TopregSub::PTR };
    let v = topreg.sysiop_sub_cken().read().bits();
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
    let topreg = unsafe { &*pac::TopregSub::PTR };
    let v = topreg.sysiop_sub_cken().read().bits();
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
