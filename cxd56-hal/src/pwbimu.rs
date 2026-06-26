//! CXD5602PWBIMU add-on board — dual-bus interface.
//!
//! The CXD5602PWBIMU is *not* a bare IMU chip addressed directly. The board
//! fronts the sensor with one or more on-board PSoC microcontrollers, and the
//! Spresense host talks to them over **two buses at once**:
//!
//! * **I2C0 @ 400 kHz — the control plane.** Identity, full-scale range, ODR,
//!   interrupt enable, FIFO threshold, output enable, calibration, mode.
//! * **SPI5 (+DMA) — the data plane.** The high-rate 6-axis sample stream,
//!   triggered by `DRDY`.
//!
//! [`Pwbimu`] is a single handle over **both** buses: it owns the I2C control
//! plane (power sequencing, `whoami`, ODR / full-scale / output-enable) *and* the
//! SPI5 data plane (the DRDY-gated 6-axis sample read), so from one object you
//! can power_on and stream [`ImuSample`]s.
//!
//! # Reading axis data
//!
//! 1. [`Pwbimu::new`] → a handle in the [`Off`] state.
//! 2. [`Pwbimu::power_on`] — sequence the board up **and probe its identity**;
//!    on success returns it in [`Idle`], on no-answer returns [`NotResponding`].
//! 3. [`Pwbimu::whoami`] (optional) — read the full identity for logging.
//! 4. [`Pwbimu::configure`] — enable the DRDY line, set ODR and full-scale range.
//! 5. [`Pwbimu::enable`] — start the 6-axis stream; returns it in [`Streaming`].
//! 6. [`Pwbimu::read_sample_blocking`] — wait for DRDY, read one [`ImuSample`].
//!
//! The lifecycle is tracked in the type system (see [`state`]): each step is only
//! reachable in the right state, so e.g. reading a sample before [`enable`] or
//! configuring before [`power_on`] is a **compile error**, not a runtime hang.
//! The transitions ([`power_on`], [`enable`], [`disable`]) consume the handle and
//! return it in its new state, so the idiom is `let imu = imu.power_on(&mut d)?;`.
//! [`power_on`] *earns* the [`Idle`] state by confirming the board answered; use
//! [`power_on_unchecked`] to drive the lines open-loop without a probe.
//!
//! Steps 2–5 ride the I2C control plane; step 6 the SPI5 data plane — but all are
//! methods on the same [`Pwbimu`], and all fallible ones share one error type
//! ([`Error`]). The PSoC emits each sample as IEEE-754 floats already scaled to
//! g / dps, so no conversion is needed host-side.
//!
//! [`enable`]: Pwbimu::enable
//! [`disable`]: Pwbimu::disable
//! [`power_on`]: Pwbimu::power_on
//! [`power_on_unchecked`]: Pwbimu::power_on_unchecked
//!
//! # The board is powered off until you sequence it
//!
//! You cannot just open I2C0 and read a register: the board's power rail and
//! reset (`XRST`) are gated off out of reset. The PSoCs only answer on I2C after
//! the power → reset-pulse → 150 ms-settle sequence performed by
//! [`Pwbimu::power_on`] (mirroring the NuttX implementation `cxd5602pwbimu_open`). Call it once
//! after construction, before any register access.
//!
//! # Identity / `whoami`
//!
//! There is no register literally named `WHOAMI`; the NuttX driver uses
//! [`reg::FW_VER`] (`0x10`) as the presence/identity probe. A successful read
//! returning a plausible firmware-version byte means the primary PSoC at I2C
//! `0x10` answered — that is the `whoami` succeeding. See [`Pwbimu::whoami`].
//!
//! # Board wiring (fixed by the add-on)
//!
//! | Signal           | CXD5602 pad   | Arduino | This crate                 |
//! |------------------|---------------|---------|----------------------------|
//! | Power enable     | `EMMC_DATA2`  | D20     | `gp_emmc_data2`            |
//! | Reset (`XRST`)   | `I2S0_BCK`    | D26     | `gp_i2s0_bck`             |
//! | I2C SCL          | `I2C0_BCK`    | D15     | `gp_i2c0_bck` (via the bus) |
//! | I2C SDA          | `I2C0_BDT`    | D14     | `gp_i2c0_bdt` (via the bus) |
//! | SPI SCK          | `EMMC_CLK`    | D23     | (via the SPI5 bus)         |
//! | SPI MOSI         | `EMMC_DATA0`  | D16     | (via the SPI5 bus)         |
//! | SPI MISO         | `EMMC_DATA1`  | D17     | (via the SPI5 bus)         |
//! | SPI chip-select  | `I2S0_DATA_IN`| D19     | `gp_i2s0_data_in`          |
//! | Data ready (DRDY)| `EMMC_DATA3`  | D21     | `gp_emmc_data3`            |
//!
//! Note the IMU's chip-select is the **GPIO** `I2S0_DATA_IN` (driven by
//! [`Pwbimu`] around each sample read), *not* the PL022 hardware frame-select on
//! `EMMC_CMD` that the SPI5 bus toggles — `EMMC_CMD` is unused by the add-on.

