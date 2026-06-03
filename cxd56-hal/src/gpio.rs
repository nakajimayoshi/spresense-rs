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

macro_rules! impl_pinreg {
    ($($reg:ident),+ $(,)?) => {$(
        impl sealed::Sealed for pac::topreg::$reg {
            fn read_bits(&self) -> u32 {
                self.read().bits()
            }
            fn write_bits(&self, val: u32) {
                self.modify(|_, w| unsafe { w.bits(val) });
            }
        }
        impl PinReg for pac::topreg::$reg {}
    )+};
}

// Every TOPREG GP_* register shares the IN[0] / OUT[8] / DIR[16] layout, so a
// single macro covers them all.
impl_pinreg!(
    // Main-board LEDs + Arduino D14 (pre-existing)
    GpI2s1Bck,
    GpI2s1Lrck,
    GpI2s1DataIn,
    GpI2s1DataOut,
    GpI2c4Bck,
    // JP1 header
    GpUart2Txd,
    GpUart2Rxd,
    GpUart2Rts,
    GpUart2Cts,
    GpI2s0Bck,
    GpI2s0Lrck,
    GpEmmcCmd,
    GpEmmcClk,
    GpSenIrqIn,
    // JP2 header
    GpEmmcData3,
    GpEmmcData2,
    GpI2s0DataIn,
    GpI2s0DataOut,
    GpEmmcData1,
    GpEmmcData0,
    GpI2c0Bck,
    GpI2c0Bdt,
);

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
    /// `gp_i2s1_data_out` = LED3). The remaining fields are the digital GPIO
    /// pins broken out on the main-board headers JP1 and JP2; field names follow
    /// the CXD5602 signal name, and the doc comments give the Arduino pin label.
    /// Pins default to mode0 (GPIO); selecting an alternate function (UART2,
    /// I2S0, SPI5, I2C0, …) is done via the `IOCAPP_IOMD` / `IOCSYS_IOMD1`
    /// mode-mux registers, which share a group across all pins of a peripheral.
    pub struct Parts {
        pub gp_i2s1_bck: GpioPin<pac::topreg::GpI2s1Bck>,
        pub gp_i2s1_lrck: GpioPin<pac::topreg::GpI2s1Lrck>,
        pub gp_i2s1_data_in: GpioPin<pac::topreg::GpI2s1DataIn>,
        pub gp_i2s1_data_out: GpioPin<pac::topreg::GpI2s1DataOut>,
        pub gp_i2c4_bck: GpioPin<pac::topreg::GpI2c4Bck>,

        // --- JP1 header ---
        /// JP1 pin 2 — UART2_TX (Arduino D01).
        pub gp_uart2_txd: GpioPin<pac::topreg::GpUart2Txd>,
        /// JP1 pin 3 — UART2_RX (Arduino D00).
        pub gp_uart2_rxd: GpioPin<pac::topreg::GpUart2Rxd>,
        /// JP1 pin 4 — UART2_RTS (Arduino D28).
        pub gp_uart2_rts: GpioPin<pac::topreg::GpUart2Rts>,
        /// JP1 pin 5 — UART2_CTS (Arduino D27).
        pub gp_uart2_cts: GpioPin<pac::topreg::GpUart2Cts>,
        /// JP1 pin 6 — I2S0_BCK (Arduino D26).
        pub gp_i2s0_bck: GpioPin<pac::topreg::GpI2s0Bck>,
        /// JP1 pin 7 — I2S0_LRCK (Arduino D25).
        pub gp_i2s0_lrck: GpioPin<pac::topreg::GpI2s0Lrck>,
        /// JP1 pin 8 — SPI5_CS_X / EMMC_CMD (Arduino D24).
        pub gp_emmc_cmd: GpioPin<pac::topreg::GpEmmcCmd>,
        /// JP1 pin 9 — SPI5_SCK / EMMC_CLK (Arduino D23).
        pub gp_emmc_clk: GpioPin<pac::topreg::GpEmmcClk>,
        /// JP1 pin 12 — SEN_IRQ (Arduino D22).
        pub gp_sen_irq_in: GpioPin<pac::topreg::GpSenIrqIn>,

        // --- JP2 header ---
        /// JP2 pin 4 — GPIO / EMMC_DATA3 (Arduino D21).
        pub gp_emmc_data3: GpioPin<pac::topreg::GpEmmcData3>,
        /// JP2 pin 5 — GPIO / EMMC_DATA2 (Arduino D20).
        pub gp_emmc_data2: GpioPin<pac::topreg::GpEmmcData2>,
        /// JP2 pin 6 — I2S0_DATA_IN (Arduino D19).
        pub gp_i2s0_data_in: GpioPin<pac::topreg::GpI2s0DataIn>,
        /// JP2 pin 7 — I2S0_DATA_OUT (Arduino D18).
        pub gp_i2s0_data_out: GpioPin<pac::topreg::GpI2s0DataOut>,
        /// JP2 pin 8 — SPI5_MISO / EMMC_DATA1 (Arduino D17).
        pub gp_emmc_data1: GpioPin<pac::topreg::GpEmmcData1>,
        /// JP2 pin 9 — SPI5_MOSI / EMMC_DATA0 (Arduino D16).
        pub gp_emmc_data0: GpioPin<pac::topreg::GpEmmcData0>,
        /// JP2 pin 11 — I2C0_SCL / I2C0_BCK (Arduino D15).
        pub gp_i2c0_bck: GpioPin<pac::topreg::GpI2c0Bck>,
        /// JP2 pin 12 — I2C0_SDA / I2C0_BDT (Arduino D14).
        pub gp_i2c0_bdt: GpioPin<pac::topreg::GpI2c0Bdt>,
    }

    impl Parts {
        pub fn new(_topreg: pac::Topreg) -> Self {
            // Safety: ownership of `pac::Topreg` — obtainable only via
            // `pac::Peripherals::take()` — guarantees no other code holds
            // a reference to this register block. The accessor in
            // `crate::regs::topreg()` documents the general aliasing invariant.
            let block = crate::regs::topreg();
            Self {
                gp_i2s1_bck: unsafe { GpioPin::new(block.gp_i2s1_bck()) },
                gp_i2s1_lrck: unsafe { GpioPin::new(block.gp_i2s1_lrck()) },
                gp_i2s1_data_in: unsafe { GpioPin::new(block.gp_i2s1_data_in()) },
                gp_i2s1_data_out: unsafe { GpioPin::new(block.gp_i2s1_data_out()) },
                gp_i2c4_bck: unsafe { GpioPin::new(block.gp_i2c4_bck()) },

                // JP1 header
                gp_uart2_txd: unsafe { GpioPin::new(block.gp_uart2_txd()) },
                gp_uart2_rxd: unsafe { GpioPin::new(block.gp_uart2_rxd()) },
                gp_uart2_rts: unsafe { GpioPin::new(block.gp_uart2_rts()) },
                gp_uart2_cts: unsafe { GpioPin::new(block.gp_uart2_cts()) },
                gp_i2s0_bck: unsafe { GpioPin::new(block.gp_i2s0_bck()) },
                gp_i2s0_lrck: unsafe { GpioPin::new(block.gp_i2s0_lrck()) },
                gp_emmc_cmd: unsafe { GpioPin::new(block.gp_emmc_cmd()) },
                gp_emmc_clk: unsafe { GpioPin::new(block.gp_emmc_clk()) },
                gp_sen_irq_in: unsafe { GpioPin::new(block.gp_sen_irq_in()) },

                // JP2 header
                gp_emmc_data3: unsafe { GpioPin::new(block.gp_emmc_data3()) },
                gp_emmc_data2: unsafe { GpioPin::new(block.gp_emmc_data2()) },
                gp_i2s0_data_in: unsafe { GpioPin::new(block.gp_i2s0_data_in()) },
                gp_i2s0_data_out: unsafe { GpioPin::new(block.gp_i2s0_data_out()) },
                gp_emmc_data1: unsafe { GpioPin::new(block.gp_emmc_data1()) },
                gp_emmc_data0: unsafe { GpioPin::new(block.gp_emmc_data0()) },
                gp_i2c0_bck: unsafe { GpioPin::new(block.gp_i2c0_bck()) },
                gp_i2c0_bdt: unsafe { GpioPin::new(block.gp_i2c0_bdt()) },
            }
        }
    }
}
