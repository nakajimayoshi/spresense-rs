//! Profile-aware PL022 SSP (SPI) driver for the CXD5602.
//!
//! Wraps the ARM PrimeCell PL022 Synchronous Serial Port controller present on
//! the CXD5602's **SPI5** (`IMG WSPI`) block. Follows the same sealed-trait +
//! GAT lifetime pattern as [`crate::uart_alt`]:
//!
//! - The struct is generic over the PAC SPI token (`pac::Spi5`).
//! - `SPI5` uses the **dynamic** `img_wspi` clock (derived from `appsmp` via a
//!   gear divider, re-sampled by [`Clock::request_perf`]). Therefore
//!   `Spi::new` returns `Spi<'clk, pac::Spi5>`, where `'clk` borrows the
//!   [`Clock`] and prevents a perf-mode change while the bus is live.
//!
//! # Supported peripherals
//!
//! | Token | Block | Headers | Max SCK |
//! |---|---|---|---|
//! | [`pac::Spi5`] | IMG WSPI | JP1-8/9 (CS/SCK), JP2-8/9 (MISO/MOSI) | ~13 Mbps (HP) / ~6.5 Mbps (LP) |
//!
//! # Voltage
//!
//! **SPI5 pads are 1.8 V.** Never connect to 3.3 V or 5 V signals.
//!
//! # Usage
//!
//! ```no_run
//! use cxd56_hal::spi_alt::{Spi, SpiConfig};
//! use cxd56_hal::pac;
//! use embedded_hal::spi::{SpiBus, MODE_0};
//! use fugit::Hertz;
//!
//! let mut spi = Spi::new(dp.spi5, pins, SpiConfig::default(), &clock)?;
//! let mut buf = [0xA5u8, 0x5A];
//! spi.transfer_in_place(&mut buf)?;
//! ```
//!
//! # Pin mapping (EMMC-A group, Func2)
//!
//! | SPI5 signal | Spresense pin | Arduino | EMMC pad |
//! |---|---|---|---|
//! | SCK | JP1-9 | D23 | EMMC_CLK |
//! | CS_X | JP1-8 | D24 | EMMC_CMD |
//! | MOSI | JP2-9 | D16 | EMMC_DATA0 |
//! | MISO | JP2-8 | D17 | EMMC_DATA1 |
//!
//! # Loopback
//!
//! Set [`SpiConfig::loopback`] to `true` to enable the PL022 internal
//! loopback mode (`SSPCR1.LBM`): the TX shift register is connected directly
//! to the RX shift register without touching the pads. No external wiring
//! required.

use core::marker::PhantomData;

use embedded_hal::spi::{ErrorType, Mode, SpiBus, MODE_0};
use fugit::Hertz;
use thiserror::Error;

use crate::clocks::{Clock, ClockError, PeripheralId, SpiPort};
use crate::gpio::GpioPin;
use crate::pac;
use crate::regs::topreg;

// ============================================================================
// Public configuration types
// ============================================================================

/// Configuration applied once at [`Spi::new`].
///
/// All fields are public; use `..Default::default()` to fill unset fields.
#[derive(Clone, Debug)]
pub struct SpiConfig {
    /// Target SCK frequency. Rounded down to the nearest achievable rate via
    /// `SSPCPSR.CPSDVSR`. Default: 1 MHz (safe for loopback self-tests).
    pub frequency: Hertz<u32>,

    /// SPI clock polarity and phase (Motorola SPI modes 0–3).
    /// Default: `MODE_0` (CPOL=0, CPHA=0).
    pub mode: Mode,

    /// Data frame size in bits (4..=16). Written to `SSPCR0.DSS` as
    /// `bits - 1`. Default: 8.
    pub bits: u8,

    /// Enable the PL022 internal loopback mode (`SSPCR1.LBM = 1`): the TX
    /// serial shifter output is connected to the RX serial shifter input
    /// internally. No external wiring required when `true`. Default: `false`.
    pub loopback: bool,
}

