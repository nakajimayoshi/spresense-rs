#[doc = "Register `FIF_PULL_WRD1` reader"]
pub type R = crate::R<FifPullWrd1Spec>;
#[doc = "Register `FIF_PULL_WRD1` writer"]
pub type W = crate::W<FifPullWrd1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RX data word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_pull_wrd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_pull_wrd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifPullWrd1Spec;
impl crate::RegisterSpec for FifPullWrd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fif_pull_wrd1::R`](R) reader structure"]
impl crate::Readable for FifPullWrd1Spec {}
#[doc = "`write(|w| ..)` method takes [`fif_pull_wrd1::W`](W) writer structure"]
impl crate::Writable for FifPullWrd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIF_PULL_WRD1 to value 0"]
impl crate::Resettable for FifPullWrd1Spec {}
