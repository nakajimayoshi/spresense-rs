#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `RIMMIS` reader - nUARTRI modem masked interrupt status"]
pub type RimmisR = crate::BitReader;
#[doc = "Field `CTSMMIS` reader - nUARTCTS modem masked interrupt status"]
pub type CtsmmisR = crate::BitReader;
#[doc = "Field `DCDMMIS` reader - nUARTDCD modem masked interrupt status"]
pub type DcdmmisR = crate::BitReader;
#[doc = "Field `DSRMMIS` reader - nUARTDSR modem masked interrupt status"]
pub type DsrmmisR = crate::BitReader;
#[doc = "Field `RXMIS` reader - Receive masked interrupt status"]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - Transmit masked interrupt status"]
pub type TxmisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - Receive timeout masked interrupt status"]
pub type RtmisR = crate::BitReader;
#[doc = "Field `FEMIS` reader - Framing error masked interrupt status"]
pub type FemisR = crate::BitReader;
#[doc = "Field `PEMIS` reader - Parity error masked interrupt status"]
pub type PemisR = crate::BitReader;
#[doc = "Field `BEMIS` reader - Break error masked interrupt status"]
pub type BemisR = crate::BitReader;
#[doc = "Field `OEMIS` reader - Overrun error masked interrupt status"]
pub type OemisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - nUARTRI modem masked interrupt status"]
    #[inline(always)]
    pub fn rimmis(&self) -> RimmisR {
        RimmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem masked interrupt status"]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CtsmmisR {
        CtsmmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem masked interrupt status"]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DcdmmisR {
        DcdmmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem masked interrupt status"]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DsrmmisR {
        DsrmmisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive masked interrupt status"]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit masked interrupt status"]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout masked interrupt status"]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error masked interrupt status"]
    #[inline(always)]
    pub fn femis(&self) -> FemisR {
        FemisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error masked interrupt status"]
    #[inline(always)]
    pub fn pemis(&self) -> PemisR {
        PemisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error masked interrupt status"]
    #[inline(always)]
    pub fn bemis(&self) -> BemisR {
        BemisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error masked interrupt status"]
    #[inline(always)]
    pub fn oemis(&self) -> OemisR {
        OemisR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