use core::marker::PhantomData;

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;
use embedded_hal::spi::SpiBus;

use crate::gpio::{GpioPin, Input, Level, Output};
use crate::pac::topreg::{GpEmmcData2, GpEmmcData3, GpI2s0Bck, GpI2s0DataIn};

/// Identity / control registers read over I2C (NuttX `cxd5602pwbimu` register map).
pub mod reg {
    /// Firmware version — the de-facto `whoami` (the driver's presence probe).
    pub const FW_VER: u8 = 0x10;
    /// Hardware revision.
    pub const HW_REVISION: u8 = 0x11;
    /// Hardware unique ID.
    pub const HW_UNIQUE_ID: u8 = 0x12;
    /// Full-scale range: accel in the high nibble, gyro in the low nibble.
    pub const FSR: u8 = 0x13;
    /// Output data rate (sample frequency).
    pub const ODR: u8 = 0x14;
    /// Data-ready interrupt enable — write 1 so the PSoC drives the DRDY line.
    pub const INTR_ENABLE: u8 = 0x15;
    /// 6-axis output enable: 1 = start streaming, 0 = stop.
    pub const OUTPUT_ENABLE: u8 = 0x18;
}

/// Default I2C address of the primary PSoC (`slaveid` 0).
pub const ADDR_PRIMARY: u8 = 0x10;
/// Base I2C address of the secondary PSoC bank (DIP-switch selectable).
pub const ADDR_SECONDARY: u8 = 0x30;

/// Power-rail settle after asserting the power pin (NuttX `up_mdelay(2)` in the
/// board `power()` callback).
const POWER_SETTLE_MS: u32 = 2;
/// Reset (`XRST`) low-pulse width (NuttX `up_udelay(20)`).
const RESET_PULSE_US: u32 = 20;
/// PSoC firmware boot time after reset release (NuttX `up_mdelay(150)`).
const BOOT_SETTLE_MS: u32 = 150;

/// Chip-select edge guard around an SPI sample read (NuttX board `csx`
/// callback: `up_udelay(5)` on each edge).
const CSX_GUARD_US: u32 = 5;

/// Bytes per SPI sample — `sizeof(cxd5602pwbimu_data_t)`: `timestamp` (u32) +
/// `temp` + `gx,gy,gz` + `ax,ay,az` (7 × f32).
pub const SAMPLE_BYTES: usize = 32;

/// SPI read flag — high bit set in the first transmitted byte (NuttX `recv`).
const SPI_READ_FLAG: u8 = 0x80;

/// Output data rate / sample frequency, written to [`reg::ODR`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Odr {
    Hz15,
    Hz30,
    Hz60,
    Hz120,
    Hz240,
    Hz480,
    Hz960,
    Hz1920,
}

impl Odr {
    /// Register encoding (`ODR_*`, 0x00–0x07).
    pub fn code(self) -> u8 {
        self as u8
    }

    /// The nominal sample rate in Hz.
    pub fn hz(self) -> u32 {
        match self {
            Odr::Hz15 => 15,
            Odr::Hz30 => 30,
            Odr::Hz60 => 60,
            Odr::Hz120 => 120,
            Odr::Hz240 => 240,
            Odr::Hz480 => 480,
            Odr::Hz960 => 960,
            Odr::Hz1920 => 1920,
        }
    }
}

