use crate::clocks::{DynClock, FixedClock, PeripheralId};
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

/// Builder for a type-erased, profile-aware UART driver.
///
/// Consuming the PAC peripheral token (`pac::Uart1` / `pac::Uart2`) ensures
/// exclusive ownership — no second driver can be constructed for the same
/// hardware. Call [`with_fixed_clock`] or [`with_dyn_clock`] to finish
/// construction and obtain a [`Uart`].
///
/// [`with_fixed_clock`]: UartBuilder::with_fixed_clock
/// [`with_dyn_clock`]: UartBuilder::with_dyn_clock
pub struct UartBuilder {
    base: usize,
    id: PeripheralId,
    pinmux: fn(),
    config: UartConfig,
}

impl UartBuilder {
    /// Consume `uart1` (the SYSIOP_SUB debug UART, on-board CP2102N bridge).
    pub fn uart1(uart: pac::Uart1, config: UartConfig) -> Self {
        let base = pac::Uart1::PTR as usize;
        let _ = uart; // consume the ownership token; Periph is a ZST with no destructor
        Self { base, id: PeripheralId::Uart1, pinmux: uart1_pinmux, config }
    }

    /// Consume `uart2` (the IMG-domain UART, JP1 extension header pins 2-5).
    pub fn uart2(uart: pac::Uart2, config: UartConfig) -> Self {
        let base = pac::Uart2::PTR as usize;
        let _ = uart;
        Self { base, id: PeripheralId::ImgUart, pinmux: uart2_pinmux, config }
    }

    /// Finish with a fixed clock source (rate unchanged by `request_perf`).
    ///
    /// Reads `clk.hz()` and drops the borrow — the returned [`Uart`] carries
    /// `'static`, requiring no ongoing borrow of the clock or `Crg`.
    pub fn with_fixed_clock<C: FixedClock>(self, clk: &C) -> Result<Uart<'static>, UartError> {
        let (regs, id) = self.build(clk.hz())?;
        Ok(Uart { regs, id, _life: PhantomData })
    }

    /// Finish with a dynamic clock source (rate tracks `request_perf`).
    ///
    /// The returned [`Uart`]`<'a>` borrows `clk` for `'a`, which — when `clk`
    /// is a field borrowed from a [`Clock`](crate::clocks::Clock) — pins the
    /// owning `Clock`, preventing `request_perf` while the UART is alive.
    pub fn with_dyn_clock<'a, C: DynClock>(self, clk: &'a C) -> Result<Uart<'a>, UartError> {
        let (regs, id) = self.build(clk.hz())?;
        Ok(Uart { regs, id, _life: PhantomData })
    }

    fn build(self, hz: Hertz<u32>) -> Result<(*const pac::uart1::RegisterBlock, PeripheralId), UartError> {
        self.id.enable()?;
        (self.pinmux)();
        let regs = self.base as *const pac::uart1::RegisterBlock;
        pl011_init(unsafe { &*regs }, hz.to_Hz(), &self.config)?;
        Ok((regs, self.id))
    }
}

/// Type-erased, blocking PL011 UART driver.
///
/// The lifetime `'a` is:
/// - `'static` when built via [`UartBuilder::with_fixed_clock`] — the baud
///   rate does not depend on the APP operating point.
/// - Tied to the dynamic clock source when built via
///   [`UartBuilder::with_dyn_clock`] — the UART borrows the clock, which
///   pins the owning [`Clock`](crate::clocks::Clock), preventing
///   `request_perf` until this value is dropped.
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