impl Default for SpiConfig {
    fn default() -> Self {
        Self {
            frequency: Hertz::<u32>::Hz(1_000_000),
            mode: MODE_0,
            bits: 8,
            loopback: false,
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors returned by the SPI driver.
#[derive(Debug, Error)]
pub enum SpiError {
    /// Clock gate enable or disable failed.
    #[error("clock gate error: {0}")]
    Clock(#[from] ClockError),

    /// The requested frequency cannot be achieved with the PL022 baud
    /// divisors (`CPSDVSR` ∈ 2..=254, even).
    #[error("SPI frequency out of range for baud divisors")]
    Frequency,

    /// The TX or RX FIFO did not become ready within the bounded retry count.
    #[error("SPI transfer timed out")]
    Timeout,
}

impl embedded_hal::spi::Error for SpiError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Other
    }
}

// ============================================================================
// Sealed peripheral trait
// ============================================================================

mod sealed {
    use core::ops::Deref;

    use fugit::Hertz;

    use crate::clocks::{Clock, PeripheralId};
    use crate::pac;

    /// Private trait that bounds [`super::SpiPeriph`].
    ///
    /// The `Deref` supertrait gives uniform register-block access — `&*spi`
    /// yields `&pac::spi4::RegisterBlock` regardless of which PAC token is
    /// used (same mechanism as `i2c_alt.rs`).
    pub trait Sealed: Deref<Target = pac::spi4::RegisterBlock> {
        /// Clock-gate identifier used by [`PeripheralId::enable`] /
        /// [`PeripheralId::disable`].
        const ID: PeripheralId;

        /// Route this SPI's pads to the SPI function.
        fn pinmux();

        /// Restore this SPI's pads to Func0 (GPIO).
        fn unpinmux();

        /// Read the current base clock for this SPI block from hardware.
        ///
        /// Called *after* [`PeripheralId::enable`] has programmed the gear
        /// register, so [`Clock::freeze`] returns the freshly-set value rather
        /// than the potentially-stale cached [`crate::clocks::Dyn`] field.
        fn base_hz(clock: &Clock) -> Hertz<u32>;
    }
}

/// GPIO tokens for the SPI5 pads (EMMC-A group, Func2).
///
/// Consumed by [`Spi::new`] to enforce at the type level that no other code
/// can drive these pads as GPIO while the SPI bus is live. Returned by
/// [`Spi::free`] with the pads restored to Func0 (GPIO).
///
/// | Field  | Spresense pad | Arduino | Header |
/// |--------|--------------|---------|--------|
/// | `sck`  | EMMC_CLK     | D23     | JP1-9  |
/// | `csn`  | EMMC_CMD     | D24     | JP1-8  |
/// | `mosi` | EMMC_DATA0   | D16     | JP2-9  |
/// | `miso` | EMMC_DATA1   | D17     | JP2-8  |
pub struct Spi5Pins {
    pub sck:  GpioPin<pac::topreg::GpEmmcClk>,
    pub csn:  GpioPin<pac::topreg::GpEmmcCmd>,
    pub mosi: GpioPin<pac::topreg::GpEmmcData0>,
    pub miso: GpioPin<pac::topreg::GpEmmcData1>,
}

/// Maps a PAC SPI token to its clock-gate, pin-mux, and base clock.
///
/// Sealed — implemented only for [`pac::Spi5`] today within this crate.
/// Downstream code cannot add new implementors.
pub trait SpiPeriph: sealed::Sealed {
    /// Return type from [`Spi::new`], parameterised by the lifetime of the
    /// `&Clock` borrow.
    ///
    /// `pac::Spi5` → `Spi<'clk, pac::Spi5>`: `img_wspi` is a `Dyn` clock
    /// that changes with [`Clock::request_perf`]; the `Spi` borrows `Clock`
    /// for `'clk`, blocking perf changes until dropped.
    type Output<'clk>;

    /// GPIO pin bundle consumed at construction and returned by [`Spi::free`].
    type Pins;

    #[doc(hidden)]
    fn wrap<'clk>(spi: Self, pins: Self::Pins) -> Self::Output<'clk>;
}

// ============================================================================
// pac::Spi5 impl (Dyn arm — mirrors uart_alt pac::Uart2)
// ============================================================================

impl sealed::Sealed for pac::Spi5 {
    const ID: PeripheralId = PeripheralId::Spi5;

    fn pinmux() {
        spi5_pinmux();
    }

    fn unpinmux() {
        spi5_unpinmux();
    }

    fn base_hz(clock: &Clock) -> Hertz<u32> {
        // img_wspi is Dyn. spi5_enable() (called just before base_hz) writes
        // GEAR_IMG_WSPI = 0x0001_0004 (N=1, M=4). freeze() re-reads the gear
        // from hardware to get the post-enable value rather than relying on the
        // stale cached Clock field.
        clock.freeze().spi(SpiPort::Spi5)
    }
}

impl SpiPeriph for pac::Spi5 {
    /// `img_wspi` is Dyn: the borrow of `Clock` is retained for `'clk`.
    /// Mirrors `uart_alt::UartPeriph for pac::Uart2` (`:125-137`).
    type Output<'clk> = Spi<'clk, pac::Spi5>;
    type Pins = Spi5Pins;

    fn wrap<'clk>(spi: Self, pins: Self::Pins) -> Self::Output<'clk> {
        Spi { spi, pins, _life: PhantomData }
    }
}

// ============================================================================
// Spi<'clk, S> driver struct
// ============================================================================

/// Generic, profile-aware PL022 SSP driver.
///
/// `S` is the PAC SPI token (currently only [`pac::Spi5`]).
///
/// The lifetime `'clk` is tied to the [`Clock`] borrow:
/// - `pac::Spi5` → `Spi<'clk, pac::Spi5>`: `img_wspi` is Dyn; the `Spi`
///   borrows `Clock` for `'clk`, preventing [`Clock::request_perf`] until
///   dropped — exactly as `uart_alt::Uart<'clk, pac::Uart2>`.
///
/// Use [`Spi::new`] to construct; `S` is inferred from the PAC token passed.
pub struct Spi<'clk, S: SpiPeriph> {
    spi: S,
    pins: S::Pins,
    _life: PhantomData<&'clk ()>,
}

// SAFETY: PAC `Periph` is Send (exclusive ownership token), and we consume it
// at construction. Single-core use.
unsafe impl<S: SpiPeriph + Send> Send for Spi<'_, S> {}

impl<'clk, S: SpiPeriph> Spi<'clk, S> {
    /// Enable the SPI peripheral, configure the PL022 registers, and return
    /// the correctly-lifetimed driver.
    ///
    /// `spi` (the PAC token, a ZST) is consumed to ensure exclusive hardware
    /// access. `S` is inferred from the token; no turbofish needed.
    ///
    /// The return type is resolved through [`SpiPeriph::Output`]:
    /// - `S = pac::Spi5` → [`Spi<'a, pac::Spi5>`]: `img_wspi` is Dyn; the
    ///   returned `Spi` borrows `clock` for `'a`, preventing
    ///   [`Clock::request_perf`] until dropped.
    /// Consume the SPI peripheral and its GPIO pin tokens, configure the
    /// PL022, and return the driver.
    ///
    /// `pins` enforces at the type level that no other code can use these pads
    /// as GPIO while the bus is live. Call [`Spi::free`] to release them.
    #[allow(clippy::new_ret_no_self)] // intentional: returns S::Output<'a>
    pub fn new<'a>(spi: S, pins: S::Pins, config: SpiConfig, clock: &'a Clock)
        -> Result<S::Output<'a>, SpiError>
    {
        // Enable clock gate (spi5_enable: AppSub domain + img_acquire + gear M=4).
        S::ID.enable()?;
        // Configure IO-mux to route the SPI signals to the board pads.
        S::pinmux();
        // Re-read img_wspi from hardware registers (post-enable, so the gear is set).
        let base = S::base_hz(clock).to_Hz();

        // Disable SSP before (re)configuration. PL022 requires SSE=0 to safely
        // write SSPCR0, SSPCPSR, or change LBM. We zero the whole register.
        spi.sspcr1().write(|w| unsafe { w.bits(0) });

        // Compute baud-rate divisors.
        let (cpsdvsr, scr) = baud_divisors(base, config.frequency.to_Hz())
            .ok_or(SpiError::Frequency)?;

        // SSPCPSR — clock prescale divisor (must be even, 2..=254).
        spi.sspcpsr().write(|w| unsafe { w.bits(cpsdvsr) });

        // SSPCR0 — DSS[3:0] | FRF=00 (Motorola) | SPO[6] | SPH[7] | SCR[15:8].
        let dss = ((config.bits as u32).saturating_sub(1)).clamp(3, 15);
        let (spo, sph) = mode_bits(config.mode);
        spi.sspcr0().write(|w| unsafe {
            w.bits(dss | ((spo as u32) << 6) | ((sph as u32) << 7) | (scr << 8))
        });

        // Mask all interrupts (driver polls status registers, never IRQs).
        spi.sspimsc().write(|w| unsafe { w.bits(0) });
        // Clear any pending Rx-timeout and Rx-overrun interrupt latches.
        spi.sspicr().write(|w| unsafe { w.bits(0x3) });

        // Drain any stale bytes that may be in the RX FIFO.
        while spi.sspsr().read().rne().bit_is_set() {
            let _ = spi.sspdr().read();
        }

        // SSPCR1 — master mode (MS=0), LBM from config, SSE=1.
        //   bit 0 = LBM (loopback), bit 1 = SSE (enable), bit 2 = MS (0=master).
        let cr1 = if config.loopback { 0b0011u32 } else { 0b0010u32 };
        spi.sspcr1().write(|w| unsafe { w.bits(cr1) });

        Ok(S::wrap(spi, pins))
    }

