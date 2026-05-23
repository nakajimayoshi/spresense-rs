///Register `CRG_INT_STAT_RAW0` reader
pub type R = crate::R<CrgIntStatRaw0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**TOPREG clock-ready raw interrupt status 0 (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_int_stat_raw0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrgIntStatRaw0Spec;
impl crate::RegisterSpec for CrgIntStatRaw0Spec {
    type Ux = u32;
}
///`read()` method returns [`crg_int_stat_raw0::R`](R) reader structure
impl crate::Readable for CrgIntStatRaw0Spec {}
///`reset()` method sets CRG_INT_STAT_RAW0 to value 0
impl crate::Resettable for CrgIntStatRaw0Spec {}
