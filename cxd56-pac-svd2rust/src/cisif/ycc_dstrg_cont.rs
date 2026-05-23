///Register `YCC_DSTRG_CONT` reader
pub type R = crate::R<YccDstrgContSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**YCC data frame memory storage size counter

You can [`read`](crate::Reg::read) this register and get [`ycc_dstrg_cont::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct YccDstrgContSpec;
impl crate::RegisterSpec for YccDstrgContSpec {
    type Ux = u16;
}
///`read()` method returns [`ycc_dstrg_cont::R`](R) reader structure
impl crate::Readable for YccDstrgContSpec {}
///`reset()` method sets YCC_DSTRG_CONT to value 0
impl crate::Resettable for YccDstrgContSpec {}
