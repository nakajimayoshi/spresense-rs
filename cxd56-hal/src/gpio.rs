use crate::pac;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

// Bit positions — identical layout for every TOPREG GP_* register.
const IN_BIT: u32 = 1 << 0;
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

impl sealed::Sealed for pac::topreg::GpI2s1Bck {
    fn read_bits(&self) -> u32 {
        self.read().bits()
    }
    fn write_bits(&self, val: u32) {
        self.modify(|_, w| unsafe { w.bits(val) });
    }
}
impl PinReg for pac::topreg::GpI2s1Bck {}

impl sealed::Sealed for pac::topreg::GpI2c4Bck {
    fn read_bits(&self) -> u32 {
        self.read().bits()
    }
    fn write_bits(&self, val: u32) {
        self.modify(|_, w| unsafe { w.bits(val) });
    }
}
impl PinReg for pac::topreg::GpI2c4Bck {}

impl sealed::Sealed for pac::topreg::GpI2s1Lrck {
    fn read_bits(&self) -> u32 {
        self.read().bits()
    }
    fn write_bits(&self, val: u32) {
        self.modify(|_, w| unsafe { w.bits(val) });
    }
}
impl PinReg for pac::topreg::GpI2s1Lrck {}

impl sealed::Sealed for pac::topreg::GpI2s1DataIn {
    fn read_bits(&self) -> u32 {
        self.read().bits()
    }
    fn write_bits(&self, val: u32) {
        self.modify(|_, w| unsafe { w.bits(val) });
    }
}
impl PinReg for pac::topreg::GpI2s1DataIn {}

impl sealed::Sealed for pac::topreg::GpI2s1DataOut {
    fn read_bits(&self) -> u32 {
        self.read().bits()
    }
    fn write_bits(&self, val: u32) {
        self.modify(|_, w| unsafe { w.bits(val) });
    }
}
impl PinReg for pac::topreg::GpI2s1DataOut {}

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
            Level::Low => raw & !OUT_BIT,
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
            Level::Low => self.set_low(),
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
        if self.is_high() {
            Level::High
        } else {
            Level::Low
        }
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

/// Per-port split structs.
///
/// Mirrors the convention used by `stm32f4xx-hal` (`gpioa::Parts`),
/// `nrf-hal` (`p0::Parts`), etc. Construct via [`Parts::new`], which
/// consumes the corresponding PAC singleton — proving exclusive access
/// so no `unsafe` is needed at the call site.
pub mod pins {
    use super::GpioPin;
    use crate::pac;

    /// GPIO pins accessible via the TOPREG GP_* registers.
    ///
    /// The four `gp_i2s1_*` pins drive the Spresense main-board LEDs
    /// (`gp_i2s1_bck` = LED0, `gp_i2s1_lrck` = LED1, `gp_i2s1_data_in` = LED2,
    /// `gp_i2s1_data_out` = LED3); `gp_i2c4_bck` is the Arduino D14 header pin.
    pub struct Parts {
        pub gp_i2s1_bck: GpioPin<pac::topreg::GpI2s1Bck>,
        pub gp_i2s1_lrck: GpioPin<pac::topreg::GpI2s1Lrck>,
        pub gp_i2s1_data_in: GpioPin<pac::topreg::GpI2s1DataIn>,
        pub gp_i2s1_data_out: GpioPin<pac::topreg::GpI2s1DataOut>,
        pub gp_i2c4_bck: GpioPin<pac::topreg::GpI2c4Bck>,
    }

    impl Parts {
        pub fn new(_topreg: pac::Topreg) -> Self {
            // Safety: ownership of `pac::Topreg` — obtainable only via
            // `pac::Peripherals::take()` — guarantees no other code holds
            // a reference to this register block.
            let block = unsafe { &*pac::Topreg::PTR };
            Self {
                gp_i2s1_bck: unsafe { GpioPin::new(block.gp_i2s1_bck()) },
                gp_i2s1_lrck: unsafe { GpioPin::new(block.gp_i2s1_lrck()) },
                gp_i2s1_data_in: unsafe { GpioPin::new(block.gp_i2s1_data_in()) },
                gp_i2s1_data_out: unsafe { GpioPin::new(block.gp_i2s1_data_out()) },
                gp_i2c4_bck: unsafe { GpioPin::new(block.gp_i2c4_bck()) },
            }
        }
    }
}
