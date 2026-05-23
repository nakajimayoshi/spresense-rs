///Register `CKSEL_SYSIOP_SUB` reader
pub type R = crate::R<CkselSysiopSubSpec>;
///Register `CKSEL_SYSIOP_SUB` writer
pub type W = crate::W<CkselSysiopSubSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SYSIOP sub-domain clock source select

You can [`read`](crate::Reg::read) this register and get [`cksel_sysiop_sub::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_sysiop_sub::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkselSysiopSubSpec;
impl crate::RegisterSpec for CkselSysiopSubSpec {
    type Ux = u32;
}
///`read()` method returns [`cksel_sysiop_sub::R`](R) reader structure
impl crate::Readable for CkselSysiopSubSpec {}
///`write(|w| ..)` method takes [`cksel_sysiop_sub::W`](W) writer structure
impl crate::Writable for CkselSysiopSubSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKSEL_SYSIOP_SUB to value 0
impl crate::Resettable for CkselSysiopSubSpec {}
