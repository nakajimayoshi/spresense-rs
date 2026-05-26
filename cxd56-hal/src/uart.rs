use crate::clocks::{ClockError, Clocks, PeripheralId, buses};
use crate::pac;
use core::fmt;
use embedded_hal_nb::nb;
use embedded_hal_nb::serial::{ErrorKind, ErrorType};
use embedded_io;

#[derive(Debug)]
pub enum UartError {
    /// Baud rate divisor would be zero or exceed the 16-bit IBRD register.
    BadBaud,
    /// UART clock could not be enabled.
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

// TODO(iopad): factor pin-mux helpers into a dedicated iopad module once SPI4/SDIO/I2S need it.

// UART1 — debug console UART in the SYSIOP_SUB (COM) domain.
// TXD = SPI0_CS_X (pin 17): IO_SPI0_CS_X (TOPREG+0x844), IOCSYS_IOMD0 SPI0A=1.
// RXD = SPI0_SCK  (pin 18): IO_SPI0_SCK  (TOPREG+0x848), input enabled.
// Reference: cxd5602_pinconfig.h:510, cxd5602_topreg.h:149,159-160,
//            cxd56_pinconfig.c:53,139-141,391.
fn uart1_pinmux() {
    let topreg = unsafe { &*pac::Topreg::PTR };
    // TXD: 2mA drive (LOWEMI=1), float (PDN=1, PUN=1), input disabled (ENZI=0).
    topreg.io_spi0_cs_x().write(|w| {
        w.lowemi()
            .set_bit()
            .pdn()
            .set_bit()
            .pun()
            .set_bit()
            .enzi()
            .clear_bit()
    });
    // RXD: same but input enabled (ENZI=1).
    topreg.io_spi0_sck().write(|w| {
        w.lowemi()
            .set_bit()
            .pdn()
            .set_bit()
            .pun()
            .set_bit()
            .enzi()
            .set_bit()
    });
    // SPI0A[13:12] = Func1 → UART1.  FieldWriter<2> needs unsafe bits().
    topreg
        .iocsys_iomd0()
        .modify(|_, w| unsafe { w.spi0a().bits(1) });
}

// UART2 — IMG-domain UART connected to the extension-board JP1 header (pins 2-5).
// TXD = P1n_00 (pin 67): IO_UART2_TXD (TOPREG+0x90c), IOCAPP_IOMD UART2=1.
// RXD = P1n_01 (pin 68): IO_UART2_RXD (TOPREG+0x910), input enabled.
// Reference: CXD5602 user manual §3.1 pp.66,71-74; cxd5602_pinconfig.h:356-357,577.
fn uart2_pinmux() {
    let topreg = unsafe { &*pac::Topreg::PTR };
    // TXD: 2mA drive (LOWEMI=1), float (PDN=1, PUN=1), input disabled (ENZI=0).
    topreg.io_uart2_txd().write(|w| {
        w.lowemi()
            .set_bit()
            .pdn()
            .set_bit()
            .pun()
            .set_bit()
            .enzi()
            .clear_bit()
    });
    // RXD: same but input enabled (ENZI=1).
    topreg.io_uart2_rxd().write(|w| {
        w.lowemi()
            .set_bit()
            .pdn()
            .set_bit()
            .pun()
            .set_bit()
            .enzi()
            .set_bit()
    });
    // UART2[3:2] = Func1 → UART2.  FieldWriter<2> needs unsafe bits().
    topreg
        .iocapp_iomd()
        .modify(|_, w| unsafe { w.uart2().bits(1) });
}

/// Compute baud-rate divisors from a clock frequency. Returns `(ibrd, fbrd)` or
/// `Err(UartError::BadBaud)` if the divisor is zero or overflows the 16-bit IBRD.
/// PL011 baud: BRD = f / (16 * baud), split as IBRD (integer) + FBRD (6-bit fraction).
fn brd(f_uart: u32, baud: u32) -> Result<(u16, u8), UartError> {
    let brd_x64 = (f_uart as u64 * 4) / baud as u64;
    let ibrd = (brd_x64 >> 6) as u32;
    let fbrd = (brd_x64 & 0x3F) as u32;
    if ibrd == 0 || ibrd > 0xFFFF {
        return Err(UartError::BadBaud);
    }
    Ok((ibrd as u16, fbrd as u8))
}

// ---------- PL011 init sequence (mirrors cxd56_serial.c:454-516) ----------
//
// The sequence is identical for UART1 and UART2. svd2rust generates distinct
// types for each peripheral, so the body is duplicated. TODO(pl011): factor
// into a macro when a third UART instance is added.

/// Blocking driver for UART1 (the SYSIOP_SUB "debug" UART wired to the
/// on-board CP2102N USB-to-serial bridge on the Spresense main board).
pub struct Uart1 {
    uart: pac::Uart1,
}

impl Uart1 {
    /// Initialise UART1 with the given baud rate and frame format.
    ///
    /// Enables the COM-bus clock, programs the SPI0A pinmux to UART1 mode,
    /// and initialises the PL011 control registers. Must be called after
    /// `Clocks::freeze`. No hardware flow control — UART1 has none on the
    /// Spresense main board.
    pub fn new(uart: pac::Uart1, clocks: &Clocks, config: UartConfig) -> Result<Self, UartError> {
        PeripheralId::Uart1.enable()?;
        uart1_pinmux();

        // clocks.com is the COM baseclock (cxd56_get_com_baseclock). UART1 has
        // no per-port gear register, so this value is stable from freeze() onward.
        let f_uart = clocks.com.to_Hz();
        let (ibrd, fbrd) = brd(f_uart, config.baud_rate)?;

        // Disable UART before reconfiguring (PL011 spec §3.3.4).
        uart.cr().write(|w| unsafe { w.bits(0) });
        uart.lcr_h().write(|w| unsafe { w.bits(0) });
        // Clear DMA enables; write RSR/ECR (offset 0x004) to clear the four
        // receive-error stickies (OE/BE/PE/FE). PL011 spec: offset 0x004 is
        // RSR on read and ECR on write; writing all four error bits clears them.
        // Mirrors cxd56_serial.c:478-479.
        uart.dmacr().write(|w| unsafe { w.bits(0) });
        uart.rsr()
            .write(|w| w.roe().error().rbe().error().rpe().error().rfe().error());

        uart.ibrd().write(|w| unsafe { w.baud_divint().bits(ibrd) });
        uart.fbrd()
            .write(|w| unsafe { w.baud_divfrac().bits(fbrd) });

        // LCR_H written after IBRD/FBRD latches the baud-rate generator (PL011
        // spec §3.3.4). Write format bits only — FEN comes in a separate write.
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

        // FIFO interrupt thresholds = 1/8 full; clear all interrupt sources.
        uart.ifls().write(|w| unsafe { w.bits(0) });
        uart.icr().write(|w| unsafe { w.bits(0x7ff) });

        // Enable FIFOs, then enable UART — two separate writes (cxd56_serial.c:499-505).
        uart.lcr_h()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 4)) }); // FEN
        uart.cr()
            .write(|w| w.uarten().enabled().txe().enabled().rxe().enabled());

        Ok(Self { uart })
    }

    /// Transmit one byte, blocking until the TX FIFO has room.
    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        while self.uart.fr().read().txff().bit_is_set() {}
        self.uart.dr().write(|w| unsafe { w.bits(byte as u32) });
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

