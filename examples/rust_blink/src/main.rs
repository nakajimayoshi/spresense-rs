//! Blink LED0 (GP_I2S1_BCK / pin 97) and read GP_I2C4_BCK (pin 1).
//!
//! Build: cargo build --example gpio_blink --target thumbv7em-none-eabihf --features rt
//!
//! This example exists to verify that the GPIO0 block in patch.yml is correct:
//!   - baseAddress 0x04102000 (not 0x04120000)
//!   - GP_I2C4_BCK at offset 0x000  ((1-1)*4)
//!   - PIN97       at offset 0x168  ((97-7)*4)
//!   - DIR[16] active-low (0 = output enabled)
//!   - OUT[8]  data bit
//!   - IN[0]   sampled input (read)

#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

use cxd56_pac::pac::*;

/// Cycles to spin between LED toggles at ~153.6 MHz (≈ 0.5 s).
const DELAY_CYCLES: u32 = 76_800_000;

#[entry]
fn main() -> ! {
    // Safety: no other code runs on this core before this point.
    let gpio0 = unsafe { Gpio0::steal() };

    // Configure PIN97 (LED0) as output, initially low.
    // DIR is active-low: clear_bit() = 0 = drive output.
    gpio0.pin97().write(|w| w.dir().clear_bit().out().clear_bit());

    // Leave GP_I2C4_BCK (pin 1) as input (reset default: DIR=1, high-Z input).

    loop {
        // Read GP_I2C4_BCK input level.
        let pin1_high = gpio0.gp_i2c4_bck().read().in_().bit_is_set();

        // Toggle LED to match the input level (high → LED on, low → LED off).
        gpio0.pin97().modify(|_, w| {
            if pin1_high {
                w.out().set_bit()
            } else {
                w.out().clear_bit()
            }
        });

        asm::delay(DELAY_CYCLES);

        // Toggle LED off regardless, so there is always a visible blink.
        gpio0.pin97().modify(|_, w| w.out().clear_bit());
        asm::delay(DELAY_CYCLES);
    }
}