/// Accelerometer full-scale range (±g). High nibble of [`reg::FSR`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccelRange {
    G2,
    G4,
    G8,
    G16,
}

impl AccelRange {
    /// Register encoding (`FSR_ACCEL_*`, pre-shift, 0x00–0x03).
    pub fn code(self) -> u8 {
        self as u8
    }
}

/// Gyroscope full-scale range (±dps). Low nibble of [`reg::FSR`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GyroRange {
    Dps125,
    Dps250,
    Dps500,
    Dps1000,
    Dps2000,
    Dps4000,
}

impl GyroRange {
    /// Register encoding (`FSR_GYRO_*`, 0x00–0x05).
    pub fn code(self) -> u8 {
        self as u8
    }
}

/// [`Pwbimu::whoami`] — the three identity registers returned from i2c whoami
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    /// [`reg::FW_VER`] — firmware version.
    pub fw_ver: u8,
    /// [`reg::HW_REVISION`] — hardware revision.
    pub hw_rev: u8,
    /// [`reg::HW_UNIQUE_ID`] — hardware unique ID.
    pub hw_uid: u8,
}

/// Error from a [`Pwbimu`] operation, tagged by which bus produced it.
///
/// Control-plane methods ([`whoami`](Pwbimu::whoami),
/// [`configure`](Pwbimu::configure), [`enable`](Pwbimu::enable), …) yield
/// [`Error::I2c`]; the data-plane reads ([`read_sample`](Pwbimu::read_sample),
/// [`read_sample_blocking`](Pwbimu::read_sample_blocking)) yield [`Error::Spi`].
/// A single error type across both buses means the whole bring-up-to-stream flow
/// can be `?`-chained on one `Result`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<I2cErr, SpiErr> {
    /// An error on the I2C control plane.
    I2c(I2cErr),
    /// An error on the SPI data plane.
    Spi(SpiErr),
}

/// The [`Error`] type shared by every fallible [`Pwbimu`] method, written in
/// terms of the I2C bus `I` and SPI bus `S` (so `BusError<I, S>` instead of the
/// spelled-out `Error<I::Error, S::Error>`).
pub type BusError<I, S> = Error<
    <I as embedded_hal::i2c::ErrorType>::Error,
    <S as embedded_hal::spi::ErrorType>::Error,
>;

/// Returned by [`Pwbimu::power_on`] when the board does not answer the identity
/// probe after the power-up sequence.
///
/// The board has been powered back down to [`Off`] (rail off, reset asserted), so
/// [`imu`](Self::imu) is ready to retry [`power_on`](Pwbimu::power_on) — e.g.
/// after re-seating the add-on — or to drop. [`source`](Self::source) is the
/// underlying bus error from the failed identity read.
pub struct NotResponding<I: I2c, S: SpiBus> {
    /// The handle, powered back down to [`Off`] and ready to retry.
    pub imu: Pwbimu<I, S, Off>,
    /// The bus error from the failed identity read ([`reg::FW_VER`]).
    pub source: BusError<I, S>,
}

/// Lifecycle states for [`Pwbimu`]'s typestate parameter — see [`state`].
mod sealed {
    pub trait State {}
}

/// The board lifecycle as zero-sized typestate markers.
///
/// [`Pwbimu`]'s third type parameter is one of these, and each state exposes
/// only the methods legal in it, so the bring-up order is enforced at compile
/// time: you cannot read a sample before [`enable`](super::Pwbimu::enable), or
/// configure before [`power_on`](super::Pwbimu::power_on). The markers are
/// zero-sized and carried as [`PhantomData`](core::marker::PhantomData), so they
/// cost nothing at runtime.
///
/// ```text
///   new ─▶ Off ─power_on()─▶ Idle ─enable()─▶ Streaming
///                             ▲                   │
///                             └─── disable() ─────┘
///   (power_off() returns any state to Off)
/// ```
pub mod state {
    /// Just constructed: power rail off, `XRST` asserted. Only
    /// [`power_on`](super::Pwbimu::power_on) (and [`power_off`](super::Pwbimu::power_off)) exist.
    pub enum Off {}
    /// Powered and booted, output stopped: identity / [`configure`](super::Pwbimu::configure)
    /// available, plus [`enable`](super::Pwbimu::enable) to start streaming.
    pub enum Idle {}
    /// Output running: [`read_sample`](super::Pwbimu::read_sample) /
    /// [`read_sample_blocking`](super::Pwbimu::read_sample_blocking) available, plus
    /// [`disable`](super::Pwbimu::disable) to stop.
    pub enum Streaming {}

