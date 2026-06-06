use crate::clocks::{Clock, PeripheralId};
use crate::pac;
use crate::uart::{Parity, StopBits, UartConfig, UartError, WordLength, brd, uart1_pinmux, uart2_pinmux};
use core::fmt;
use core::marker::PhantomData;
use embedded_hal_nb::nb;
use embedded_hal_nb::serial::{ErrorKind, ErrorType};
use fugit::Hertz;

// Run the PL011 initialisation sequence over a type-erased register block.
//
// UART1 and UART2 both implement PL011. Their RegisterBlock types are distinct
// in the PAC but identical at every offset this driver touches; the only
// difference is offset 0x1C (UART1 = reserved, UART2 = RTO register), which
// is never accessed here. Casting UART2's base address to
// *const uart1::RegisterBlock is therefore sound.
fn pl011_init(
    uart: &pac::uart1::RegisterBlock,
    f_uart: u32,
    config: &UartConfig,
) -> Result<(), UartError> {
    let (ibrd, fbrd) = brd(f_uart, config.baud_rate)?;

    uart.cr().write(|w| unsafe { w.bits(0) });
    uart.lcr_h().write(|w| unsafe { w.bits(0) });
    uart.dmacr().write(|w| unsafe { w.bits(0) });
    uart.rsr()
        .write(|w| w.roe().error().rbe().error().rpe().error().rfe().error());

    uart.ibrd().write(|w| unsafe { w.baud_divint().bits(ibrd) });
    uart.fbrd()
        .write(|w| unsafe { w.baud_divfrac().bits(fbrd) });

    uart.lcr_h().write(|w| {
        let w = match config.word_length {
            WordLength::Eight => w.wlen()._8bits(),
            WordLength::Seven => w.wlen()._7bits(),
            WordLength::Six => w.wlen()._6bits(),
            WordLength::Five => w.wlen()._5bits(),
        };
        let w = match config.stop_bits {
            StopBits::One => w.stp2().not_selected(),
            StopBits::Two => w.stp2().selected(),
        };
        match config.parity {
            Parity::None => w.pen().disabled(),
            Parity::Even => w.pen().enabled().eps().even_parity(),
            Parity::Odd => w.pen().enabled().eps().odd_parity(),
        }
    });

    uart.ifls().write(|w| unsafe { w.bits(0) });
    uart.icr().write(|w| unsafe { w.bits(0x7ff) });

    uart.lcr_h()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 4)) }); // FEN
    uart.cr()
        .write(|w| w.uarten().enabled().txe().enabled().rxe().enabled());

    Ok(())
}

mod sealed {
    pub trait Sealed {}
}

/// Maps a PAC UART token to its HAL wiring: clock-gate id, register base,
/// and pin-mux routine.
///
/// Sealed — implemented only for [`pac::Uart1`] and [`pac::Uart2`] within
/// this crate. Downstream code cannot add new implementors.
pub trait UartPeriph: sealed::Sealed {
    /// Clock-gate / reset id used by [`PeripheralId::enable`].
    const ID: PeripheralId;
    /// Route this UART's signals to the board pins.
    fn pinmux();
    /// Register base, type-erased to `uart1::RegisterBlock`.
    ///
    /// UART2's `RegisterBlock` is identical at every offset this driver
    /// touches (see `pl011_init`), so the cast from `uart2::RegisterBlock`
    /// is sound.
    fn regs() -> *const pac::uart1::RegisterBlock;

    /// The type returned by [`UartBuilder::build`] and how its lifetime
    /// relates to the [`Clock`] borrow:
    ///
    /// - `pac::Uart1`: `Output<'a> = Uart<'static>` — COM is a Fixed clock;
    ///   the borrow of `Clock` ends at `build`, pinning nothing.
    /// - `pac::Uart2`: `Output<'a> = Uart<'a>` — IMG_UART is a Dyn clock;
    ///   the UART borrows `Clock` for `'a`, blocking
    ///   [`Clock::request_perf`] until dropped.
    type Output<'a>;

