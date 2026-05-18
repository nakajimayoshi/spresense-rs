use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};

use crate::{FlashError, SerialIo};

const OPEN_BAUD: u32 = 115200;
const READ_TIMEOUT_MS: u64 = 30;

pub struct Serial {
    port: Box<dyn SerialPort>,
}

impl Serial {
    pub fn open(path: &str) -> Result<Self, FlashError> {
        let port = serialport::new(path, OPEN_BAUD)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .timeout(Duration::from_millis(READ_TIMEOUT_MS))
            .open()
            .map_err(|e| FlashError::OpenPort(format!("{path}: {e}")))?;
        Ok(Serial { port })
    }

    /// Read bytes until LF, timeout, or 512 bytes; returns the raw line including any trailing CR/LF.
    pub fn read_line(&mut self) -> Vec<u8> {
        let mut line = Vec::with_capacity(128);
        let mut single = [0u8; 1];
        loop {
            match self.port.read(&mut single) {
                Ok(1) => {
                    line.push(single[0]);
                    if single[0] == b'\n' {
                        break;
                    }
                    if line.len() >= 512 {
                        break;
                    }
                }
                Ok(_) | Err(_) => break,
            }
        }
        line
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), FlashError> {
        self.port.write_all(data).map_err(FlashError::Io)?;
        self.port.flush().map_err(FlashError::Io)
    }

    /// Sleep 1 s then flush the input buffer — matches Python discard_inputs().
    pub fn discard_inputs(&mut self) {
        thread::sleep(Duration::from_secs(1));
        let _ = self.port.clear(ClearBuffer::Input);
    }

    /// DTR False → True → False — matches Python reboot().
    pub fn pulse_dtr(&mut self) {
        let _ = self.port.write_data_terminal_ready(false);
        let _ = self.port.write_data_terminal_ready(true);
        let _ = self.port.write_data_terminal_ready(false);
    }

    pub fn set_baud(&mut self, rate: u32) -> Result<(), FlashError> {
        self.port.set_baud_rate(rate).map_err(|e| {
            FlashError::Io(io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
    }
}

impl SerialIo for Serial {
    fn getc(&mut self, n: usize, timeout_ms: u64) -> Vec<u8> {
        let _ = self.port.set_timeout(Duration::from_millis(timeout_ms));
        let mut buf = vec![0u8; n];
        match self.port.read(&mut buf) {
            Ok(k) => {
                buf.truncate(k);
                buf
            }
            Err(_) => vec![],
        }
    }

    fn putc(&mut self, data: &[u8], _timeout_ms: u64) {
        let _ = self.port.write_all(data);
        let _ = self.port.flush();
    }
}
