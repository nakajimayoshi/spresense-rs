#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use embedded_hal::i2c::I2c;
use panic_halt as _;

use cxd56_hal::i2c::{I2c0, I2cConfig};
use cxd56_hal::pac;
use cxd56_hal::uart::{Uart1, UartConfig};
use cxd56_hal::clocks::{Config, RccExt};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let clocks = dp.crg.constrain(Config::default()).freeze();

    let mut uart = Uart1::new(dp.uart1, &clocks, UartConfig::default()).unwrap();
    let mut i2c = I2c0::new(dp.i2c0, &clocks, I2cConfig::default()).unwrap();

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
