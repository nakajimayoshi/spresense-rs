///Register `CKDIV_PMU` reader
pub type R = crate::R<CkdivPmuSpec>;
///Register `CKDIV_PMU` writer
pub type W = crate::W<CkdivPmuSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**PMU clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_pmu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_pmu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkdivPmuSpec;
impl crate::RegisterSpec for CkdivPmuSpec {
    type Ux = u32;
}
///`read()` method returns [`ckdiv_pmu::R`](R) reader structure
impl crate::Readable for CkdivPmuSpec {}
///`write(|w| ..)` method takes [`ckdiv_pmu::W`](W) writer structure
impl crate::Writable for CkdivPmuSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKDIV_PMU to value 0
impl crate::Resettable for CkdivPmuSpec {}
