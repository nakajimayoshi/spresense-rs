///RX data read complete.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIF_PULL_CMP(pub u32);
impl FIF_PULL_CMP {
    ///RX data read complete.
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CMP(&self) -> super::vals::PULL_CMP {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PULL_CMP::from_bits(val as u8)
    }
    ///RX data read complete.
    #[inline(always)]
    pub const fn set_PULL_CMP(&mut self, val: super::vals::PULL_CMP) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FIF_PULL_CMP {
    #[inline(always)]
    fn default() -> FIF_PULL_CMP {
        FIF_PULL_CMP(0)
    }
}
impl core::fmt::Debug for FIF_PULL_CMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIF_PULL_CMP")
            .field("PULL_CMP", &self.PULL_CMP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIF_PULL_CMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIF_PULL_CMP {{ PULL_CMP: {:?} }}", self.PULL_CMP())
    }
}
///RX buffer is empty (=1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIF_PULL_EMP(pub u32);
impl FIF_PULL_EMP {
    ///RX buffer is empty.
    #[must_use]
    #[inline(always)]
    pub const fn EMPTY_FLAG(&self) -> super::vals::EMPTY_FLAG {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EMPTY_FLAG::from_bits(val as u8)
    }
    ///RX buffer is empty.
    #[inline(always)]
    pub const fn set_EMPTY_FLAG(&mut self, val: super::vals::EMPTY_FLAG) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FIF_PULL_EMP {
    #[inline(always)]
    fn default() -> FIF_PULL_EMP {
        FIF_PULL_EMP(0)
    }
}
impl core::fmt::Debug for FIF_PULL_EMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIF_PULL_EMP")
            .field("EMPTY_FLAG", &self.EMPTY_FLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIF_PULL_EMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIF_PULL_EMP {{ EMPTY_FLAG: {:?} }}", self.EMPTY_FLAG())
    }
}
///RX data word 0.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIF_PULL_WRD0(pub u32);
impl FIF_PULL_WRD0 {
    ///RX data word 0.
    #[must_use]
    #[inline(always)]
    pub const fn DATA_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    ///RX data word 0.
    #[inline(always)]
    pub const fn set_DATA_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    ///Sender ID.
    #[must_use]
    #[inline(always)]
    pub const fn SENDER_ID(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    ///Sender ID.
    #[inline(always)]
    pub const fn set_SENDER_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FIF_PULL_WRD0 {
    #[inline(always)]
    fn default() -> FIF_PULL_WRD0 {
        FIF_PULL_WRD0(0)
    }
}
impl core::fmt::Debug for FIF_PULL_WRD0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIF_PULL_WRD0")
            .field("DATA_0", &self.DATA_0())
            .field("SENDER_ID", &self.SENDER_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIF_PULL_WRD0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIF_PULL_WRD0 {{ DATA_0: {=u32:?}, SENDER_ID: {=u8:?} }}",
            self.DATA_0(),
            self.SENDER_ID()
        )
    }
}
///TX data write complete.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIF_PUSH_CMP(pub u32);
impl FIF_PUSH_CMP {
    ///TX data write complete.
    #[must_use]
    #[inline(always)]
    pub const fn PUSH_CMP(&self) -> super::vals::PUSH_CMP {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PUSH_CMP::from_bits(val as u8)
    }
    ///TX data write complete.
    #[inline(always)]
    pub const fn set_PUSH_CMP(&mut self, val: super::vals::PUSH_CMP) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FIF_PUSH_CMP {
    #[inline(always)]
    fn default() -> FIF_PUSH_CMP {
        FIF_PUSH_CMP(0)
    }
}
impl core::fmt::Debug for FIF_PUSH_CMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIF_PUSH_CMP")
            .field("PUSH_CMP", &self.PUSH_CMP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIF_PUSH_CMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIF_PUSH_CMP {{ PUSH_CMP: {:?} }}", self.PUSH_CMP())
    }
}
///TX buffer is full (=1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIF_PUSH_FULL(pub u32);
impl FIF_PUSH_FULL {
    ///TX buffer is full.
    #[must_use]
    #[inline(always)]
    pub const fn FULL_FLAG(&self) -> super::vals::FULL_FLAG {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FULL_FLAG::from_bits(val as u8)
    }
    ///TX buffer is full.
    #[inline(always)]
    pub const fn set_FULL_FLAG(&mut self, val: super::vals::FULL_FLAG) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FIF_PUSH_FULL {
    #[inline(always)]
    fn default() -> FIF_PUSH_FULL {
        FIF_PUSH_FULL(0)
    }
}
impl core::fmt::Debug for FIF_PUSH_FULL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIF_PUSH_FULL")
            .field("FULL_FLAG", &self.FULL_FLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIF_PUSH_FULL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FIF_PUSH_FULL {{ FULL_FLAG: {:?} }}", self.FULL_FLAG())
    }
}
///TX data word 0.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FIF_PUSH_WRD0(pub u32);
impl FIF_PUSH_WRD0 {
    ///TX data word 0.
    #[must_use]
    #[inline(always)]
    pub const fn DATA_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    ///TX data word 0.
    #[inline(always)]
    pub const fn set_DATA_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    ///Target ID.
    #[must_use]
    #[inline(always)]
    pub const fn RECEIVER_ID(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    ///Target ID.
    #[inline(always)]
    pub const fn set_RECEIVER_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FIF_PUSH_WRD0 {
    #[inline(always)]
    fn default() -> FIF_PUSH_WRD0 {
        FIF_PUSH_WRD0(0)
    }
}
impl core::fmt::Debug for FIF_PUSH_WRD0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIF_PUSH_WRD0")
            .field("DATA_0", &self.DATA_0())
            .field("RECEIVER_ID", &self.RECEIVER_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIF_PUSH_WRD0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FIF_PUSH_WRD0 {{ DATA_0: {=u32:?}, RECEIVER_ID: {=u8:?} }}",
            self.DATA_0(),
            self.RECEIVER_ID()
        )
    }
}
