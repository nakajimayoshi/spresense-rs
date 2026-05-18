// nash bootloader dialog — port of FlashWriter class in flash_writer.py

use std::{
    io::stdout,
    path::Path,
    thread,
    time::Duration,
    fs,
};

use crate::{FlashError, serial::Serial, xmodem};

const ROM_MSG: &[u8] = b"Welcome to nash";
const XMDM_MSG: &[u8] = b"Waiting for XMODEM (CRC or 1K) transfer. Ctrl-X to cancel.";
const MAX_DOT_COUNT: usize = 70;

/// Pulse DTR and spam "r" until we see the nash banner, then wait for the
/// "updater" prompt and send the dropped-first-char workaround newline.
pub fn cancel_autoboot(serial: &mut Serial) -> Result<(), FlashError> {
    serial.pulse_dtr();

    let mut retry = 0usize;
    loop {
        if retry > 10 {
            serial.pulse_dtr();
            retry = 0;
        }
        let line = serial.read_line();
        serial.write_all(b"r")?; // keep autoboot suppressed
        if line.windows(ROM_MSG.len()).any(|w| w == ROM_MSG) {
            break;
        }
        retry += 1;
    }

    // Wait for "updater" prompt.
    loop {
        let line = serial.read_line();
        if line.windows(b"updater".len()).any(|w| w == b"updater") {
            break;
        }
    }

    // Workaround: first character sometimes dropped — send air-shot LF then sync.
    serial.write_all(b"\n")?;
    serial.discard_inputs();
    Ok(())
}

/// Read lines until "updater" appears, then sleep 100 ms.
pub fn wait_for_prompt(serial: &mut Serial) -> Result<(), FlashError> {
    loop {
        let line = serial.read_line();
        if line.windows(b"updater".len()).any(|w| w == b"updater") {
            thread::sleep(Duration::from_millis(100));
            return Ok(());
        }
        // Print non-empty lines (mirrors PRINT_RAW_COMMAND behaviour — always on
        // for the simplified CLI).
        let trimmed: Vec<u8> = line
            .iter()
            .copied()
            .filter(|&b| b != b'\r' && b != b'\n')
            .collect();
        if !trimmed.is_empty() && !trimmed.starts_with(XMDM_MSG) {
            if let Ok(s) = std::str::from_utf8(&trimmed) {
                eprintln!("{s}");
            }
        }
    }
}

/// Write `cmd\n` and read the echo line back.
pub fn send_line(serial: &mut Serial, cmd: &str) -> Result<(), FlashError> {
    let mut payload = cmd.as_bytes().to_vec();
    payload.push(b'\n');
    serial.write_all(&payload)?;
    let echo = serial.read_line();
    if let Ok(s) = std::str::from_utf8(&echo) {
        let s = s.trim();
        if !s.is_empty() {
            eprintln!("{s}");
        }
    }
    Ok(())
}

/// Wait for `needle` to appear as a substring of any received line.
fn wait_for(serial: &mut Serial, needle: &[u8]) -> Result<(), FlashError> {
    loop {
        let line = serial.read_line();
        if line.windows(needle.len()).any(|w| w == needle) {
            thread::sleep(Duration::from_millis(100));
            return Ok(());
        }
    }
}

/// Send one SPK file via XMODEM-1K, with optional baud switch.
pub fn install_one(
    serial: &mut Serial,
    spk: &Path,
    xmodem_baud: Option<u32>,
) -> Result<(), FlashError> {
    let file_size = fs::metadata(spk)
        .map_err(FlashError::Io)?
        .len();

    let spk_name = spk.display();

    let mut cmd = String::from("install");
    if let Some(baud) = xmodem_baud {
        cmd.push_str(&format!(" -b {baud}"));
    }

    send_line(serial, &cmd)?;
    eprintln!("Install {spk_name}");

    // Print the progress bar header.
    let half = MAX_DOT_COUNT / 2;
    eprintln!(
        "|0%{}50%{}100%|",
        "-".repeat(half - 3),
        "-".repeat(MAX_DOT_COUNT - half - 5),
    );

    // Wait for the XMODEM banner before switching baud / starting transfer.
    wait_for(serial, XMDM_MSG)?;

    if let Some(baud) = xmodem_baud {
        serial.set_baud(baud)?;
        serial.discard_inputs();
    }

    // Set up progress tracking.
    let mut dot_count = 0usize;
    let on_progress = |bytes: u64| {
        let target = (bytes * MAX_DOT_COUNT as u64 / file_size) as usize;
        let target = target.min(MAX_DOT_COUNT);
        while dot_count < target {
            use std::io::Write;
            print!("#");
            let _ = stdout().flush();
            dot_count += 1;
        }
        if dot_count == MAX_DOT_COUNT {
            println!();
        }
    };

    let mut file = fs::File::open(spk).map_err(FlashError::Io)?;
    xmodem::send_xmodem_1k(&mut file, serial, file_size, on_progress)?;

    if xmodem_baud.is_some() {
        serial.set_baud(115200)?;
    }

    wait_for_prompt(serial)?;
    Ok(())
}
