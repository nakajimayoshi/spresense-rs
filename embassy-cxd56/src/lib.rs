#![no_std]

pub use cxd56_pac_chiptool as pac;

pub mod gpio;

embassy_hal_internal::peripherals! {
    PIN97,
}

pub fn init() -> Peripherals {
    Peripherals::take()
}