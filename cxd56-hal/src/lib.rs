#![no_std]

pub use cxd56_pac_svd2rust as pac;

pub mod clocks;
pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod uart;
