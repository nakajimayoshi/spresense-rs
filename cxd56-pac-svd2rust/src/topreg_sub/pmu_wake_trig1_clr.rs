///Register `PMU_WAKE_TRIG1_CLR` reader
pub type R = crate::R<PmuWakeTrig1ClrSpec>;
///Register `PMU_WAKE_TRIG1_CLR` writer
pub type W = crate::W<PmuWakeTrig1ClrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**GPIO interrupt clear register, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig1_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig1_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrig1ClrSpec;
impl crate::RegisterSpec for PmuWakeTrig1ClrSpec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig1_clr::R`](R) reader structure
impl crate::Readable for PmuWakeTrig1ClrSpec {}
///`write(|w| ..)` method takes [`pmu_wake_trig1_clr::W`](W) writer structure
impl crate::Writable for PmuWakeTrig1ClrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMU_WAKE_TRIG1_CLR to value 0
impl crate::Resettable for PmuWakeTrig1ClrSpec {}
