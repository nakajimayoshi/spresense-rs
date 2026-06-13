use crate::pac;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

/// Input pull configuration for a pad's internal resistors.
///
/// Programmed via the per-pin TOPREG `IO_*` register, whose `PUN` (pull-up) and
/// `PDN` (pull-down) bits are both *active-low* (0 = that resistor enabled).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pull {
    /// No pull; the pin floats when undriven (`PUN=1, PDN=1`).
    Floating,
    /// Internal pull-up to the 1.8 V pad rail (`PUN=0, PDN=1`).
    Up,
    /// Internal pull-down to ground (`PUN=1, PDN=0`).
    Down,
    /// Bus keeper: both resistors on, weakly holding the last driven level
    /// (`PUN=0, PDN=0`). Matches NuttX `PINCONF_BUSKEEPER`.
    BusKeeper,
}

// Bit positions — identical layout for every TOPREG GP_* register.
const IN_BIT: u32 = 1 << 0;
const OUT_BIT: u32 = 1 << 8;
const DIR_BIT: u32 = 1 << 16; // active-low: 0 = drive output, 1 = high-Z input

// Bit positions — identical layout for every TOPREG IO_* (IOCELL) pad register.
// PUN/PDN are active-low: 0 = that pull resistor enabled. LOWEMI (bit 24, drive
// strength) is deliberately never written, so it keeps whatever value pinmux set.
const ENZI_BIT: u32 = 1 << 0; // input buffer: 1 = enabled
const PUN_BIT: u32 = 1 << 8; // pull-up:   0 = enabled, 1 = off
const PDN_BIT: u32 = 1 << 16; // pull-down: 0 = enabled, 1 = off

mod sealed {
    pub trait Sealed {
        fn read_bits(&self) -> u32;
        fn write_bits(&self, val: u32);
        fn read_io_bits(&self) -> u32;
        fn write_io_bits(&self, val: u32);
    }
}

/// Marker trait for svd2rust GPIO register accessors with the standard
/// IN[0] / OUT[8] / DIR[16] (active-low) field layout, paired with the pin's
/// `IO_*` (IOCELL) pad register for input-buffer and pull configuration.
pub trait PinReg: sealed::Sealed + 'static {}

// Each pin has a GP_* data register (`$gp`, IN/OUT/DIR) and a matching IO_*
// pad register (`$io`, ENZI/PUN/PDN/LOWEMI). The GP_* one is owned by the
// `GpioPin`; the IO_* one is reached through the shared `regs::topreg()` block
// (its aliasing invariant is documented there) since it is not individually
// owned. Macros cannot case-convert `GpFoo` → `io_foo`, so the pairs are
// spelled out explicitly.
macro_rules! impl_pinreg {
    ($($gp:ident => $io:ident),+ $(,)?) => {$(
        impl sealed::Sealed for pac::topreg::$gp {
            fn read_bits(&self) -> u32 {
                self.read().bits()
            }
            fn write_bits(&self, val: u32) {
                self.modify(|_, w| unsafe { w.bits(val) });
            }
            fn read_io_bits(&self) -> u32 {
                crate::regs::topreg().$io().read().bits()
            }
            fn write_io_bits(&self, val: u32) {
                crate::regs::topreg().$io().modify(|_, w| unsafe { w.bits(val) });
            }
        }
        impl PinReg for pac::topreg::$gp {}
    )+};
}

impl_pinreg!(
    // UART1 console pads (SPI0_CS_X = TX, SPI0_SCK = RX); Func0 is GPIO.
    GpSpi0CsX => io_spi0_cs_x,
    GpSpi0Sck => io_spi0_sck,
    // Main-board LEDs + Arduino D14 (pre-existing)
    GpI2s1Bck => io_i2s1_bck,
    GpI2s1Lrck => io_i2s1_lrck,
    GpI2s1DataIn => io_i2s1_data_in,
    GpI2s1DataOut => io_i2s1_data_out,
    GpI2c4Bck => io_i2c4_bck,
    // JP1 header
    GpUart2Txd => io_uart2_txd,
    GpUart2Rxd => io_uart2_rxd,
    GpUart2Rts => io_uart2_rts,
    GpUart2Cts => io_uart2_cts,
    GpI2s0Bck => io_i2s0_bck,
    GpI2s0Lrck => io_i2s0_lrck,
    GpEmmcCmd => io_emmc_cmd,
    GpEmmcClk => io_emmc_clk,
    GpSenIrqIn => io_sen_irq_in,
    // JP2 header
    GpEmmcData3 => io_emmc_data3,
    GpEmmcData2 => io_emmc_data2,
    GpI2s0DataIn => io_i2s0_data_in,
    GpI2s0DataOut => io_i2s0_data_out,
    GpEmmcData1 => io_emmc_data1,
    GpEmmcData0 => io_emmc_data0,
    GpI2c0Bck => io_i2c0_bck,
    GpI2c0Bdt => io_i2c0_bdt,
);

