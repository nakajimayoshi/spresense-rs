#[doc = "Register `DMACC0Configuration` reader"]
pub type R = crate::R<Dmacc0configurationSpec>;
#[doc = "Register `DMACC0Configuration` writer"]
pub type W = crate::W<Dmacc0configurationSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0configuration::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0configuration::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmacc0configurationSpec;
impl crate::RegisterSpec for Dmacc0configurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacc0configuration::R`](R) reader structure"]
impl crate::Readable for Dmacc0configurationSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacc0configuration::W`](W) writer structure"]
impl crate::Writable for Dmacc0configurationSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACC0Configuration to value 0"]
impl crate::Resettable for Dmacc0configurationSpec {}
