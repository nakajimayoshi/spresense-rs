///Control Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CR(pub u32);
impl CR {
    ///UART enable.
    #[must_use]
    #[inline(always)]
    pub const fn UARTEN(&self) -> super::vals::UARTEN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UARTEN::from_bits(val as u8)
    }
    ///UART enable.
    #[inline(always)]
    pub const fn set_UARTEN(&mut self, val: super::vals::UARTEN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///SIR enable.
    #[must_use]
    #[inline(always)]
    pub const fn SIREN(&self) -> super::vals::SIREN {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SIREN::from_bits(val as u8)
    }
    ///SIR enable.
    #[inline(always)]
    pub const fn set_SIREN(&mut self, val: super::vals::SIREN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///SIR low-power IrDA.
    #[must_use]
    #[inline(always)]
    pub const fn SIRLP(&self) -> super::vals::SIRLP {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SIRLP::from_bits(val as u8)
    }
    ///SIR low-power IrDA.
    #[inline(always)]
    pub const fn set_SIRLP(&mut self, val: super::vals::SIRLP) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Invert SIR input.
    #[must_use]
    #[inline(always)]
    pub const fn SIRIINV(&self) -> super::vals::SIRIINV {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SIRIINV::from_bits(val as u8)
    }
    ///Invert SIR input.
    #[inline(always)]
    pub const fn set_SIRIINV(&mut self, val: super::vals::SIRIINV) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    ///Invert SIR output.
    #[must_use]
    #[inline(always)]
    pub const fn SIROINV(&self) -> super::vals::SIROINV {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SIROINV::from_bits(val as u8)
    }
    ///Invert SIR output.
    #[inline(always)]
    pub const fn set_SIROINV(&mut self, val: super::vals::SIROINV) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    ///RxD mask.
    #[must_use]
    #[inline(always)]
    pub const fn RXDMSK(&self) -> super::vals::RXDMSK {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::RXDMSK::from_bits(val as u8)
    }
    ///RxD mask.
    #[inline(always)]
    pub const fn set_RXDMSK(&mut self, val: super::vals::RXDMSK) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    ///Invert DTR signal.
    #[must_use]
    #[inline(always)]
    pub const fn DTRINV(&self) -> super::vals::DTRINV {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DTRINV::from_bits(val as u8)
    }
    ///Invert DTR signal.
    #[inline(always)]
    pub const fn set_DTRINV(&mut self, val: super::vals::DTRINV) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    ///Loopback enable.
    #[must_use]
    #[inline(always)]
    pub const fn LBE(&self) -> super::vals::LBE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::LBE::from_bits(val as u8)
    }
    ///Loopback enable.
    #[inline(always)]
    pub const fn set_LBE(&mut self, val: super::vals::LBE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    ///Transmit enable.
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> super::vals::TXE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TXE::from_bits(val as u8)
    }
    ///Transmit enable.
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: super::vals::TXE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    ///Receive enable.
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> super::vals::RXE {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::RXE::from_bits(val as u8)
    }
    ///Receive enable.
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: super::vals::RXE) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    ///Data transmit ready.
    #[must_use]
    #[inline(always)]
    pub const fn DTR(&self) -> super::vals::DTR {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::DTR::from_bits(val as u8)
    }
    ///Data transmit ready.
    #[inline(always)]
    pub const fn set_DTR(&mut self, val: super::vals::DTR) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    ///Request to send.
    #[must_use]
    #[inline(always)]
    pub const fn RTS(&self) -> super::vals::RTS {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::RTS::from_bits(val as u8)
    }
    ///Request to send.
    #[inline(always)]
    pub const fn set_RTS(&mut self, val: super::vals::RTS) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    ///nUARTOut1 modem status.
    #[must_use]
    #[inline(always)]
    pub const fn Out1(&self) -> super::vals::Out1 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Out1::from_bits(val as u8)
    }
    ///nUARTOut1 modem status.
    #[inline(always)]
    pub const fn set_Out1(&mut self, val: super::vals::Out1) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    ///nUARTOut2 modem status.
    #[must_use]
    #[inline(always)]
    pub const fn Out2(&self) -> super::vals::Out2 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Out2::from_bits(val as u8)
    }
    ///nUARTOut2 modem status.
    #[inline(always)]
    pub const fn set_Out2(&mut self, val: super::vals::Out2) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    ///RTS hardware flow control enable.
    #[must_use]
    #[inline(always)]
    pub const fn RTSEn(&self) -> super::vals::RTSEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::RTSEn::from_bits(val as u8)
    }
    ///RTS hardware flow control enable.
    #[inline(always)]
    pub const fn set_RTSEn(&mut self, val: super::vals::RTSEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    ///CTS hardware flow control enable.
    #[must_use]
    #[inline(always)]
    pub const fn CTSEn(&self) -> super::vals::CTSEn {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::CTSEn::from_bits(val as u8)
    }
    ///CTS hardware flow control enable.
    #[inline(always)]
    pub const fn set_CTSEn(&mut self, val: super::vals::CTSEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for CR {
    #[inline(always)]
    fn default() -> CR {
        CR(0)
    }
}
impl core::fmt::Debug for CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("UARTEN", &self.UARTEN())
            .field("SIREN", &self.SIREN())
            .field("SIRLP", &self.SIRLP())
            .field("SIRIINV", &self.SIRIINV())
            .field("SIROINV", &self.SIROINV())
            .field("RXDMSK", &self.RXDMSK())
            .field("DTRINV", &self.DTRINV())
            .field("LBE", &self.LBE())
            .field("TXE", &self.TXE())
            .field("RXE", &self.RXE())
            .field("DTR", &self.DTR())
            .field("RTS", &self.RTS())
            .field("Out1", &self.Out1())
            .field("Out2", &self.Out2())
            .field("RTSEn", &self.RTSEn())
            .field("CTSEn", &self.CTSEn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CR {{ UARTEN: {:?}, SIREN: {:?}, SIRLP: {:?}, SIRIINV: {:?}, SIROINV: {:?}, RXDMSK: {:?}, DTRINV: {:?}, LBE: {:?}, TXE: {:?}, RXE: {:?}, DTR: {:?}, RTS: {:?}, Out1: {:?}, Out2: {:?}, RTSEn: {:?}, CTSEn: {:?} }}",
            self.UARTEN(),
            self.SIREN(),
            self.SIRLP(),
            self.SIRIINV(),
            self.SIROINV(),
            self.RXDMSK(),
            self.DTRINV(),
            self.LBE(),
            self.TXE(),
            self.RXE(),
            self.DTR(),
            self.RTS(),
            self.Out1(),
            self.Out2(),
            self.RTSEn(),
            self.CTSEn()
        )
    }
}
///DMA Control Regsiter.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACR(pub u32);
impl DMACR {
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
    ///DMA on error.
    #[must_use]
    #[inline(always)]
    pub const fn DMAONERR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///DMA on error.
    #[inline(always)]
    pub const fn set_DMAONERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for DMACR {
    #[inline(always)]
    fn default() -> DMACR {
        DMACR(0)
    }
}
impl core::fmt::Debug for DMACR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACR")
            .field("RXDMAE", &self.RXDMAE())
            .field("TXDMAE", &self.TXDMAE())
            .field("DMAONERR", &self.DMAONERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACR {{ RXDMAE: {:?}, TXDMAE: {:?}, DMAONERR: {=bool:?} }}",
            self.RXDMAE(),
            self.TXDMAE(),
            self.DMAONERR()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DR(pub u32);
impl DR {
    ///Data.
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Data.
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    ///Framing Error.
    #[must_use]
    #[inline(always)]
    pub const fn FE(&self) -> super::vals::FE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FE::from_bits(val as u8)
    }
    ///Framing Error.
    #[inline(always)]
    pub const fn set_FE(&mut self, val: super::vals::FE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    ///Parity Error.
    #[must_use]
    #[inline(always)]
    pub const fn PE(&self) -> super::vals::PE {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PE::from_bits(val as u8)
    }
    ///Parity Error.
    #[inline(always)]
    pub const fn set_PE(&mut self, val: super::vals::PE) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    ///Break Error.
    #[must_use]
    #[inline(always)]
    pub const fn BE(&self) -> super::vals::BE {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::BE::from_bits(val as u8)
    }
    ///Break Error.
    #[inline(always)]
    pub const fn set_BE(&mut self, val: super::vals::BE) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    ///Overrun Error.
    #[must_use]
    #[inline(always)]
    pub const fn OE(&self) -> super::vals::OE {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::OE::from_bits(val as u8)
    }
    ///Overrun Error.
    #[inline(always)]
    pub const fn set_OE(&mut self, val: super::vals::OE) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for DR {
    #[inline(always)]
    fn default() -> DR {
        DR(0)
    }
}
impl core::fmt::Debug for DR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("DATA", &self.DATA())
            .field("FE", &self.FE())
            .field("PE", &self.PE())
            .field("BE", &self.BE())
            .field("OE", &self.OE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DR {{ DATA: {=u8:?}, FE: {:?}, PE: {:?}, BE: {:?}, OE: {:?} }}",
            self.DATA(),
            self.FE(),
            self.PE(),
            self.BE(),
            self.OE()
        )
    }
}
///The fractional part of the baud rate divisor.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBRD(pub u32);
impl FBRD {
    ///The fractional baud rate divisor.
    #[must_use]
    #[inline(always)]
    pub const fn BAUD_DIVFRAC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    ///The fractional baud rate divisor.
    #[inline(always)]
    pub const fn set_BAUD_DIVFRAC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for FBRD {
    #[inline(always)]
    fn default() -> FBRD {
        FBRD(0)
    }
}
impl core::fmt::Debug for FBRD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBRD")
            .field("BAUD_DIVFRAC", &self.BAUD_DIVFRAC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBRD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FBRD {{ BAUD_DIVFRAC: {=u8:?} }}", self.BAUD_DIVFRAC())
    }
}
///Flags Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FR(pub u32);
impl FR {
    ///Clear to send.
    #[must_use]
    #[inline(always)]
    pub const fn CTS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Clear to send.
    #[inline(always)]
    pub const fn set_CTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Data set ready.
    #[must_use]
    #[inline(always)]
    pub const fn DSR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Data set ready.
    #[inline(always)]
    pub const fn set_DSR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Data carrier detect.
    #[must_use]
    #[inline(always)]
    pub const fn DCD(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Data carrier detect.
    #[inline(always)]
    pub const fn set_DCD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///UART busy.
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///UART busy.
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Receive FIFO empty.
    #[must_use]
    #[inline(always)]
    pub const fn RXFE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Receive FIFO empty.
    #[inline(always)]
    pub const fn set_RXFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Transmit FIFO full.
    #[must_use]
    #[inline(always)]
    pub const fn TXFF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Transmit FIFO full.
    #[inline(always)]
    pub const fn set_TXFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Receive FIFO full.
    #[must_use]
    #[inline(always)]
    pub const fn RXFF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Receive FIFO full.
    #[inline(always)]
    pub const fn set_RXFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Transmit FIFO empty.
    #[must_use]
    #[inline(always)]
    pub const fn TXFE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Transmit FIFO empty.
    #[inline(always)]
    pub const fn set_TXFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Ring Indicator.
    #[must_use]
    #[inline(always)]
    pub const fn RI(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Ring Indicator.
    #[inline(always)]
    pub const fn set_RI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for FR {
    #[inline(always)]
    fn default() -> FR {
        FR(0)
    }
}
impl core::fmt::Debug for FR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FR")
            .field("CTS", &self.CTS())
            .field("DSR", &self.DSR())
            .field("DCD", &self.DCD())
            .field("BUSY", &self.BUSY())
            .field("RXFE", &self.RXFE())
            .field("TXFF", &self.TXFF())
            .field("RXFF", &self.RXFF())
            .field("TXFE", &self.TXFE())
            .field("RI", &self.RI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FR {{ CTS: {=bool:?}, DSR: {=bool:?}, DCD: {=bool:?}, BUSY: {=bool:?}, RXFE: {=bool:?}, TXFF: {=bool:?}, RXFF: {=bool:?}, TXFE: {=bool:?}, RI: {=bool:?} }}",
            self.CTS(),
            self.DSR(),
            self.DCD(),
            self.BUSY(),
            self.RXFE(),
            self.TXFF(),
            self.RXFF(),
            self.TXFE(),
            self.RI()
        )
    }
}
///The integer part of the baud rate divisor.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IBRD(pub u32);
impl IBRD {
    ///The integer baud rate divisor.
    #[must_use]
    #[inline(always)]
    pub const fn BAUD_DIVINT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///The integer baud rate divisor.
    #[inline(always)]
    pub const fn set_BAUD_DIVINT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IBRD {
    #[inline(always)]
    fn default() -> IBRD {
        IBRD(0)
    }
}
impl core::fmt::Debug for IBRD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBRD")
            .field("BAUD_DIVINT", &self.BAUD_DIVINT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IBRD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IBRD {{ BAUD_DIVINT: {=u16:?} }}", self.BAUD_DIVINT())
    }
}
///Interrupt Clear Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICR(pub u32);
impl ICR {
    ///nUARTRI modem interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn RIMIC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///nUARTRI modem interrupt clear.
    #[inline(always)]
    pub const fn set_RIMIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///nUARTCTS modem interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn CTSMIC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///nUARTCTS modem interrupt clear.
    #[inline(always)]
    pub const fn set_CTSMIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///nUARTDCD modem interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn DCDMIC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///nUARTDCD modem interrupt clear.
    #[inline(always)]
    pub const fn set_DCDMIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///nUARTDSR modem interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn DSRMIC(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///nUARTDSR modem interrupt clear.
    #[inline(always)]
    pub const fn set_DSRMIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Receive interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn RXIC(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Receive interrupt clear.
    #[inline(always)]
    pub const fn set_RXIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Transmit interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn TXIC(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Transmit interrupt clear.
    #[inline(always)]
    pub const fn set_TXIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Receive timeout interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn RTIC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Receive timeout interrupt clear.
    #[inline(always)]
    pub const fn set_RTIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Framing error interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn FEIC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Framing error interrupt clear.
    #[inline(always)]
    pub const fn set_FEIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Parity error interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn PEIC(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Parity error interrupt clear.
    #[inline(always)]
    pub const fn set_PEIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Break error interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn BEIC(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Break error interrupt clear.
    #[inline(always)]
    pub const fn set_BEIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///Overrun error interrupt clear.
    #[must_use]
    #[inline(always)]
    pub const fn OEIC(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///Overrun error interrupt clear.
    #[inline(always)]
    pub const fn set_OEIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for ICR {
    #[inline(always)]
    fn default() -> ICR {
        ICR(0)
    }
}
impl core::fmt::Debug for ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("RIMIC", &self.RIMIC())
            .field("CTSMIC", &self.CTSMIC())
            .field("DCDMIC", &self.DCDMIC())
            .field("DSRMIC", &self.DSRMIC())
            .field("RXIC", &self.RXIC())
            .field("TXIC", &self.TXIC())
            .field("RTIC", &self.RTIC())
            .field("FEIC", &self.FEIC())
            .field("PEIC", &self.PEIC())
            .field("BEIC", &self.BEIC())
            .field("OEIC", &self.OEIC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ICR {{ RIMIC: {=bool:?}, CTSMIC: {=bool:?}, DCDMIC: {=bool:?}, DSRMIC: {=bool:?}, RXIC: {=bool:?}, TXIC: {=bool:?}, RTIC: {=bool:?}, FEIC: {=bool:?}, PEIC: {=bool:?}, BEIC: {=bool:?}, OEIC: {=bool:?} }}",
            self.RIMIC(),
            self.CTSMIC(),
            self.DCDMIC(),
            self.DSRMIC(),
            self.RXIC(),
            self.TXIC(),
            self.RTIC(),
            self.FEIC(),
            self.PEIC(),
            self.BEIC(),
            self.OEIC()
        )
    }
}
///Interrupt FIFO Level Select Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IFLS(pub u32);
impl IFLS {
    ///Transmit interrupt FIFO level select.
    #[must_use]
    #[inline(always)]
    pub const fn TXIFLSEL(&self) -> super::vals::TXIFLSEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::TXIFLSEL::from_bits(val as u8)
    }
    ///Transmit interrupt FIFO level select.
    #[inline(always)]
    pub const fn set_TXIFLSEL(&mut self, val: super::vals::TXIFLSEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    ///Receive interrupt FIFO level select.
    #[must_use]
    #[inline(always)]
    pub const fn RXIFLSEL(&self) -> super::vals::RXIFLSEL {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::RXIFLSEL::from_bits(val as u8)
    }
    ///Receive interrupt FIFO level select.
    #[inline(always)]
    pub const fn set_RXIFLSEL(&mut self, val: super::vals::RXIFLSEL) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
}
impl Default for IFLS {
    #[inline(always)]
    fn default() -> IFLS {
        IFLS(0)
    }
}
impl core::fmt::Debug for IFLS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFLS")
            .field("TXIFLSEL", &self.TXIFLSEL())
            .field("RXIFLSEL", &self.RXIFLSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IFLS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IFLS {{ TXIFLSEL: {:?}, RXIFLSEL: {:?} }}",
            self.TXIFLSEL(),
            self.RXIFLSEL()
        )
    }
}
///IrDA Low-Power Counter Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ILPR(pub u32);
impl ILPR {
    ///8-bit low-power divisor value.
    #[must_use]
    #[inline(always)]
    pub const fn ILPDVSR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///8-bit low-power divisor value.
    #[inline(always)]
    pub const fn set_ILPDVSR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ILPR {
    #[inline(always)]
    fn default() -> ILPR {
        ILPR(0)
    }
}
impl core::fmt::Debug for ILPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILPR")
            .field("ILPDVSR", &self.ILPDVSR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ILPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ILPR {{ ILPDVSR: {=u8:?} }}", self.ILPDVSR())
    }
}
///Interrupt Mask Set and Clear Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IMSC(pub u32);
impl IMSC {
    ///nUARTRI modem interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RIMIM(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///nUARTRI modem interrupt mask.
    #[inline(always)]
    pub const fn set_RIMIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///nUARTCTS modem interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn CTSMIM(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///nUARTCTS modem interrupt mask.
    #[inline(always)]
    pub const fn set_CTSMIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///nUARTDCD modem interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn DCDMIM(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///nUARTDCD modem interrupt mask.
    #[inline(always)]
    pub const fn set_DCDMIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///nUARTDSR modem interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn DSRMIM(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///nUARTDSR modem interrupt mask.
    #[inline(always)]
    pub const fn set_DSRMIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Receive interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RXIM(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Receive interrupt mask.
    #[inline(always)]
    pub const fn set_RXIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Transmit interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn TXIM(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Transmit interrupt mask.
    #[inline(always)]
    pub const fn set_TXIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Receive timeout interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn RTIM(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Receive timeout interrupt mask.
    #[inline(always)]
    pub const fn set_RTIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Framing error interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn FEIM(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Framing error interrupt mask.
    #[inline(always)]
    pub const fn set_FEIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Parity error interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn PEIM(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Parity error interrupt mask.
    #[inline(always)]
    pub const fn set_PEIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Break error interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn BEIM(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Break error interrupt mask.
    #[inline(always)]
    pub const fn set_BEIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///Overrun error interrupt mask.
    #[must_use]
    #[inline(always)]
    pub const fn OEIM(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///Overrun error interrupt mask.
    #[inline(always)]
    pub const fn set_OEIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for IMSC {
    #[inline(always)]
    fn default() -> IMSC {
        IMSC(0)
    }
}
impl core::fmt::Debug for IMSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMSC")
            .field("RIMIM", &self.RIMIM())
            .field("CTSMIM", &self.CTSMIM())
            .field("DCDMIM", &self.DCDMIM())
            .field("DSRMIM", &self.DSRMIM())
            .field("RXIM", &self.RXIM())
            .field("TXIM", &self.TXIM())
            .field("RTIM", &self.RTIM())
            .field("FEIM", &self.FEIM())
            .field("PEIM", &self.PEIM())
            .field("BEIM", &self.BEIM())
            .field("OEIM", &self.OEIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IMSC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IMSC {{ RIMIM: {=bool:?}, CTSMIM: {=bool:?}, DCDMIM: {=bool:?}, DSRMIM: {=bool:?}, RXIM: {=bool:?}, TXIM: {=bool:?}, RTIM: {=bool:?}, FEIM: {=bool:?}, PEIM: {=bool:?}, BEIM: {=bool:?}, OEIM: {=bool:?} }}",
            self.RIMIM(),
            self.CTSMIM(),
            self.DCDMIM(),
            self.DSRMIM(),
            self.RXIM(),
            self.TXIM(),
            self.RTIM(),
            self.FEIM(),
            self.PEIM(),
            self.BEIM(),
            self.OEIM()
        )
    }
}
///Line Control Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LCR_H(pub u32);
impl LCR_H {
    ///Send break.
    #[must_use]
    #[inline(always)]
    pub const fn BRK(&self) -> super::vals::BRK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::BRK::from_bits(val as u8)
    }
    ///Send break.
    #[inline(always)]
    pub const fn set_BRK(&mut self, val: super::vals::BRK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Parity enable.
    #[must_use]
    #[inline(always)]
    pub const fn PEN(&self) -> super::vals::PEN {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PEN::from_bits(val as u8)
    }
    ///Parity enable.
    #[inline(always)]
    pub const fn set_PEN(&mut self, val: super::vals::PEN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///Even parity select.
    #[must_use]
    #[inline(always)]
    pub const fn EPS(&self) -> super::vals::EPS {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EPS::from_bits(val as u8)
    }
    ///Even parity select.
    #[inline(always)]
    pub const fn set_EPS(&mut self, val: super::vals::EPS) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Two stop bits select.
    #[must_use]
    #[inline(always)]
    pub const fn STP2(&self) -> super::vals::STP2 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::STP2::from_bits(val as u8)
    }
    ///Two stop bits select.
    #[inline(always)]
    pub const fn set_STP2(&mut self, val: super::vals::STP2) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    ///Enable FIFOs.
    #[must_use]
    #[inline(always)]
    pub const fn FEN(&self) -> super::vals::FEN {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::FEN::from_bits(val as u8)
    }
    ///Enable FIFOs.
    #[inline(always)]
    pub const fn set_FEN(&mut self, val: super::vals::FEN) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    ///Word Length.
    #[must_use]
    #[inline(always)]
    pub const fn WLEN(&self) -> super::vals::WLEN {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::WLEN::from_bits(val as u8)
    }
    ///Word Length.
    #[inline(always)]
    pub const fn set_WLEN(&mut self, val: super::vals::WLEN) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    ///Stick parity select.
    #[must_use]
    #[inline(always)]
    pub const fn SPS(&self) -> super::vals::SPS {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SPS::from_bits(val as u8)
    }
    ///Stick parity select.
    #[inline(always)]
    pub const fn set_SPS(&mut self, val: super::vals::SPS) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for LCR_H {
    #[inline(always)]
    fn default() -> LCR_H {
        LCR_H(0)
    }
}
impl core::fmt::Debug for LCR_H {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCR_H")
            .field("BRK", &self.BRK())
            .field("PEN", &self.PEN())
            .field("EPS", &self.EPS())
            .field("STP2", &self.STP2())
            .field("FEN", &self.FEN())
            .field("WLEN", &self.WLEN())
            .field("SPS", &self.SPS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LCR_H {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LCR_H {{ BRK: {:?}, PEN: {:?}, EPS: {:?}, STP2: {:?}, FEN: {:?}, WLEN: {:?}, SPS: {:?} }}",
            self.BRK(),
            self.PEN(),
            self.EPS(),
            self.STP2(),
            self.FEN(),
            self.WLEN(),
            self.SPS()
        )
    }
}
///Masked Interrupt Status Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MIS(pub u32);
impl MIS {
    ///nUARTRI modem masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn RIMMIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///nUARTRI modem masked interrupt status.
    #[inline(always)]
    pub const fn set_RIMMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///nUARTCTS modem masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn CTSMMIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///nUARTCTS modem masked interrupt status.
    #[inline(always)]
    pub const fn set_CTSMMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///nUARTDCD modem masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn DCDMMIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///nUARTDCD modem masked interrupt status.
    #[inline(always)]
    pub const fn set_DCDMMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///nUARTDSR modem masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn DSRMMIS(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///nUARTDSR modem masked interrupt status.
    #[inline(always)]
    pub const fn set_DSRMMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Receive masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn RXMIS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Receive masked interrupt status.
    #[inline(always)]
    pub const fn set_RXMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Transmit masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn TXMIS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Transmit masked interrupt status.
    #[inline(always)]
    pub const fn set_TXMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Receive timeout masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn RTMIS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Receive timeout masked interrupt status.
    #[inline(always)]
    pub const fn set_RTMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Framing error masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn FEMIS(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Framing error masked interrupt status.
    #[inline(always)]
    pub const fn set_FEMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Parity error masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn PEMIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Parity error masked interrupt status.
    #[inline(always)]
    pub const fn set_PEMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Break error masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn BEMIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Break error masked interrupt status.
    #[inline(always)]
    pub const fn set_BEMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///Overrun error masked interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn OEMIS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///Overrun error masked interrupt status.
    #[inline(always)]
    pub const fn set_OEMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("RIMMIS", &self.RIMMIS())
            .field("CTSMMIS", &self.CTSMMIS())
            .field("DCDMMIS", &self.DCDMMIS())
            .field("DSRMMIS", &self.DSRMMIS())
            .field("RXMIS", &self.RXMIS())
            .field("TXMIS", &self.TXMIS())
            .field("RTMIS", &self.RTMIS())
            .field("FEMIS", &self.FEMIS())
            .field("PEMIS", &self.PEMIS())
            .field("BEMIS", &self.BEMIS())
            .field("OEMIS", &self.OEMIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MIS {{ RIMMIS: {=bool:?}, CTSMMIS: {=bool:?}, DCDMMIS: {=bool:?}, DSRMMIS: {=bool:?}, RXMIS: {=bool:?}, TXMIS: {=bool:?}, RTMIS: {=bool:?}, FEMIS: {=bool:?}, PEMIS: {=bool:?}, BEMIS: {=bool:?}, OEMIS: {=bool:?} }}",
            self.RIMMIS(),
            self.CTSMMIS(),
            self.DCDMMIS(),
            self.DSRMMIS(),
            self.RXMIS(),
            self.TXMIS(),
            self.RTMIS(),
            self.FEMIS(),
            self.PEMIS(),
            self.BEMIS(),
            self.OEMIS()
        )
    }
}
///Raw Interrupt Status Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RIS(pub u32);
impl RIS {
    ///nUARTRI modem interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn RIRRIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///nUARTRI modem interrupt status.
    #[inline(always)]
    pub const fn set_RIRRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///nUARTCTS modem interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn CTSMRIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///nUARTCTS modem interrupt status.
    #[inline(always)]
    pub const fn set_CTSMRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///nUARTDCD modem interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn DCDMRIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///nUARTDCD modem interrupt status.
    #[inline(always)]
    pub const fn set_DCDMRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///nUARTDSR modem interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn DSRMRIS(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///nUARTDSR modem interrupt status.
    #[inline(always)]
    pub const fn set_DSRMRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Receive interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn RXRIS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Receive interrupt status.
    #[inline(always)]
    pub const fn set_RXRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Transmit interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn TXRIS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Transmit interrupt status.
    #[inline(always)]
    pub const fn set_TXRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Receive timeout interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn RTRIS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Receive timeout interrupt status.
    #[inline(always)]
    pub const fn set_RTRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Framing error interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn FERIS(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Framing error interrupt status.
    #[inline(always)]
    pub const fn set_FERIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Parity error interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn PERIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Parity error interrupt status.
    #[inline(always)]
    pub const fn set_PERIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Break error interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn BERIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Break error interrupt status.
    #[inline(always)]
    pub const fn set_BERIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///Overrun error interrupt status.
    #[must_use]
    #[inline(always)]
    pub const fn OERIS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///Overrun error interrupt status.
    #[inline(always)]
    pub const fn set_OERIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("RIRRIS", &self.RIRRIS())
            .field("CTSMRIS", &self.CTSMRIS())
            .field("DCDMRIS", &self.DCDMRIS())
            .field("DSRMRIS", &self.DSRMRIS())
            .field("RXRIS", &self.RXRIS())
            .field("TXRIS", &self.TXRIS())
            .field("RTRIS", &self.RTRIS())
            .field("FERIS", &self.FERIS())
            .field("PERIS", &self.PERIS())
            .field("BERIS", &self.BERIS())
            .field("OERIS", &self.OERIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RIS {{ RIRRIS: {=bool:?}, CTSMRIS: {=bool:?}, DCDMRIS: {=bool:?}, DSRMRIS: {=bool:?}, RXRIS: {=bool:?}, TXRIS: {=bool:?}, RTRIS: {=bool:?}, FERIS: {=bool:?}, PERIS: {=bool:?}, BERIS: {=bool:?}, OERIS: {=bool:?} }}",
            self.RIRRIS(),
            self.CTSMRIS(),
            self.DCDMRIS(),
            self.DSRMRIS(),
            self.RXRIS(),
            self.TXRIS(),
            self.RTRIS(),
            self.FERIS(),
            self.PERIS(),
            self.BERIS(),
            self.OERIS()
        )
    }
}
///Receive Status and Clear Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSR(pub u32);
impl RSR {
    ///Framing Error.
    #[must_use]
    #[inline(always)]
    pub const fn RFE(&self) -> super::vals::RFE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RFE::from_bits(val as u8)
    }
    ///Framing Error.
    #[inline(always)]
    pub const fn set_RFE(&mut self, val: super::vals::RFE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    ///Parity Error.
    #[must_use]
    #[inline(always)]
    pub const fn RPE(&self) -> super::vals::RPE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RPE::from_bits(val as u8)
    }
    ///Parity Error.
    #[inline(always)]
    pub const fn set_RPE(&mut self, val: super::vals::RPE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    ///Break Error.
    #[must_use]
    #[inline(always)]
    pub const fn RBE(&self) -> super::vals::RBE {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RBE::from_bits(val as u8)
    }
    ///Break Error.
    #[inline(always)]
    pub const fn set_RBE(&mut self, val: super::vals::RBE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    ///Overrun Error.
    #[must_use]
    #[inline(always)]
    pub const fn ROE(&self) -> super::vals::ROE {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ROE::from_bits(val as u8)
    }
    ///Overrun Error.
    #[inline(always)]
    pub const fn set_ROE(&mut self, val: super::vals::ROE) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for RSR {
    #[inline(always)]
    fn default() -> RSR {
        RSR(0)
    }
}
impl core::fmt::Debug for RSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSR")
            .field("RFE", &self.RFE())
            .field("RPE", &self.RPE())
            .field("RBE", &self.RBE())
            .field("ROE", &self.ROE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RSR {{ RFE: {:?}, RPE: {:?}, RBE: {:?}, ROE: {:?} }}",
            self.RFE(),
            self.RPE(),
            self.RBE(),
            self.ROE()
        )
    }
}
///Receive Timeout Configuration Register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTO(pub u32);
impl RTO {
    ///Recieve timeout.
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    ///Recieve timeout.
    #[inline(always)]
    pub const fn set_TIMEOUT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RTO {
    #[inline(always)]
    fn default() -> RTO {
        RTO(0)
    }
}
impl core::fmt::Debug for RTO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTO")
            .field("TIMEOUT", &self.TIMEOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RTO {{ TIMEOUT: {=u16:?} }}", self.TIMEOUT())
    }
}
