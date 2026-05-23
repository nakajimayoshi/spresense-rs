///Register `PWD_RESET0` reader
pub type R = crate::R<PwdReset0Spec>;
///Register `PWD_RESET0` writer
pub type W = crate::W<PwdReset0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Per-domain power reset control

You can [`read`](crate::Reg::read) this register and get [`pwd_reset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd_reset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PwdReset0Spec;
impl crate::RegisterSpec for PwdReset0Spec {
    type Ux = u32;
}
///`read()` method returns [`pwd_reset0::R`](R) reader structure
impl crate::Readable for PwdReset0Spec {}
///`write(|w| ..)` method takes [`pwd_reset0::W`](W) writer structure
impl crate::Writable for PwdReset0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWD_RESET0 to value 0
impl crate::Resettable for PwdReset0Spec {}
