///Register `PMU_WAKE_TRIG_CPUINTSEL2` reader
pub type R = crate::R<PmuWakeTrigCpuintsel2Spec>;
///Register `PMU_WAKE_TRIG_CPUINTSEL2` writer
pub type W = crate::W<PmuWakeTrigCpuintsel2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CPU interrupt route select, second bank (unused by GPIO driver)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_cpuintsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_cpuintsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrigCpuintsel2Spec;
impl crate::RegisterSpec for PmuWakeTrigCpuintsel2Spec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig_cpuintsel2::R`](R) reader structure
impl crate::Readable for PmuWakeTrigCpuintsel2Spec {}
///`write(|w| ..)` method takes [`pmu_wake_trig_cpuintsel2::W`](W) writer structure
impl crate::Writable for PmuWakeTrigCpuintsel2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMU_WAKE_TRIG_CPUINTSEL2 to value 0
impl crate::Resettable for PmuWakeTrigCpuintsel2Spec {}
