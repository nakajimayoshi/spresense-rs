#![no_std]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

/// Morse code SOS pattern: three short blinks, three long blinks, three short blinks
pub fn sos<O, D>(led: &mut O, delay: &mut D)
where
    O: OutputPin,
    D: DelayNs,
{
    for _ in 0..3 {
        led.set_high().ok();
        delay.delay_ms(150);
        led.set_low().ok();
        delay.delay_ms(150);
    }
    delay.delay_ms(1000);
}

/// Strobe pattern: rapid on/off blinking
pub fn strobe<O, D>(led: &mut O, delay: &mut D)
where
    O: OutputPin,
    D: DelayNs,
{
    for _ in 0..10 {
        led.set_high().ok();
        delay.delay_ms(50);
        led.set_low().ok();
        delay.delay_ms(50);
    }
    delay.delay_ms(500);
}
