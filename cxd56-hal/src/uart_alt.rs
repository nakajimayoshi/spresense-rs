use arm_pl011_uart as pl011;
use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use embedded_hal_nb::nb;
use embedded_hal_nb::serial::ErrorType;
use embedded_io;
use fugit::Hertz;
use thiserror::Error;

use crate::clocks::{Clock, ClockError, PeripheralId};
use crate::pac;
use crate::uart::{Parity, StopBits, UartConfig, WordLength, uart1_pinmux, uart2_pinmux};

// Map our config types to the external driver's LineConfig.
fn line_config(config: &UartConfig) -> pl011::LineConfig {
    pl011::LineConfig {
        data_bits: match config.word_length {
            WordLength::Five => pl011::DataBits::Bits5,
            WordLength::Six => pl011::DataBits::Bits6,
            WordLength::Seven => pl011::DataBits::Bits7,
            WordLength::Eight => pl011::DataBits::Bits8,
        },
        parity: match config.parity {
            Parity::None => pl011::Parity::None,
            Parity::Even => pl011::Parity::Even,
            Parity::Odd => pl011::Parity::Odd,
        },
        stop_bits: match config.stop_bits {
            StopBits::One => pl011::StopBits::One,
            StopBits::Two => pl011::StopBits::Two,
        },
    }
}

mod sealed {
    use fugit::Hertz;

    use super::pac;
    use crate::clocks::{Clock, PeripheralId};

    pub trait Sealed {
        const ID: PeripheralId;
        /// Route this UART's signals to the board pins.
        fn pinmux();
        /// Register base, type-erased to `uart1::RegisterBlock`.
        ///
        /// Used to derive the `PL011Registers` pointer for the external driver.
        /// Both UART1 and UART2 share the same PL011 register layout over the
        /// 0x1000-byte MMIO window; the cast of UART2's base address is sound on
        /// the same argument as the former direct PAC-cast approach.
        fn regs() -> *const pac::uart1::RegisterBlock;

        /// Sample this peripheral's base clock. Returns a `Copy` [`Hertz`] so
        /// the borrow of `clock` ends here; any lifetime tie lives in `Output`.
        fn base_hz(clock: &Clock) -> Hertz<u32>;
    }
}

/// Maps a PAC UART token to its HAL wiring: clock-gate id, register base,
/// and pin-mux routine.
///
/// Sealed — implemented only for [`pac::Uart1`] and [`pac::Uart2`] within
/// this crate. Downstream code cannot add new implementors.
pub trait UartPeriph: sealed::Sealed {
    /// The type returned by [`Uart::new`] and how its lifetime relates to the
    /// [`Clock`] borrow:
    ///
    /// - `pac::Uart1`: `Output<'clk> = Uart<'static, pac::Uart1>` — COM is a
    ///   Fixed clock; the borrow of `Clock` ends at `new`, pinning nothing.
    /// - `pac::Uart2`: `Output<'clk> = Uart<'clk, pac::Uart2>` — IMG_UART is
    ///   a Dyn clock; the UART borrows `Clock` for `'clk`, blocking
    ///   [`Clock::request_perf`] until dropped.
    type Output<'clk>;

    /// Wrap an initialised inner driver in the correctly-lifetimed driver
    /// struct. Bodies are textually identical across impls; only the return
    /// *type* (and therefore the `'clk` propagation) differs.
    #[doc(hidden)]
    fn wrap<'clk>(inner: pl011::Uart<'static>) -> Self::Output<'clk>;
}

impl sealed::Sealed for pac::Uart1 {
    const ID: PeripheralId = PeripheralId::Uart1;

    fn pinmux() {
        uart1_pinmux()
    }

    fn regs() -> *const pac::uart1::RegisterBlock {
        pac::Uart1::PTR
    }

    fn base_hz(clock: &Clock) -> Hertz<u32> {
        clock.com.hz()
    }
}
impl UartPeriph for pac::Uart1 {
    // COM is Fixed/Copy — Output<'clk> = Uart<'static, _>; 'clk is unused in
    // the returned value (the PhantomData marker is 'static). The Clock borrow
    // ends at the call site, pinning nothing.
    type Output<'clk> = Uart<'static, pac::Uart1>;

