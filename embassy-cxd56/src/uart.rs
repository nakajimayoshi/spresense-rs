//! Blocking PL011 UART driver, hand-rolled directly on the chiptool PAC.
//!
//! The init sequence mirrors the NuttX-derived flow in `cxd56-hal/src/uart.rs`
//! (itself `cxd56_serial.c:454-516`), translated to the chiptool/metapac
//! register API. No external PL011 crate is used.
//!
//! Both UART instances are exposed through the generic [`Uart`] driver. The
//! base clock used for the baud divisor is **supplied by the caller** via
//! [`Config::src_clk_hz`] — see [`crate::clock`] for the (quickest) clock story.
//!
//! ```ignore
//! let p = embassy_cxd56::init();
//! let mut uart = Uart::new(
//!     p.UART1, p.SPI0_CS_X, p.SPI0_SCK,
//!     Config { src_clk_hz: COM_HZ, ..Default::default() },
//! ).unwrap();
//! writeln!(uart, "hello").ok();
//! ```

use core::fmt;

use embassy_hal_internal::{Peri, PeripheralType};
use embedded_hal_nb::nb;
use embedded_hal_nb::serial::{ErrorKind, ErrorType};

use crate::clock;
use crate::pac;
use crate::peripherals;

// Both UART1 and UART2 share the identical PL011 register layout; unify the
// driver on the UART1 register-block type and reach UART2 by reinterpreting its
// base address (same soundness argument as cxd56-hal/src/uart_alt.rs).
use pac::UART1::{regs, vals};
type Regs = pac::UART1::UART1;

/// Errors from [`Uart::new`].
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartError {
    /// Baud divisor would be zero or exceed the 16-bit IBRD register — usually
    /// a wrong or unset [`Config::src_clk_hz`].
    BadBaud,
}

/// Number of data bits per frame.
pub enum WordLength {
    Five,
    Six,
    Seven,
    Eight,
}

/// Number of stop bits per frame.
pub enum StopBits {
    One,
    Two,
}

/// Parity mode.
pub enum Parity {
    None,
    Even,
    Odd,
}

/// UART line configuration.
///
/// [`Default`] is 115 200 8N1 but **leaves `src_clk_hz` at 0**, which forces a
/// [`UartError::BadBaud`] if the caller forgets to set the base clock. Always
/// set `src_clk_hz` to the UART's base-clock rate (e.g. `clocks.com` for UART1,
/// ≈ 39 MHz for UART2).
pub struct Config {
    /// Target baud rate (e.g. `115_200`).
    pub baud_rate: u32,
    /// Base clock feeding the UART, in Hz — the divisor reference.
    pub src_clk_hz: u32,
    pub word_length: WordLength,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baud_rate: 115_200,
            src_clk_hz: 0,
            word_length: WordLength::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
        }
    }
}

/// Compute PL011 baud divisors from a clock frequency. Returns `(ibrd, fbrd)`.
/// BRD = f / (16 * baud), split as IBRD (integer) + FBRD (6-bit fraction).
/// Copied verbatim from `cxd56-hal/src/uart.rs::brd`.
fn brd(f_uart: u32, baud: u32) -> Result<(u16, u8), UartError> {
    if baud == 0 {
        return Err(UartError::BadBaud);
    }
    let brd_x64 = (f_uart as u64 * 4) / baud as u64;
    let ibrd = (brd_x64 >> 6) as u32;
    let fbrd = (brd_x64 & 0x3F) as u32;
    if ibrd == 0 || ibrd > 0xFFFF {
        return Err(UartError::BadBaud);
    }
    Ok((ibrd as u16, fbrd as u8))
}

// ---------- Pin-mux (TOPREG), translated from cxd56-hal/src/uart.rs:66-126 ----

/// UART1: route SPI0_CS_X (TXD) / SPI0_SCK (RXD) pads to Func1 = UART1.
fn uart1_pinmux() {
    // TXD: 2 mA drive (LOWEMI=1), float (PDN=1, PUN=1), input disabled (ENZI=0).
    pac::TOPREG.IO_SPI0_CS_X().write(|w| {
        w.set_LOWEMI(true);
        w.set_PDN(true);
        w.set_PUN(true);
        w.set_ENZI(false);
    });
    // RXD: same, but input enabled (ENZI=1).
    pac::TOPREG.IO_SPI0_SCK().write(|w| {
        w.set_LOWEMI(true);
        w.set_PDN(true);
        w.set_PUN(true);
        w.set_ENZI(true);
    });
    // SPI0A[13:12] = Func1 → UART1.
    pac::TOPREG.IOCSYS_IOMD0().modify(|w| w.set_SPI0A(1));
}

/// Restore SPI0A mux to Func0 (GPIO).
fn uart1_unpinmux() {
    pac::TOPREG.IOCSYS_IOMD0().modify(|w| w.set_SPI0A(0));
}

