///Register `CRG_MON` reader
pub type R = crate::R<CrgMonSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**CRG monitor status (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_mon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrgMonSpec;
impl crate::RegisterSpec for CrgMonSpec {
    type Ux = u32;
}
///`read()` method returns [`crg_mon::R`](R) reader structure
impl crate::Readable for CrgMonSpec {}
///`reset()` method sets CRG_MON to value 0
impl crate::Resettable for CrgMonSpec {}
