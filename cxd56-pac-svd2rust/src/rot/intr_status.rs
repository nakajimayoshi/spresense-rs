///Register `INTR_STATUS` reader
pub type R = crate::R<IntrStatusSpec>;
///Field `END_STS` reader - Done
pub type EndStsR = crate::BitReader;
///Field `RD_ERR_STS` reader - Read Error
pub type RdErrStsR = crate::BitReader;
///Field `WR_ERR_STS` reader - Write Error
pub type WrErrStsR = crate::BitReader;
impl R {
    ///Bit 0 - Done
    #[inline(always)]
    pub fn end_sts(&self) -> EndStsR {
        EndStsR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Read Error
    #[inline(always)]
    pub fn rd_err_sts(&self) -> RdErrStsR {
        RdErrStsR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Write Error
    #[inline(always)]
    pub fn wr_err_sts(&self) -> WrErrStsR {
        WrErrStsR::new(((self.bits >> 9) & 1) != 0)
    }
}
/**Interrupt Status

You can [`read`](crate::Reg::read) this register and get [`intr_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntrStatusSpec;
impl crate::RegisterSpec for IntrStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`intr_status::R`](R) reader structure
impl crate::Readable for IntrStatusSpec {}
///`reset()` method sets INTR_STATUS to value 0
impl crate::Resettable for IntrStatusSpec {}
