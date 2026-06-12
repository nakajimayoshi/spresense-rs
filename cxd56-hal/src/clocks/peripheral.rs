//! Per-peripheral clock enable/disable + SPI gear adjust.
//!
//! Each [`PeripheralId`] dispatches to a bespoke sequence — there is no
//! uniform shape across the CXD5602's clock subsystem, so we keep the
//! per-peripheral logic explicit and short.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::regs::{crg, topreg, topreg_sub};
use fugit::Hertz;

use super::GearConfig;
use super::gate::{self, ImgClient, SysiopBridgeClient, sysiop_sub_bits};
use super::pmu::{self, PmuDomain};
use super::reset::{self, ResetReg};

/// Errors surfaced from clock-enable/disable.
#[derive(Debug, thiserror::Error)]
pub enum ClockError {
    /// PMU power-domain never asserted/de-asserted within 20000 polls.
    #[error("PMU power-domain did not settle within the poll budget")]
    PmuTimeout,
    /// This [`PeripheralId`] is not implemented in v1 of the HAL.
    #[error("peripheral not implemented in this HAL")]
    Unimplemented,
}

/// Errors from the gear-divider methods
/// ([`Clock::set_gear`](super::Clock::set_gear),
/// [`Clock::set_spi_gear`](super::Clock::set_spi_gear)).
#[derive(Debug, thiserror::Error)]
pub enum GearError {
    /// [`Clock::set_spi_gear`](super::Clock::set_spi_gear) was called on a
    /// peripheral that is not an SPI port (only `Spi4`/`Spi5` take a
    /// max-frequency gear).
    #[error("peripheral is not an SPI port (use set_gear)")]
    NotASpi,
    /// [`Clock::set_gear`](super::Clock::set_gear) was called on a peripheral
    /// that has no APP-local M/N gear divider.
    #[error("peripheral has no APP-local gear divider")]
    NoGearDivider,
    /// `maxfreq` was zero.
    #[error("requested frequency was zero")]
    ZeroFreq,
    /// `appsmp` parent clock is zero (clock tree mis-sampled).
    #[error("parent (APPSMP) clock sampled as zero")]
    ParentClockZero,
    /// Divisor passed to [`Clock::set_gear`](super::Clock::set_gear) is
    /// outside the register's `1..=max` range.
    #[error("gear divisor {divisor} out of range 1..={max}")]
    DivisorOutOfRange { divisor: u32, max: u32 },
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
            PeripheralId::LpAdc => scu_lpadc_enable(),
            // Audio MCLK from the external audio crystal (Spresense main-board
            // default). Callers needing a specific source/divider should call
            // [`super::audio::audio_clock_enable`] directly instead.
            PeripheralId::Audio => super::audio::audio_clock_enable(super::audio::AudMclk::Ext, 0),
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
            PeripheralId::LpAdc => scu_lpadc_disable(),
            PeripheralId::Audio => super::audio::audio_clock_disable(),
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

    /// Set the APP-local **gear divider**, producing a peripheral base clock
    /// of `appsmp / divisor`. Updates the configured-divisor cache, then
    /// writes the gear register (N=1, clock un-gated).
    ///
    /// Valid for every peripheral that owns an M/N gear on the APP-local CRG:
    /// [`ImgUart`](Self::ImgUart) (UART2), [`Spi4`](Self::Spi4),
    /// [`Spi5`](Self::Spi5), [`Usb`](Self::Usb), and [`Sdio`](Self::Sdio).
    /// `divisor` is the M field and must lie in `1..=max`, where `max` is
    /// `0x7f` for `ImgUart`/`Spi4`, `0xf` for `Spi5`, and `0x3` for
    /// `Usb`/`Sdio`.
    ///
    /// Crate-internal: the public entry points are
    /// [`Config::gear`](super::Config::gear) (at constrain) and
    /// [`Clock::set_gear`](super::Clock::set_gear) (runtime — `&mut Clock`,
    /// so peripherals borrowing a dynamic clock block it at compile time).
    pub(crate) fn set_gear(self, divisor: u32) -> Result<(), GearError> {
        let spec = self.gear_spec().ok_or(GearError::NoGearDivider)?;
        if divisor == 0 || divisor > spec.max_divisor {
            return Err(GearError::DivisorOutOfRange {
                divisor,
                max: spec.max_divisor,
            });
        }
        spec.cache.store(divisor, Ordering::Relaxed);
        (spec.write)(0x0001_0000 | divisor);
        Ok(())
    }

    /// The **configured** gear divisor (M) for this peripheral, or `None` if
    /// it has no gear. This is the value [`enable`](Self::enable) programs
    /// and the value the [`Clocks`](super::Clocks) snapshot math uses; it
    /// reflects [`Config::gear`](super::Config::gear) plus any later
    /// [`Clock::set_gear`](super::Clock::set_gear) calls, independent of
    /// whether the hardware register has been programmed yet.
    pub(crate) fn configured_gear(self) -> Option<u32> {
        self.gear_spec().map(|spec| spec.cache.load(Ordering::Relaxed))
    }

