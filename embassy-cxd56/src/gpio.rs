//! General purpose input/output (GPIO) driver.
#![macro_use]

use embassy_hal_internal::Peri;
use embassy_hal_internal::{PeripheralType, impl_peripheral};

use crate::{pac, peripherals};
use crate::pac::common::{Reg, RW};

// GP_* register bit positions — identical across all TOPREG GP_* registers.
const IN_BIT: u32 = 1 << 0;    // sampled input level (read-only)
const OUT_BIT: u32 = 1 << 8;   // output data
const DIR_BIT: u32 = 1 << 16;  // direction, active-low: 0 = output, 1 = high-Z input

// IO_* pad register bit positions.
const ENZI_BIT: u32 = 1 << 0;   // input buffer enable
const PUN_BIT: u32 = 1 << 8;    // pull-up,   active-low: 0 = enabled
const PDN_BIT: u32 = 1 << 16;   // pull-down, active-low: 0 = enabled
const LOWEMI_BIT: u32 = 1 << 24; // drive strength: 0 = 4 mA, 1 = 2 mA

// Offset within TOPREG where the contiguous GP_* run begins.
const GP_BASE: usize = 0x2000;

/// Pull setting for an input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// No pull.
    None,
    /// Internal pull-up resistor.
    Up,
    /// Internal pull-down resistor.
    Down,
}

/// GPIO input driver.
pub struct Input<'d> {
    pub(crate) pin: Flex<'d>,
}

impl<'d> Input<'d> {
    /// Create GPIO input driver for a [Pin] with the provided [Pull] configuration.
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, pull: Pull) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_as_input(pull);
        Self { pin }
    }

    /// Get whether the pin input level is high.
    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    /// Get whether the pin input level is low.
    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// Get the pin input level.
    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }
}

impl Input<'static> {
    /// Persist the pin's configuration for the rest of the program's lifetime.
    pub fn persist(self) {
        self.pin.persist()
    }
}

/// Digital input or output level.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Level {
    /// Logical low.
    Low,
    /// Logical high.
    High,
}

impl From<bool> for Level {
    fn from(val: bool) -> Self {
        match val {
            true => Self::High,
            false => Self::Low,
        }
    }
}

impl From<Level> for bool {
    fn from(level: Level) -> bool {
        match level {
            Level::Low => false,
            Level::High => true,
        }
    }
}

/// Drive strength settings for a given output level.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum LevelDrive {
    /// Disconnect (do not drive the output at all)
    Disconnect = 2,
    /// Standard
    Standard = 0,
    /// High drive
    High = 1,
}

/// Drive strength for an output pin.
///
/// CXD5602 has two levels via the `LOWEMI` pad bit: `Standard` = 2 mA, `High` = 4 mA.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputDrive {
    /// 2 mA drive (LOWEMI = 1).
    Standard,
    /// 4 mA drive (LOWEMI = 0).
    High,
}

/// GPIO output driver.
pub struct Output<'d> {
    pub(crate) pin: Flex<'d>,
}

impl<'d> Output<'d> {
    /// Create GPIO output driver for a [Pin] with the provided [Level] and [OutputDrive].
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, initial_output: Level, drive: OutputDrive) -> Self {
        let mut pin = Flex::new(pin);
        match initial_output {
            Level::High => pin.set_high(),
            Level::Low => pin.set_low(),
        }
        pin.set_as_output(drive);
        Self { pin }
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high()
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low()
    }

    /// Toggle the output level.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle()
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Get whether the output level is set to high.
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    /// Get whether the output level is set to low.
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }

    /// Get the current output level.
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.pin.get_output_level()
    }
}

impl Output<'static> {
    /// Persist the pin's configuration for the rest of the program's lifetime.
    pub fn persist(self) {
        self.pin.persist()
    }
}

/// GPIO flexible pin.
///
/// Can be configured as disconnected, input, output, or both. The output level
/// register bit is preserved while not in output mode, so the level is
/// 'remembered' when direction is changed.
pub struct Flex<'d> {
    pub(crate) pin: Peri<'d, AnyPin>,
}

