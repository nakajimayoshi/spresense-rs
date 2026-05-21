use embassy_hal_internal::Peri;

use crate::pac;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

mod sealed {
    pub trait Sealed {
        fn read_raw(&self) -> u32;
        // Writes a fully-computed value back to the pin's register.
        // Uses PAC modify (read+write) because write_value would require
        // spelling out the per-pin register newtype path.
        fn write_raw(&self, val: u32);
    }
}

pub trait Pin: sealed::Sealed + embassy_hal_internal::PeripheralType + 'static {}

// Bit positions — identical layout for every GPIO0 register.
const IN_BIT: u32  = 1 << 0;
const OUT_BIT: u32 = 1 << 8;
const DIR_BIT: u32 = 1 << 16; // active-low: 0 = output drive, 1 = high-Z input

impl sealed::Sealed for crate::peripherals::PIN97 {
    fn read_raw(&self) -> u32 {
        pac::GPIO0.PIN97().read().0
    }
    fn write_raw(&self, val: u32) {
        pac::GPIO0.PIN97().modify(|r| r.0 = val);
    }
}

impl Pin for crate::peripherals::PIN97 {}

/// Push-pull output.
pub struct Output<'d, T: Pin> {
    pin: Peri<'d, T>,
}

impl<'d, T: Pin> Output<'d, T> {
    pub fn new(pin: Peri<'d, T>, initial: Level) -> Self {
        let raw = pin.read_raw() & !DIR_BIT; // DIR=0 → output drive
        let raw = match initial {
            Level::High => raw | OUT_BIT,
            Level::Low  => raw & !OUT_BIT,
        };
        pin.write_raw(raw);
        Self { pin }
    }

    pub fn set_high(&mut self) {
        let raw = self.pin.read_raw() | OUT_BIT;
        self.pin.write_raw(raw);
    }

    pub fn set_low(&mut self) {
        let raw = self.pin.read_raw() & !OUT_BIT;
        self.pin.write_raw(raw);
    }

    pub fn set_level(&mut self, l: Level) {
        match l {
            Level::High => self.set_high(),
            Level::Low  => self.set_low(),
        }
    }

    pub fn is_set_high(&self) -> bool {
        self.pin.read_raw() & OUT_BIT != 0
    }
}

/// High-Z input (no pull configuration — pull lives in a separate IOCONFIG block).
pub struct Input<'d, T: Pin> {
    pin: Peri<'d, T>,
}

impl<'d, T: Pin> Input<'d, T> {
    pub fn new(pin: Peri<'d, T>) -> Self {
        let raw = pin.read_raw() | DIR_BIT; // DIR=1 → high-Z input
        pin.write_raw(raw);
        Self { pin }
    }

    pub fn is_high(&self) -> bool { self.pin.read_raw() & IN_BIT != 0 }
    pub fn is_low(&self) -> bool  { !self.is_high() }

    pub fn get_level(&self) -> Level {
        if self.is_high() { Level::High } else { Level::Low }
    }
}
