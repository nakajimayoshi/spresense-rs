///Register `FIF_PUSH_WRD1` reader
pub type R = crate::R<FifPushWrd1Spec>;
///Register `FIF_PUSH_WRD1` writer
pub type W = crate::W<FifPushWrd1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**TX data word 1

You can [`read`](crate::Reg::read) this register and get [`fif_push_wrd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_push_wrd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FifPushWrd1Spec;
impl crate::RegisterSpec for FifPushWrd1Spec {
    type Ux = u32;
}
///`read()` method returns [`fif_push_wrd1::R`](R) reader structure
impl crate::Readable for FifPushWrd1Spec {}
///`write(|w| ..)` method takes [`fif_push_wrd1::W`](W) writer structure
impl crate::Writable for FifPushWrd1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIF_PUSH_WRD1 to value 0
impl crate::Resettable for FifPushWrd1Spec {}
