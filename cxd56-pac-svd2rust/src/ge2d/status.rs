///Register `STATUS` reader
pub type R = crate::R<StatusSpec>;
///Field `NREQ` reader - Requesting Normal Descriptor
pub type NreqR = crate::BitReader;
///Field `SREQ` reader - Reqesting Descriptor Stop
pub type SreqR = crate::BitReader;
///Field `NDCR` reader - Normal Descriptor Command Running Status
pub type NdcrR = crate::BitReader;
///Field `ISER` reader - ISE Running Status
pub type IserR = crate::BitReader;
impl R {
    ///Bit 0 - Requesting Normal Descriptor
    #[inline(always)]
    pub fn nreq(&self) -> NreqR {
        NreqR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Reqesting Descriptor Stop
    #[inline(always)]
    pub fn sreq(&self) -> SreqR {
        SreqR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Normal Descriptor Command Running Status
    #[inline(always)]
    pub fn ndcr(&self) -> NdcrR {
        NdcrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 24 - ISE Running Status
    #[inline(always)]
    pub fn iser(&self) -> IserR {
        IserR::new(((self.bits >> 24) & 1) != 0)
    }
}
/**Status Register

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for StatusSpec {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for StatusSpec {}