    impl super::sealed::State for Off {}
    impl super::sealed::State for Idle {}
    impl super::sealed::State for Streaming {}
}

pub use state::{Idle, Off, Streaming};

/// CXD5602PWBIMU board handle — both buses in one object, with the board
/// lifecycle tracked in the type system.
///
/// Owns everything the add-on board needs: the I2C bus `I` (any
/// [`embedded_hal::i2c::I2c`] master — typically this crate's
/// [`crate::i2c_alt::I2c`] on [`pac::I2c0`](crate::pac::I2c0)) for the control
/// plane, the SPI bus `S` (any [`embedded_hal::spi::SpiBus`] — typically
/// [`crate::spi_alt::Spi`] on [`pac::Spi5`](crate::pac::Spi5)) for the data
/// plane, plus the board's four fixed GPIOs (power, reset, chip-select, DRDY).
///
/// The `St` type parameter is a [`state`] marker that gates which methods exist;
/// the lifecycle is `new` → [`Off`] → [`power_on`](Self::power_on) → [`Idle`] →
/// [`enable`](Self::enable) → [`Streaming`]. The state transitions **consume**
/// `self` and return the handle in its new state, so the wrong-order call (read
/// before enable, configure before power-on) is a compile error rather than a
/// runtime hang or I2C NACK. All fallible methods share the [`Error`] type.
pub struct Pwbimu<I, S, St> {
    i2c: I,
    spi: S,
    power: Output<GpEmmcData2>,
    reset: Output<GpI2s0Bck>,
    csx: Output<GpI2s0DataIn>,
    drdy: Input<GpEmmcData3>,
    addr: u8,
    _state: PhantomData<St>,
}

// --- state-agnostic: construction-independent accessors + power-down ---------

impl<I, S, St: sealed::State> Pwbimu<I, S, St> {
    /// Move the handle (all owned buses and pins) into a different typestate.
    fn into_state<New: sealed::State>(self) -> Pwbimu<I, S, New> {
        Pwbimu {
            i2c: self.i2c,
            spi: self.spi,
            power: self.power,
            reset: self.reset,
            csx: self.csx,
            drdy: self.drdy,
            addr: self.addr,
            _state: PhantomData,
        }
    }

    /// Select the PSoC I2C slave address (e.g. [`ADDR_PRIMARY`] / [`ADDR_SECONDARY`]).
    ///
    /// Available in any state (it only changes which slave subsequent control
    /// writes target); set it before [`configure`](Self::configure).
    pub fn with_address(mut self, addr: u8) -> Self {
        self.addr = addr;
        self
    }

    /// The currently selected I2C slave address.
    pub fn address(&self) -> u8 {
        self.addr
    }

    /// Cut the power rail and assert reset, returning the handle to [`Off`].
    ///
    /// Drives `XRST` low (reset asserted) then the power pin low. From here you
    /// can re-sequence with [`power_on`](Self::power_on). Available in any state
    /// — including [`Streaming`], where it is the hard way to stop (it kills the
    /// rail rather than clearing `OUTPUT_ENABLE` like [`disable`](Self::disable)).
    pub fn power_off(mut self) -> Pwbimu<I, S, Off> {
        self.reset.set_low();
        self.power.set_low();
        self.into_state()
    }

    /// Borrow the underlying I2C bus.
    ///
    /// An escape hatch for control-plane registers not wrapped here. It bypasses
    /// the typestate, so prefer the dedicated methods — a raw write to
    /// [`reg::OUTPUT_ENABLE`] here will desync the tracked state.
    pub fn i2c(&mut self) -> &mut I {
        &mut self.i2c
    }

    /// Borrow the underlying SPI bus. Like [`i2c`](Self::i2c), an escape hatch
    /// that bypasses the typestate.
    pub fn spi(&mut self) -> &mut S {
        &mut self.spi
    }
}

