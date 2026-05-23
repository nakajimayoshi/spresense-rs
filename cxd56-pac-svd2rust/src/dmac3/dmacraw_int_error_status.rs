///Register `DMACRawIntErrorStatus` reader
pub type R = crate::R<DmacrawIntErrorStatusSpec>;
///Field `RawIntErrorStatus` reader - Status of the error interrupt prior to masking
pub type RawIntErrorStatusR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Status of the error interrupt prior to masking
    #[inline(always)]
    pub fn raw_int_error_status(&self) -> RawIntErrorStatusR {
        RawIntErrorStatusR::new((self.bits & 0xff) as u8)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacraw_int_error_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacrawIntErrorStatusSpec;
impl crate::RegisterSpec for DmacrawIntErrorStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacraw_int_error_status::R`](R) reader structure
impl crate::Readable for DmacrawIntErrorStatusSpec {}
///`reset()` method sets DMACRawIntErrorStatus to value 0
impl crate::Resettable for DmacrawIntErrorStatusSpec {}