    /// Sample this peripheral's base clock. Returns a `Copy` [`Hertz`] so
    /// the borrow of `clock` ends here; any lifetime tie lives in `Output`.
    fn base_hz(clock: &Clock) -> Hertz<u32>;

    /// Wrap an initialised register base and gate id in the
    /// correctly-lifetimed driver. Bodies are textually identical across
    /// impls; only the return *type* (and therefore the `'a` propagation)
    /// differs.
    fn wrap<'a>(regs: *const pac::uart1::RegisterBlock, id: PeripheralId)
        -> Self::Output<'a>;
}

impl sealed::Sealed for pac::Uart1 {}
impl UartPeriph for pac::Uart1 {
    const ID: PeripheralId = PeripheralId::Uart1;
    fn pinmux() { uart1_pinmux() }
    fn regs() -> *const pac::uart1::RegisterBlock { pac::Uart1::PTR }

    // COM is Fixed/Copy — Output<'a> = Uart<'static>; 'a is unused in the
    // returned value (the PhantomData marker is 'static). The Clock borrow
    // ends at the call site, pinning nothing.
    type Output<'a> = Uart<'static>;
    fn base_hz(clock: &Clock) -> Hertz<u32> { clock.com.hz() }
    fn wrap<'a>(regs: *const pac::uart1::RegisterBlock, id: PeripheralId)
        -> Self::Output<'a> { Uart { regs, id, _life: PhantomData } }
}

impl sealed::Sealed for pac::Uart2 {}
impl UartPeriph for pac::Uart2 {
    const ID: PeripheralId = PeripheralId::ImgUart;
    fn pinmux() { uart2_pinmux() }
    fn regs() -> *const pac::uart1::RegisterBlock { pac::Uart2::PTR as *const _ }

    // IMG_UART is Dyn — Output<'a> = Uart<'a>; the returned Uart borrows
    // the Clock for 'a, blocking Clock::request_perf until dropped.
    type Output<'a> = Uart<'a>;
    fn base_hz(clock: &Clock) -> Hertz<u32> { clock.img_uart().hz() }
    fn wrap<'a>(regs: *const pac::uart1::RegisterBlock, id: PeripheralId)
        -> Self::Output<'a> { Uart { regs, id, _life: PhantomData } }
}

/// Builder for a type-erased, profile-aware UART driver.
///
/// Generic over the PAC peripheral token `U` (`pac::Uart1` or `pac::Uart2`).
/// Consuming that token ensures exclusive hardware ownership. Use
/// [`UartBuilder::new`] to construct the builder — `U` is inferred from the
/// token you pass — then call [`build`](UartBuilder::build) to obtain a
/// [`Uart`]. The return type and its lifetime are determined by `U` via
/// [`UartPeriph::Output`]:
///
/// - `UartBuilder<pac::Uart1>` — COM is a Fixed clock; `.build` returns
///   [`Uart<'static>`], pinning nothing.
/// - `UartBuilder<pac::Uart2>` — IMG_UART is a Dyn clock; `.build` returns
///   [`Uart<'a>`] that borrows the [`Clock`], preventing
///   [`Clock::request_perf`] while the UART is alive.
pub struct UartBuilder<U> {
    config: UartConfig,
    _uart: PhantomData<U>,
}

impl<U: UartPeriph> UartBuilder<U> {
    /// Consume `uart` (the PAC ownership token) and record `config`.
    ///
    /// `U` is inferred from the token — no need to name it explicitly:
    /// ```ignore
    /// let builder = UartBuilder::new(dp.uart1, UartConfig::default());
    /// ```
    pub fn new(uart: U, config: UartConfig) -> Self {
        let _ = uart; // ZST ownership token; no destructor to run
        Self { config, _uart: PhantomData }
    }

    fn init(self, hz: Hertz<u32>)
        -> Result<(*const pac::uart1::RegisterBlock, PeripheralId), UartError>
    {
        U::ID.enable()?;
        U::pinmux();
        let regs = U::regs();
        pl011_init(unsafe { &*regs }, hz.to_Hz(), &self.config)?;
        Ok((regs, U::ID))
    }

