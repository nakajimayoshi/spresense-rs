#![no_std]

pub use cxd56_pac_svd2rust as pac;

pub mod clocks;
#[cfg(feature = "critical-section-impl")]
pub mod critical_section_impl;
pub mod delay;
pub mod delay_alt;
pub mod farapi;
pub mod gpio;
pub mod i2c;
pub mod adc;
pub mod audio_aca;
pub mod i2c_alt;
pub mod i2s_alt;
pub mod interrupt;
pub mod i2s_alt;
pub mod multicore;
pub mod pmic;
pub mod spi_alt;
pub(crate) mod regs;
pub mod timer;
pub mod uart;
pub mod uart_alt;
pub mod watchdog;
