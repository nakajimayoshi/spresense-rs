#[doc = "Register `FIF_PUSH_WRD0` reader"]
pub type R = crate::R<FifPushWrd0Spec>;
#[doc = "Register `FIF_PUSH_WRD0` writer"]
pub type W = crate::W<FifPushWrd0Spec>;
#[doc = "Field `DATA_0` reader - TX data word 0"]
pub type Data0R = crate::FieldReader<u32>;
#[doc = "Field `DATA_0` writer - TX data word 0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `RECEIVER_ID` reader - Target ID"]
pub type ReceiverIdR = crate::FieldReader;
#[doc = "Field `RECEIVER_ID` writer - Target ID"]
pub type ReceiverIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:27 - TX data word 0"]
    #[inline(always)]
    pub fn data_0(&self) -> Data0R {
        Data0R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Target ID"]
    #[inline(always)]
    pub fn receiver_id(&self) -> ReceiverIdR {
        ReceiverIdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - TX data word 0"]
    #[inline(always)]
    pub fn data_0(&mut self) -> Data0W<'_, FifPushWrd0Spec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 28:31 - Target ID"]
    #[inline(always)]
    pub fn receiver_id(&mut self) -> ReceiverIdW<'_, FifPushWrd0Spec> {
        ReceiverIdW::new(self, 28)
    }
}
#[doc = "TX data word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_push_wrd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_push_wrd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifPushWrd0Spec;
impl crate::RegisterSpec for FifPushWrd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fif_push_wrd0::R`](R) reader structure"]
impl crate::Readable for FifPushWrd0Spec {}
#[doc = "`write(|w| ..)` method takes [`fif_push_wrd0::W`](W) writer structure"]
impl crate::Writable for FifPushWrd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIF_PUSH_WRD0 to value 0"]
impl crate::Resettable for FifPushWrd0Spec {}