    fn wrap<'clk>(inner: pl011::Uart<'static>) -> Self::Output<'clk> {
        Uart {
            inner,
            _uart: PhantomData,
            _life: PhantomData,
        }
    }
}

impl sealed::Sealed for pac::Uart2 {
    const ID: PeripheralId = PeripheralId::ImgUart;
    fn pinmux() {
        uart2_pinmux()
    }
    fn regs() -> *const pac::uart1::RegisterBlock {
        pac::Uart2::PTR as *const _
    }
    fn base_hz(clock: &Clock) -> Hertz<u32> {
        clock.img_uart().hz()
    }
}
impl UartPeriph for pac::Uart2 {
    // IMG_UART is Dyn — Output<'clk> = Uart<'clk, _>; the returned Uart
    // borrows the Clock for 'clk, blocking Clock::request_perf until dropped.
    type Output<'clk> = Uart<'clk, pac::Uart2>;

    fn wrap<'clk>(inner: pl011::Uart<'static>) -> Self::Output<'clk> {
        Uart {
            inner,
            _uart: PhantomData,
            _life: PhantomData,
        }
    }
}

/// Errors from the profile-aware UART driver.
///
/// Wraps foreign error types so they are not re-exported through this
/// module's public API. Use [`core::error::Error::source`] to inspect the
/// underlying cause.
#[derive(Debug, Error)]
pub enum UartError {
    /// Clock gate enable or disable failed.
    #[error("clock gate error: {0}")]
    Clock(#[from] ClockError),
    /// PL011 driver error (baud divisor overflow, or RX overrun/parity/framing/break).
    #[error("pl011 error: {0}")]
    Pl011(#[from] pl011::Error),
}

impl embedded_hal_nb::serial::Error for UartError {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        match self {
            UartError::Pl011(e) => embedded_hal_nb::serial::Error::kind(e),
            UartError::Clock(_) => embedded_hal_nb::serial::ErrorKind::Other,
        }
    }
}

impl embedded_io::Error for UartError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            UartError::Pl011(e) => embedded_io::Error::kind(e),
            UartError::Clock(_) => embedded_io::ErrorKind::Other,
        }
    }
}

/// Generic, profile-aware PL011 UART driver backed by [`arm_pl011_uart::Uart`].
///
/// `U` is the PAC UART token (`pac::Uart1` or `pac::Uart2`). Consuming the
/// token at construction ensures exclusive hardware ownership.
///
/// The lifetime `'clk` is:
/// - `'static` when built from [`pac::Uart1`] — UART1 uses the fixed COM
///   clock whose rate is independent of the APP operating point; the borrow
///   of `Clock` ends at [`Uart::new`].
/// - Tied to the [`Clock`](crate::clocks::Clock) when built from
///   [`pac::Uart2`] — UART2 uses the dynamic IMG_UART clock, so the UART
///   borrows the `Clock`, preventing [`Clock::request_perf`] until dropped.
///
/// Use [`Uart::new`] to construct the driver — `U` is inferred from the PAC
/// token you pass. The return type and its lifetime are determined by `U` via
/// [`UartPeriph::Output`].
pub struct Uart<'clk, U: UartPeriph> {
    inner: pl011::Uart<'static>,
    _uart: PhantomData<U>,
    _life: PhantomData<&'clk ()>,
}

// Exclusive peripheral ownership — the PAC token was consumed at construction,
// matching the PAC `Periph` Send impl.
unsafe impl<U: UartPeriph> Send for Uart<'_, U> {}

