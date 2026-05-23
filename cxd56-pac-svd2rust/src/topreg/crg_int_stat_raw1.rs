///Register `CRG_INT_STAT_RAW1` reader
pub type R = crate::R<CrgIntStatRaw1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**TOPREG clock-ready raw interrupt status 1 (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_int_stat_raw1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrgIntStatRaw1Spec;
impl crate::RegisterSpec for CrgIntStatRaw1Spec {
    type Ux = u32;
}
///`read()` method returns [`crg_int_stat_raw1::R`](R) reader structure
impl crate::Readable for CrgIntStatRaw1Spec {}
///`reset()` method sets CRG_INT_STAT_RAW1 to value 0
impl crate::Resettable for CrgIntStatRaw1Spec {}
