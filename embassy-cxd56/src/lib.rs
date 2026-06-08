#![no_std]

pub use cxd56_pac_chiptool as pac;

pub mod gpio;

embassy_hal_internal::peripherals! {
    UART1,
    UART2,
    I2S0,
    SPI5,
    I2C0,
    AIN4,
    AIN5,

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
    P1w_00,
    P1w_01,
    P1w_02,
    P1w_03,
}


pub fn init() -> Peripherals {
    Peripherals::take()
}