    // One full-duplex 16-bit word exchange:
    //   write `word` → TX FIFO, then read one word from RX FIFO.
    // Bounded-spin on both TNF (TX not-full) and RNE (RX not-empty).
    // Returns Timeout if either FIFO doesn't become ready within the retry limit.
    fn transfer_word(&mut self, word: u16) -> Result<u16, SpiError> {
        // Wait until TX FIFO has room.
        let mut retry = 100_000u32;
        while !self.spi.sspsr().read().tnf().bit_is_set() {
            retry = retry.checked_sub(1).ok_or(SpiError::Timeout)?;
        }
        // Write the data word to the TX FIFO.
        self.spi.sspdr().write(|w| unsafe { w.bits(word as u32) });

        // Wait until RX FIFO is non-empty.
        retry = 100_000u32;
        while !self.spi.sspsr().read().rne().bit_is_set() {
            retry = retry.checked_sub(1).ok_or(SpiError::Timeout)?;
        }
        // Read and return the received word.
        Ok(self.spi.sspdr().read().data().bits())
    }

    /// Disable the SPI bus, restore the pads to GPIO (Func0), and return the
    /// consumed PAC token and GPIO pin bundle.
    ///
    /// The returned [`S::Pins`](SpiPeriph::Pins) can be passed to
    /// [`gpio::pins::Parts`](crate::gpio::pins::Parts) methods or used directly
    /// as [`GpioPin`] tokens again.
    pub fn free(self) -> (S, S::Pins) {
        // Mirror Drop: disable SSP then gate clock.
        self.spi.sspcr1().write(|w| unsafe { w.bits(0) });
        S::ID.disable().ok();
        S::unpinmux();
        let md = core::mem::ManuallyDrop::new(self);
        // SAFETY: Each field is moved out exactly once through a raw pointer.
        // ManuallyDrop prevents the struct-level Drop (which would call
        // disable() a second time) from running.
        let spi = unsafe { core::ptr::read(&md.spi) };
        let pins = unsafe { core::ptr::read(&md.pins) };
        (spi, pins)
    }
}

