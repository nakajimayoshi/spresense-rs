///Register `PMU_CORE_CKEN` reader
pub type R = crate::R<PmuCoreCkenSpec>;
///Register `PMU_CORE_CKEN` writer
pub type W = crate::W<PmuCoreCkenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**PMU core clock enable

You can [`read`](crate::Reg::read) this register and get [`pmu_core_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_core_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuCoreCkenSpec;
impl crate::RegisterSpec for PmuCoreCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`pmu_core_cken::R`](R) reader structure
impl crate::Readable for PmuCoreCkenSpec {}
///`write(|w| ..)` method takes [`pmu_core_cken::W`](W) writer structure
impl crate::Writable for PmuCoreCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMU_CORE_CKEN to value 0
impl crate::Resettable for PmuCoreCkenSpec {}
