#![no_std]
#![no_main]

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use cxd56_blink_debug::{sos, strobe};
use defmt::*;
use defmt_serial as _;
use embedded_hal::delay::DelayNs;
use panic_probe as _;
use static_cell::StaticCell;

use cxd56_hal::delay_alt::Delay;
use cxd56_hal::gpio::Level;
use cxd56_hal::pac;
use cxd56_hal::{
    clocks::{Config, RccExt},
    gpio::pins::Parts,
    uart_alt::{Uart, Uart1Pins},
};

static SERIAL: StaticCell<Uart<'static, pac::Uart1>> = StaticCell::new();

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = Peripherals::take().unwrap();

    let crg = pac.crg.constrain(Config::default());
    let clock = crg.into_clock();

    // UART1 for console output. COM clock is Fixed → Uart<'static, Uart1>.
    let parts = Parts::new(pac.topreg);
    let uart1_pins = Uart1Pins {
        tx: parts.gp_spi0_cs_x,
        rx: parts.gp_spi0_sck,
    };
    let uart =
        Uart::new(pac.uart1, uart1_pins, Default::default(), &clock).expect("uart1 init failed");

    let mut led = parts.gp_i2s1_bck.into_output(Level::Low);
    let mut delay = Delay::new(core.SYST, &clock);

    sos(&mut led, &mut delay);

    defmt_serial::defmt_serial(SERIAL.init(uart));

    let mut n: u32 = 0;
    loop {
        info!("hello from spresense rust, n={}", n);
        n = n.wrapping_add(1);
        delay.delay_ms(500);
        strobe(&mut led, &mut delay);
    }
}
