///Register `RIS` reader
pub type R = crate::R<RisSpec>;
///Field `RIRRIS` reader - nUARTRI modem interrupt status
pub type RirrisR = crate::BitReader;
///Field `CTSMRIS` reader - nUARTCTS modem interrupt status
pub type CtsmrisR = crate::BitReader;
///Field `DCDMRIS` reader - nUARTDCD modem interrupt status
pub type DcdmrisR = crate::BitReader;
///Field `DSRMRIS` reader - nUARTDSR modem interrupt status
pub type DsrmrisR = crate::BitReader;
///Field `RXRIS` reader - Receive interrupt status
pub type RxrisR = crate::BitReader;
///Field `TXRIS` reader - Transmit interrupt status
pub type TxrisR = crate::BitReader;
///Field `RTRIS` reader - Receive timeout interrupt status
pub type RtrisR = crate::BitReader;
///Field `FERIS` reader - Framing error interrupt status
pub type FerisR = crate::BitReader;
///Field `PERIS` reader - Parity error interrupt status
pub type PerisR = crate::BitReader;
///Field `BERIS` reader - Break error interrupt status
pub type BerisR = crate::BitReader;
///Field `OERIS` reader - Overrun error interrupt status
pub type OerisR = crate::BitReader;
impl R {
    ///Bit 0 - nUARTRI modem interrupt status
    #[inline(always)]
    pub fn rirris(&self) -> RirrisR {
        RirrisR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - nUARTCTS modem interrupt status
    #[inline(always)]
    pub fn ctsmris(&self) -> CtsmrisR {
        CtsmrisR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - nUARTDCD modem interrupt status
    #[inline(always)]
    pub fn dcdmris(&self) -> DcdmrisR {
        DcdmrisR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - nUARTDSR modem interrupt status
    #[inline(always)]
    pub fn dsrmris(&self) -> DsrmrisR {
        DsrmrisR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive interrupt status
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit interrupt status
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive timeout interrupt status
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Framing error interrupt status
    #[inline(always)]
    pub fn feris(&self) -> FerisR {
        FerisR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Parity error interrupt status
    #[inline(always)]
    pub fn peris(&self) -> PerisR {
        PerisR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Break error interrupt status
    #[inline(always)]
    pub fn beris(&self) -> BerisR {
        BerisR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Overrun error interrupt status
    #[inline(always)]
    pub fn oeris(&self) -> OerisR {
        OerisR::new(((self.bits >> 10) & 1) != 0)
    }
}
/**Raw Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
///`read()` method returns [`ris::R`](R) reader structure
impl crate::Readable for RisSpec {}
///`reset()` method sets RIS to value 0
impl crate::Resettable for RisSpec {}