pub struct PwbImuParts<I, S> {
    pub i2c: I,
    pub spi: S,
    pub power: GpioPin<GpEmmcData2>,
    pub reset: GpioPin<GpI2s0Bck>,
    pub csx: GpioPin<GpI2s0DataIn>,
    pub drdy: GpioPin<GpEmmcData3>,
}

// --- Off: construction + power-up --------------------------------------------

impl<I, S> Pwbimu<I, S, Off> {
    /// Consume both buses and the four board GPIOs and configure the board in its
    /// powered-off, reset-asserted, stream-idle state.
    ///
    /// The power pin is driven **low** (rail off) and `XRST` **low** (reset
    /// asserted, since it is active-low); the SPI chip-select is driven **high**
    /// (deasserted, also active-low) and DRDY is set up as a pull-down input
    /// (active-high data-ready, matching the NuttX board config). Nothing answers
    /// on either bus yet; call [`power_on`](Self::power_on) to sequence the board
    /// up. Defaults to the primary PSoC address ([`ADDR_PRIMARY`]); use
    /// [`with_address`](Self::with_address) to select another.
    ///
    /// The SPI bus must already be configured for `MODE_0`, 8 bits, 1 MHz (the
    /// default [`SpiConfig`](crate::spi_alt::SpiConfig)).
    pub fn new(parts: PwbImuParts<I, S>) -> Self {
        Self {
            i2c: parts.i2c,
            spi: parts.spi,
            power: parts.power.into_output(Level::Low),
            reset: parts.reset.into_output(Level::Low),
            csx: parts.csx.into_output(Level::High),
            drdy: parts.drdy.into_pull_down_input(),
            addr: ADDR_PRIMARY,
            _state: PhantomData,
        }
    }

    /// Sequence the board up **without** confirming it answered, advancing the
    /// handle from [`Off`] to [`Idle`].
    ///
    /// Mirrors NuttX `cxd5602pwbimu_open`: drive power **high** and let the rail
    /// settle (2 ms), pulse `XRST` low for 20 µs, then release it high and wait
    /// 150 ms for the PSoC firmware to boot.
    ///
    /// This is open-loop — it only drives GPIOs and delays, so it cannot fail,
    /// but the resulting [`Idle`] is an *unverified* claim: nothing has checked
    /// that a board is actually present and responding. Prefer
    /// [`power_on`](Self::power_on), which runs this same sequence and then probes
    /// the identity register, so reaching [`Idle`] proves the PSoC answered. Use
    /// this only when you deliberately want to drive the lines without a bus probe
    /// (e.g. bring-up testing). To re-sequence a board that is already up,
    /// [`power_off`](Self::power_off) it back to [`Off`] first.
    pub fn power_on_unchecked<D: DelayNs>(mut self, delay: &mut D) -> Pwbimu<I, S, Idle> {
        // 1. Enable the power rail and let it settle.
        self.power.set_high();
        delay.delay_ms(POWER_SETTLE_MS);
        // 2. Assert reset (XRST active-low) for the reset pulse width.
        self.reset.set_low();
        delay.delay_us(RESET_PULSE_US);
        // 3. Release reset and wait for the PSoC firmware to boot.
        self.reset.set_high();
        delay.delay_ms(BOOT_SETTLE_MS);
        self.into_state()
    }
}

// --- Off (with buses): checked power-up --------------------------------------

impl<I: I2c, S: SpiBus> Pwbimu<I, S, Off> {
    /// Sequence the board up and confirm it responds, advancing the handle from
    /// [`Off`] to [`Idle`].
    ///
    /// Runs [`power_on_unchecked`](Self::power_on_unchecked), then reads the
    /// firmware-version register ([`reg::FW_VER`] — NuttX's presence probe). On
    /// success the board is in [`Idle`], and unlike `power_on_unchecked` that
    /// state is *earned*: the PSoC answered at least once, so an absent or
    /// mis-seated board is caught right here with a clear error instead of
    /// surfacing as a confusing failure several calls later in
    /// [`configure`](Pwbimu::configure).
    ///
    /// On a probe failure the board is powered back down to [`Off`] and handed
    /// back inside [`NotResponding`], ready to retry (e.g. after re-seating the
    /// add-on). Note this proves the board was alive *at power-on*, not
    /// permanently — every later control/data call still returns a [`Result`]
    /// that surfaces a board which has since died or been unplugged.
    pub fn power_on<D: DelayNs>(
        self,
        delay: &mut D,
    ) -> Result<Pwbimu<I, S, Idle>, NotResponding<I, S>> {
        let mut imu = self.power_on_unchecked(delay);
        match imu.fw_version() {
            Ok(_) => Ok(imu),
            Err(source) => Err(NotResponding {
                imu: imu.power_off(),
                source,
            }),
        }
    }
}

