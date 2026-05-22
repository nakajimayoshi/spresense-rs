use crate::pac;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

// Bit positions — identical layout for every GPIO0 register.
const IN_BIT: u32  = 1 << 0;
const OUT_BIT: u32 = 1 << 8;
const DIR_BIT: u32 = 1 << 16; // active-low: 0 = drive output, 1 = high-Z input

mod sealed {
    pub trait Sealed {
        fn read_bits(&self) -> u32;
        fn write_bits(&self, val: u32);
    }
}

/// Marker trait for svd2rust GPIO register accessors with the standard
/// IN[0] / OUT[8] / DIR[16] (active-low) field layout.
pub trait PinReg: sealed::Sealed + 'static {}

impl sealed::Sealed for pac::gpio0::Pin97 {
    fn read_bits(&self) -> u32 {
        self.read().bits()
    }
    fn write_bits(&self, val: u32) {
        self.modify(|_, w| unsafe { w.bits(val) });
    }
}
impl PinReg for pac::gpio0::Pin97 {}

impl sealed::Sealed for pac::gpio0::GpI2c4Bck {
    fn read_bits(&self) -> u32 {
        self.read().bits()
    }
    fn write_bits(&self, val: u32) {
        self.modify(|_, w| unsafe { w.bits(val) });
    }
}
impl PinReg for pac::gpio0::GpI2c4Bck {}

/// Unconfigured GPIO pin. Call [`into_output`](GpioPin::into_output) or
/// [`into_input`](GpioPin::into_input) to configure the direction.
pub struct GpioPin<R: PinReg> {
    reg: &'static R,
}

impl<R: PinReg> GpioPin<R> {
    /// # Safety
    /// Caller must ensure exclusive access to this pin register for the
    /// lifetime of the program (no other `GpioPin` or direct register access
    /// may exist simultaneously).
    pub unsafe fn new(reg: &'static R) -> Self {
        Self { reg }
    }

    pub fn into_output(self, initial: Level) -> Output<R> {
        let raw = self.reg.read_bits() & !DIR_BIT; // DIR=0 → drive output
        let raw = match initial {
            Level::High => raw | OUT_BIT,
            Level::Low  => raw & !OUT_BIT,
        };
        self.reg.write_bits(raw);
        Output { reg: self.reg }
    }

    pub fn into_input(self) -> Input<R> {
        let raw = self.reg.read_bits() | DIR_BIT; // DIR=1 → high-Z input
        self.reg.write_bits(raw);
        Input { reg: self.reg }
    }
}

/// Push-pull output.
pub struct Output<R: PinReg> {
    reg: &'static R,
}

impl<R: PinReg> Output<R> {
    pub fn set_high(&mut self) {
        let raw = self.reg.read_bits() | OUT_BIT;
        self.reg.write_bits(raw);
    }

    pub fn set_low(&mut self) {
        let raw = self.reg.read_bits() & !OUT_BIT;
        self.reg.write_bits(raw);
    }

    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low  => self.set_low(),
        }
    }

    pub fn is_set_high(&self) -> bool {
        self.reg.read_bits() & OUT_BIT != 0
    }
}

impl<R: PinReg> embedded_hal::digital::ErrorType for Output<R> {
    type Error = core::convert::Infallible;
}

impl<R: PinReg> embedded_hal::digital::OutputPin for Output<R> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Output::set_high(self);
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Output::set_low(self);
        Ok(())
    }
}

impl<R: PinReg> embedded_hal::digital::StatefulOutputPin for Output<R> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Output::is_set_high(self))
    }
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!Output::is_set_high(self))
    }
}

/// High-Z input (no pull — pull configuration lives in a separate IOCONFIG block).
pub struct Input<R: PinReg> {
    reg: &'static R,
}

impl<R: PinReg> Input<R> {
    pub fn is_high(&self) -> bool {
        self.reg.read_bits() & IN_BIT != 0
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    pub fn get_level(&self) -> Level {
        if self.is_high() { Level::High } else { Level::Low }
    }
}

impl<R: PinReg> embedded_hal::digital::ErrorType for Input<R> {
    type Error = core::convert::Infallible;
}

impl<R: PinReg> embedded_hal::digital::InputPin for Input<R> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_high(self))
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_low(self))
    }
}
