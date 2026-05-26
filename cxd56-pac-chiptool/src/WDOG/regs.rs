///Watchdog control register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOGCONTROL(pub u32);
impl WDOGCONTROL {
    ///Interrupt output enable (starts the counter).
    #[must_use]
    #[inline(always)]
    pub const fn INTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Interrupt output enable (starts the counter).
    #[inline(always)]
    pub const fn set_INTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Reset output enable (enables chip reset on second underflow).
    #[must_use]
    #[inline(always)]
    pub const fn RESEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Reset output enable (enables chip reset on second underflow).
    #[inline(always)]
    pub const fn set_RESEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for WDOGCONTROL {
    #[inline(always)]
    fn default() -> WDOGCONTROL {
        WDOGCONTROL(0)
    }
}
impl core::fmt::Debug for WDOGCONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOGCONTROL")
            .field("INTEN", &self.INTEN())
            .field("RESEN", &self.RESEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDOGCONTROL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WDOGCONTROL {{ INTEN: {=bool:?}, RESEN: {=bool:?} }}",
            self.INTEN(),
            self.RESEN()
        )
    }
}
///Watchdog lock register (write 0x1ACCE551 to unlock).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOGLOCK(pub u32);
impl WDOGLOCK {
    ///Register write-access state (0 = unlocked, 1 = locked).
    #[must_use]
    #[inline(always)]
    pub const fn ACCESS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Register write-access state (0 = unlocked, 1 = locked).
    #[inline(always)]
    pub const fn set_ACCESS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for WDOGLOCK {
    #[inline(always)]
    fn default() -> WDOGLOCK {
        WDOGLOCK(0)
    }
}
impl core::fmt::Debug for WDOGLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOGLOCK")
            .field("ACCESS", &self.ACCESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDOGLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WDOGLOCK {{ ACCESS: {=bool:?} }}", self.ACCESS())
    }
}
///Watchdog masked interrupt status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOGMIS(pub u32);
impl WDOGMIS {
    ///Masked interrupt status (RAWINT AND INTEN).
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Masked interrupt status (RAWINT AND INTEN).
    #[inline(always)]
    pub const fn set_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for WDOGMIS {
    #[inline(always)]
    fn default() -> WDOGMIS {
        WDOGMIS(0)
    }
}
impl core::fmt::Debug for WDOGMIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOGMIS").field("INT", &self.INT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDOGMIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WDOGMIS {{ INT: {=bool:?} }}", self.INT())
    }
}
///Watchdog raw interrupt status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOGRIS(pub u32);
impl WDOGRIS {
    ///Raw interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn RAWINT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Raw interrupt status.
    #[inline(always)]
    pub const fn set_RAWINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for WDOGRIS {
    #[inline(always)]
    fn default() -> WDOGRIS {
        WDOGRIS(0)
    }
}
impl core::fmt::Debug for WDOGRIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOGRIS")
            .field("RAWINT", &self.RAWINT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDOGRIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WDOGRIS {{ RAWINT: {=bool:?} }}", self.RAWINT())
    }
}
