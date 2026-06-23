#![no_std]

pub use cxd56_pac_chiptool as pac;

mod clock;
pub mod gpio;
pub mod uart;

embassy_hal_internal::peripherals! {
    UART1,
    UART2,
    I2S0,
    SPI5,
    I2C0,
    AIN4,
    AIN5,

    /// UART1 TXD pad (SPI0_CS_X, pin 17) — Func1 routes it to UART1.
    SPI0_CS_X,
    /// UART1 RXD pad (SPI0_SCK, pin 18) — Func1 routes it to UART1.
    SPI0_SCK,

    P1e_00,
    P1j_00,
    P1j_01,
    P1n_00,
    P1n_01,
    P1n_02,
    P1n_03,
    P1p_00,
    P1p_01,
    P1p_02,
    P1p_03,
    P1q_00,
    P1q_01,
    P1v_00,
    P1v_01,
    P1v_02,
    P1v_03,
    /// LED0, I2S1 BCK
    P1w_00,
    /// LED1
    P1w_01,
    /// LED2
    P1w_02,
    /// LED3
    P1w_03,
}


pub fn init() -> Peripherals {
    Peripherals::take()
}