impl<'d> Flex<'d> {
    /// Wrap the pin in a `Flex`. The pin starts disconnected.
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>) -> Self {
        Self { pin: pin.into() }
    }

    /// Put the pin into input mode.
    #[inline]
    pub fn set_as_input(&mut self, pull: Pull) {
        // DIR=1 → high-Z input
        self.pin.gp_reg().modify(|r| *r |= DIR_BIT);
        if let Some(io) = self.pin.io_reg() {
            io.modify(|r| {
                *r |= ENZI_BIT;
                match pull {
                    Pull::None  => { *r |= PUN_BIT | PDN_BIT; }
                    Pull::Up    => { *r &= !PUN_BIT; *r |= PDN_BIT; }
                    Pull::Down  => { *r &= !PDN_BIT; *r |= PUN_BIT; }
                }
            });
        }
    }

    /// Put the pin into output mode.
    #[inline]
    pub fn set_as_output(&mut self, drive: OutputDrive) {
        // DIR=0 → drive output
        self.pin.gp_reg().modify(|r| *r &= !DIR_BIT);
        if let Some(io) = self.pin.io_reg() {
            io.modify(|r| {
                *r &= !ENZI_BIT;
                *r |= PUN_BIT | PDN_BIT; // no pull
                match drive {
                    OutputDrive::High     => { *r &= !LOWEMI_BIT; }
                    OutputDrive::Standard => { *r |= LOWEMI_BIT; }
                }
            });
        }
    }

    /// Put the pin into input + output mode (e.g. open-drain).
    #[inline]
    pub fn set_as_input_output(&mut self, pull: Pull, drive: OutputDrive) {
        // DIR=0 (drives output) + ENZI=1 (input buffer reads back the line)
        self.pin.gp_reg().modify(|r| *r &= !DIR_BIT);
        if let Some(io) = self.pin.io_reg() {
            io.modify(|r| {
                *r |= ENZI_BIT;
                match pull {
                    Pull::None  => { *r |= PUN_BIT | PDN_BIT; }
                    Pull::Up    => { *r &= !PUN_BIT; *r |= PDN_BIT; }
                    Pull::Down  => { *r &= !PDN_BIT; *r |= PUN_BIT; }
                }
                match drive {
                    OutputDrive::High     => { *r &= !LOWEMI_BIT; }
                    OutputDrive::Standard => { *r |= LOWEMI_BIT; }
                }
            });
        }
    }

    /// Put the pin into disconnected / high-Z mode.
    #[inline]
    pub fn set_as_disconnected(&mut self) {
        self.pin.gp_reg().modify(|r| *r |= DIR_BIT);
        if let Some(io) = self.pin.io_reg() {
            io.modify(|r| {
                *r &= !ENZI_BIT;
                *r |= PUN_BIT | PDN_BIT;
            });
        }
    }

    /// Get whether the pin input level is high.
    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.gp_reg().read() & IN_BIT != 0
    }

    /// Get whether the pin input level is low.
    #[inline]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// Get the pin input level.
    #[inline]
    pub fn get_level(&self) -> Level {
        self.is_high().into()
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.gp_reg().modify(|r| *r |= OUT_BIT);
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.gp_reg().modify(|r| *r &= !OUT_BIT);
    }

    /// Toggle the output level.
    #[inline]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }

    /// Get whether the output level is set to high.
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.gp_reg().read() & OUT_BIT != 0
    }

    /// Get whether the output level is set to low.
    #[inline]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// Get the current output level.
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.is_set_high().into()
    }
}

impl Flex<'static> {
    /// Persist the pin's configuration for the rest of the program's lifetime.
    pub fn persist(self) {
        core::mem::forget(self);
    }
}

impl<'d> Drop for Flex<'d> {
    fn drop(&mut self) {
        self.set_as_disconnected();
    }
}

pub(crate) trait SealedPin {
    /// GP register index for this pin. Maps to TOPREG offset `GP_BASE + index*4`.
    fn pin(&self) -> u8;

    #[inline]
    fn _pin(&self) -> u8 {
        self.pin()
    }

    /// Returns the GP_* direction/level register for this pin.
    #[inline]
    fn gp_reg(&self) -> Reg<u32, RW> {
        let base = pac::TOPREG.as_ptr() as usize;
        unsafe { Reg::from_ptr((base + GP_BASE + self.pin() as usize * 4) as *mut u32) }
    }

    /// Returns the IO_* pad configuration register for this pin, if one exists.
    ///
    /// The I2S1/LED pins (indices 90–93) have no `IO_*` accessor in the PAC.
    /// Pull and drive settings are silently ignored for those pins.
    #[inline]
    fn io_reg(&self) -> Option<Reg<u32, RW>> {
        let off: usize = match self.pin() {
            36 => 0x0894,                // GP_SEN_IRQ_IN
            43 => 0x08b0, 44 => 0x08b4, // GP_I2C0_BCK, GP_I2C0_BDT
            60 => 0x090c, 61 => 0x0910, // GP_UART2_TXD, GP_UART2_RXD
            62 => 0x0914, 63 => 0x0918, // GP_UART2_CTS, GP_UART2_RTS
            68 => 0x092c, 69 => 0x0930, // GP_EMMC_CLK, GP_EMMC_CMD
            70 => 0x0934, 71 => 0x0938, // GP_EMMC_DATA0, GP_EMMC_DATA1
            72 => 0x093c, 73 => 0x0940, // GP_EMMC_DATA2, GP_EMMC_DATA3
            86 => 0x0974, 87 => 0x0978, // GP_I2S0_BCK, GP_I2S0_LRCK
            88 => 0x097c, 89 => 0x0980, // GP_I2S0_DATA_IN, GP_I2S0_DATA_OUT
            _ => return None,           // GP_I2S1_* (LEDs, 90–93): no pad reg in PAC
        };
        let base = pac::TOPREG.as_ptr() as usize;
        Some(unsafe { Reg::from_ptr((base + off) as *mut u32) })
    }
}

