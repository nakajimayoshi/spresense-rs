///Register `CKSEL_SYSIOP` reader
pub type R = crate::R<CkselSysiopSpec>;
///Register `CKSEL_SYSIOP` writer
pub type W = crate::W<CkselSysiopSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SYSIOP clock source select

You can [`read`](crate::Reg::read) this register and get [`cksel_sysiop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_sysiop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkselSysiopSpec;
impl crate::RegisterSpec for CkselSysiopSpec {
    type Ux = u32;
}
///`read()` method returns [`cksel_sysiop::R`](R) reader structure
impl crate::Readable for CkselSysiopSpec {}
///`write(|w| ..)` method takes [`cksel_sysiop::W`](W) writer structure
impl crate::Writable for CkselSysiopSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKSEL_SYSIOP to value 0
impl crate::Resettable for CkselSysiopSpec {}
