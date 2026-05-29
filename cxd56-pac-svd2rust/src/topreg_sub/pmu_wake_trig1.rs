///Register `PMU_WAKE_TRIG1` reader
pub type R = crate::R<PmuWakeTrig1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**GPIO interrupt masked status, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrig1Spec;
impl crate::RegisterSpec for PmuWakeTrig1Spec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig1::R`](R) reader structure
impl crate::Readable for PmuWakeTrig1Spec {}
///`reset()` method sets PMU_WAKE_TRIG1 to value 0
impl crate::Resettable for PmuWakeTrig1Spec {}
