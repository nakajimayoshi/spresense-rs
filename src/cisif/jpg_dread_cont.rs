#[doc = "Register `JPG_DREAD_CONT` reader"]
pub type R = crate::R<JpgDreadContSpec>;
#[doc = "Register `JPG_DREAD_CONT` writer"]
pub type W = crate::W<JpgDreadContSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "JPEG data frame memory read counter\n\nYou can [`read`](crate::Reg::read) this register and get [`jpg_dread_cont::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpg_dread_cont::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JpgDreadContSpec;
impl crate::RegisterSpec for JpgDreadContSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jpg_dread_cont::R`](R) reader structure"]
impl crate::Readable for JpgDreadContSpec {}
#[doc = "`write(|w| ..)` method takes [`jpg_dread_cont::W`](W) writer structure"]
impl crate::Writable for JpgDreadContSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JPG_DREAD_CONT to value 0"]
impl crate::Resettable for JpgDreadContSpec {}