impl<'clk, U: UartPeriph> Uart<'clk, U> {
    /// Enable the peripheral, run the PL011 init sequence, and return the
    /// correctly-lifetimed driver.
    ///
    /// Consumes `uart` (the PAC ownership token, a ZST) for exclusive hardware
    /// access. The return type is resolved per token via [`UartPeriph::Output`]:
    ///
    /// - `U = pac::Uart1` → [`Uart<'static, pac::Uart1>`]: COM is Fixed/Copy;
    ///   the borrow of `clock` ends here, pinning nothing.
    /// - `U = pac::Uart2` → [`Uart<'a, pac::Uart2>`]: IMG_UART is Dyn; the
    ///   returned `Uart` borrows `clock` for `'a`, preventing
    ///   [`Clock::request_perf`] (needs `&mut Clock`) until dropped.
    #[allow(clippy::new_ret_no_self)] // intentional: returns U::Output<'a> (branded by the clock-lifetime GAT)
    pub fn new<'a>(uart: U, config: UartConfig, clock: &'a Clock) -> Result<U::Output<'a>, UartError> {
        let hz = U::base_hz(clock);
        let _ = uart; // ZST ownership token; no destructor to run
        U::ID.enable()?;
        U::pinmux();
        // SAFETY: `U::regs()` returns the fixed, properly-aligned base address of
        // this UART's PL011 register block. We consumed the PAC token `U` above,
        // ensuring there is no other alias to this peripheral for the program's
        // lifetime. Both UART1 and UART2 expose the same PL011 register layout
        // over the 0x1000-byte MMIO window (`PL011Registers` is
        // `#[repr(C, align(4))]`); casting UART2's base address is sound on the
        // same argument as the former direct PAC-cast. The `'static` lifetime
        // reflects that the MMIO address is valid for the entire program.
        let ptr = NonNull::new(U::regs() as *mut pl011::PL011Registers)
            .expect("PL011 base address is null");
        let mut inner = pl011::Uart::new(unsafe { pl011::UniqueMmioPointer::new(ptr) });
        inner.enable(line_config(&config), config.baud_rate, hz.to_Hz())?;
        Ok(U::wrap(inner))
    }

    /// Transmit one byte, blocking until the TX FIFO has room.
    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        while self.inner.is_tx_fifo_full() {}
        self.inner.write_word(byte);
    }

    /// Read one byte if the RX FIFO is non-empty, otherwise return `None`.
    ///
    /// Returns `None` on both an empty FIFO and RX errors (overrun/parity/
    /// framing/break); use the [`embedded_hal_nb`] or [`embedded_io`] trait
    /// impls to observe error detail.
    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        self.inner.read_word().ok().flatten()
    }

    /// Block until the TX FIFO and shift register are empty.
    #[inline]
    pub fn flush(&mut self) {
        while self.inner.is_busy() {}
    }
}

impl<U: UartPeriph> Drop for Uart<'_, U> {
    fn drop(&mut self) {
        U::ID.disable().ok();
    }
}

impl<U: UartPeriph> fmt::Write for Uart<'_, U> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        fmt::Write::write_str(&mut self.inner, s)
    }
}

impl<U: UartPeriph> ErrorType for Uart<'_, U> {
    type Error = UartError;
}

impl<U: UartPeriph> embedded_hal_nb::serial::Read<u8> for Uart<'_, U> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        embedded_hal_nb::serial::Read::read(&mut self.inner)
            .map_err(|e| e.map(UartError::from))
    }
}

impl<U: UartPeriph> embedded_hal_nb::serial::Write<u8> for Uart<'_, U> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        embedded_hal_nb::serial::Write::write(&mut self.inner, word)
            .map_err(|e| e.map(UartError::from))
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        embedded_hal_nb::serial::Write::flush(&mut self.inner)
            .map_err(|e| e.map(UartError::from))
    }
}

impl<U: UartPeriph> embedded_io::ErrorType for Uart<'_, U> {
    type Error = UartError;
}

impl<U: UartPeriph> embedded_io::Write for Uart<'_, U> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        embedded_io::Write::write(&mut self.inner, buf).map_err(UartError::from)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        embedded_io::Write::flush(&mut self.inner).map_err(UartError::from)
    }
}

impl<U: UartPeriph> embedded_io::Read for Uart<'_, U> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        embedded_io::Read::read(&mut self.inner, buf).map_err(UartError::from)
    }
}
