#[doc = "Register `SSPCPSR` reader"]
pub type R = crate::R<SspcpsrSpec>;
#[doc = "Register `SSPCPSR` writer"]
pub type W = crate::W<SspcpsrSpec>;
#[doc = "Field `CPSDVSR` reader - Clock prescale divisor"]
pub type CpsdvsrR = crate::FieldReader;
#[doc = "Field `CPSDVSR` writer - Clock prescale divisor"]
pub type CpsdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock prescale divisor"]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CpsdvsrR {
        CpsdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescale divisor"]
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CpsdvsrW<'_, SspcpsrSpec> {
        CpsdvsrW::new(self, 0)
    }
}
#[doc = "Clock prescale register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspcpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspcpsrSpec;
impl crate::RegisterSpec for SspcpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspcpsr::R`](R) reader structure"]
impl crate::Readable for SspcpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sspcpsr::W`](W) writer structure"]
impl crate::Writable for SspcpsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSPCPSR to value 0"]
impl crate::Resettable for SspcpsrSpec {}
