#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use embedded_hal::i2c::I2c;
use panic_halt as _;

use cxd56_hal::uart_alt::Uart;
use cxd56_hal::{
    clocks::{Config, RccExt},
    i2c_alt::I2c0Pins,
};
use cxd56_hal::{gpio::pins::Parts, i2c_alt::I2c as HalI2c, uart_alt::Uart1Pins};
use cxd56_hal::{i2c_alt::I2cConfig, pac};

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let crg = pac.crg.constrain(Config::default());
    let clock = crg.into_clock();

    // UART1 for console output. COM clock is Fixed → Uart<'static, Uart1>.
    let parts = Parts::new(pac.topreg);
    let uart1_pins = Uart1Pins {
        tx: parts.gp_spi0_cs_x,
        rx: parts.gp_spi0_sck,
    };
    let i2c0_pins = I2c0Pins {
        scl: parts.gp_i2c0_bck,
        sda: parts.gp_i2c0_bdt,
    };
    let mut uart =
        Uart::new(pac.uart1, uart1_pins, Default::default(), &clock).expect("uart1 init failed");
    let mut i2c = HalI2c::new(pac.i2c0, i2c0_pins, &clock, I2cConfig::default()).unwrap();

    writeln!(uart, "I2C0 scan (0x03..0x77):").ok();

    let mut found = 0u32;
    for addr in 0x03u8..=0x77 {
        // A zero-length write probes for an ACK on the address byte.
        if i2c.write(addr, &[]).is_ok() {
            writeln!(uart, "  found 0x{:02x}", addr).ok();
            found += 1;
        }
    }

    writeln!(uart, "{} device(s) found.", found).ok();

    loop {
        cortex_m::asm::wfe();
    }
}
