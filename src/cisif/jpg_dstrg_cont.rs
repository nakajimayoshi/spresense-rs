#[doc = "Register `JPG_DSTRG_CONT` reader"]
pub type R = crate::R<JpgDstrgContSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "JPEG data frame memory storage size counter\n\nYou can [`read`](crate::Reg::read) this register and get [`jpg_dstrg_cont::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JpgDstrgContSpec;
impl crate::RegisterSpec for JpgDstrgContSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`jpg_dstrg_cont::R`](R) reader structure"]
impl crate::Readable for JpgDstrgContSpec {}
#[doc = "`reset()` method sets JPG_DSTRG_CONT to value 0"]
impl crate::Resettable for JpgDstrgContSpec {}
