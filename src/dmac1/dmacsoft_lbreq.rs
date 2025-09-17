#[doc = "Register `DMACSoftLBReq` reader"]
pub type R = crate::R<DmacsoftLbreqSpec>;
#[doc = "Register `DMACSoftLBReq` writer"]
pub type W = crate::W<DmacsoftLbreqSpec>;
#[doc = "Field `SoftLBReq` reader - Software last burst request"]
pub type SoftLbreqR = crate::FieldReader<u16>;
#[doc = "Field `SoftLBReq` writer - Software last burst request"]
pub type SoftLbreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Software last burst request"]
    #[inline(always)]
    pub fn soft_lbreq(&self) -> SoftLbreqR {
        SoftLbreqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Software last burst request"]
    #[inline(always)]
    pub fn soft_lbreq(&mut self) -> SoftLbreqW<'_, DmacsoftLbreqSpec> {
        SoftLbreqW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsoft_lbreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_lbreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacsoftLbreqSpec;
impl crate::RegisterSpec for DmacsoftLbreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacsoft_lbreq::R`](R) reader structure"]
impl crate::Readable for DmacsoftLbreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacsoft_lbreq::W`](W) writer structure"]
impl crate::Writable for DmacsoftLbreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACSoftLBReq to value 0"]
impl crate::Resettable for DmacsoftLbreqSpec {}
