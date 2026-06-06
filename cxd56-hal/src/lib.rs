#![no_std]

pub use cxd56_pac_svd2rust as pac;

pub mod clocks;
pub mod delay;
pub mod delay_alt;
pub mod gpio;
pub mod i2c;
pub mod multicore;
pub(crate) mod regs;
pub mod uart;
pub mod uart_alt;
