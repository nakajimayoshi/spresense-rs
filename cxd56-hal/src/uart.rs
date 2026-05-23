use crate::clocks::{ClockError, Clocks, PeripheralId};
use crate::pac;
use core::fmt;
use embedded_hal_nb::nb;
use embedded_hal_nb::serial::{ErrorKind, ErrorType};

#[derive(Debug)]
pub enum UartError {
    /// Baud rate divisor would be zero or exceed the 16-bit IBRD register.
    BadBaud,
    /// IMG-UART clock could not be enabled.
    Clock(ClockError),
}

impl From<ClockError> for UartError {
    fn from(e: ClockError) -> Self {
        Self::Clock(e)
    }
}

pub enum WordLength {
    Five,
    Six,
    Seven,
    Eight,
}

pub enum StopBits {
    One,
    Two,
}

pub enum Parity {
    None,
    Even,
    Odd,
}

pub struct UartConfig {
    pub baud_rate: u32,
    pub word_length: WordLength,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baud_rate: 115_200,
            word_length: WordLength::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
        }
    }
}

/// Blocking driver for UART2 (the IMG-domain UART connected to the on-board
/// CP2102N USB-to-serial bridge on the Spresense main board).
pub struct Uart2 {
    uart: pac::Uart2,
}

impl Uart2 {
    /// Initialise UART2 with the given baud rate and frame format.
    ///
    /// Enables the IMG-UART clock (≈ 39 MHz) and programs the PL011
    /// control registers.  Must be called after `Clocks::freeze`.
    pub fn new(
        uart: pac::Uart2,
        clocks: &Clocks,
        config: UartConfig,
    ) -> Result<Self, UartError> {
        PeripheralId::ImgUart.enable()?;

        let f_uart = clocks.img_uart.to_Hz();

        // PL011 baud-rate divisor: BRD = f_uart / (16 * baud)
        // Compute as brd_x64 = f_uart * 4 / baud (avoids overflow for typical
        // embedded clocks; f_uart ≤ 156 MHz fits in u64).
        let brd_x64 = (f_uart as u64 * 4) / config.baud_rate as u64;
        let ibrd = (brd_x64 >> 6) as u32;
        let fbrd = (brd_x64 & 0x3F) as u32;

        // IBRD must be non-zero (IBRD=0 disables the UART) and fit in 16 bits.
        if ibrd == 0 || ibrd > 0xFFFF {
            return Err(UartError::BadBaud);
        }

        // Disable UART before reconfiguring (PL011 spec §3.3.4).
        uart.cr().write(|w| unsafe { w.bits(0) });

        uart.ibrd()
            .write(|w| unsafe { w.baud_divint().bits(ibrd as u16) });
        uart.fbrd()
            .write(|w| unsafe { w.baud_divfrac().bits(fbrd as u8) });

        // LCR_H must be written after IBRD/FBRD; the write latches the
        // divisors into the baud-rate generator (PL011 spec §3.3.4).
        uart.lcr_h().write(|w| {
            let w = w.fen().enabled(); // enable FIFOs
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

        uart.cr()
            .write(|w| w.uarten().enabled().txe().enabled().rxe().enabled());

        Ok(Self { uart })
    }

    /// Transmit one byte, blocking until the TX FIFO has room.
    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        while self.uart.fr().read().txff().bit_is_set() {}
        self.uart
            .dr()
            .write(|w| unsafe { w.bits(byte as u32) });
    }

    /// Read one byte if the RX FIFO is non-empty, otherwise return `None`.
    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.uart.fr().read().rxfe().bit_is_set() {
            None
        } else {
            Some(self.uart.dr().read().bits() as u8)
        }
    }

    /// Block until the TX FIFO and shift register are empty.
    #[inline]
    pub fn flush(&mut self) {
        while self.uart.fr().read().busy().bit_is_set() {}
    }
}

impl fmt::Write for Uart2 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl ErrorType for Uart2 {
    type Error = ErrorKind;
}

impl embedded_hal_nb::serial::Read<u8> for Uart2 {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        match self.read_byte() {
            Some(b) => Ok(b),
            None => Err(nb::Error::WouldBlock),
        }
    }
}

impl embedded_hal_nb::serial::Write<u8> for Uart2 {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.uart.fr().read().txff().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            self.uart.dr().write(|w| unsafe { w.bits(word as u32) });
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.uart.fr().read().busy().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}
