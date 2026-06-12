#![no_std]
#![no_main]

//! External loopback test for I2S0 on the Spresense main board.
//!
//! Wiring: one jumper from `I2S0_DATA_OUT` to `I2S0_DATA_IN` — physically
//! **JP2-7 (Arduino D18) → JP2-6 (Arduino D19)**, adjacent pins. BCK (D26) and
//! LRCK (D25) stay free for a scope.
//!
//! The test brings up I2S0 as master @ 48 kHz duplex, then repeatedly runs a
//! full-duplex one-shot DMA transfer: a recognizable 16-bit stereo pattern is
//! transmitted out `SDOUT1` while `SDIN1` is captured into a larger RX buffer.
//! With the jumper in place the transmitted frames come back after the SRC
//! group delay, so the verifier searches the RX capture for the longest
//! contiguous run of the TX pattern and reports PASS/FAIL over the UART1
//! console (115200 8N1).
//!
//! Without the jumper the line floats/idles and the run length stays ~0 —
//! useful as a negative control.

use core::fmt::Write;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cxd56_hal::clocks::{Config, RccExt};
use cxd56_hal::delay_alt::Delay;
use cxd56_hal::gpio::pins::Parts;
use cxd56_hal::i2s_alt::{I2s, I2s0, I2s0Pins, I2sConfig};
use cxd56_hal::pac;
use cxd56_hal::uart_alt::{Uart, Uart1Pins};

/// TX pattern length in stereo samples (one `u32` = L | R<<16).
const TX_LEN: usize = 256;
/// RX capture length — oversized to absorb the DMA start skew + SRC delay.
const RX_LEN: usize = 1024;
/// PASS threshold: contiguous round-trip samples required.
const PASS_RUN: usize = TX_LEN * 3 / 4;

/// Distinct, never-zero L/R values per index so a match can't be idle line.
fn pattern(i: usize) -> u32 {
    let l = 0xA000 | (i as u32 & 0x0FFF);
    let r = 0xB000 | (i as u32 & 0x0FFF);
    (r << 16) | l
}

/// Longest run of consecutive `tx` samples found contiguously in `rx`,
/// starting the pattern at any offset. Returns (run_length, rx_offset).
fn best_run(tx: &[u32], rx: &[u32]) -> (usize, usize) {
    let mut best = (0usize, 0usize);
    for start in 0..rx.len() {
        if rx[start] != tx[0] {
            continue;
        }
        let mut n = 0;
        while start + n < rx.len() && n < tx.len() && rx[start + n] == tx[n] {
            n += 1;
        }
        if n > best.0 {
            best = (n, start);
        }
    }
    best
}

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = Peripherals::take().unwrap();

    let crg = pac.crg.constrain(Config::default());
    let clocks = crg.into_clock();

    let mut delay = Delay::new(core.SYST, &clocks);
    let parts = Parts::new(pac.topreg);

    // UART1 console for the verdict.
    let uart1_pins = Uart1Pins {
        tx: parts.gp_spi0_cs_x,
        rx: parts.gp_spi0_sck,
    };
    let mut uart = Uart::new(pac.uart1, uart1_pins, Default::default(), &clocks).unwrap();

    let i2s0_pins = I2s0Pins {
        bck: parts.gp_i2s0_bck,
        lrck: parts.gp_i2s0_lrck,
        data_in: parts.gp_i2s0_data_in,
        data_out: parts.gp_i2s0_data_out,
    };

    let mut i2s = match I2s::<I2s0>::new(pac.audio, i2s0_pins, &clocks, I2sConfig::default()) {
        Ok(i2s) => {
            let _ = writeln!(
                uart,
                "I2S0 loopback test: jumper D18 (JP2-7, DATA_OUT) -> D19 (JP2-6, DATA_IN)"
            );
            i2s
        }
        Err(e) => {
            let _ = writeln!(uart, "I2S0 bring-up failed: {e:?}");
            loop {
                delay.delay_ms(1000);
            }
        }
    };

    let mut tx = [0u32; TX_LEN];
    for (i, w) in tx.iter_mut().enumerate() {
        *w = pattern(i);
    }

    let mut rx = [0u32; RX_LEN];
    let mut iteration: u32 = 0;

    loop {
        iteration += 1;
        rx.fill(0);

        match i2s.transfer_16_blocking(&tx, &mut rx) {
            Ok(()) => {
                let (run, offset) = best_run(&tx, &rx);
                let nonzero = rx.iter().filter(|w| **w != 0).count();
                let verdict = if run >= PASS_RUN { "PASS" } else { "FAIL" };
                let _ = writeln!(
                    uart,
                    "#{iteration}: {verdict} - {run}/{TX_LEN} contiguous samples \
                     (rx offset {offset}, {nonzero}/{RX_LEN} rx words non-zero)"
                );
            }
            Err(e) => {
                let _ = writeln!(uart, "#{iteration}: DMA error: {e:?}");
            }
        }

        delay.delay_ms(2000);
    }
}