impl<S: SpiPeriph> Drop for Spi<'_, S> {
    fn drop(&mut self) {
        // Disable the SSP (SSE=0, LBM=0) before gating the clock.
        self.spi.sspcr1().write(|w| unsafe { w.bits(0) });
        S::ID.disable().ok();
    }
}

// ============================================================================
// embedded-hal 1.0 SpiBus<u8>
// ============================================================================

impl<S: SpiPeriph> ErrorType for Spi<'_, S> {
    type Error = SpiError;
}

/// Full-duplex byte-oriented SPI bus.
///
/// All five methods delegate to [`transfer_word`](Spi::transfer_word), which
/// exchanges one 16-bit word at a time (only the low 8 bits are meaningful
/// when the data-frame size is 8). CS is **not** managed here (this is
/// `SpiBus`, not `SpiDevice`) — drive a GPIO CS pin yourself.
impl<S: SpiPeriph> SpiBus<u8> for Spi<'_, S> {
    /// Send `0x00` for each byte, populating `words` with received bytes.
    fn read(&mut self, words: &mut [u8]) -> Result<(), SpiError> {
        for slot in words.iter_mut() {
            *slot = self.transfer_word(0x00)? as u8;
        }
        Ok(())
    }

    /// Send every byte in `words`, discarding received bytes.
    fn write(&mut self, words: &[u8]) -> Result<(), SpiError> {
        for &w in words {
            self.transfer_word(w as u16)?;
        }
        Ok(())
    }

    /// Full-duplex transfer: send each byte in `write`, receive into `read`.
    ///
    /// If the slices differ in length the shorter side is padded (with `0x00`
    /// for TX, discarded for RX).
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), SpiError> {
        let len = read.len().max(write.len());
        for i in 0..len {
            let tx = write.get(i).copied().unwrap_or(0);
            let rx = self.transfer_word(tx as u16)? as u8;
            if let Some(slot) = read.get_mut(i) {
                *slot = rx;
            }
        }
        Ok(())
    }

    /// In-place full-duplex transfer: each byte is transmitted, then
    /// overwritten with the simultaneously received byte.
    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), SpiError> {
        for w in words.iter_mut() {
            *w = self.transfer_word(*w as u16)? as u8;
        }
        Ok(())
    }

    /// Block until the TX FIFO is empty and the SSP is idle.
    fn flush(&mut self) -> Result<(), SpiError> {
        let mut retry = 1_000_000u32;
        while self.spi.sspsr().read().bsy().bit_is_set() {
            retry = retry.checked_sub(1).ok_or(SpiError::Timeout)?;
        }
        Ok(())
    }
}