// --- Idle: control plane (identity + configuration) --------------------------

impl<I: I2c, S: SpiBus> Pwbimu<I, S, Idle> {
    /// Read `buf.len()` bytes starting at register `reg` (NuttX `getregsn`).
    ///
    /// Issues the standard register-pointer-then-read transaction: write the
    /// 1-byte register address with a repeated start (no STOP), then read `buf`.
    pub fn read_regs(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), BusError<I, S>> {
        self.i2c.write_read(self.addr, &[reg], buf).map_err(Error::I2c)
    }

    /// Read a single register.
    pub fn read_reg(&mut self, reg: u8) -> Result<u8, BusError<I, S>> {
        let mut buf = [0u8];
        self.read_regs(reg, &mut buf)?;
        Ok(buf[0])
    }

    /// Read [`reg::FW_VER`] — the firmware version / de-facto `whoami` byte.
    pub fn fw_version(&mut self) -> Result<u8, BusError<I, S>> {
        self.read_reg(reg::FW_VER)
    }

    /// Read [`reg::HW_REVISION`].
    pub fn hw_revision(&mut self) -> Result<u8, BusError<I, S>> {
        self.read_reg(reg::HW_REVISION)
    }

    /// Read [`reg::HW_UNIQUE_ID`].
    pub fn hw_unique_id(&mut self) -> Result<u8, BusError<I, S>> {
        self.read_reg(reg::HW_UNIQUE_ID)
    }

    /// Read all three identity registers in one call.
    ///
    /// A successful return (and a plausible `fw_ver`) means the PSoC at the
    /// selected [`address`](Self::address) answered — the `whoami` succeeded.
    pub fn whoami(&mut self) -> Result<Identity, BusError<I, S>> {
        Ok(Identity {
            fw_ver: self.read_reg(reg::FW_VER)?,
            hw_rev: self.read_reg(reg::HW_REVISION)?,
            hw_uid: self.read_reg(reg::HW_UNIQUE_ID)?,
        })
    }

    /// Write a single control register (NuttX `putreg8`): a 2-byte I2C write of
    /// `[reg, val]`.
    pub fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), BusError<I, S>> {
        self.i2c.write(self.addr, &[reg, val]).map_err(Error::I2c)
    }

    /// Configure the board for sampling: enable the DRDY line and set the output
    /// data rate and full-scale ranges.
    ///
    /// Legal only in [`Idle`] — the PSoC accepts these writes only while the
    /// output is stopped (its `STATE_READY`), which the typestate guarantees.
    /// Writes [`reg::INTR_ENABLE`] = 1 (so DRDY pulses per sample), then
    /// [`reg::ODR`], then [`reg::FSR`] (`accel` in the high nibble, `gyro` in the
    /// low nibble). Stays in [`Idle`]; call [`enable`](Self::enable) to start.
    pub fn configure(
        &mut self,
        odr: Odr,
        accel: AccelRange,
        gyro: GyroRange,
    ) -> Result<(), BusError<I, S>> {
        self.write_reg(reg::INTR_ENABLE, 1)?;
        self.write_reg(reg::ODR, odr.code())?;
        self.write_reg(reg::FSR, (accel.code() << 4) | gyro.code())?;
        Ok(())
    }

    /// Start the 6-axis output stream ([`reg::OUTPUT_ENABLE`] = 1), advancing the
    /// handle from [`Idle`] to [`Streaming`].
    ///
    /// After this, DRDY pulses at the configured [`Odr`] and samples are read via
    /// [`read_sample`](Pwbimu::read_sample). On an I2C error the handle is
    /// consumed (not returned) — the board is left in an indeterminate state, so
    /// recover by reconstructing or power-cycling rather than retrying in place.
    pub fn enable(mut self) -> Result<Pwbimu<I, S, Streaming>, BusError<I, S>> {
        self.write_reg(reg::OUTPUT_ENABLE, 1)?;
        Ok(self.into_state())
    }
}

