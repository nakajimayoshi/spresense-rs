#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cxd56_hal::clocks::{Config, RccExt};
use cxd56_hal::pac;
use cxd56_hal::{delay_alt::Delay, uart_alt::UartBuilder};

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = Peripherals::take().unwrap();

    let crg = pac.crg.constrain(Config::default());
    let clocks = crg.into_clock();

    let mut delay = Delay::new(core.SYST, &clocks);

    let builder = UartBuilder::new(pac.uart1, Default::default());
    let mut uart = builder.build(&clocks).unwrap();

    let mut n: u32 = 0;
    loop {
        let _ = writeln!(uart, "hello from spresense rust, n={n}");
        n = n.wrapping_add(1);
        delay.delay_ms(500);
    }
}
