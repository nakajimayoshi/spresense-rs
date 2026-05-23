///Timer mode, prescaler, and interrupt control.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONTROL(pub u32);
impl CONTROL {
    ///Reload behavior on zero.
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Reload behavior on zero.
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Counter width.
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Counter width.
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Input clock prescaler.
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    ///Input clock prescaler.
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    ///Interrupt enable.
    #[must_use]
    #[inline(always)]
    pub const fn INTENABLE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Interrupt enable.
    #[inline(always)]
    pub const fn set_INTENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Counter mode.
    #[must_use]
    #[inline(always)]
    pub const fn PERIODIC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Counter mode.
    #[inline(always)]
    pub const fn set_PERIODIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Timer enable.
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Timer enable.
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for CONTROL {
    #[inline(always)]
    fn default() -> CONTROL {
        CONTROL(0)
    }
}
impl core::fmt::Debug for CONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL")
            .field("MODE", &self.MODE())
            .field("SIZE", &self.SIZE())
            .field("DIV", &self.DIV())
            .field("INTENABLE", &self.INTENABLE())
            .field("PERIODIC", &self.PERIODIC())
            .field("ENABLE", &self.ENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONTROL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONTROL {{ MODE: {=bool:?}, SIZE: {=bool:?}, DIV: {=u8:?}, INTENABLE: {=bool:?}, PERIODIC: {=bool:?}, ENABLE: {=bool:?} }}",
            self.MODE(),
            self.SIZE(),
            self.DIV(),
            self.INTENABLE(),
            self.PERIODIC(),
            self.ENABLE()
        )
    }
}
///Masked interrupt status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MIS(pub u32);
impl MIS {
    ///Masked interrupt (RIS AND INTENABLE).
    #[must_use]
    #[inline(always)]
    pub const fn TIMER_INTERRUPT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Masked interrupt (RIS AND INTENABLE).
    #[inline(always)]
    pub const fn set_TIMER_INTERRUPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for MIS {
    #[inline(always)]
    fn default() -> MIS {
        MIS(0)
    }
}
impl core::fmt::Debug for MIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("TIMER_INTERRUPT", &self.TIMER_INTERRUPT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MIS {{ TIMER_INTERRUPT: {=bool:?} }}",
            self.TIMER_INTERRUPT()
        )
    }
}
///Raw interrupt status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RIS(pub u32);
impl RIS {
    ///Raw interrupt (set when counter reaches zero).
    #[must_use]
    #[inline(always)]
    pub const fn TIMER_INTERRUPT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Raw interrupt (set when counter reaches zero).
    #[inline(always)]
    pub const fn set_TIMER_INTERRUPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RIS {
    #[inline(always)]
    fn default() -> RIS {
        RIS(0)
    }
}
impl core::fmt::Debug for RIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("TIMER_INTERRUPT", &self.TIMER_INTERRUPT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RIS {{ TIMER_INTERRUPT: {=bool:?} }}",
            self.TIMER_INTERRUPT()
        )
    }
}