/// UART2: route P1n_00 (TXD) / P1n_01 (RXD) pads to Func1 = UART2.
fn uart2_pinmux() {
    pac::TOPREG.IO_UART2_TXD().write(|w| {
        w.set_LOWEMI(true);
        w.set_PDN(true);
        w.set_PUN(true);
        w.set_ENZI(false);
    });
    pac::TOPREG.IO_UART2_RXD().write(|w| {
        w.set_LOWEMI(true);
        w.set_PDN(true);
        w.set_PUN(true);
        w.set_ENZI(true);
    });
    // UART2[3:2] = Func1 → UART2.
    pac::TOPREG.IOCAPP_IOMD().modify(|w| w.set_UART2(1));
}

/// Restore UART2 mux to Func0 (GPIO).
fn uart2_unpinmux() {
    pac::TOPREG.IOCAPP_IOMD().modify(|w| w.set_UART2(0));
}

// ---------- Instance / pin traits ----------

mod sealed {
    pub trait Instance {}
    pub trait TxPin<T> {}
    pub trait RxPin<T> {}
}

/// A UART peripheral instance (`UART1` or `UART2`). Sealed.
#[allow(private_bounds)]
pub trait Instance: sealed::Instance + PeripheralType + 'static {
    /// MMIO base of this UART's PL011 register block.
    #[doc(hidden)]
    const BASE: usize;
    /// The TX/RX pad peripherals this UART must be given.
    #[doc(hidden)]
    type TxPin: PeripheralType;
    #[doc(hidden)]
    type RxPin: PeripheralType;

    /// Type-erased register block (UART2 reinterpreted as the UART1 layout).
    #[doc(hidden)]
    fn regs() -> Regs {
        // SAFETY: `BASE` is the fixed, aligned base of this UART's PL011 block;
        // UART1/UART2 share the identical register layout.
        unsafe { Regs::from_ptr(Self::BASE as *mut ()) }
    }
    #[doc(hidden)]
    fn enable_clock();
    #[doc(hidden)]
    fn pinmux();
    #[doc(hidden)]
    fn unpinmux();
}

/// Marker for a pad that can be this instance's TX pin. Sealed.
#[allow(private_bounds)]
pub trait TxPin<T: Instance>: sealed::TxPin<T> + PeripheralType {}
/// Marker for a pad that can be this instance's RX pin. Sealed.
#[allow(private_bounds)]
pub trait RxPin<T: Instance>: sealed::RxPin<T> + PeripheralType {}

macro_rules! impl_instance {
    ($inst:ident, $base:literal, $tx:ident, $rx:ident, $enable:path, $pinmux:path, $unpinmux:path) => {
        impl sealed::Instance for peripherals::$inst {}
        impl Instance for peripherals::$inst {
            const BASE: usize = $base;
            type TxPin = peripherals::$tx;
            type RxPin = peripherals::$rx;
            fn enable_clock() {
                $enable()
            }
            fn pinmux() {
                $pinmux()
            }
            fn unpinmux() {
                $unpinmux()
            }
        }
        impl sealed::TxPin<peripherals::$inst> for peripherals::$tx {}
        impl TxPin<peripherals::$inst> for peripherals::$tx {}
        impl sealed::RxPin<peripherals::$inst> for peripherals::$rx {}
        impl RxPin<peripherals::$inst> for peripherals::$rx {}
    };
}

impl_instance!(
    UART1, 0x041a_c000, SPI0_CS_X, SPI0_SCK,
    clock::uart1_enable, uart1_pinmux, uart1_unpinmux
);
impl_instance!(
    UART2, 0x0210_3000, P1n_00, P1n_01,
    clock::uart2_enable, uart2_pinmux, uart2_unpinmux
);

// ---------- Driver ----------

/// Blocking PL011 UART driver.
///
/// `T` is the instance token (`peripherals::UART1`/`UART2`). [`Uart::new`]
/// consumes the instance token plus its TX and RX pad tokens, so the pads
/// cannot be reused as GPIO while the UART is live. On drop the UART is disabled
/// and the pads are restored to GPIO function.
pub struct Uart<'d, T: Instance> {
    _uart: Peri<'d, T>,
    _tx: Peri<'d, T::TxPin>,
    _rx: Peri<'d, T::RxPin>,
}

