//! Serial-port resolution: honour an explicit choice, otherwise auto-detect a
//! USB serial adapter and prompt when the choice is ambiguous.

use std::io::{self, BufRead, IsTerminal, Write};

use anyhow::{bail, Context, Result};
use serialport::{SerialPortInfo, SerialPortType, UsbPortInfo};

/// Silicon Labs USB vendor id — the CP2102N used as the Spresense console adapter.
const CP210X_VID: u16 = 0x10c4;

/// Resolve the serial port to use.
///
/// Order of precedence:
/// 1. `explicit` — a `--port` value or the `SPRESENSE_PORT` env var.
/// 2. The sole USB serial adapter, if exactly one is present.
/// 3. An interactive pick among the USB serial adapters (only on a TTY).
///
/// Errors if no port can be determined.
pub fn resolve(explicit: Option<&str>) -> Result<String> {
    if let Some(port) = explicit {
        return Ok(port.to_string());
    }

    let ports = serialport::available_ports().context("enumerating serial ports")?;

    // Prefer real USB serial adapters; the CP210x console bridge floats to the top.
    let mut usb: Vec<&SerialPortInfo> = ports
        .iter()
        .filter(|p| matches!(p.port_type, SerialPortType::UsbPort(_)))
        .collect();
    usb.sort_by_key(|p| match &p.port_type {
        SerialPortType::UsbPort(info) if info.vid == CP210X_VID => 0,
        _ => 1,
    });

    match usb.as_slice() {
        [] => {
            if ports.is_empty() {
                bail!("no serial ports found — pass --port or set SPRESENSE_PORT");
            }
            let listing = ports
                .iter()
                .map(|p| format!("  {}", describe(p)))
                .collect::<Vec<_>>()
                .join("\n");
            bail!(
                "no USB serial adapter found; available ports:\n{listing}\n\
                 pass --port or set SPRESENSE_PORT to choose one"
            );
        }
        [only] => {
            log::info!("Using serial port {}", describe(only));
            Ok(only.port_name.clone())
        }
        many => {
            if io::stdin().is_terminal() {
                prompt(many)
            } else {
                let listing = many
                    .iter()
                    .map(|p| format!("  {}", describe(p)))
                    .collect::<Vec<_>>()
                    .join("\n");
                bail!(
                    "multiple USB serial adapters found:\n{listing}\n\
                     pass --port or set SPRESENSE_PORT to choose one"
                );
            }
        }
    }
}

/// Numbered interactive picker, read from stdin.
fn prompt(ports: &[&SerialPortInfo]) -> Result<String> {
    let mut stderr = io::stderr();
    writeln!(stderr, "Multiple serial ports found:")?;
    for (i, p) in ports.iter().enumerate() {
        writeln!(stderr, "  [{i}] {}", describe(p))?;
    }

    let stdin = io::stdin();
    loop {
        write!(stderr, "Select a port [0-{}]: ", ports.len() - 1)?;
        stderr.flush()?;

        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            bail!("no port selected (end of input)");
        }
        match line.trim().parse::<usize>() {
            Ok(i) if i < ports.len() => return Ok(ports[i].port_name.clone()),
            _ => writeln!(stderr, "Invalid selection, try again.")?,
        }
    }
}

/// Human-readable one-line description of a port for logs and prompts.
fn describe(info: &SerialPortInfo) -> String {
    match &info.port_type {
        SerialPortType::UsbPort(UsbPortInfo {
            vid,
            pid,
            manufacturer,
            product,
            ..
        }) => {
            let label = product
                .as_deref()
                .or(manufacturer.as_deref())
                .unwrap_or("USB serial");
            format!("{} — {label} ({vid:04x}:{pid:04x})", info.port_name)
        }
        other => format!("{} — {other:?}", info.port_name),
    }
}