    /// Read back the current gear divisor (the M field), or `None` if this
    /// peripheral has no gear divider or its divider is currently gated
    /// (`M == 0`). The base clock is `appsmp / divisor`.
    pub fn gear_divisor(self) -> Option<u32> {
        let spec = self.gear_spec()?;
        let m = (spec.read)() & spec.max_divisor;
        (m != 0).then_some(m)
    }

    /// Set a SPI port's clock gear so the resulting frequency is **at most**
    /// `maxfreq`. Valid for [`PeripheralId::Spi4`] and [`PeripheralId::Spi5`].
    /// Mirrors `cxd56_spi_clock_gear_adjust` (cxd56_clock.c:1133).
    ///
    /// Crate-internal — the public entry point is
    /// [`Clock::set_spi_gear`](super::Clock::set_spi_gear).
    pub(crate) fn set_spi_gear(self, appsmp: Hertz<u32>, maxfreq: Hertz<u32>) -> Result<(), GearError> {
        if !matches!(self, PeripheralId::Spi4 | PeripheralId::Spi5) {
            return Err(GearError::NotASpi);
        }
        if maxfreq.to_Hz() == 0 {
            return Err(GearError::ZeroFreq);
        }
        let baseclk = appsmp.to_Hz();
        if baseclk == 0 {
            return Err(GearError::ParentClockZero);
        }
        let spec = self.gear_spec().ok_or(GearError::NotASpi)?;
        // Ceiling-divide by (2 * maxfreq) — SPI hardware does an internal /2.
        let denom = maxfreq.to_Hz().saturating_mul(2);
        let divisor = baseclk.div_ceil(denom).clamp(1, spec.max_divisor);
        spec.cache.store(divisor, Ordering::Relaxed);
        (spec.write)(0x0001_0000 | divisor);
        Ok(())
    }

    /// Static description of this peripheral's APP-local gear divider, or
    /// `None` if it has none. Single source of truth for the per-peripheral
    /// register and divisor-field width shared by [`set_gear`](Self::set_gear),
    /// [`set_spi_gear`](Self::set_spi_gear), [`gear_divisor`](Self::gear_divisor),
    /// and the enable sequences.
    fn gear_spec(self) -> Option<GearSpec> {
        let spec = match self {
            PeripheralId::ImgUart => GearSpec {
                max_divisor: 0x7f,
                cache: &GEAR_M_IMG_UART,
                write: |v| {
                    crg().gear_img_uart().write(|w| unsafe { w.bits(v) });
                },
                read: || crg().gear_img_uart().read().bits(),
            },
            PeripheralId::Spi4 => GearSpec {
                max_divisor: 0x7f,
                cache: &GEAR_M_IMG_SPI,
                write: |v| {
                    crg().gear_img_spi().write(|w| unsafe { w.bits(v) });
                },
                read: || crg().gear_img_spi().read().bits(),
            },
            PeripheralId::Spi5 => GearSpec {
                max_divisor: 0xf,
                cache: &GEAR_M_IMG_WSPI,
                write: |v| {
                    crg().gear_img_wspi().write(|w| unsafe { w.bits(v) });
                },
                read: || crg().gear_img_wspi().read().bits(),
            },
            PeripheralId::Usb => GearSpec {
                max_divisor: 0x3,
                cache: &GEAR_M_USB,
                write: |v| {
                    crg().gear_per_usb().write(|w| unsafe { w.bits(v) });
                },
                read: || crg().gear_per_usb().read().bits(),
            },
            PeripheralId::Sdio => GearSpec {
                max_divisor: 0x3,
                cache: &GEAR_M_SDIO,
                write: |v| {
                    crg().gear_per_sdio().write(|w| unsafe { w.bits(v) });
                },
                read: || crg().gear_per_sdio().read().bits(),
            },
            _ => return None,
        };
        Some(spec)
    }
}

/// Static description of a peripheral's APP-local M/N gear divider.
///
/// For all gear peripherals the output is `appsmp * N / M` with the N field a
/// single bit (0 = gated, 1 = passthrough), so the divider reduces to
/// `appsmp / M`.
struct GearSpec {
    /// Largest value the M (divisor) field can hold (its bit-mask).
    max_divisor: u32,
    /// Configured divisor for this gear — see the `GEAR_M_*` statics.
    cache: &'static AtomicU32,
    /// Write a raw `(N << 16) | M` word into the gear register.
    write: fn(u32),
    /// Read the raw gear register.
    read: fn() -> u32,
}

// Configured gear divisors (M field), one per gear-divided peripheral.
// Seeded from `Config::gear` at `Crg::constrain` and updated by
// `Clock::set_gear`/`set_spi_gear`; `enable()` programs the cached value and
// the `Clocks` snapshot math divides `appsmp` by it. Initial values match
// `GearConfig::default()` so a peripheral enabled before `constrain` still
// gets the NuttX bring-up defaults.
static GEAR_M_IMG_UART: AtomicU32 = AtomicU32::new(4);
static GEAR_M_IMG_SPI: AtomicU32 = AtomicU32::new(2);
static GEAR_M_IMG_WSPI: AtomicU32 = AtomicU32::new(4);
static GEAR_M_USB: AtomicU32 = AtomicU32::new(2);
static GEAR_M_SDIO: AtomicU32 = AtomicU32::new(2);

