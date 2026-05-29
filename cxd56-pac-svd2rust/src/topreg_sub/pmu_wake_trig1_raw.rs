///Register `PMU_WAKE_TRIG1_RAW` reader
pub type R = crate::R<PmuWakeTrig1RawSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**GPIO interrupt raw status, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig1_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrig1RawSpec;
impl crate::RegisterSpec for PmuWakeTrig1RawSpec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig1_raw::R`](R) reader structure
impl crate::Readable for PmuWakeTrig1RawSpec {}
///`reset()` method sets PMU_WAKE_TRIG1_RAW to value 0
impl crate::Resettable for PmuWakeTrig1RawSpec {}
