#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cxd56_hal::clocks::{Config, RccExt};
use cxd56_hal::delay::Delay;
use cxd56_hal::gpio::{pins, Level};
use cxd56_hal::pac;
use cxd56_hal::uart::{Uart1, UartConfig};

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = Peripherals::take().unwrap();

    // Configure the clock tree (defaults are safe for the CXD5602).
    let crg = pac.crg.constrain(Config::default());
    let clocks = crg.freeze();

    // Onboard LED0 (GP_I2S1_BCK), driven low to start.
    let pins = pins::Parts::new(pac.topreg);
    let mut led = pins.gp_i2s1_bck.into_output(Level::Low);

    let mut delay = Delay::new(core.SYST, &clocks);

    // UART1 is the Spresense USB-serial console (115200 8N1 by default).
    let mut uart =
        Uart1::new(pac.uart1, &clocks, UartConfig::default()).expect("uart1 init failed");

    let mut n: u32 = 0;
    loop {
        led.set_high();
        let _ = writeln!(uart, "hello from {{project-name}}, n={n}");
        delay.delay_ms(500);
        led.set_low();
        delay.delay_ms(500);
        n = n.wrapping_add(1);
    }
}
