//! UART2 with a reconfigured gear divider.
//!
//! UART2 (IMG_UART) lives in the APP_SUB domain and derives its base clock
//! from the APP CPU clock through an M/N "gear" divider: base = appsmp / M.
//! The HAL default is M=4 (≈39 MHz at HP); this example sets M=2 (≈78 MHz)
//! with [`PeripheralId::set_gear`] before bringing the port up, doubling the
//! maximum reachable baud rate (PL011 max baud = base / 16).
//!
//! Wiring: UART2 is on the JP1 extension header — TXD = P1n_00 (JP1 pin 2),
//! RXD = P1n_01 (JP1 pin 3). Attach a 3.3 V serial adapter at 921 600 baud to
//! see the stream and echo. The USB console (UART1) prints the clock report,
//! so the gear reconfiguration is observable without any extra hardware.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cxd56_hal::clocks::{Config, PeripheralId, RccExt};
use cxd56_hal::delay_alt::Delay;
use cxd56_hal::gpio::pins::Parts;
use cxd56_hal::pac;
use cxd56_hal::uart::UartConfig;
use cxd56_hal::uart_alt::{Uart, Uart1Pins, Uart2Pins};

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = Peripherals::take().unwrap();

    let crg = pac.crg.constrain(Config::default());

    let gear_before = PeripheralId::ImgUart.gear_divisor();

    // Reconfigure the gear *before* `into_clock()`: the Uart2 driver computes
    // its baud divisor from the Clock snapshot taken there. `enable()` inside
    // `Uart::new` preserves a user-set IMG_UART divisor, so M=2 survives
    // peripheral bring-up.
    PeripheralId::ImgUart.set_gear(2).unwrap();

    let clocks = crg.into_clock();

    let mut delay = Delay::new(core.SYST, &clocks);

    let parts = Parts::new(pac.topreg);
    let uart1_pins = Uart1Pins { tx: parts.gp_spi0_cs_x, rx: parts.gp_spi0_sck };
    let uart2_pins = Uart2Pins { tx: parts.gp_uart2_txd, rx: parts.gp_uart2_rxd };

    // UART1: USB console at the default 115 200 baud, for the clock report.
    let mut console = Uart::new(pac.uart1, uart1_pins, Default::default(), &clocks).unwrap();

    let appsmp = clocks.appsmp().hz().to_Hz();
    let img_uart = clocks.img_uart().hz().to_Hz();
    let _ = writeln!(console, "appsmp:        {appsmp} Hz");
    let _ = writeln!(console, "gear at reset: {gear_before:?}");
    let _ = writeln!(
        console,
        "gear now:      {:?} (img_uart = appsmp / M)",
        PeripheralId::ImgUart.gear_divisor()
    );
    let _ = writeln!(console, "img_uart:      {img_uart} Hz");
    let _ = writeln!(console, "max baud:      {} (img_uart / 16)", img_uart / 16);

    // UART2 on JP1 at 921 600 baud, clocked from the reconfigured base.
    let config = UartConfig { baud_rate: 921_600, ..Default::default() };
    let mut uart2 = Uart::new(pac.uart2, uart2_pins, config, &clocks).unwrap();

    let mut n: u32 = 0;
    loop {
        let _ = writeln!(uart2, "hello from uart2 at 921600 baud, n={n}");
        while let Some(byte) = uart2.read_byte() {
            uart2.write_byte(byte);
        }
        let _ = writeln!(console, "uart2 tick n={n}");
        n = n.wrapping_add(1);
        delay.delay_ms(500);
    }
}
