///Register `PMU_WAKE_TRIG_EN1` reader
pub type R = crate::R<PmuWakeTrigEn1Spec>;
///Register `PMU_WAKE_TRIG_EN1` writer
pub type W = crate::W<PmuWakeTrigEn1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Positive wake-trigger enable, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrigEn1Spec;
impl crate::RegisterSpec for PmuWakeTrigEn1Spec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig_en1::R`](R) reader structure
impl crate::Readable for PmuWakeTrigEn1Spec {}
///`write(|w| ..)` method takes [`pmu_wake_trig_en1::W`](W) writer structure
impl crate::Writable for PmuWakeTrigEn1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMU_WAKE_TRIG_EN1 to value 0
impl crate::Resettable for PmuWakeTrigEn1Spec {}
