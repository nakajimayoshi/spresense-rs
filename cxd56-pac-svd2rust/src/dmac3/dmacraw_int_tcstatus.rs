///Register `DMACRawIntTCStatus` reader
pub type R = crate::R<DmacrawIntTcstatusSpec>;
///Field `RawIntTCStatus` reader - Status of the terminal count interrupt prior to masking
pub type RawIntTcstatusR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Status of the terminal count interrupt prior to masking
    #[inline(always)]
    pub fn raw_int_tcstatus(&self) -> RawIntTcstatusR {
        RawIntTcstatusR::new((self.bits & 0xff) as u8)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacraw_int_tcstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacrawIntTcstatusSpec;
impl crate::RegisterSpec for DmacrawIntTcstatusSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacraw_int_tcstatus::R`](R) reader structure
impl crate::Readable for DmacrawIntTcstatusSpec {}
///`reset()` method sets DMACRawIntTCStatus to value 0
impl crate::Resettable for DmacrawIntTcstatusSpec {}
