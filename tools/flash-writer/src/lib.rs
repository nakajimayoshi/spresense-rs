pub mod serial;
pub mod xmodem;
pub mod nash;

use std::{io, path::Path};

/// Trait abstraction over the serial I/O operations XMODEM needs.
/// Allows unit testing xmodem.rs without real hardware.
pub trait SerialIo {
    /// Read up to `n` bytes, blocking for at most `timeout_ms` milliseconds.
    fn getc(&mut self, n: usize, timeout_ms: u64) -> Vec<u8>;
    /// Write all bytes in `data`, blocking until sent.
    fn putc(&mut self, data: &[u8], timeout_ms: u64);
}

#[derive(Debug)]
pub enum FlashError {
    OpenPort(String),
    Io(io::Error),
    Timeout(&'static str),
    XmodemAborted,
    NashError(String),
}

impl std::fmt::Display for FlashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlashError::OpenPort(msg) => write!(f, "cannot open port: {msg}"),
            FlashError::Io(e) => write!(f, "I/O error: {e}"),
            FlashError::Timeout(what) => write!(f, "timed out waiting for {what}"),
            FlashError::XmodemAborted => write!(f, "XMODEM transfer aborted by receiver"),
            FlashError::NashError(msg) => write!(f, "nash error: {msg}"),
        }
    }
}

impl std::error::Error for FlashError {}

pub struct FlashOptions<'a> {
    /// Serial port device path, e.g. `/dev/ttyUSB0`.
    pub port: &'a str,
    /// SPK files to install, in order.
    pub packages: &'a [&'a Path],
    /// Pulse DTR to reset the board before the nash dialog.
    pub dtr_reset: bool,
    /// High-speed baud rate for the XMODEM phase; `None` stays at 115200.
    pub xmodem_baud: Option<u32>,
    /// Send `set bootable M0P` after all packages are installed.
    pub set_bootable: bool,
    /// Send `reboot` at the end.
    pub reboot: bool,
}

/// Flash one or more SPK packages onto a Spresense board over UART.
pub fn flash(opts: &FlashOptions) -> Result<(), FlashError> {
    let mut serial = serial::Serial::open(opts.port)?;

    if opts.dtr_reset {
        nash::cancel_autoboot(&mut serial)?;
    }

    for spk in opts.packages {
        nash::install_one(&mut serial, spk, opts.xmodem_baud)?;
    }

    if opts.set_bootable {
        eprintln!(">>> Save Configuration to FlashROM ...");
        nash::send_line(&mut serial, "set bootable M0P")?;
        nash::wait_for_prompt(&mut serial)?;
    }

    nash::send_line(&mut serial, "sync")?;
    nash::wait_for_prompt(&mut serial)?;

    if opts.reboot {
        eprintln!("Restarting the board ...");
        nash::send_line(&mut serial, "reboot")?;
    }

    Ok(())
}