impl<'d, T: Instance> Uart<'d, T> {
    /// Enable the clock + pin-mux and initialise the PL011 for `config`.
    pub fn new(
        uart: Peri<'d, T>,
        tx: Peri<'d, T::TxPin>,
        rx: Peri<'d, T::RxPin>,
        config: Config,
    ) -> Result<Self, UartError> {
        let (ibrd, fbrd) = brd(config.src_clk_hz, config.baud_rate)?;

        T::enable_clock();
        T::pinmux();
        let r = T::regs();

        // Disable UART before reconfiguring (PL011 spec §3.3.4).
        r.CR().write_value(regs::CR(0));
        r.LCR_H().write_value(regs::LCR_H(0));
        // Clear DMA enables; write RSR/ECR to clear the receive-error stickies
        // (OE/BE/PE/FE) — any write to that address clears them.
        r.DMACR().write_value(regs::DMACR(0));
        r.RSR().write_value(regs::RSR(0x0F));

        r.IBRD().write(|w| w.set_BAUD_DIVINT(ibrd));
        r.FBRD().write(|w| w.set_BAUD_DIVFRAC(fbrd));

        // LCR_H write latches the baud generator. Format bits only here — FEN is
        // a separate write below.
        r.LCR_H().write(|w| {
            w.set_WLEN(match config.word_length {
                WordLength::Five => vals::WLEN::N5bits,
                WordLength::Six => vals::WLEN::N6bits,
                WordLength::Seven => vals::WLEN::N7bits,
                WordLength::Eight => vals::WLEN::N8bits,
            });
            w.set_STP2(match config.stop_bits {
                StopBits::One => vals::STP2::NotSelected,
                StopBits::Two => vals::STP2::Selected,
            });
            match config.parity {
                Parity::None => {
                    w.set_PEN(vals::PEN::Disabled);
                }
                Parity::Even => {
                    w.set_PEN(vals::PEN::Enabled);
                    w.set_EPS(vals::EPS::EvenParity);
                }
                Parity::Odd => {
                    w.set_PEN(vals::PEN::Enabled);
                    w.set_EPS(vals::EPS::OddParity);
                }
            }
        });

        // FIFO interrupt thresholds = 1/8 full; clear all interrupt sources.
        r.IFLS().write_value(regs::IFLS(0));
        r.ICR().write_value(regs::ICR(0x7FF));

        // Enable FIFOs, then enable UART — two separate writes.
        r.LCR_H().modify(|w| w.set_FEN(vals::FEN::Enabled));
        r.CR().write(|w| {
            w.set_UARTEN(vals::UARTEN::Enabled);
            w.set_TXE(vals::TXE::Enabled);
            w.set_RXE(vals::RXE::Enabled);
        });

        Ok(Self {
            _uart: uart,
            _tx: tx,
            _rx: rx,
        })
    }

    /// Transmit one byte, blocking until the TX FIFO has room.
    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        let r = T::regs();
        while r.FR().read().TXFF() {}
        r.DR().write_value(regs::DR(byte as u32));
    }

    /// Read one byte if the RX FIFO is non-empty, otherwise `None`.
    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        let r = T::regs();
        if r.FR().read().RXFE() {
            None
        } else {
            Some(r.DR().read().0 as u8)
        }
    }

    /// Block until the TX FIFO and shift register are empty.
    #[inline]
    pub fn flush(&mut self) {
        while T::regs().FR().read().BUSY() {}
    }

    /// Enable/disable PL011 internal loopback (UARTCR.LBE) for self-test.
    #[inline]
    pub fn set_loopback(&mut self, on: bool) {
        T::regs().CR().modify(|w| {
            w.set_LBE(if on {
                vals::LBE::Enabled
            } else {
                vals::LBE::Disabled
            })
        });
    }
}

impl<T: Instance> Drop for Uart<'_, T> {
    fn drop(&mut self) {
        // Disable the UART and restore the pads to GPIO before the Peri tokens
        // release the pins.
        T::regs().CR().write_value(regs::CR(0));
        T::unpinmux();
    }
}

impl<T: Instance> fmt::Write for Uart<'_, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl<T: Instance> ErrorType for Uart<'_, T> {
    type Error = ErrorKind;
}

impl<T: Instance> embedded_hal_nb::serial::Read<u8> for Uart<'_, T> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        match self.read_byte() {
            Some(b) => Ok(b),
            None => Err(nb::Error::WouldBlock),
        }
    }
}

impl<T: Instance> embedded_hal_nb::serial::Write<u8> for Uart<'_, T> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let r = T::regs();
        if r.FR().read().TXFF() {
            Err(nb::Error::WouldBlock)
        } else {
            r.DR().write_value(regs::DR(word as u32));
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if T::regs().FR().read().BUSY() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl<T: Instance> embedded_io::ErrorType for Uart<'_, T> {
    type Error = embedded_io::ErrorKind;
}

impl<T: Instance> embedded_io::Write for Uart<'_, T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for &byte in buf {
            self.write_byte(byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while T::regs().FR().read().BUSY() {}
        Ok(())
    }
}

impl<T: Instance> embedded_io::Read for Uart<'_, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let r = T::regs();
        while r.FR().read().RXFE() {}
        let mut count = 0;
        while count < buf.len() {
            if r.FR().read().RXFE() {
                break;
            }
            buf[count] = r.DR().read().0 as u8;
            count += 1;
        }
        Ok(count)
    }
}