/// Interface for a Pin that can be configured by an [Input] or [Output] driver,
/// or converted to an [AnyPin].
#[allow(private_bounds)]
pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    /// GP register index for this pin.
    #[inline]
    fn pin(&self) -> u8 {
        self._pin()
    }
}

/// Type-erased GPIO pin.
pub struct AnyPin {
    pub(crate) pin: u8,
}

impl AnyPin {
    /// Create an [AnyPin] for a specific GP register index.
    ///
    /// # Safety
    /// `pin` must be a valid GP register index not currently in use by another driver.
    #[inline]
    pub unsafe fn steal(pin: u8) -> Peri<'static, Self> {
        unsafe { Peri::new_unchecked(Self { pin }) }
    }
}

impl_peripheral!(AnyPin);
impl Pin for AnyPin {}
impl SealedPin for AnyPin {
    #[inline]
    fn pin(&self) -> u8 {
        self.pin
    }
}

// ====================

macro_rules! impl_pin {
    ($type:ident, $idx:expr) => {
        impl crate::gpio::Pin for peripherals::$type {}
        impl crate::gpio::SealedPin for peripherals::$type {
            #[inline]
            fn pin(&self) -> u8 {
                $idx
            }
        }
        impl From<peripherals::$type> for crate::gpio::AnyPin {
            fn from(_: peripherals::$type) -> Self {
                Self { pin: $idx }
            }
        }
    };
}

// P1e — SEN_IRQ_IN block
impl_pin!(P1e_00, 36);   // GP_SEN_IRQ_IN  — SEN_IRQ / Arduino D22 (JP1)

// P1j — I2C0 block
impl_pin!(P1j_00, 43);   // GP_I2C0_BCK    — I2C0_SCL / Arduino D15 (JP2)
impl_pin!(P1j_01, 44);   // GP_I2C0_BDT    — I2C0_SDA / Arduino D14 (JP2)

// P1n — UART2 block
impl_pin!(P1n_00, 60);   // GP_UART2_TXD   — UART2_TX / Arduino D01 (JP1)
impl_pin!(P1n_01, 61);   // GP_UART2_RXD   — UART2_RX / Arduino D00 (JP1)
impl_pin!(P1n_02, 62);   // GP_UART2_CTS   — UART2_CTS / Arduino D27 (JP1)
impl_pin!(P1n_03, 63);   // GP_UART2_RTS   — UART2_RTS / Arduino D28 (JP1)

// P1p — EMMCA / SPI5 block
impl_pin!(P1p_00, 68);   // GP_EMMC_CLK    — SPI5_SCK / Arduino D23 (JP1)
impl_pin!(P1p_01, 69);   // GP_EMMC_CMD    — SPI5_CS_X / Arduino D24 (JP1)
impl_pin!(P1p_02, 70);   // GP_EMMC_DATA0  — SPI5_MOSI / Arduino D16 (JP2)
impl_pin!(P1p_03, 71);   // GP_EMMC_DATA1  — SPI5_MISO / Arduino D17 (JP2)

// P1q — EMMCB block
impl_pin!(P1q_00, 72);   // GP_EMMC_DATA2  — GPIO / Arduino D20 (JP2)
impl_pin!(P1q_01, 73);   // GP_EMMC_DATA3  — GPIO / Arduino D21 (JP2)

// P1v — I2S0 block
impl_pin!(P1v_00, 86);   // GP_I2S0_BCK      — I2S0_BCK / Arduino D26 (JP1)
impl_pin!(P1v_01, 87);   // GP_I2S0_LRCK     — I2S0_LRCK / Arduino D25 (JP1)
impl_pin!(P1v_02, 88);   // GP_I2S0_DATA_IN  — I2S0_DATA_IN / Arduino D19 (JP2)
impl_pin!(P1v_03, 89);   // GP_I2S0_DATA_OUT — I2S0_DATA_OUT / Arduino D18 (JP2)

// P1w — I2S1 / LED block (onboard LEDs; no IO_* pad regs in PAC)
impl_pin!(P1w_00, 90);   // GP_I2S1_BCK      — LED0
impl_pin!(P1w_01, 91);   // GP_I2S1_LRCK     — LED1
impl_pin!(P1w_02, 92);   // GP_I2S1_DATA_IN  — LED2
impl_pin!(P1w_03, 93);   // GP_I2S1_DATA_OUT — LED3