    /// Enable the peripheral, run the PL011 init sequence, and return the
    /// correctly-lifetimed driver.
    ///
    /// The return type is resolved per token via [`UartPeriph::Output`]:
    /// - `U = pac::Uart1` → [`Uart<'static>`]: COM is Fixed/Copy; the borrow
    ///   of `clock` ends here, pinning nothing.
    /// - `U = pac::Uart2` → [`Uart<'a>`]: IMG_UART is Dyn; the returned
    ///   `Uart` borrows `clock` for `'a`, preventing
    ///   [`Clock::request_perf`] (needs `&mut Clock`) until dropped.
    pub fn build<'a>(self, clock: &'a Clock) -> Result<U::Output<'a>, UartError> {
        let hz = U::base_hz(clock);
        let (regs, id) = self.init(hz)?;
        Ok(U::wrap(regs, id))
    }
}

/// Type-erased, blocking PL011 UART driver.
///
/// The lifetime `'a` is:
/// - `'static` when built from [`UartBuilder<pac::Uart1>`] — UART1 uses the
///   fixed COM clock whose rate is independent of the APP operating point.
/// - Tied to the [`Clock`](crate::clocks::Clock) when built from
///   [`UartBuilder<pac::Uart2>`] — UART2 uses the dynamic IMG_UART clock,
///   so the UART borrows the `Clock`, preventing [`Clock::request_perf`]
///   until this value is dropped.
pub struct Uart<'a> {
    regs: *const pac::uart1::RegisterBlock,
    id: PeripheralId,
    _life: PhantomData<&'a ()>,
}

// The raw pointer is logically owned and exclusively controlled, matching
// the PAC `Periph` Send impl.
unsafe impl Send for Uart<'_> {}

impl Drop for Uart<'_> {
    fn drop(&mut self) {
        self.id.disable().ok();
    }
}

impl<'a> Uart<'a> {
    #[inline]
    fn regs(&self) -> &pac::uart1::RegisterBlock {
        unsafe { &*self.regs }
    }

    /// Transmit one byte, blocking until the TX FIFO has room.
    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        while self.regs().fr().read().txff().bit_is_set() {}
        self.regs().dr().write(|w| unsafe { w.bits(byte as u32) });
    }

    /// Read one byte if the RX FIFO is non-empty, otherwise return `None`.
    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.regs().fr().read().rxfe().bit_is_set() {
            None
        } else {
            Some(self.regs().dr().read().bits() as u8)
        }
    }

    /// Block until the TX FIFO and shift register are empty.
    #[inline]
    pub fn flush(&mut self) {
        while self.regs().fr().read().busy().bit_is_set() {}
    }
}

impl fmt::Write for Uart<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl ErrorType for Uart<'_> {
    type Error = ErrorKind;
}

impl embedded_hal_nb::serial::Read<u8> for Uart<'_> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        match self.read_byte() {
            Some(b) => Ok(b),
            None => Err(nb::Error::WouldBlock),
        }
    }
}

impl embedded_hal_nb::serial::Write<u8> for Uart<'_> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.regs().fr().read().txff().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            self.regs().dr().write(|w| unsafe { w.bits(word as u32) });
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.regs().fr().read().busy().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl embedded_io::ErrorType for Uart<'_> {
    type Error = embedded_io::ErrorKind;
}

impl embedded_io::Write for Uart<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for &byte in buf {
            self.write_byte(byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while nb::block!(<Self as embedded_hal_nb::serial::Write<u8>>::flush(self)).is_err() {}
        Ok(())
    }
}

impl embedded_io::Read for Uart<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        while self.regs().fr().read().rxfe().bit_is_set() {}
        let mut count = 0;
        while count < buf.len() {
            if self.regs().fr().read().rxfe().bit_is_set() {
                break;
            }
            buf[count] = self.regs().dr().read().bits() as u8;
            count += 1;
        }
        Ok(count)
    }
}