impl fmt::Write for Uart1 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl ErrorType for Uart1 {
    type Error = ErrorKind;
}

impl embedded_hal_nb::serial::Read<u8> for Uart1 {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        match self.read_byte() {
            Some(b) => Ok(b),
            None => Err(nb::Error::WouldBlock),
        }
    }
}

impl embedded_hal_nb::serial::Write<u8> for Uart1 {
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

impl embedded_io::ErrorType for Uart1 {
    type Error = embedded_io::ErrorKind;
}

impl embedded_io::Write for Uart1 {
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

impl embedded_io::Read for Uart1 {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        while self.uart.fr().read().rxfe().bit_is_set() {}
        let mut count = 0;
        while count < buf.len() {
            if self.uart.fr().read().rxfe().bit_is_set() {
                break;
            }
            buf[count] = self.uart.dr().read().bits() as u8;
            count += 1;
        }
        Ok(count)
    }
}

/// Blocking driver for UART2 (the IMG-domain UART connected to JP1 pins 2-5 on
/// the Spresense extension/Arduino header). Note: the on-board CP2102N USB-serial
/// bridge is wired to UART1, not UART2.
pub struct Uart2 {
    uart: pac::Uart2,
}

impl Uart2 {
    /// Initialise UART2 with the given baud rate and frame format.
    ///
    /// Enables the IMG-UART clock (≈ 39 MHz) and programs the PL011
    /// control registers. Must be called after `Clocks::freeze`.
    pub fn new(uart: pac::Uart2, clocks: &Clocks, config: UartConfig) -> Result<Self, UartError> {
        PeripheralId::ImgUart.enable()?;
        uart2_pinmux();

        // img_uart_enable() just programmed GEAR_IMG_UART = 0x0001_0004.
        // clocks.img_uart was snapshotted at freeze() time, before that write,
        // so re-derive from the live gear register using the stable APPSMP value.
        let f_uart = buses::img_uart_hz(clocks.appsmp.to_Hz());
        let (ibrd, fbrd) = brd(f_uart, config.baud_rate)?;

        // Disable UART before reconfiguring (PL011 spec §3.3.4).
        uart.cr().write(|w| unsafe { w.bits(0) });
        uart.lcr_h().write(|w| unsafe { w.bits(0) });
        // Clear DMA enables; write RSR/ECR to clear receive-error stickies.
        // Mirrors cxd56_serial.c:478-479.
        uart.dmacr().write(|w| unsafe { w.bits(0) });
        uart.rsr()
            .write(|w| w.roe().error().rbe().error().rpe().error().rfe().error());

        uart.ibrd().write(|w| unsafe { w.baud_divint().bits(ibrd) });
        uart.fbrd()
            .write(|w| unsafe { w.baud_divfrac().bits(fbrd) });

        // LCR_H written after IBRD/FBRD latches the baud-rate generator (PL011
        // spec §3.3.4). Write format bits only — FEN comes in a separate write.
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

        // FIFO interrupt thresholds = 1/8 full; clear all interrupt sources.
        uart.ifls().write(|w| unsafe { w.bits(0) });
        uart.icr().write(|w| unsafe { w.bits(0x7ff) });

        // Enable FIFOs, then enable UART — two separate writes (cxd56_serial.c:499-505).
        uart.lcr_h()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 4)) }); // FEN
        uart.cr()
            .write(|w| w.uarten().enabled().txe().enabled().rxe().enabled());

        Ok(Self { uart })
    }

    /// Transmit one byte, blocking until the TX FIFO has room.
    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        while self.uart.fr().read().txff().bit_is_set() {}
        self.uart.dr().write(|w| unsafe { w.bits(byte as u32) });
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

impl embedded_io::ErrorType for Uart2 {
    type Error = embedded_io::ErrorKind;
}

impl embedded_io::Write for Uart2 {
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

impl embedded_io::Read for Uart2 {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        while self.uart.fr().read().rxfe().bit_is_set() {}
        let mut count = 0;
        while count < buf.len() {
            if self.uart.fr().read().rxfe().bit_is_set() {
                break;
            }
            buf[count] = self.uart.dr().read().bits() as u8;
            count += 1;
        }
        Ok(count)
    }
}
