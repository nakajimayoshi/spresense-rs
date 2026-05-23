///Register `CKSEL_SCU` reader
pub type R = crate::R<CkselScuSpec>;
///Register `CKSEL_SCU` writer
pub type W = crate::W<CkselScuSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SCU clock source select

You can [`read`](crate::Reg::read) this register and get [`cksel_scu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_scu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkselScuSpec;
impl crate::RegisterSpec for CkselScuSpec {
    type Ux = u32;
}
///`read()` method returns [`cksel_scu::R`](R) reader structure
impl crate::Readable for CkselScuSpec {}
///`write(|w| ..)` method takes [`cksel_scu::W`](W) writer structure
impl crate::Writable for CkselScuSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKSEL_SCU to value 0
impl crate::Resettable for CkselScuSpec {}
