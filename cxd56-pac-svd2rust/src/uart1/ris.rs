#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RIRRIS` reader - nUARTRI modem interrupt status"]
pub type RirrisR = crate::BitReader;
#[doc = "Field `CTSMRIS` reader - nUARTCTS modem interrupt status"]
pub type CtsmrisR = crate::BitReader;
#[doc = "Field `DCDMRIS` reader - nUARTDCD modem interrupt status"]
pub type DcdmrisR = crate::BitReader;
#[doc = "Field `DSRMRIS` reader - nUARTDSR modem interrupt status"]
pub type DsrmrisR = crate::BitReader;
#[doc = "Field `RXRIS` reader - Receive interrupt status"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - Transmit interrupt status"]
pub type TxrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - Receive timeout interrupt status"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `FERIS` reader - Framing error interrupt status"]
pub type FerisR = crate::BitReader;
#[doc = "Field `PERIS` reader - Parity error interrupt status"]
pub type PerisR = crate::BitReader;
#[doc = "Field `BERIS` reader - Break error interrupt status"]
pub type BerisR = crate::BitReader;
#[doc = "Field `OERIS` reader - Overrun error interrupt status"]
pub type OerisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt status"]
    #[inline(always)]
    pub fn rirris(&self) -> RirrisR {
        RirrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt status"]
    #[inline(always)]
    pub fn ctsmris(&self) -> CtsmrisR {
        CtsmrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt status"]
    #[inline(always)]
    pub fn dcdmris(&self) -> DcdmrisR {
        DcdmrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt status"]
    #[inline(always)]
    pub fn dsrmris(&self) -> DsrmrisR {
        DsrmrisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt status"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt status"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt status"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt status"]
    #[inline(always)]
    pub fn feris(&self) -> FerisR {
        FerisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt status"]
    #[inline(always)]
    pub fn peris(&self) -> PerisR {
        PerisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt status"]
    #[inline(always)]
    pub fn beris(&self) -> BerisR {
        BerisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt status"]
    #[inline(always)]
    pub fn oeris(&self) -> OerisR {
        OerisR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {}
