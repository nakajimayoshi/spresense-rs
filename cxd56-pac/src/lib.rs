#![no_std]

#[cfg(feature = "svd2rust")]
pub use cxd56_pac_svd2rust as pac;

#[cfg(feature = "chiptool")]
pub use cxd56_pac_chiptool as pac;
