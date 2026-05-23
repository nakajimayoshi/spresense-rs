///Register `CKSEL_PMU` reader
pub type R = crate::R<CkselPmuSpec>;
///Register `CKSEL_PMU` writer
pub type W = crate::W<CkselPmuSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**PMU clock source select

You can [`read`](crate::Reg::read) this register and get [`cksel_pmu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_pmu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkselPmuSpec;
impl crate::RegisterSpec for CkselPmuSpec {
    type Ux = u32;
}
///`read()` method returns [`cksel_pmu::R`](R) reader structure
impl crate::Readable for CkselPmuSpec {}
///`write(|w| ..)` method takes [`cksel_pmu::W`](W) writer structure
impl crate::Writable for CkselPmuSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKSEL_PMU to value 0
impl crate::Resettable for CkselPmuSpec {}