// ============================================================================
// SPI5 pinmux helper
// ============================================================================

fn spi5_unpinmux() {
    topreg()
        .iocapp_iomd()
        .modify(|_, w| unsafe { w.emmca().bits(0) });
}

// Route EMMC_CLK/CMD/DATA0/DATA1 → SPI5 SCK/CS_X/MOSI/MISO (Func2).
//
// Pad settings follow the i2c0_pinmux style (i2c_alt.rs:127-137):
//   - All pads: lowemi (4 mA → 2 mA drive, reduces EMI at SPI frequencies).
//   - MISO (DATA1): enzi set — enables the input receiver so the pad drives
//     the SPI shift register. Output pads (SCK/CMD/DATA0) don't need enzi.
fn spi5_pinmux() {
    // Output pads: lower drive strength.
    topreg()
        .io_emmc_clk()
        .write(|w| w.lowemi().set_bit());
    topreg()
        .io_emmc_cmd()
        .write(|w| w.lowemi().set_bit());
    topreg()
        .io_emmc_data0()
        .write(|w| w.lowemi().set_bit());
    // MISO pad: input receiver + pull-up (weak, prevents floating) + lowemi.
    topreg()
        .io_emmc_data1()
        .write(|w| w.enzi().set_bit().pun().set_bit().lowemi().set_bit());

    // Mux EMMC_A pin group (bits [7:6] = EMMCA) to Func2 → SPI5.
    topreg()
        .iocapp_iomd()
        .modify(|_, w| unsafe { w.emmca().bits(2) });
}

// ============================================================================
// Baud-rate and mode helpers
// ============================================================================

/// Compute PL022 baud divisors so that SCK ≤ `target_hz`.
///
/// PL022 formula: SCK = SSPCLK / (CPSDVSR × (1 + SCR))
///
/// This implementation follows the NuttX `cxd56_spi.c spi_setfrequency`
/// approach: use SCR = 0 and vary only CPSDVSR (even, 2..=254).
/// Returns `None` if `target_hz` is 0 or unachievable.
fn baud_divisors(base_hz: u32, target_hz: u32) -> Option<(u32, u32)> {
    if target_hz == 0 || base_hz == 0 {
        return None;
    }
    // Smallest even CPSDVSR such that SCK = base / CPSDVSR ≤ target.
    // Ceiling-divide base by target, then round up to even.
    let raw = base_hz.div_ceil(target_hz);       // smallest integer divisor ≥ ratio
    let cpsdvsr = (raw + 1) & !1u32;             // round up to even
    let cpsdvsr = cpsdvsr.clamp(2, 254);
    // Verify the result is achievable (for very low targets, cpsdvsr caps at 254).
    Some((cpsdvsr, 0)) // SCR = 0
}

/// Extract SPO (polarity) and SPH (phase) bits from a [`Mode`].
fn mode_bits(mode: Mode) -> (bool, bool) {
    use embedded_hal::spi::{Phase, Polarity};
    let spo = matches!(mode.polarity, Polarity::IdleHigh);       // CPOL=1 → SPO=1
    let sph = matches!(mode.phase, Phase::CaptureOnSecondTransition); // CPHA=1 → SPH=1
    (spo, sph)
}
