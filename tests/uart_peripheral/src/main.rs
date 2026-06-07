//! On-hardware UART peripheral test (Option 1 — plain `defmt` + sentinel).
//!
//! Runs three sub-tests, logs each result over UART1 via `defmt-serial`, and
//! ends with a `TEST RESULT: PASS`/`FAIL` line that `cargo-spresense-flash
//! --test` matches to derive a process exit code. This is the lightweight,
//! framework-free style; see `../gpio_levels` for the `defmt-test` style.
//!
//!   [1/3] console_uart1         — UART1 console + defmt-serial come up (reaching
//!                                 the host over defmt *is* the assertion).
//!   [2/3] uart2_internal_loopback — UART2 in PL011 loopback (UARTCR.LBE): write a
//!                                 byte pattern, read it back, assert equal. No wiring.
//!   [3/3] uart2_external_loopback — same over the real JP1 pads with a D01↔D00
//!                                 jumper. Gated behind `--features external-loopback`.

#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_serial as _;
use panic_probe as _;
use static_cell::StaticCell;

use cxd56_hal::clocks::{Clocks, Config, RccExt};
use cxd56_hal::pac;
use cxd56_hal::uart::{Uart1, Uart2, UartConfig};

static SERIAL: StaticCell<Uart1> = StaticCell::new();

/// Transmit one byte and read it straight back (loopback), asserting a match.
fn echo_byte(uart: &mut Uart2, expect: u8) -> Result<(), &'static str> {
    uart.write_byte(expect);
    uart.flush();
    // After flush() the byte has been clocked out and looped to RX; poll the RX
    // FIFO with a bounded spin so a wiring/clock fault fails fast instead of hanging.
    for _ in 0..1_000_000 {
        if let Some(got) = uart.read_byte() {
            return if got == expect { Ok(()) } else { Err("byte mismatch") };
        }
    }
    Err("RX timeout")
}

/// Drive a UART2 instance through a fixed byte pattern via `echo_byte`.
fn run_pattern(uart: &mut Uart2) -> Result<(), &'static str> {
    while uart.read_byte().is_some() {} // drain stale RX
    for &b in &[0xA5u8, 0x5A, 0x00, 0xFF, b'R', b'U', b'S', b'T'] {
        echo_byte(uart, b)?;
    }
    Ok(())
}

/// [2/3] UART2 self-test using on-chip loopback — needs no external wiring.
fn uart2_internal_loopback(uart2: pac::Uart2, clocks: &Clocks) -> Result<(), &'static str> {
    let mut uart =
        Uart2::new(uart2, clocks, UartConfig::default()).map_err(|_| "Uart2::new failed")?;
    uart.set_loopback(true);
    let result = run_pattern(&mut uart);
    uart.set_loopback(false);
    result
}

/// [3/3] UART2 over the real JP1 pads — requires a D01 (TXD) ↔ D00 (RXD) jumper.
#[cfg(feature = "external-loopback")]
fn uart2_external_loopback(uart2: pac::Uart2, clocks: &Clocks) -> Result<(), &'static str> {
    let mut uart =
        Uart2::new(uart2, clocks, UartConfig::default()).map_err(|_| "Uart2::new failed")?;
    // No LBE: bytes travel out TXD and back in RXD over the jumper wire.
    run_pattern(&mut uart)
}

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let crg = pac.crg.constrain(Config::default());
    let clocks = crg.freeze();

    // UART1 is the console + defmt result channel.
    let uart1 = Uart1::new(pac.uart1, &clocks, UartConfig::default()).expect("uart1 init failed");
    defmt_serial::defmt_serial(SERIAL.init(uart1));

    defmt::println!("uart_peripheral: starting UART tests");
    let mut all_ok = true;

    // [1/3] Reaching here over defmt proves UART1 TX + the console path works.
    defmt::println!("[1/3] console_uart1: PASS (UART1 console up, defmt-serial logging)");

    // [2/3] UART2 internal loopback (no wiring).
    match uart2_internal_loopback(pac.uart2, &clocks) {
        Ok(()) => defmt::println!("[2/3] uart2_internal_loopback: PASS"),
        Err(e) => {
            all_ok = false;
            defmt::println!("[2/3] uart2_internal_loopback: FAIL: {}", e);
        }
    }

    // [3/3] UART2 external loopback (needs a jumper; off by default).
    #[cfg(feature = "external-loopback")]
    {
        // Re-acquire UART2: the internal-loopback instance above already consumed
        // and dropped the PAC token. Safe here — single-threaded, no live UART2.
        let uart2 = unsafe { pac::Peripherals::steal() }.uart2;
        match uart2_external_loopback(uart2, &clocks) {
            Ok(()) => defmt::println!("[3/3] uart2_external_loopback: PASS"),
            Err(e) => {
                all_ok = false;
                defmt::println!("[3/3] uart2_external_loopback: FAIL: {}", e);
            }
        }
    }
    #[cfg(not(feature = "external-loopback"))]
    defmt::println!("[3/3] uart2_external_loopback: SKIPPED (--features external-loopback + jumper JP1 D01↔D00)");

    if all_ok {
        defmt::println!("TEST RESULT: PASS");
    } else {
        defmt::println!("TEST RESULT: FAIL");
    }

    // Verdict delivered; halt cleanly (the UART FIFO has already drained the line).
    loop {
        asm::wfi();
    }
}
