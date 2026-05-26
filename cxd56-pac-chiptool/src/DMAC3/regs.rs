#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACC0Control(pub u32);
impl DMACC0Control {
    ///Transfer size.
    #[must_use]
    #[inline(always)]
    pub const fn TransferSize(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    ///Transfer size.
    #[inline(always)]
    pub const fn set_TransferSize(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    ///Destination burst size.
    #[must_use]
    #[inline(always)]
    pub const fn DBSize(&self) -> super::vals::DBSize {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::DBSize::from_bits(val as u8)
    }
    ///Destination burst size.
    #[inline(always)]
    pub const fn set_DBSize(&mut self, val: super::vals::DBSize) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    ///Destination transfer width.
    #[must_use]
    #[inline(always)]
    pub const fn DWidth(&self) -> super::vals::DWidth {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::DWidth::from_bits(val as u8)
    }
    ///Destination transfer width.
    #[inline(always)]
    pub const fn set_DWidth(&mut self, val: super::vals::DWidth) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    ///Source AHB master select.
    #[must_use]
    #[inline(always)]
    pub const fn S(&self) -> super::vals::S {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::S::from_bits(val as u8)
    }
    ///Source AHB master select.
    #[inline(always)]
    pub const fn set_S(&mut self, val: super::vals::S) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    ///Destination AHB master select.
    #[must_use]
    #[inline(always)]
    pub const fn D(&self) -> super::vals::D {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::D::from_bits(val as u8)
    }
    ///Destination AHB master select.
    #[inline(always)]
    pub const fn set_D(&mut self, val: super::vals::D) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    ///Source increment.
    #[must_use]
    #[inline(always)]
    pub const fn SI(&self) -> super::vals::SI {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SI::from_bits(val as u8)
    }
    ///Source increment.
    #[inline(always)]
    pub const fn set_SI(&mut self, val: super::vals::SI) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    ///Destination increment.
    #[must_use]
    #[inline(always)]
    pub const fn DI(&self) -> super::vals::DI {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::DI::from_bits(val as u8)
    }
    ///Destination increment.
    #[inline(always)]
    pub const fn set_DI(&mut self, val: super::vals::DI) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    ///Terminal count interrupt enable.
    #[must_use]
    #[inline(always)]
    pub const fn I(&self) -> super::vals::I {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::I::from_bits(val as u8)
    }
    ///Terminal count interrupt enable.
    #[inline(always)]
    pub const fn set_I(&mut self, val: super::vals::I) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for DMACC0Control {
    #[inline(always)]
    fn default() -> DMACC0Control {
        DMACC0Control(0)
    }
}
impl core::fmt::Debug for DMACC0Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACC0Control")
            .field("TransferSize", &self.TransferSize())
            .field("DBSize", &self.DBSize())
            .field("DWidth", &self.DWidth())
            .field("S", &self.S())
            .field("D", &self.D())
            .field("SI", &self.SI())
            .field("DI", &self.DI())
            .field("I", &self.I())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACC0Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACC0Control {{ TransferSize: {=u32:?}, DBSize: {:?}, DWidth: {:?}, S: {:?}, D: {:?}, SI: {:?}, DI: {:?}, I: {:?} }}",
            self.TransferSize(),
            self.DBSize(),
            self.DWidth(),
            self.S(),
            self.D(),
            self.SI(),
            self.DI(),
            self.I()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACC0DefLLI(pub u32);
impl DMACC0DefLLI {
    ///Bus master select.
    #[must_use]
    #[inline(always)]
    pub const fn DEFLM(&self) -> super::vals::DEFLM {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DEFLM::from_bits(val as u8)
    }
    ///Bus master select.
    #[inline(always)]
    pub const fn set_DEFLM(&mut self, val: super::vals::DEFLM) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Enable Default LLI.
    #[must_use]
    #[inline(always)]
    pub const fn DEFLE(&self) -> super::vals::DEFLE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DEFLE::from_bits(val as u8)
    }
    ///Enable Default LLI.
    #[inline(always)]
    pub const fn set_DEFLE(&mut self, val: super::vals::DEFLE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DEFLLI(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_DEFLLI(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DMACC0DefLLI {
    #[inline(always)]
    fn default() -> DMACC0DefLLI {
        DMACC0DefLLI(0)
    }
}
impl core::fmt::Debug for DMACC0DefLLI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACC0DefLLI")
            .field("DEFLM", &self.DEFLM())
            .field("DEFLE", &self.DEFLE())
            .field("DEFLLI", &self.DEFLLI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACC0DefLLI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACC0DefLLI {{ DEFLM: {:?}, DEFLE: {:?}, DEFLLI: {=u32:?} }}",
            self.DEFLM(),
            self.DEFLE(),
            self.DEFLLI()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACC0DestAddr(pub u32);
impl DMACC0DestAddr {
    ///DMA destination address.
    #[must_use]
    #[inline(always)]
    pub const fn DestAddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///DMA destination address.
    #[inline(always)]
    pub const fn set_DestAddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMACC0DestAddr {
    #[inline(always)]
    fn default() -> DMACC0DestAddr {
        DMACC0DestAddr(0)
    }
}
impl core::fmt::Debug for DMACC0DestAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACC0DestAddr")
            .field("DestAddr", &self.DestAddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACC0DestAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACC0DestAddr {{ DestAddr: {=u32:?} }}",
            self.DestAddr()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACC0LLI(pub u32);
impl DMACC0LLI {
    ///AHB master select for loading the next LLI.
    #[must_use]
    #[inline(always)]
    pub const fn LM(&self) -> super::vals::LM {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LM::from_bits(val as u8)
    }
    ///AHB master select for loading the next LLI.
    #[inline(always)]
    pub const fn set_LM(&mut self, val: super::vals::LM) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Linked list item.
    #[must_use]
    #[inline(always)]
    pub const fn LLI(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    ///Linked list item.
    #[inline(always)]
    pub const fn set_LLI(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DMACC0LLI {
    #[inline(always)]
    fn default() -> DMACC0LLI {
        DMACC0LLI(0)
    }
}
impl core::fmt::Debug for DMACC0LLI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACC0LLI")
            .field("LM", &self.LM())
            .field("LLI", &self.LLI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACC0LLI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACC0LLI {{ LM: {:?}, LLI: {=u32:?} }}",
            self.LM(),
            self.LLI()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACC0SrcAddr(pub u32);
impl DMACC0SrcAddr {
    ///DMA source address.
    #[must_use]
    #[inline(always)]
    pub const fn SrcAddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///DMA source address.
    #[inline(always)]
    pub const fn set_SrcAddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMACC0SrcAddr {
    #[inline(always)]
    fn default() -> DMACC0SrcAddr {
        DMACC0SrcAddr(0)
    }
}
impl core::fmt::Debug for DMACC0SrcAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACC0SrcAddr")
            .field("SrcAddr", &self.SrcAddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACC0SrcAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMACC0SrcAddr {{ SrcAddr: {=u32:?} }}", self.SrcAddr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACConfiguration(pub u32);
impl DMACConfiguration {
    ///DMAC enable.
    #[must_use]
    #[inline(always)]
    pub const fn E(&self) -> super::vals::E {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::E::from_bits(val as u8)
    }
    ///DMAC enable.
    #[inline(always)]
    pub const fn set_E(&mut self, val: super::vals::E) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///AHB Master 1 endianess configuration.
    #[must_use]
    #[inline(always)]
    pub const fn M1(&self) -> super::vals::M1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::M1::from_bits(val as u8)
    }
    ///AHB Master 1 endianess configuration.
    #[inline(always)]
    pub const fn set_M1(&mut self, val: super::vals::M1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///AHB Master 2 endianess configuration.
    #[must_use]
    #[inline(always)]
    pub const fn M2(&self) -> super::vals::M2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::M2::from_bits(val as u8)
    }
    ///AHB Master 2 endianess configuration.
    #[inline(always)]
    pub const fn set_M2(&mut self, val: super::vals::M2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Transfer Size Extended.
    #[must_use]
    #[inline(always)]
    pub const fn TS(&self) -> super::vals::TS {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::TS::from_bits(val as u8)
    }
    ///Transfer Size Extended.
    #[inline(always)]
    pub const fn set_TS(&mut self, val: super::vals::TS) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    ///Default LLI enabled.
    #[must_use]
    #[inline(always)]
    pub const fn DLLI(&self) -> super::vals::DLLI {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::DLLI::from_bits(val as u8)
    }
    ///Default LLI enabled.
    #[inline(always)]
    pub const fn set_DLLI(&mut self, val: super::vals::DLLI) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    ///Trigger function enabled.
    #[must_use]
    #[inline(always)]
    pub const fn TR(&self) -> super::vals::TR {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::TR::from_bits(val as u8)
    }
    ///Trigger function enabled.
    #[inline(always)]
    pub const fn set_TR(&mut self, val: super::vals::TR) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    ///DMAC Arbitration logic.
    #[must_use]
    #[inline(always)]
    pub const fn ARB(&self) -> super::vals::ARB {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ARB::from_bits(val as u8)
    }
    ///DMAC Arbitration logic.
    #[inline(always)]
    pub const fn set_ARB(&mut self, val: super::vals::ARB) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    ///Transfer Size Extended.
    #[must_use]
    #[inline(always)]
    pub const fn FSIZE(&self) -> super::vals::FSIZE {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::FSIZE::from_bits(val as u8)
    }
    ///Transfer Size Extended.
    #[inline(always)]
    pub const fn set_FSIZE(&mut self, val: super::vals::FSIZE) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for DMACConfiguration {
    #[inline(always)]
    fn default() -> DMACConfiguration {
        DMACConfiguration(0)
    }
}
impl core::fmt::Debug for DMACConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACConfiguration")
            .field("E", &self.E())
            .field("M1", &self.M1())
            .field("M2", &self.M2())
            .field("TS", &self.TS())
            .field("DLLI", &self.DLLI())
            .field("TR", &self.TR())
            .field("ARB", &self.ARB())
            .field("FSIZE", &self.FSIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACConfiguration {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACConfiguration {{ E: {:?}, M1: {:?}, M2: {:?}, TS: {:?}, DLLI: {:?}, TR: {:?}, ARB: {:?}, FSIZE: {:?} }}",
            self.E(),
            self.M1(),
            self.M2(),
            self.TS(),
            self.DLLI(),
            self.TR(),
            self.ARB(),
            self.FSIZE()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACEnbldChns(pub u32);
impl DMACEnbldChns {
    ///Channel enable status.
    #[must_use]
    #[inline(always)]
    pub const fn EnabledChannels(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Channel enable status.
    #[inline(always)]
    pub const fn set_EnabledChannels(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DMACEnbldChns {
    #[inline(always)]
    fn default() -> DMACEnbldChns {
        DMACEnbldChns(0)
    }
}
impl core::fmt::Debug for DMACEnbldChns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACEnbldChns")
            .field("EnabledChannels", &self.EnabledChannels())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACEnbldChns {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACEnbldChns {{ EnabledChannels: {=u8:?} }}",
            self.EnabledChannels()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACIntErrClr(pub u32);
impl DMACIntErrClr {
    ///Interrupt error clear.
    #[must_use]
    #[inline(always)]
    pub const fn IntErrClr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Interrupt error clear.
    #[inline(always)]
    pub const fn set_IntErrClr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DMACIntErrClr {
    #[inline(always)]
    fn default() -> DMACIntErrClr {
        DMACIntErrClr(0)
    }
}
impl core::fmt::Debug for DMACIntErrClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACIntErrClr")
            .field("IntErrClr", &self.IntErrClr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACIntErrClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACIntErrClr {{ IntErrClr: {=u8:?} }}",
            self.IntErrClr()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACIntErrorStatus(pub u32);
impl DMACIntErrorStatus {
    ///Interrupt error status.
    #[must_use]
    #[inline(always)]
    pub const fn IntErrorStatus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Interrupt error status.
    #[inline(always)]
    pub const fn set_IntErrorStatus(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DMACIntErrorStatus {
    #[inline(always)]
    fn default() -> DMACIntErrorStatus {
        DMACIntErrorStatus(0)
    }
}
impl core::fmt::Debug for DMACIntErrorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACIntErrorStatus")
            .field("IntErrorStatus", &self.IntErrorStatus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACIntErrorStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACIntErrorStatus {{ IntErrorStatus: {=u8:?} }}",
            self.IntErrorStatus()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACIntStatus(pub u32);
impl DMACIntStatus {
    ///Status of the DMA interrupts after masking.
    #[must_use]
    #[inline(always)]
    pub const fn IntStatus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Status of the DMA interrupts after masking.
    #[inline(always)]
    pub const fn set_IntStatus(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DMACIntStatus {
    #[inline(always)]
    fn default() -> DMACIntStatus {
        DMACIntStatus(0)
    }
}
impl core::fmt::Debug for DMACIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACIntStatus")
            .field("IntStatus", &self.IntStatus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACIntStatus {{ IntStatus: {=u8:?} }}",
            self.IntStatus()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACIntTCClear(pub u32);
impl DMACIntTCClear {
    ///Terminal count request clear.
    #[must_use]
    #[inline(always)]
    pub const fn IntTCClear(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Terminal count request clear.
    #[inline(always)]
    pub const fn set_IntTCClear(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DMACIntTCClear {
    #[inline(always)]
    fn default() -> DMACIntTCClear {
        DMACIntTCClear(0)
    }
}
impl core::fmt::Debug for DMACIntTCClear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACIntTCClear")
            .field("IntTCClear", &self.IntTCClear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACIntTCClear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACIntTCClear {{ IntTCClear: {=u8:?} }}",
            self.IntTCClear()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACIntTCStatus(pub u32);
impl DMACIntTCStatus {
    ///Interrupt terminal count request status.
    #[must_use]
    #[inline(always)]
    pub const fn IntTCStatus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Interrupt terminal count request status.
    #[inline(always)]
    pub const fn set_IntTCStatus(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DMACIntTCStatus {
    #[inline(always)]
    fn default() -> DMACIntTCStatus {
        DMACIntTCStatus(0)
    }
}
impl core::fmt::Debug for DMACIntTCStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACIntTCStatus")
            .field("IntTCStatus", &self.IntTCStatus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACIntTCStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACIntTCStatus {{ IntTCStatus: {=u8:?} }}",
            self.IntTCStatus()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACRawIntErrorStatus(pub u32);
impl DMACRawIntErrorStatus {
    ///Status of the error interrupt prior to masking.
    #[must_use]
    #[inline(always)]
    pub const fn RawIntErrorStatus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Status of the error interrupt prior to masking.
    #[inline(always)]
    pub const fn set_RawIntErrorStatus(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DMACRawIntErrorStatus {
    #[inline(always)]
    fn default() -> DMACRawIntErrorStatus {
        DMACRawIntErrorStatus(0)
    }
}
impl core::fmt::Debug for DMACRawIntErrorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRawIntErrorStatus")
            .field("RawIntErrorStatus", &self.RawIntErrorStatus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACRawIntErrorStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACRawIntErrorStatus {{ RawIntErrorStatus: {=u8:?} }}",
            self.RawIntErrorStatus()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACRawIntTCStatus(pub u32);
impl DMACRawIntTCStatus {
    ///Status of the terminal count interrupt prior to masking.
    #[must_use]
    #[inline(always)]
    pub const fn RawIntTCStatus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Status of the terminal count interrupt prior to masking.
    #[inline(always)]
    pub const fn set_RawIntTCStatus(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DMACRawIntTCStatus {
    #[inline(always)]
    fn default() -> DMACRawIntTCStatus {
        DMACRawIntTCStatus(0)
    }
}
impl core::fmt::Debug for DMACRawIntTCStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRawIntTCStatus")
            .field("RawIntTCStatus", &self.RawIntTCStatus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACRawIntTCStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACRawIntTCStatus {{ RawIntTCStatus: {=u8:?} }}",
            self.RawIntTCStatus()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACSREQMask(pub u32);
impl DMACSREQMask {
    ///Mask SREQ signals between DMAC and peripherals.
    #[must_use]
    #[inline(always)]
    pub const fn DMACSREQMask(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///Mask SREQ signals between DMAC and peripherals.
    #[inline(always)]
    pub const fn set_DMACSREQMask(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DMACSREQMask {
    #[inline(always)]
    fn default() -> DMACSREQMask {
        DMACSREQMask(0)
    }
}
impl core::fmt::Debug for DMACSREQMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACSREQMask")
            .field("DMACSREQMask", &self.DMACSREQMask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACSREQMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACSREQMask {{ DMACSREQMask: {=u16:?} }}",
            self.DMACSREQMask()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACSoftBReq(pub u32);
impl DMACSoftBReq {
    ///Software burst request.
    #[must_use]
    #[inline(always)]
    pub const fn SoftBReq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///Software burst request.
    #[inline(always)]
    pub const fn set_SoftBReq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DMACSoftBReq {
    #[inline(always)]
    fn default() -> DMACSoftBReq {
        DMACSoftBReq(0)
    }
}
impl core::fmt::Debug for DMACSoftBReq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACSoftBReq")
            .field("SoftBReq", &self.SoftBReq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACSoftBReq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMACSoftBReq {{ SoftBReq: {=u16:?} }}", self.SoftBReq())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACSoftLBReq(pub u32);
impl DMACSoftLBReq {
    ///Software last burst request.
    #[must_use]
    #[inline(always)]
    pub const fn SoftLBReq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///Software last burst request.
    #[inline(always)]
    pub const fn set_SoftLBReq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DMACSoftLBReq {
    #[inline(always)]
    fn default() -> DMACSoftLBReq {
        DMACSoftLBReq(0)
    }
}
impl core::fmt::Debug for DMACSoftLBReq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACSoftLBReq")
            .field("SoftLBReq", &self.SoftLBReq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACSoftLBReq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACSoftLBReq {{ SoftLBReq: {=u16:?} }}",
            self.SoftLBReq()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACSoftLSReq(pub u32);
impl DMACSoftLSReq {
    ///Software last single request.
    #[must_use]
    #[inline(always)]
    pub const fn SoftLSReq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///Software last single request.
    #[inline(always)]
    pub const fn set_SoftLSReq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DMACSoftLSReq {
    #[inline(always)]
    fn default() -> DMACSoftLSReq {
        DMACSoftLSReq(0)
    }
}
impl core::fmt::Debug for DMACSoftLSReq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACSoftLSReq")
            .field("SoftLSReq", &self.SoftLSReq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACSoftLSReq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACSoftLSReq {{ SoftLSReq: {=u16:?} }}",
            self.SoftLSReq()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACSoftSReq(pub u32);
impl DMACSoftSReq {
    ///Software single request.
    #[must_use]
    #[inline(always)]
    pub const fn SoftSReq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///Software single request.
    #[inline(always)]
    pub const fn set_SoftSReq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DMACSoftSReq {
    #[inline(always)]
    fn default() -> DMACSoftSReq {
        DMACSoftSReq(0)
    }
}
impl core::fmt::Debug for DMACSoftSReq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACSoftSReq")
            .field("SoftSReq", &self.SoftSReq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACSoftSReq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMACSoftSReq {{ SoftSReq: {=u16:?} }}", self.SoftSReq())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACSync(pub u32);
impl DMACSync {
    ///DMA synchronization logic for DMA request signals enabled or disabled.
    #[must_use]
    #[inline(always)]
    pub const fn DMACSync(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///DMA synchronization logic for DMA request signals enabled or disabled.
    #[inline(always)]
    pub const fn set_DMACSync(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DMACSync {
    #[inline(always)]
    fn default() -> DMACSync {
        DMACSync(0)
    }
}
impl core::fmt::Debug for DMACSync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACSync")
            .field("DMACSync", &self.DMACSync())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACSync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMACSync {{ DMACSync: {=u16:?} }}", self.DMACSync())
    }
}
