#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cxd56_hal::gpio::{pins, Level};
use cxd56_hal::pac;
use cxd56_hal::uart::{Uart1, UartConfig};
use cxd56_hal::{
    clocks::{Config, RccExt},
    delay::Delay,
};
use cxd56_blink_debug::{sos, strobe};

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = Peripherals::take().unwrap();

    let crg = pac.crg.constrain(Config::default());
    let clocks = crg.freeze();

    let pins = pins::Parts::new(pac.topreg);
    let mut led = pins.gp_i2s1_bck.into_output(Level::Low);
    let mut delay = Delay::new(core.SYST, &clocks);

    sos(&mut led, &mut delay);

    let mut uart =
        Uart1::new(pac.uart1, &clocks, UartConfig::default()).expect("uart1 init failed");

    let mut n: u32 = 0;
    loop {
        let _ = writeln!(uart, "hello from spresense rust, n={n}");
        n = n.wrapping_add(1);
        delay.delay_ms(500);
        strobe(&mut led, &mut delay);
    }
}
