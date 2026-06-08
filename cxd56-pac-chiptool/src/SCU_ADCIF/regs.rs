///ADCIF register-map revision code (RO; reads 0xADC1F000).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCIF_DCT(pub u32);
impl ADCIF_DCT {
    ///Revision code (always 0xADC1F000).
    #[must_use]
    #[inline(always)]
    pub const fn REVISION(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Revision code (always 0xADC1F000).
    #[inline(always)]
    pub const fn set_REVISION(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADCIF_DCT {
    #[inline(always)]
    fn default() -> ADCIF_DCT {
        ADCIF_DCT(0)
    }
}
impl core::fmt::Debug for ADCIF_DCT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCIF_DCT")
            .field("REVISION", &self.REVISION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCIF_DCT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADCIF_DCT {{ REVISION: {=u32:?} }}", self.REVISION())
    }
}
///HPADC0 clock selection (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC0_A0(pub u32);
impl HPADC0_A0 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC0_A0 {
    #[inline(always)]
    fn default() -> HPADC0_A0 {
        HPADC0_A0(0)
    }
}
impl core::fmt::Debug for HPADC0_A0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC0_A0")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC0_A0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC0_A0 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC0 enable control (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC0_A1(pub u32);
impl HPADC0_A1 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC0_A1 {
    #[inline(always)]
    fn default() -> HPADC0_A1 {
        HPADC0_A1(0)
    }
}
impl core::fmt::Debug for HPADC0_A1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC0_A1")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC0_A1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC0_A1 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC0 clock enable (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC0_A2(pub u32);
impl HPADC0_A2 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC0_A2 {
    #[inline(always)]
    fn default() -> HPADC0_A2 {
        HPADC0_A2(0)
    }
}
impl core::fmt::Debug for HPADC0_A2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC0_A2")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC0_A2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC0_A2 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC0 LPF control (placeholder; reset=0x100).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC0_A3(pub u32);
impl HPADC0_A3 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC0_A3 {
    #[inline(always)]
    fn default() -> HPADC0_A3 {
        HPADC0_A3(0)
    }
}
impl core::fmt::Debug for HPADC0_A3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC0_A3")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC0_A3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC0_A3 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC0 software reset (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC0_D0(pub u32);
impl HPADC0_D0 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC0_D0 {
    #[inline(always)]
    fn default() -> HPADC0_D0 {
        HPADC0_D0(0)
    }
}
impl core::fmt::Debug for HPADC0_D0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC0_D0")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC0_D0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC0_D0 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC0 basic setting (placeholder; reset=0x10).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC0_D1(pub u32);
impl HPADC0_D1 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC0_D1 {
    #[inline(always)]
    fn default() -> HPADC0_D1 {
        HPADC0_D1(0)
    }
}
impl core::fmt::Debug for HPADC0_D1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC0_D1")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC0_D1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC0_D1 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC0 ADC data acceptance (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC0_D2(pub u32);
impl HPADC0_D2 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC0_D2 {
    #[inline(always)]
    fn default() -> HPADC0_D2 {
        HPADC0_D2(0)
    }
}
impl core::fmt::Debug for HPADC0_D2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC0_D2")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC0_D2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC0_D2 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC1 clock selection (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC1_A0(pub u32);
impl HPADC1_A0 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC1_A0 {
    #[inline(always)]
    fn default() -> HPADC1_A0 {
        HPADC1_A0(0)
    }
}
impl core::fmt::Debug for HPADC1_A0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC1_A0")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC1_A0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC1_A0 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC1 enable control (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC1_A1(pub u32);
impl HPADC1_A1 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC1_A1 {
    #[inline(always)]
    fn default() -> HPADC1_A1 {
        HPADC1_A1(0)
    }
}
impl core::fmt::Debug for HPADC1_A1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC1_A1")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC1_A1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC1_A1 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC1 clock enable (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC1_A2(pub u32);
impl HPADC1_A2 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC1_A2 {
    #[inline(always)]
    fn default() -> HPADC1_A2 {
        HPADC1_A2(0)
    }
}
impl core::fmt::Debug for HPADC1_A2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC1_A2")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC1_A2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC1_A2 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC1 LPF control (placeholder; reset=0x100).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC1_A3(pub u32);
impl HPADC1_A3 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC1_A3 {
    #[inline(always)]
    fn default() -> HPADC1_A3 {
        HPADC1_A3(0)
    }
}
impl core::fmt::Debug for HPADC1_A3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC1_A3")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC1_A3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC1_A3 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC1 software reset (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC1_D0(pub u32);
impl HPADC1_D0 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC1_D0 {
    #[inline(always)]
    fn default() -> HPADC1_D0 {
        HPADC1_D0(0)
    }
}
impl core::fmt::Debug for HPADC1_D0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC1_D0")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC1_D0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC1_D0 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC1 basic setting (placeholder; reset=0x10).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC1_D1(pub u32);
impl HPADC1_D1 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC1_D1 {
    #[inline(always)]
    fn default() -> HPADC1_D1 {
        HPADC1_D1(0)
    }
}
impl core::fmt::Debug for HPADC1_D1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC1_D1")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC1_D1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC1_D1 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC1 ADC data acceptance (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC1_D2(pub u32);
impl HPADC1_D2 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC1_D2 {
    #[inline(always)]
    fn default() -> HPADC1_D2 {
        HPADC1_D2(0)
    }
}
impl core::fmt::Debug for HPADC1_D2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC1_D2")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC1_D2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC1_D2 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC common clock control (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC_AC0(pub u32);
impl HPADC_AC0 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC_AC0 {
    #[inline(always)]
    fn default() -> HPADC_AC0 {
        HPADC_AC0(0)
    }
}
impl core::fmt::Debug for HPADC_AC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC_AC0")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC_AC0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC_AC0 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC common enable control (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC_AC1(pub u32);
impl HPADC_AC1 {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC_AC1 {
    #[inline(always)]
    fn default() -> HPADC_AC1 {
        HPADC_AC1(0)
    }
}
impl core::fmt::Debug for HPADC_AC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC_AC1")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC_AC1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC_AC1 {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC two-element vector mode control (placeholder).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC_DC(pub u32);
impl HPADC_DC {
    ///Raw register value.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Raw register value.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HPADC_DC {
    #[inline(always)]
    fn default() -> HPADC_DC {
        HPADC_DC(0)
    }
}
impl core::fmt::Debug for HPADC_DC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC_DC")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC_DC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC_DC {{ BITS: {=u32:?} }}", self.BITS())
    }
}
///HPADC ch%s FIFO Read Port (SEN_AIN0/1, Arduino A4/A5).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPADC_FIFO(pub u32);
impl HPADC_FIFO {
    ///10-bit HPADC sample, right-justified.
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    ///10-bit HPADC sample, right-justified.
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u32) & 0x03ff) << 6usize);
    }
}
impl Default for HPADC_FIFO {
    #[inline(always)]
    fn default() -> HPADC_FIFO {
        HPADC_FIFO(0)
    }
}
impl core::fmt::Debug for HPADC_FIFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPADC_FIFO")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPADC_FIFO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HPADC_FIFO {{ DATA: {=u16:?} }}", self.DATA())
    }
}
///LPADC Enable Control — LV_ADC_EN (UM Table ADC-106).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_A0(pub u32);
impl LPADC_A0 {
    ///LPADC enable: 0=standby, 1=active.
    #[must_use]
    #[inline(always)]
    pub const fn LV_ADC_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///LPADC enable: 0=standby, 1=active.
    #[inline(always)]
    pub const fn set_LV_ADC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for LPADC_A0 {
    #[inline(always)]
    fn default() -> LPADC_A0 {
        LPADC_A0(0)
    }
}
impl core::fmt::Debug for LPADC_A0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_A0")
            .field("LV_ADC_EN", &self.LV_ADC_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_A0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPADC_A0 {{ LV_ADC_EN: {=bool:?} }}", self.LV_ADC_EN())
    }
}
///LPADC Channel Switching Control (UM Table ADC-107).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_A1(pub u32);
impl LPADC_A1 {
    ///Channel switching mode.
    #[must_use]
    #[inline(always)]
    pub const fn LV_CH_SEL_MODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    ///Channel switching mode.
    #[inline(always)]
    pub const fn set_LV_CH_SEL_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    ///ADCH selector for GNSS (active channel: 0=CH0 … 3=CH3).
    #[must_use]
    #[inline(always)]
    pub const fn LV_CH_SEL_INV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    ///ADCH selector for GNSS (active channel: 0=CH0 … 3=CH3).
    #[inline(always)]
    pub const fn set_LV_CH_SEL_INV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for LPADC_A1 {
    #[inline(always)]
    fn default() -> LPADC_A1 {
        LPADC_A1(0)
    }
}
impl core::fmt::Debug for LPADC_A1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_A1")
            .field("LV_CH_SEL_MODE", &self.LV_CH_SEL_MODE())
            .field("LV_CH_SEL_INV", &self.LV_CH_SEL_INV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_A1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPADC_A1 {{ LV_CH_SEL_MODE: {=u8:?}, LV_CH_SEL_INV: {=u8:?} }}",
            self.LV_CH_SEL_MODE(),
            self.LV_CH_SEL_INV()
        )
    }
}
///LPADC analog timing register 0 (bit0 must be cleared, cxd56_adc.c:469).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_AT0(pub u32);
impl LPADC_AT0 {
    ///Analog timing control bit 0 (SDK always writes 0).
    #[must_use]
    #[inline(always)]
    pub const fn TIMING0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Analog timing control bit 0 (SDK always writes 0).
    #[inline(always)]
    pub const fn set_TIMING0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for LPADC_AT0 {
    #[inline(always)]
    fn default() -> LPADC_AT0 {
        LPADC_AT0(0)
    }
}
impl core::fmt::Debug for LPADC_AT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_AT0")
            .field("TIMING0", &self.TIMING0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_AT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPADC_AT0 {{ TIMING0: {=bool:?} }}", self.TIMING0())
    }
}
///LPADC analog warm-up timing (SDK sets bits\[7:0\]=0x04, cxd56_adc.c:462).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_AT1(pub u32);
impl LPADC_AT1 {
    ///Analog warm-up count (SDK: 0x04).
    #[must_use]
    #[inline(always)]
    pub const fn WARMUP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Analog warm-up count (SDK: 0x04).
    #[inline(always)]
    pub const fn set_WARMUP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for LPADC_AT1 {
    #[inline(always)]
    fn default() -> LPADC_AT1 {
        LPADC_AT1(0)
    }
}
impl core::fmt::Debug for LPADC_AT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_AT1")
            .field("WARMUP", &self.WARMUP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_AT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPADC_AT1 {{ WARMUP: {=u8:?} }}", self.WARMUP())
    }
}
///LPADC Software Reset, common to all channels (UM Table ADC-108).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_D0(pub u32);
impl LPADC_D0 {
    ///LPADC software reset (write 1 to reset, then clear).
    #[must_use]
    #[inline(always)]
    pub const fn SW_RESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///LPADC software reset (write 1 to reset, then clear).
    #[inline(always)]
    pub const fn set_SW_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for LPADC_D0 {
    #[inline(always)]
    fn default() -> LPADC_D0 {
        LPADC_D0(0)
    }
}
impl core::fmt::Debug for LPADC_D0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_D0")
            .field("SW_RESET", &self.SW_RESET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_D0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPADC_D0 {{ SW_RESET: {=bool:?} }}", self.SW_RESET())
    }
}
///LPADC ch0 sample-rate divider and FIFO watermark (UM Table ADC-109).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_D1(pub u32);
impl LPADC_D1 {
    ///FIFO almost-full watermark (default 8; set to 1 for immediate notification).
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_WATERMARK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    ///FIFO almost-full watermark (default 8; set to 1 for immediate notification).
    #[inline(always)]
    pub const fn set_FIFO_WATERMARK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    ///DMA handshake-signal enable for this channel.
    #[must_use]
    #[inline(always)]
    pub const fn DMA_HS_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///DMA handshake-signal enable for this channel.
    #[inline(always)]
    pub const fn set_DMA_HS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Sampling frequency divider: rate = CK_SCU_U32KL / 2^n (0=÷1 … 15=÷32768).
    #[must_use]
    #[inline(always)]
    pub const fn SAMP_RATIO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    ///Sampling frequency divider: rate = CK_SCU_U32KL / 2^n (0=÷1 … 15=÷32768).
    #[inline(always)]
    pub const fn set_SAMP_RATIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    ///PWM synchronisation enables, bits\[8:0\] (bit8=disabled).
    #[must_use]
    #[inline(always)]
    pub const fn SAMP_RATIO2(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x01ff;
        val as u16
    }
    ///PWM synchronisation enables, bits\[8:0\] (bit8=disabled).
    #[inline(always)]
    pub const fn set_SAMP_RATIO2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 12usize)) | (((val as u32) & 0x01ff) << 12usize);
    }
}
impl Default for LPADC_D1 {
    #[inline(always)]
    fn default() -> LPADC_D1 {
        LPADC_D1(0)
    }
}
impl core::fmt::Debug for LPADC_D1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_D1")
            .field("FIFO_WATERMARK", &self.FIFO_WATERMARK())
            .field("DMA_HS_EN", &self.DMA_HS_EN())
            .field("SAMP_RATIO", &self.SAMP_RATIO())
            .field("SAMP_RATIO2", &self.SAMP_RATIO2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_D1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPADC_D1 {{ FIFO_WATERMARK: {=u8:?}, DMA_HS_EN: {=bool:?}, SAMP_RATIO: {=u8:?}, SAMP_RATIO2: {=u16:?} }}",
            self.FIFO_WATERMARK(),
            self.DMA_HS_EN(),
            self.SAMP_RATIO(),
            self.SAMP_RATIO2()
        )
    }
}
///LPADC per-channel FIFO enable (UM Table ADC-110). Bit n=1 enables ADCIF hardware FIFO for channel n.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_D2(pub u32);
impl LPADC_D2 {
    ///Per-channel FIFO enable: bit0=ch0, bit1=ch1, bit2=ch2, bit3=ch3.
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_EN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    ///Per-channel FIFO enable: bit0=ch0, bit1=ch1, bit2=ch2, bit3=ch3.
    #[inline(always)]
    pub const fn set_FIFO_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for LPADC_D2 {
    #[inline(always)]
    fn default() -> LPADC_D2 {
        LPADC_D2(0)
    }
}
impl core::fmt::Debug for LPADC_D2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_D2")
            .field("FIFO_EN", &self.FIFO_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_D2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPADC_D2 {{ FIFO_EN: {=u8:?} }}", self.FIFO_EN())
    }
}
///LPADC ch1 sample-rate divider and FIFO watermark (UM Table ADC-111).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_D4(pub u32);
impl LPADC_D4 {
    ///FIFO watermark.
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_WATERMARK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    ///FIFO watermark.
    #[inline(always)]
    pub const fn set_FIFO_WATERMARK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    ///DMA handshake enable.
    #[must_use]
    #[inline(always)]
    pub const fn DMA_HS_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///DMA handshake enable.
    #[inline(always)]
    pub const fn set_DMA_HS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Frequency divider: 0=÷1 … 15=÷32768.
    #[must_use]
    #[inline(always)]
    pub const fn SAMP_RATIO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    ///Frequency divider: 0=÷1 … 15=÷32768.
    #[inline(always)]
    pub const fn set_SAMP_RATIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    ///PWM sync enables \[8:0\].
    #[must_use]
    #[inline(always)]
    pub const fn SAMP_RATIO2(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x01ff;
        val as u16
    }
    ///PWM sync enables \[8:0\].
    #[inline(always)]
    pub const fn set_SAMP_RATIO2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 12usize)) | (((val as u32) & 0x01ff) << 12usize);
    }
}
impl Default for LPADC_D4 {
    #[inline(always)]
    fn default() -> LPADC_D4 {
        LPADC_D4(0)
    }
}
impl core::fmt::Debug for LPADC_D4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_D4")
            .field("FIFO_WATERMARK", &self.FIFO_WATERMARK())
            .field("DMA_HS_EN", &self.DMA_HS_EN())
            .field("SAMP_RATIO", &self.SAMP_RATIO())
            .field("SAMP_RATIO2", &self.SAMP_RATIO2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_D4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPADC_D4 {{ FIFO_WATERMARK: {=u8:?}, DMA_HS_EN: {=bool:?}, SAMP_RATIO: {=u8:?}, SAMP_RATIO2: {=u16:?} }}",
            self.FIFO_WATERMARK(),
            self.DMA_HS_EN(),
            self.SAMP_RATIO(),
            self.SAMP_RATIO2()
        )
    }
}
///LPADC ch2 (SEN_AIN4 / Arduino A2) sample-rate and watermark.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_D5(pub u32);
impl LPADC_D5 {
    ///FIFO watermark.
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_WATERMARK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    ///FIFO watermark.
    #[inline(always)]
    pub const fn set_FIFO_WATERMARK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    ///DMA handshake enable.
    #[must_use]
    #[inline(always)]
    pub const fn DMA_HS_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///DMA handshake enable.
    #[inline(always)]
    pub const fn set_DMA_HS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Frequency divider: 0=÷1 … 15=÷32768.
    #[must_use]
    #[inline(always)]
    pub const fn SAMP_RATIO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    ///Frequency divider: 0=÷1 … 15=÷32768.
    #[inline(always)]
    pub const fn set_SAMP_RATIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    ///PWM sync enables \[8:0\].
    #[must_use]
    #[inline(always)]
    pub const fn SAMP_RATIO2(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x01ff;
        val as u16
    }
    ///PWM sync enables \[8:0\].
    #[inline(always)]
    pub const fn set_SAMP_RATIO2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 12usize)) | (((val as u32) & 0x01ff) << 12usize);
    }
}
impl Default for LPADC_D5 {
    #[inline(always)]
    fn default() -> LPADC_D5 {
        LPADC_D5(0)
    }
}
impl core::fmt::Debug for LPADC_D5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_D5")
            .field("FIFO_WATERMARK", &self.FIFO_WATERMARK())
            .field("DMA_HS_EN", &self.DMA_HS_EN())
            .field("SAMP_RATIO", &self.SAMP_RATIO())
            .field("SAMP_RATIO2", &self.SAMP_RATIO2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_D5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPADC_D5 {{ FIFO_WATERMARK: {=u8:?}, DMA_HS_EN: {=bool:?}, SAMP_RATIO: {=u8:?}, SAMP_RATIO2: {=u16:?} }}",
            self.FIFO_WATERMARK(),
            self.DMA_HS_EN(),
            self.SAMP_RATIO(),
            self.SAMP_RATIO2()
        )
    }
}
///LPADC ch3 (SEN_AIN5 / Arduino A3) sample-rate and watermark.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_D6(pub u32);
impl LPADC_D6 {
    ///FIFO watermark.
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_WATERMARK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    ///FIFO watermark.
    #[inline(always)]
    pub const fn set_FIFO_WATERMARK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    ///DMA handshake enable.
    #[must_use]
    #[inline(always)]
    pub const fn DMA_HS_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///DMA handshake enable.
    #[inline(always)]
    pub const fn set_DMA_HS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Frequency divider: 0=÷1 … 15=÷32768.
    #[must_use]
    #[inline(always)]
    pub const fn SAMP_RATIO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    ///Frequency divider: 0=÷1 … 15=÷32768.
    #[inline(always)]
    pub const fn set_SAMP_RATIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    ///PWM sync enables \[8:0\].
    #[must_use]
    #[inline(always)]
    pub const fn SAMP_RATIO2(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x01ff;
        val as u16
    }
    ///PWM sync enables \[8:0\].
    #[inline(always)]
    pub const fn set_SAMP_RATIO2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 12usize)) | (((val as u32) & 0x01ff) << 12usize);
    }
}
impl Default for LPADC_D6 {
    #[inline(always)]
    fn default() -> LPADC_D6 {
        LPADC_D6(0)
    }
}
impl core::fmt::Debug for LPADC_D6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_D6")
            .field("FIFO_WATERMARK", &self.FIFO_WATERMARK())
            .field("DMA_HS_EN", &self.DMA_HS_EN())
            .field("SAMP_RATIO", &self.SAMP_RATIO())
            .field("SAMP_RATIO2", &self.SAMP_RATIO2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_D6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPADC_D6 {{ FIFO_WATERMARK: {=u8:?}, DMA_HS_EN: {=bool:?}, SAMP_RATIO: {=u8:?}, SAMP_RATIO2: {=u16:?} }}",
            self.FIFO_WATERMARK(),
            self.DMA_HS_EN(),
            self.SAMP_RATIO(),
            self.SAMP_RATIO2()
        )
    }
}
///LPADC ch%s FIFO Read Port — dequeue one 10-bit sample (MSB-aligned in 16-bit word; DATA = bits\[15:6\] as 0..=1023).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPADC_FIFO(pub u32);
impl LPADC_FIFO {
    ///10-bit ADC result, right-justified (raw bits\[15:6\] of MSB-aligned word; multiply by VDDA_LPADC/1023 for volts).
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    ///10-bit ADC result, right-justified (raw bits\[15:6\] of MSB-aligned word; multiply by VDDA_LPADC/1023 for volts).
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u32) & 0x03ff) << 6usize);
    }
}
impl Default for LPADC_FIFO {
    #[inline(always)]
    fn default() -> LPADC_FIFO {
        LPADC_FIFO(0)
    }
}
impl core::fmt::Debug for LPADC_FIFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPADC_FIFO")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPADC_FIFO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPADC_FIFO {{ DATA: {=u16:?} }}", self.DATA())
    }
}
///ADCIF clock and power control (reset=0x1 = clocked; write 0 to gate).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCU_ADCIF_CKPOWER(pub u32);
impl SCU_ADCIF_CKPOWER {
    ///Clock/power control bits.
    #[must_use]
    #[inline(always)]
    pub const fn BITS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Clock/power control bits.
    #[inline(always)]
    pub const fn set_BITS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SCU_ADCIF_CKPOWER {
    #[inline(always)]
    fn default() -> SCU_ADCIF_CKPOWER {
        SCU_ADCIF_CKPOWER(0)
    }
}
impl core::fmt::Debug for SCU_ADCIF_CKPOWER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCU_ADCIF_CKPOWER")
            .field("BITS", &self.BITS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCU_ADCIF_CKPOWER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SCU_ADCIF_CKPOWER {{ BITS: {=u32:?} }}", self.BITS())
    }
}