/// Program a pad's `IO_*` register for input use: enable the input buffer
/// (ENZI) and select the requested pull, preserving LOWEMI (drive strength).
fn configure_input_pad<R: PinReg>(reg: &R, pull: Pull) {
    let mut raw = reg.read_io_bits() | ENZI_BIT;
    // PUN/PDN are active-low: clear a bit to enable that resistor.
    match pull {
        Pull::Floating => raw |= PUN_BIT | PDN_BIT,
        Pull::Up => {
            raw &= !PUN_BIT;
            raw |= PDN_BIT;
        }
        Pull::Down => {
            raw |= PUN_BIT;
            raw &= !PDN_BIT;
        }
        Pull::BusKeeper => raw &= !(PUN_BIT | PDN_BIT),
    }
    reg.write_io_bits(raw);
}

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

    /// Configure as a push-pull output driving `initial`.
    ///
    /// This intentionally leaves the pad's `IO_*` register untouched: the reset
    /// state already has both pulls off, the output driver works regardless of
    /// the input-buffer/pull bits, and rewriting it would clobber a deliberate
    /// drive-strength (LOWEMI) or pull setting. Pad config is orthogonal to
    /// direction, matching NuttX's split between `cxd56_gpio` and pinmux.
    pub fn into_output(self, initial: Level) -> Output<R> {
        let raw = self.reg.read_bits() & !DIR_BIT; // DIR=0 → drive output
        let raw = match initial {
            Level::High => raw | OUT_BIT,
            Level::Low => raw & !OUT_BIT,
        };
        self.reg.write_bits(raw);
        Output { reg: self.reg }
    }

    /// Configure as a high-Z input with the given pull.
    ///
    /// Enables the pad input buffer (ENZI) and programs the pull in the `IO_*`
    /// register, *then* releases the GP_* direction to high-Z — so the chosen
    /// pull is already active the instant the pin stops driving.
    pub fn into_input(self, pull: Pull) -> Input<R> {
        configure_input_pad(self.reg, pull);
        let raw = self.reg.read_bits() | DIR_BIT; // DIR=1 → high-Z input
        self.reg.write_bits(raw);
        Input { reg: self.reg }
    }

    /// Configure as a floating (no-pull) input. See [`into_input`](Self::into_input).
    pub fn into_floating_input(self) -> Input<R> {
        self.into_input(Pull::Floating)
    }

    /// Configure as an input with the internal pull-up enabled.
    pub fn into_pull_up_input(self) -> Input<R> {
        self.into_input(Pull::Up)
    }

    /// Configure as an input with the internal pull-down enabled.
    pub fn into_pull_down_input(self) -> Input<R> {
        self.into_input(Pull::Down)
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

/// High-Z input.
///
/// The pad's input buffer (ENZI) and pull (`PUN`/`PDN`, both active-low) are set
/// when the pin is configured via [`GpioPin::into_input`] and friends; change the
/// pull at runtime with [`set_pull`](Self::set_pull).
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

    /// Change the pad pull at runtime, leaving the input buffer enabled.
    ///
    /// Switching pull is not instantaneous: the weak (~100 kΩ) resistor charges
    /// the pin/trace capacitance, so allow a brief settle before sampling a pin
    /// that nothing else is driving.
    pub fn set_pull(&mut self, pull: Pull) {
        configure_input_pad(self.reg, pull);
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
        /// SPI0_CS_X — UART1_TX / on-board console.
        /// Func0 = GPIO, Func1 = UART1, Func2 = SPI0.
        pub gp_spi0_cs_x: GpioPin<pac::topreg::GpSpi0CsX>,
        /// SPI0_SCK — UART1_RX / on-board console.
        /// Func0 = GPIO, Func1 = UART1, Func2 = SPI0.
        pub gp_spi0_sck: GpioPin<pac::topreg::GpSpi0Sck>,
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
                gp_spi0_cs_x: unsafe { GpioPin::new(block.gp_spi0_cs_x()) },
                gp_spi0_sck: unsafe { GpioPin::new(block.gp_spi0_sck()) },
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
