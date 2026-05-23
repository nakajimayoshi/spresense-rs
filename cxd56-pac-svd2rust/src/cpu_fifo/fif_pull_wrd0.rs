///Register `FIF_PULL_WRD0` reader
pub type R = crate::R<FifPullWrd0Spec>;
///Register `FIF_PULL_WRD0` writer
pub type W = crate::W<FifPullWrd0Spec>;
///Field `DATA_0` reader - RX data word 0
pub type Data0R = crate::FieldReader<u32>;
///Field `DATA_0` writer - RX data word 0
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
///Field `SENDER_ID` reader - Sender ID
pub type SenderIdR = crate::FieldReader;
///Field `SENDER_ID` writer - Sender ID
pub type SenderIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:27 - RX data word 0
    #[inline(always)]
    pub fn data_0(&self) -> Data0R {
        Data0R::new(self.bits & 0x0fff_ffff)
    }
    ///Bits 28:31 - Sender ID
    #[inline(always)]
    pub fn sender_id(&self) -> SenderIdR {
        SenderIdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:27 - RX data word 0
    #[inline(always)]
    pub fn data_0(&mut self) -> Data0W<'_, FifPullWrd0Spec> {
        Data0W::new(self, 0)
    }
    ///Bits 28:31 - Sender ID
    #[inline(always)]
    pub fn sender_id(&mut self) -> SenderIdW<'_, FifPullWrd0Spec> {
        SenderIdW::new(self, 28)
    }
}
/**RX data word 0

You can [`read`](crate::Reg::read) this register and get [`fif_pull_wrd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_pull_wrd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FifPullWrd0Spec;
impl crate::RegisterSpec for FifPullWrd0Spec {
    type Ux = u32;
}
///`read()` method returns [`fif_pull_wrd0::R`](R) reader structure
impl crate::Readable for FifPullWrd0Spec {}
///`write(|w| ..)` method takes [`fif_pull_wrd0::W`](W) writer structure
impl crate::Writable for FifPullWrd0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIF_PULL_WRD0 to value 0
impl crate::Resettable for FifPullWrd0Spec {}