/// Seed the configured-divisor caches from board configuration. Called by
/// [`Crg::new`](super::Crg) before the first clock snapshot is taken.
///
/// # Panics
/// On a divisor outside its register's `1..=max` range — a board-config
/// error that would otherwise surface as a silently wrong baud/SCK rate.
pub(crate) fn seed_gear_config(gear: GearConfig) {
    for (id, m) in [
        (PeripheralId::ImgUart, gear.img_uart),
        (PeripheralId::Spi4, gear.img_spi),
        (PeripheralId::Spi5, gear.img_wspi),
        (PeripheralId::Usb, gear.usb),
        (PeripheralId::Sdio, gear.sdio),
    ] {
        let spec = id.gear_spec().unwrap();
        assert!(
            m != 0 && m <= spec.max_divisor,
            "GearConfig: {:?} divisor {} out of range 1..={}",
            id,
            m,
            spec.max_divisor
        );
        spec.cache.store(m, Ordering::Relaxed);
    }
}

/// Program a gear during bring-up: assert N (un-gate) and set M to the
/// **configured** divisor — [`Config::gear`](super::Config::gear), possibly
/// retuned via [`Clock::set_gear`](super::Clock::set_gear). Replaces NuttX's
/// hardcoded `putreg32(0x0001_00MM, …)` writes, and re-applies the
/// configuration after a [`disable`](PeripheralId::disable) cleared the
/// register.
fn enable_gear_configured(id: PeripheralId) {
    if let Some(spec) = id.gear_spec() {
        (spec.write)(0x0001_0000 | spec.cache.load(Ordering::Relaxed));
    }
}

// ============================================================================
// APP_SUB / IMG-block peripherals
// ============================================================================

fn spi4_enable() -> Result<(), ClockError> {
    pmu::enable_domain(PmuDomain::AppSub)?;
    gate::img_acquire(ImgClient::Spi4);
    // Configured M (default 2, appsmp/2); the hardware reset value is M=4 and
    // is deliberately ignored in favor of the configuration.
    enable_gear_configured(PeripheralId::Spi4);
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
    // Configured M (default 4, appsmp/4 — NuttX's bring-up value).
    enable_gear_configured(PeripheralId::Spi5);
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
    // Configured M (default 4: 156 MHz/4 ≈ 39 MHz, the NuttX default). The
    // Clocks snapshot divides appsmp by the same configured value, so the
    // driver's baud math and this write always agree.
    enable_gear_configured(PeripheralId::ImgUart);
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
    // Configured M (default 2, appsmp/2); USB needs a specific base clock.
    enable_gear_configured(PeripheralId::Usb);
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
    // Configured M (default 2, appsmp/2 — NuttX's bring-up value).
    enable_gear_configured(PeripheralId::Sdio);
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

// SCU_CKEN bit 6 — LPADC (CK_SCU_U32KL). NuttX `g_sculpadc.cken = 6`.
const SCU_LPADC: u32 = 1 << 6;

// CRG_INT_STAT_RAW0 bit 17 — LPADC clock ready. NuttX `g_sculpadc.crgintmask = 17`.
const CRG_CK_LPADC: u32 = 1 << 17;

// SWRESET_SCU bit 4 — LPADC reset release. NuttX `g_sculpadc.swreset = 4`.
const XRST_SCU_LPADC: u32 = 1 << 4;

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

/// Enable LPADC in the SCU domain. Mirrors `cxd56_scu_peri_clock_enable(&g_sculpadc)`.
/// NuttX `g_sculpadc`: cken=6, swreset=4, crgintmask=17.
fn scu_lpadc_enable() -> Result<(), ClockError> {
    pmu::enable_domain(pmu::PmuDomain::Scu)?;
    pmu::enable_analog(pmu::AnaDomain::Lpadc)?;
    scu_clock_enable();

    if topreg().scu_cken().read().bits() & SCU_LPADC != 0 {
        return Ok(()); // already enabled
    }

    scu_clock_ctrl(SCU_LPADC, CRG_CK_LPADC, true);
    scu_clock_ctrl(SCU_LPADC, CRG_CK_LPADC, false); // reset pulse
    let rst = topreg().swreset_scu().read().bits();
    topreg()
        .swreset_scu()
        .write(|w| unsafe { w.bits(rst | XRST_SCU_LPADC) }); // release reset
    scu_clock_ctrl(SCU_LPADC, CRG_CK_LPADC, true);

    Ok(())
}

/// Disable LPADC in the SCU domain.
fn scu_lpadc_disable() -> Result<(), ClockError> {
    scu_clock_ctrl(SCU_LPADC, CRG_CK_LPADC, false);
    let rst = topreg().swreset_scu().read().bits();
    topreg()
        .swreset_scu()
        .write(|w| unsafe { w.bits(rst & !XRST_SCU_LPADC) });
    pmu::disable_analog(pmu::AnaDomain::Lpadc)?;
    pmu::disable_domain(pmu::PmuDomain::Scu)?;
    Ok(())
}