// --- Streaming: data plane ---------------------------------------------------

impl<I: I2c, S: SpiBus> Pwbimu<I, S, Streaming> {
    /// Whether a fresh sample is ready (DRDY asserted high).
    pub fn data_ready(&self) -> bool {
        self.drdy.is_high()
    }

    /// Read one sample over SPI (NuttX `cxd5602pwbimu_recv`).
    ///
    /// Asserts the GPIO chip-select low (with the 5 µs setup guard), exchanges
    /// [`SAMPLE_BYTES`] bytes full-duplex — the first transmitted byte carries
    /// the read flag (`0x80`), and the bytes clocked back are the sample packet
    /// — then deasserts (with the 5 µs hold guard). `delay` supplies the
    /// chip-select edge guards. Does not check DRDY; call after
    /// [`data_ready`](Self::data_ready) or use
    /// [`read_sample_blocking`](Self::read_sample_blocking).
    pub fn read_sample<D: DelayNs>(
        &mut self,
        delay: &mut D,
    ) -> Result<ImuSample, BusError<I, S>> {
        let mut buf = [0u8; SAMPLE_BYTES];
        buf[0] = SPI_READ_FLAG;

        self.csx.set_low();
        delay.delay_us(CSX_GUARD_US);
        let r = self.spi.transfer_in_place(&mut buf);
        delay.delay_us(CSX_GUARD_US);
        self.csx.set_high();
        r.map_err(Error::Spi)?;

        Ok(ImuSample::from_bytes(&buf))
    }

    /// Spin until [`data_ready`](Self::data_ready), then
    /// [`read_sample`](Self::read_sample). Blocks the core until the PSoC
    /// asserts DRDY (paced by the configured [`Odr`]).
    pub fn read_sample_blocking<D: DelayNs>(
        &mut self,
        delay: &mut D,
    ) -> Result<ImuSample, BusError<I, S>> {
        while !self.data_ready() {}
        self.read_sample(delay)
    }

    /// Stop the output stream ([`reg::OUTPUT_ENABLE`] = 0), returning the handle
    /// from [`Streaming`] to [`Idle`] (where it can be re-[`configure`](Pwbimu::configure)d).
    ///
    /// On an I2C error the handle is consumed (not returned); see
    /// [`enable`](Pwbimu::enable).
    pub fn disable(mut self) -> Result<Pwbimu<I, S, Idle>, BusError<I, S>> {
        self.write_reg_streaming(reg::OUTPUT_ENABLE, 0)?;
        Ok(self.into_state())
    }

    /// `write_reg` for the [`Streaming`] state (the public one lives on [`Idle`]).
    fn write_reg_streaming(&mut self, reg: u8, val: u8) -> Result<(), BusError<I, S>> {
        self.i2c.write(self.addr, &[reg, val]).map_err(Error::I2c)
    }
}

/// One 6-axis sample, decoded from the 32-byte SPI packet.
///
/// The PSoC firmware emits the values as little-endian IEEE-754 floats already
/// scaled to physical units — accelerations in **g**, angular rates in
/// **dps** — so no host-side scaling against the configured [`AccelRange`] /
/// [`GyroRange`] is required.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImuSample {
    /// PSoC sample counter / timestamp.
    pub timestamp: u32,
    /// Die temperature (°C).
    pub temp: f32,
    /// Gyroscope `[x, y, z]` in dps.
    pub gyro: [f32; 3],
    /// Accelerometer `[x, y, z]` in g.
    pub accel: [f32; 3],
}

impl ImuSample {
    /// Decode a raw [`SAMPLE_BYTES`]-byte SPI packet (little-endian, field order
    /// matching NuttX `cxd5602pwbimu_data_s`: timestamp, temp, gyro, accel).
    pub fn from_bytes(b: &[u8; SAMPLE_BYTES]) -> Self {
        let f = |o: usize| f32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
        Self {
            timestamp: u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            temp: f(4),
            gyro: [f(8), f(12), f(16)],
            accel: [f(20), f(24), f(28)],
        }
    }
}
