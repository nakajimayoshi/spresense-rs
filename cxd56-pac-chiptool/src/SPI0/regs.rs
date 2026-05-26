#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CSMODE(pub u32);
impl CSMODE {
    #[must_use]
    #[inline(always)]
    pub const fn CS_MODE(&self) -> super::vals::CS_MODE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CS_MODE::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_CS_MODE(&mut self, val: super::vals::CS_MODE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CSMODE {
    #[inline(always)]
    fn default() -> CSMODE {
        CSMODE(0)
    }
}
impl core::fmt::Debug for CSMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSMODE")
            .field("CS_MODE", &self.CS_MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CSMODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CSMODE {{ CS_MODE: {:?} }}", self.CS_MODE())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLAVETYPE(pub u32);
impl SLAVETYPE {
    #[must_use]
    #[inline(always)]
    pub const fn SLAVE_TYPE(&self) -> super::vals::SLAVE_TYPE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SLAVE_TYPE::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_SLAVE_TYPE(&mut self, val: super::vals::SLAVE_TYPE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SLAVETYPE {
    #[inline(always)]
    fn default() -> SLAVETYPE {
        SLAVETYPE(0)
    }
}
impl core::fmt::Debug for SLAVETYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVETYPE")
            .field("SLAVE_TYPE", &self.SLAVE_TYPE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLAVETYPE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SLAVETYPE {{ SLAVE_TYPE: {:?} }}", self.SLAVE_TYPE())
    }
}
///Clock prescale register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPCPSR(pub u32);
impl SSPCPSR {
    ///Clock prescale divisor.
    #[must_use]
    #[inline(always)]
    pub const fn CPSDVSR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Clock prescale divisor.
    #[inline(always)]
    pub const fn set_CPSDVSR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SSPCPSR {
    #[inline(always)]
    fn default() -> SSPCPSR {
        SSPCPSR(0)
    }
}
impl core::fmt::Debug for SSPCPSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPCPSR")
            .field("CPSDVSR", &self.CPSDVSR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPCPSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SSPCPSR {{ CPSDVSR: {=u8:?} }}", self.CPSDVSR())
    }
}
///Control register 0.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPCR0(pub u32);
impl SSPCR0 {
    ///Data Size Select.
    #[must_use]
    #[inline(always)]
    pub const fn DSS(&self) -> super::vals::DSS {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DSS::from_bits(val as u8)
    }
    ///Data Size Select.
    #[inline(always)]
    pub const fn set_DSS(&mut self, val: super::vals::DSS) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    ///Frame format.
    #[must_use]
    #[inline(always)]
    pub const fn FRF(&self) -> super::vals::FRF {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::FRF::from_bits(val as u8)
    }
    ///Frame format.
    #[inline(always)]
    pub const fn set_FRF(&mut self, val: super::vals::FRF) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    ///SSPCLKOUT polarity.
    #[must_use]
    #[inline(always)]
    pub const fn SPO(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///SSPCLKOUT polarity.
    #[inline(always)]
    pub const fn set_SPO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///SSPCLKOUT phase.
    #[must_use]
    #[inline(always)]
    pub const fn SPH(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///SSPCLKOUT phase.
    #[inline(always)]
    pub const fn set_SPH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Serial clock rate.
    #[must_use]
    #[inline(always)]
    pub const fn SCR(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    ///Serial clock rate.
    #[inline(always)]
    pub const fn set_SCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for SSPCR0 {
    #[inline(always)]
    fn default() -> SSPCR0 {
        SSPCR0(0)
    }
}
impl core::fmt::Debug for SSPCR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPCR0")
            .field("DSS", &self.DSS())
            .field("FRF", &self.FRF())
            .field("SPO", &self.SPO())
            .field("SPH", &self.SPH())
            .field("SCR", &self.SCR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPCR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPCR0 {{ DSS: {:?}, FRF: {:?}, SPO: {=bool:?}, SPH: {=bool:?}, SCR: {=u8:?} }}",
            self.DSS(),
            self.FRF(),
            self.SPO(),
            self.SPH(),
            self.SCR()
        )
    }
}
///Control register 1.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPCR1(pub u32);
impl SSPCR1 {
    ///Loop back mode.
    #[must_use]
    #[inline(always)]
    pub const fn LBM(&self) -> super::vals::LBM {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LBM::from_bits(val as u8)
    }
    ///Loop back mode.
    #[inline(always)]
    pub const fn set_LBM(&mut self, val: super::vals::LBM) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Synchronous serial port enable.
    #[must_use]
    #[inline(always)]
    pub const fn SSE(&self) -> super::vals::SSE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SSE::from_bits(val as u8)
    }
    ///Synchronous serial port enable.
    #[inline(always)]
    pub const fn set_SSE(&mut self, val: super::vals::SSE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///Master or slave mode select.
    #[must_use]
    #[inline(always)]
    pub const fn MS(&self) -> super::vals::MS {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MS::from_bits(val as u8)
    }
    ///Master or slave mode select.
    #[inline(always)]
    pub const fn set_MS(&mut self, val: super::vals::MS) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Slave-mode output disable.
    #[must_use]
    #[inline(always)]
    pub const fn SOD(&self) -> super::vals::SOD {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SOD::from_bits(val as u8)
    }
    ///Slave-mode output disable.
    #[inline(always)]
    pub const fn set_SOD(&mut self, val: super::vals::SOD) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for SSPCR1 {
    #[inline(always)]
    fn default() -> SSPCR1 {
        SSPCR1(0)
    }
}
impl core::fmt::Debug for SSPCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPCR1")
            .field("LBM", &self.LBM())
            .field("SSE", &self.SSE())
            .field("MS", &self.MS())
            .field("SOD", &self.SOD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPCR1 {{ LBM: {:?}, SSE: {:?}, MS: {:?}, SOD: {:?} }}",
            self.LBM(),
            self.SSE(),
            self.MS(),
            self.SOD()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPCS(pub u32);
impl SSPCS {
    #[must_use]
    #[inline(always)]
    pub const fn SSP_CS(&self) -> super::vals::SSP_CS {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SSP_CS::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_SSP_CS(&mut self, val: super::vals::SSP_CS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for SSPCS {
    #[inline(always)]
    fn default() -> SSPCS {
        SSPCS(0)
    }
}
impl core::fmt::Debug for SSPCS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPCS")
            .field("SSP_CS", &self.SSP_CS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPCS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SSPCS {{ SSP_CS: {:?} }}", self.SSP_CS())
    }
}
///DMA control register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPDMACR(pub u32);
impl SSPDMACR {
    ///Receive DMA enable.
    #[must_use]
    #[inline(always)]
    pub const fn RXDMAE(&self) -> super::vals::RXDMAE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RXDMAE::from_bits(val as u8)
    }
    ///Receive DMA enable.
    #[inline(always)]
    pub const fn set_RXDMAE(&mut self, val: super::vals::RXDMAE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Transmit DMA enable.
    #[must_use]
    #[inline(always)]
    pub const fn TXDMAE(&self) -> super::vals::TXDMAE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::TXDMAE::from_bits(val as u8)
    }
    ///Transmit DMA enable.
    #[inline(always)]
    pub const fn set_TXDMAE(&mut self, val: super::vals::TXDMAE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for SSPDMACR {
    #[inline(always)]
    fn default() -> SSPDMACR {
        SSPDMACR(0)
    }
}
impl core::fmt::Debug for SSPDMACR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPDMACR")
            .field("RXDMAE", &self.RXDMAE())
            .field("TXDMAE", &self.TXDMAE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPDMACR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPDMACR {{ RXDMAE: {:?}, TXDMAE: {:?} }}",
            self.RXDMAE(),
            self.TXDMAE()
        )
    }
}
///Data register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPDR(pub u32);
impl SSPDR {
    ///Transmit/Receive FIFO.
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///Transmit/Receive FIFO.
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SSPDR {
    #[inline(always)]
    fn default() -> SSPDR {
        SSPDR(0)
    }
}
impl core::fmt::Debug for SSPDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPDR").field("DATA", &self.DATA()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SSPDR {{ DATA: {=u16:?} }}", self.DATA())
    }
}
///Interrupt clear register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPICR(pub u32);
impl SSPICR {
    ///Clear the SSPRORINTR interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RORIC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Clear the SSPRORINTR interrupt.
    #[inline(always)]
    pub const fn set_RORIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Clear the SSPRTINTR interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RTIC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Clear the SSPRTINTR interrupt.
    #[inline(always)]
    pub const fn set_RTIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SSPICR {
    #[inline(always)]
    fn default() -> SSPICR {
        SSPICR(0)
    }
}
impl core::fmt::Debug for SSPICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPICR")
            .field("RORIC", &self.RORIC())
            .field("RTIC", &self.RTIC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPICR {{ RORIC: {=bool:?}, RTIC: {=bool:?} }}",
            self.RORIC(),
            self.RTIC()
        )
    }
}
///Interrupt mask set or clear register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPIMSC(pub u32);
impl SSPIMSC {
    ///Receive overrun interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RORIM(&self) -> super::vals::RORIM {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RORIM::from_bits(val as u8)
    }
    ///Receive overrun interrupt mask.
    #[inline(always)]
    pub const fn set_RORIM(&mut self, val: super::vals::RORIM) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Receive timeout interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RTIM(&self) -> super::vals::RTIM {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RTIM::from_bits(val as u8)
    }
    ///Receive timeout interrupt mask.
    #[inline(always)]
    pub const fn set_RTIM(&mut self, val: super::vals::RTIM) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///Receive FIFO interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RXIM(&self) -> super::vals::RXIM {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RXIM::from_bits(val as u8)
    }
    ///Receive FIFO interrupt mask.
    #[inline(always)]
    pub const fn set_RXIM(&mut self, val: super::vals::RXIM) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Transmit FIFO interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn TXIM(&self) -> super::vals::TXIM {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TXIM::from_bits(val as u8)
    }
    ///Transmit FIFO interrupt mask.
    #[inline(always)]
    pub const fn set_TXIM(&mut self, val: super::vals::TXIM) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for SSPIMSC {
    #[inline(always)]
    fn default() -> SSPIMSC {
        SSPIMSC(0)
    }
}
impl core::fmt::Debug for SSPIMSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPIMSC")
            .field("RORIM", &self.RORIM())
            .field("RTIM", &self.RTIM())
            .field("RXIM", &self.RXIM())
            .field("TXIM", &self.TXIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPIMSC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPIMSC {{ RORIM: {:?}, RTIM: {:?}, RXIM: {:?}, TXIM: {:?} }}",
            self.RORIM(),
            self.RTIM(),
            self.RXIM(),
            self.TXIM()
        )
    }
}
///Masked interrupt status register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPMIS(pub u32);
impl SSPMIS {
    ///Receive overrun interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RORMIS(&self) -> super::vals::RORMIS {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RORMIS::from_bits(val as u8)
    }
    ///Receive overrun interrupt mask.
    #[inline(always)]
    pub const fn set_RORMIS(&mut self, val: super::vals::RORMIS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Receive timeout interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RTMIS(&self) -> super::vals::RTMIS {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RTMIS::from_bits(val as u8)
    }
    ///Receive timeout interrupt mask.
    #[inline(always)]
    pub const fn set_RTMIS(&mut self, val: super::vals::RTMIS) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///Receive FIFO interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RXMIS(&self) -> super::vals::RXMIS {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RXMIS::from_bits(val as u8)
    }
    ///Receive FIFO interrupt mask.
    #[inline(always)]
    pub const fn set_RXMIS(&mut self, val: super::vals::RXMIS) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Transmit FIFO interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn TXMIS(&self) -> super::vals::TXMIS {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TXMIS::from_bits(val as u8)
    }
    ///Transmit FIFO interrupt mask.
    #[inline(always)]
    pub const fn set_TXMIS(&mut self, val: super::vals::TXMIS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for SSPMIS {
    #[inline(always)]
    fn default() -> SSPMIS {
        SSPMIS(0)
    }
}
impl core::fmt::Debug for SSPMIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPMIS")
            .field("RORMIS", &self.RORMIS())
            .field("RTMIS", &self.RTMIS())
            .field("RXMIS", &self.RXMIS())
            .field("TXMIS", &self.TXMIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPMIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPMIS {{ RORMIS: {:?}, RTMIS: {:?}, RXMIS: {:?}, TXMIS: {:?} }}",
            self.RORMIS(),
            self.RTMIS(),
            self.RXMIS(),
            self.TXMIS()
        )
    }
}
///Raw interrupt status register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPRIS(pub u32);
impl SSPRIS {
    ///Receive overrun interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RORRIS(&self) -> super::vals::RORRIS {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RORRIS::from_bits(val as u8)
    }
    ///Receive overrun interrupt mask.
    #[inline(always)]
    pub const fn set_RORRIS(&mut self, val: super::vals::RORRIS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Receive timeout interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RTRIS(&self) -> super::vals::RTRIS {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RTRIS::from_bits(val as u8)
    }
    ///Receive timeout interrupt mask.
    #[inline(always)]
    pub const fn set_RTRIS(&mut self, val: super::vals::RTRIS) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///Receive FIFO interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RXRIS(&self) -> super::vals::RXRIS {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RXRIS::from_bits(val as u8)
    }
    ///Receive FIFO interrupt mask.
    #[inline(always)]
    pub const fn set_RXRIS(&mut self, val: super::vals::RXRIS) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Transmit FIFO interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn TXRIS(&self) -> super::vals::TXRIS {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TXRIS::from_bits(val as u8)
    }
    ///Transmit FIFO interrupt mask.
    #[inline(always)]
    pub const fn set_TXRIS(&mut self, val: super::vals::TXRIS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for SSPRIS {
    #[inline(always)]
    fn default() -> SSPRIS {
        SSPRIS(0)
    }
}
impl core::fmt::Debug for SSPRIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPRIS")
            .field("RORRIS", &self.RORRIS())
            .field("RTRIS", &self.RTRIS())
            .field("RXRIS", &self.RXRIS())
            .field("TXRIS", &self.TXRIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPRIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPRIS {{ RORRIS: {:?}, RTRIS: {:?}, RXRIS: {:?}, TXRIS: {:?} }}",
            self.RORRIS(),
            self.RTRIS(),
            self.RXRIS(),
            self.TXRIS()
        )
    }
}
///Status register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPSR(pub u32);
impl SSPSR {
    ///Transmit FIFO empty.
    #[must_use]
    #[inline(always)]
    pub const fn TFE(&self) -> super::vals::TFE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TFE::from_bits(val as u8)
    }
    ///Transmit FIFO empty.
    #[inline(always)]
    pub const fn set_TFE(&mut self, val: super::vals::TFE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Transmit FIFO not full.
    #[must_use]
    #[inline(always)]
    pub const fn TNF(&self) -> super::vals::TNF {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::TNF::from_bits(val as u8)
    }
    ///Transmit FIFO not full.
    #[inline(always)]
    pub const fn set_TNF(&mut self, val: super::vals::TNF) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///Rceive FIFIO not empty.
    #[must_use]
    #[inline(always)]
    pub const fn RNE(&self) -> super::vals::RNE {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RNE::from_bits(val as u8)
    }
    ///Rceive FIFIO not empty.
    #[inline(always)]
    pub const fn set_RNE(&mut self, val: super::vals::RNE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Receive FIFO full.
    #[must_use]
    #[inline(always)]
    pub const fn RFF(&self) -> super::vals::RFF {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::RFF::from_bits(val as u8)
    }
    ///Receive FIFO full.
    #[inline(always)]
    pub const fn set_RFF(&mut self, val: super::vals::RFF) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    ///SSP busy flag.
    #[must_use]
    #[inline(always)]
    pub const fn BSY(&self) -> super::vals::BSY {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::BSY::from_bits(val as u8)
    }
    ///SSP busy flag.
    #[inline(always)]
    pub const fn set_BSY(&mut self, val: super::vals::BSY) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for SSPSR {
    #[inline(always)]
    fn default() -> SSPSR {
        SSPSR(0)
    }
}
impl core::fmt::Debug for SSPSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPSR")
            .field("TFE", &self.TFE())
            .field("TNF", &self.TNF())
            .field("RNE", &self.RNE())
            .field("RFF", &self.RFF())
            .field("BSY", &self.BSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPSR {{ TFE: {:?}, TNF: {:?}, RNE: {:?}, RFF: {:?}, BSY: {:?} }}",
            self.TFE(),
            self.TNF(),
            self.RNE(),
            self.RFF(),
            self.BSY()
        )
    }
}
