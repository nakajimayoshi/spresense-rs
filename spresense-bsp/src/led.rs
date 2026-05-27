use embedded_hal::digital::{ErrorType, OutputPin, StatefulOutputPin};

use cxd56_hal::gpio::{GpioPin, Level, Output};
use cxd56_hal::pac::topreg::GpI2s1Bck;

pub struct Led(Output<GpI2s1Bck>);

impl Led {
    pub(crate) fn new(pin: GpioPin<GpI2s1Bck>) -> Self {
        Led(pin.into_output(Level::Low))
    }

    pub fn on(&mut self) {
        self.0.set_high();
    }

    pub fn off(&mut self) {
        self.0.set_low();
    }

    pub fn toggle(&mut self) {
        if self.0.is_set_high() {
            self.0.set_low();
        } else {
            self.0.set_high();
        }
    }

    pub fn set(&mut self, level: Level) {
        self.0.set_level(level);
    }

    pub fn is_on(&self) -> bool {
        self.0.is_set_high()
    }
}

impl ErrorType for Led {
    type Error = core::convert::Infallible;
}

impl OutputPin for Led {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.on();
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.off();
        Ok(())
    }
}

impl StatefulOutputPin for Led {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_on())
    }
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_on())
    }
}
