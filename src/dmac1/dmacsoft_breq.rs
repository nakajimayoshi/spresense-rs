#[doc = "Register `DMACSoftBReq` reader"]
pub type R = crate::R<DmacsoftBreqSpec>;
#[doc = "Register `DMACSoftBReq` writer"]
pub type W = crate::W<DmacsoftBreqSpec>;
#[doc = "Field `SoftBReq` reader - Software burst request"]
pub type SoftBreqR = crate::FieldReader<u16>;
#[doc = "Field `SoftBReq` writer - Software burst request"]
pub type SoftBreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Software burst request"]
    #[inline(always)]
    pub fn soft_breq(&self) -> SoftBreqR {
        SoftBreqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Software burst request"]
    #[inline(always)]
    pub fn soft_breq(&mut self) -> SoftBreqW<'_, DmacsoftBreqSpec> {
        SoftBreqW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsoft_breq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_breq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacsoftBreqSpec;
impl crate::RegisterSpec for DmacsoftBreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacsoft_breq::R`](R) reader structure"]
impl crate::Readable for DmacsoftBreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacsoft_breq::W`](W) writer structure"]
impl crate::Writable for DmacsoftBreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACSoftBReq to value 0"]
impl crate::Resettable for DmacsoftBreqSpec {}
