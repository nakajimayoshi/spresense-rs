///Register `CRG_INT_STAT_MSK1` reader
pub type R = crate::R<CrgIntStatMsk1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**TOPREG clock-ready masked interrupt status 1 (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_int_stat_msk1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrgIntStatMsk1Spec;
impl crate::RegisterSpec for CrgIntStatMsk1Spec {
    type Ux = u32;
}
///`read()` method returns [`crg_int_stat_msk1::R`](R) reader structure
impl crate::Readable for CrgIntStatMsk1Spec {}
///`reset()` method sets CRG_INT_STAT_MSK1 to value 0
impl crate::Resettable for CrgIntStatMsk1Spec {}
