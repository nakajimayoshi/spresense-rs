//! Read-only serial monitor: stream the board's UART output to stdout.
//!
//! This is a plain byte pump — it does not decode `defmt` frames. Press Ctrl-C
//! to exit (the default SIGINT handler terminates the process).

use std::io::{self, Write};
use std::time::Duration;

use anyhow::{Context, Result};

/// Console baud rate. The board runs its UART console at 115200 regardless of
/// the XMODEM transfer rate used while flashing.
const MONITOR_BAUD: u32 = 115_200;

/// Open `port` and copy everything it emits to stdout until interrupted.
pub fn run(port: &str) -> Result<()> {
    let mut serial = serialport::new(port, MONITOR_BAUD)
        .timeout(Duration::from_millis(100))
        .open()
        .with_context(|| format!("opening {port} for monitoring"))?;

    log::info!("Monitoring {port} @ {MONITOR_BAUD} baud — press Ctrl-C to exit");

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut buf = [0u8; 1024];
    loop {
        match serial.read(&mut buf) {
            Ok(0) => {}
            Ok(n) => {
                out.write_all(&buf[..n])?;
                out.flush()?;
            }
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {}
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e).with_context(|| format!("reading from {port}")),
        }
    }
}
