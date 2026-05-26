///Start rotation processing.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMMAND(pub u32);
impl COMMAND {
    ///Start rotation.
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> super::vals::CMD {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CMD::from_bits(val as u8)
    }
    ///Start rotation.
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: super::vals::CMD) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for COMMAND {
    #[inline(always)]
    fn default() -> COMMAND {
        COMMAND(0)
    }
}
impl core::fmt::Debug for COMMAND {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMAND").field("CMD", &self.CMD()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMMAND {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "COMMAND {{ CMD: {:?} }}", self.CMD())
    }
}
///Color Convertion Control.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONV_CTRL(pub u32);
impl CONV_CTRL {
    ///Convert RGB565 to YCbCr422.
    #[must_use]
    #[inline(always)]
    pub const fn CONV_FORMAT(&self) -> super::vals::CONV_FORMAT {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CONV_FORMAT::from_bits(val as u8)
    }
    ///Convert RGB565 to YCbCr422.
    #[inline(always)]
    pub const fn set_CONV_FORMAT(&mut self, val: super::vals::CONV_FORMAT) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    ///Scale factor for Cb and Cr.
    #[must_use]
    #[inline(always)]
    pub const fn CONV_CALC_SEL(&self) -> super::vals::CONV_CALC_SEL {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CONV_CALC_SEL::from_bits(val as u8)
    }
    ///Scale factor for Cb and Cr.
    #[inline(always)]
    pub const fn set_CONV_CALC_SEL(&mut self, val: super::vals::CONV_CALC_SEL) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for CONV_CTRL {
    #[inline(always)]
    fn default() -> CONV_CTRL {
        CONV_CTRL(0)
    }
}
impl core::fmt::Debug for CONV_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONV_CTRL")
            .field("CONV_FORMAT", &self.CONV_FORMAT())
            .field("CONV_CALC_SEL", &self.CONV_CALC_SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONV_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONV_CTRL {{ CONV_FORMAT: {:?}, CONV_CALC_SEL: {:?} }}",
            self.CONV_FORMAT(),
            self.CONV_CALC_SEL()
        )
    }
}
///Interrupt Clear.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTR_CLEAR(pub u32);
impl INTR_CLEAR {
    ///Done Interrupt Clear.
    #[must_use]
    #[inline(always)]
    pub const fn END_CLR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Done Interrupt Clear.
    #[inline(always)]
    pub const fn set_END_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Read Error Clear.
    #[must_use]
    #[inline(always)]
    pub const fn RD_ERR_CLR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Read Error Clear.
    #[inline(always)]
    pub const fn set_RD_ERR_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Write Error Clear.
    #[must_use]
    #[inline(always)]
    pub const fn WR_ERR_CLR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Write Error Clear.
    #[inline(always)]
    pub const fn set_WR_ERR_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for INTR_CLEAR {
    #[inline(always)]
    fn default() -> INTR_CLEAR {
        INTR_CLEAR(0)
    }
}
impl core::fmt::Debug for INTR_CLEAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_CLEAR")
            .field("END_CLR", &self.END_CLR())
            .field("RD_ERR_CLR", &self.RD_ERR_CLR())
            .field("WR_ERR_CLR", &self.WR_ERR_CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTR_CLEAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTR_CLEAR {{ END_CLR: {=bool:?}, RD_ERR_CLR: {=bool:?}, WR_ERR_CLR: {=bool:?} }}",
            self.END_CLR(),
            self.RD_ERR_CLR(),
            self.WR_ERR_CLR()
        )
    }
}
///Interrupt Disable.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTR_DISABLE(pub u32);
impl INTR_DISABLE {
    ///Done Interrupt Disable.
    #[must_use]
    #[inline(always)]
    pub const fn END_DIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Done Interrupt Disable.
    #[inline(always)]
    pub const fn set_END_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Read Error Disable.
    #[must_use]
    #[inline(always)]
    pub const fn RD_ERR_DIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Read Error Disable.
    #[inline(always)]
    pub const fn set_RD_ERR_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Write Error Disable.
    #[must_use]
    #[inline(always)]
    pub const fn WR_ERR_DIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Write Error Disable.
    #[inline(always)]
    pub const fn set_WR_ERR_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for INTR_DISABLE {
    #[inline(always)]
    fn default() -> INTR_DISABLE {
        INTR_DISABLE(0)
    }
}
impl core::fmt::Debug for INTR_DISABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_DISABLE")
            .field("END_DIS", &self.END_DIS())
            .field("RD_ERR_DIS", &self.RD_ERR_DIS())
            .field("WR_ERR_DIS", &self.WR_ERR_DIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTR_DISABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTR_DISABLE {{ END_DIS: {=bool:?}, RD_ERR_DIS: {=bool:?}, WR_ERR_DIS: {=bool:?} }}",
            self.END_DIS(),
            self.RD_ERR_DIS(),
            self.WR_ERR_DIS()
        )
    }
}
///Interrupt Enable.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTR_ENABLE(pub u32);
impl INTR_ENABLE {
    ///Write Error Enable.
    #[must_use]
    #[inline(always)]
    pub const fn WR_ERR_ENB(&self) -> super::vals::WR_ERR_ENB {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::WR_ERR_ENB::from_bits(val as u8)
    }
    ///Write Error Enable.
    #[inline(always)]
    pub const fn set_WR_ERR_ENB(&mut self, val: super::vals::WR_ERR_ENB) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for INTR_ENABLE {
    #[inline(always)]
    fn default() -> INTR_ENABLE {
        INTR_ENABLE(0)
    }
}
impl core::fmt::Debug for INTR_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_ENABLE")
            .field("WR_ERR_ENB", &self.WR_ERR_ENB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTR_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTR_ENABLE {{ WR_ERR_ENB: {:?} }}", self.WR_ERR_ENB())
    }
}
///Interrupt Status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTR_STATUS(pub u32);
impl INTR_STATUS {
    ///Done.
    #[must_use]
    #[inline(always)]
    pub const fn END_STS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Done.
    #[inline(always)]
    pub const fn set_END_STS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Read Error.
    #[must_use]
    #[inline(always)]
    pub const fn RD_ERR_STS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Read Error.
    #[inline(always)]
    pub const fn set_RD_ERR_STS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Write Error.
    #[must_use]
    #[inline(always)]
    pub const fn WR_ERR_STS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Write Error.
    #[inline(always)]
    pub const fn set_WR_ERR_STS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for INTR_STATUS {
    #[inline(always)]
    fn default() -> INTR_STATUS {
        INTR_STATUS(0)
    }
}
impl core::fmt::Debug for INTR_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_STATUS")
            .field("END_STS", &self.END_STS())
            .field("RD_ERR_STS", &self.RD_ERR_STS())
            .field("WR_ERR_STS", &self.WR_ERR_STS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTR_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTR_STATUS {{ END_STS: {=bool:?}, RD_ERR_STS: {=bool:?}, WR_ERR_STS: {=bool:?} }}",
            self.END_STS(),
            self.RD_ERR_STS(),
            self.WR_ERR_STS()
        )
    }
}
///RGB format selector.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGB_ALIGNMENT(pub u32);
impl RGB_ALIGNMENT {
    ///RGB Format.
    #[must_use]
    #[inline(always)]
    pub const fn FORMAT(&self) -> super::vals::FORMAT {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FORMAT::from_bits(val as u8)
    }
    ///RGB Format.
    #[inline(always)]
    pub const fn set_FORMAT(&mut self, val: super::vals::FORMAT) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for RGB_ALIGNMENT {
    #[inline(always)]
    fn default() -> RGB_ALIGNMENT {
        RGB_ALIGNMENT(0)
    }
}
impl core::fmt::Debug for RGB_ALIGNMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGB_ALIGNMENT")
            .field("FORMAT", &self.FORMAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGB_ALIGNMENT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RGB_ALIGNMENT {{ FORMAT: {:?} }}", self.FORMAT())
    }
}
///Set Rotation Angle.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SET_DIRECTION(pub u32);
impl SET_DIRECTION {
    ///Rotation Angle.
    #[must_use]
    #[inline(always)]
    pub const fn ROT(&self) -> super::vals::ROT {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::ROT::from_bits(val as u8)
    }
    ///Rotation Angle.
    #[inline(always)]
    pub const fn set_ROT(&mut self, val: super::vals::ROT) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for SET_DIRECTION {
    #[inline(always)]
    fn default() -> SET_DIRECTION {
        SET_DIRECTION(0)
    }
}
impl core::fmt::Debug for SET_DIRECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET_DIRECTION")
            .field("ROT", &self.ROT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SET_DIRECTION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SET_DIRECTION {{ ROT: {:?} }}", self.ROT())
    }
}
///Destination Image Pitch (Actual size - 1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SET_DST_PITCH(pub u32);
impl SET_DST_PITCH {
    #[must_use]
    #[inline(always)]
    pub const fn PITCH(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_PITCH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for SET_DST_PITCH {
    #[inline(always)]
    fn default() -> SET_DST_PITCH {
        SET_DST_PITCH(0)
    }
}
impl core::fmt::Debug for SET_DST_PITCH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET_DST_PITCH")
            .field("PITCH", &self.PITCH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SET_DST_PITCH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SET_DST_PITCH {{ PITCH: {=u16:?} }}", self.PITCH())
    }
}
///Source Image Horizontal Size (Actual size - 1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SET_SRC_HSIZE(pub u32);
impl SET_SRC_HSIZE {
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for SET_SRC_HSIZE {
    #[inline(always)]
    fn default() -> SET_SRC_HSIZE {
        SET_SRC_HSIZE(0)
    }
}
impl core::fmt::Debug for SET_SRC_HSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET_SRC_HSIZE")
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SET_SRC_HSIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SET_SRC_HSIZE {{ SIZE: {=u16:?} }}", self.SIZE())
    }
}
///Source Image Pitch (Actual size - 1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SET_SRC_PITCH(pub u32);
impl SET_SRC_PITCH {
    #[must_use]
    #[inline(always)]
    pub const fn PITCH(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_PITCH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for SET_SRC_PITCH {
    #[inline(always)]
    fn default() -> SET_SRC_PITCH {
        SET_SRC_PITCH(0)
    }
}
impl core::fmt::Debug for SET_SRC_PITCH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET_SRC_PITCH")
            .field("PITCH", &self.PITCH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SET_SRC_PITCH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SET_SRC_PITCH {{ PITCH: {=u16:?} }}", self.PITCH())
    }
}
///Source Image Vertical Size (Actual size - 1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SET_SRC_VSIZE(pub u32);
impl SET_SRC_VSIZE {
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for SET_SRC_VSIZE {
    #[inline(always)]
    fn default() -> SET_SRC_VSIZE {
        SET_SRC_VSIZE(0)
    }
}
impl core::fmt::Debug for SET_SRC_VSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET_SRC_VSIZE")
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SET_SRC_VSIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SET_SRC_VSIZE {{ SIZE: {=u16:?} }}", self.SIZE())
    }
}
///Running Status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    ///Running Status (1 = running).
    #[must_use]
    #[inline(always)]
    pub const fn STATUS(&self) -> super::vals::STATUS {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::STATUS::from_bits(val as u8)
    }
    ///Running Status (1 = running).
    #[inline(always)]
    pub const fn set_STATUS(&mut self, val: super::vals::STATUS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("STATUS", &self.STATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STATUS {{ STATUS: {:?} }}", self.STATUS())
    }
}
