#![no_std]
#![no_main]

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cxd56_hal::delay::Delay;
use cxd56_hal::gpio::{gpio0, Level};
use cxd56_hal::pac;

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = Peripherals::take().unwrap();

    let pins = gpio0::Parts::new(pac.gpio0);
    let mut led = pins.pin97.into_output(Level::Low);
    let mut delay = Delay::new(core.SYST);

    loop {
        led.set_high();
        delay.delay_ms(500);
        led.set_low();
        delay.delay_ms(500);
    }
}
