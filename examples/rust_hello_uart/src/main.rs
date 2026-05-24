#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cxd56_hal::gpio::{gpio0, Level};
use cxd56_hal::pac;
use cxd56_hal::uart::{Uart1, UartConfig};
use cxd56_hal::{
    clocks::{Config, RccExt},
    pac::gpio0::pin97::Pin97Spec,
};
use cxd56_hal::{delay::Delay, gpio::Output, pac::generic::Reg};

fn sos(led: &mut Output<Reg<Pin97Spec>>, delay: &mut Delay) {
    for _ in 0..3 {
        led.set_high();
        delay.delay_ms(150);
        led.set_low();
        delay.delay_ms(150);
    }
    delay.delay_ms(1000);
}

fn strobe(led: &mut Output<Reg<Pin97Spec>>, delay: &mut Delay) {
    for _ in 0..10 {
        led.set_high();
        delay.delay_ms(50);
        led.set_low();
        delay.delay_ms(50);
    }
    delay.delay_ms(500);
}

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = Peripherals::take().unwrap();

    let crg = pac.crg.constrain(Config::default());
    let clocks = crg.freeze();

    let pins = gpio0::Parts::new(pac.gpio0);
    let mut led = pins.pin97.into_output(Level::Low);
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
