///Register `DMACIntTCStatus` reader
pub type R = crate::R<DmacintTcstatusSpec>;
///Field `IntTCStatus` reader - Interrupt terminal count request status
pub type IntTcstatusR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Interrupt terminal count request status
    #[inline(always)]
    pub fn int_tcstatus(&self) -> IntTcstatusR {
        IntTcstatusR::new((self.bits & 0xff) as u8)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacint_tcstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacintTcstatusSpec;
impl crate::RegisterSpec for DmacintTcstatusSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacint_tcstatus::R`](R) reader structure
impl crate::Readable for DmacintTcstatusSpec {}
///`reset()` method sets DMACIntTCStatus to value 0
impl crate::Resettable for DmacintTcstatusSpec {}
