///Descriptor Address Set Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADDRESS_DESCRIPTOR_START(pub u32);
impl ADDRESS_DESCRIPTOR_START {
    ///must be 1.
    #[must_use]
    #[inline(always)]
    pub const fn MSEL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///must be 1.
    #[inline(always)]
    pub const fn set_MSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Descriptor Address.
    #[must_use]
    #[inline(always)]
    pub const fn DA(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    ///Descriptor Address.
    #[inline(always)]
    pub const fn set_DA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for ADDRESS_DESCRIPTOR_START {
    #[inline(always)]
    fn default() -> ADDRESS_DESCRIPTOR_START {
        ADDRESS_DESCRIPTOR_START(0)
    }
}
impl core::fmt::Debug for ADDRESS_DESCRIPTOR_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRESS_DESCRIPTOR_START")
            .field("MSEL", &self.MSEL())
            .field("DA", &self.DA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADDRESS_DESCRIPTOR_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADDRESS_DESCRIPTOR_START {{ MSEL: {=bool:?}, DA: {=u32:?} }}",
            self.MSEL(),
            self.DA()
        )
    }
}
///Command Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMD_DESCRIPTOR(pub u32);
impl CMD_DESCRIPTOR {
    ///Command.
    #[must_use]
    #[inline(always)]
    pub const fn COMMAND(&self) -> super::vals::COMMAND {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::COMMAND::from_bits(val as u8)
    }
    ///Command.
    #[inline(always)]
    pub const fn set_COMMAND(&mut self, val: super::vals::COMMAND) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for CMD_DESCRIPTOR {
    #[inline(always)]
    fn default() -> CMD_DESCRIPTOR {
        CMD_DESCRIPTOR(0)
    }
}
impl core::fmt::Debug for CMD_DESCRIPTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_DESCRIPTOR")
            .field("COMMAND", &self.COMMAND())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMD_DESCRIPTOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CMD_DESCRIPTOR {{ COMMAND: {:?} }}", self.COMMAND())
    }
}
///2D Graphics Engine Interrupt Control.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTR_ENABLE(pub u32);
impl INTR_ENABLE {
    ///Normal Descriptor Command Update Interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn HPU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Normal Descriptor Command Update Interrupt.
    #[inline(always)]
    pub const fn set_HPU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Normal Descriptor Command Finished Interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn NDF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Normal Descriptor Command Finished Interrupt.
    #[inline(always)]
    pub const fn set_NDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Normal Descriptor Command Branch Interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn NDB(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Normal Descriptor Command Branch Interrupt.
    #[inline(always)]
    pub const fn set_NDB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///Normal Descriptor Command Error Interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn NDE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///Normal Descriptor Command Error Interrupt.
    #[inline(always)]
    pub const fn set_NDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Descriptor STOP Command Done Interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn DSD(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Descriptor STOP Command Done Interrupt.
    #[inline(always)]
    pub const fn set_DSD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Read Error Interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RD_ERR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Read Error Interrupt.
    #[inline(always)]
    pub const fn set_RD_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Write Error Interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn WR_ERR(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Write Error Interrupt.
    #[inline(always)]
    pub const fn set_WR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
            .field("HPU", &self.HPU())
            .field("NDF", &self.NDF())
            .field("NDB", &self.NDB())
            .field("NDE", &self.NDE())
            .field("DSD", &self.DSD())
            .field("RD_ERR", &self.RD_ERR())
            .field("WR_ERR", &self.WR_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTR_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTR_ENABLE {{ HPU: {=bool:?}, NDF: {=bool:?}, NDB: {=bool:?}, NDE: {=bool:?}, DSD: {=bool:?}, RD_ERR: {=bool:?}, WR_ERR: {=bool:?} }}",
            self.HPU(),
            self.NDF(),
            self.NDB(),
            self.NDE(),
            self.DSD(),
            self.RD_ERR(),
            self.WR_ERR()
        )
    }
}
///Status Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    ///Requesting Normal Descriptor.
    #[must_use]
    #[inline(always)]
    pub const fn NREQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Requesting Normal Descriptor.
    #[inline(always)]
    pub const fn set_NREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Reqesting Descriptor Stop.
    #[must_use]
    #[inline(always)]
    pub const fn SREQ(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Reqesting Descriptor Stop.
    #[inline(always)]
    pub const fn set_SREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///Normal Descriptor Command Running Status.
    #[must_use]
    #[inline(always)]
    pub const fn NDCR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Normal Descriptor Command Running Status.
    #[inline(always)]
    pub const fn set_NDCR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///ISE Running Status.
    #[must_use]
    #[inline(always)]
    pub const fn ISER(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///ISE Running Status.
    #[inline(always)]
    pub const fn set_ISER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
            .field("NREQ", &self.NREQ())
            .field("SREQ", &self.SREQ())
            .field("NDCR", &self.NDCR())
            .field("ISER", &self.ISER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ NREQ: {=bool:?}, SREQ: {=bool:?}, NDCR: {=bool:?}, ISER: {=bool:?} }}",
            self.NREQ(),
            self.SREQ(),
            self.NDCR(),
            self.ISER()
        )
    }
}
