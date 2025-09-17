#[doc = "Register `DMACIntErrorStatus` reader"]
pub type R = crate::R<DmacintErrorStatusSpec>;
#[doc = "Field `IntErrorStatus` reader - Interrupt error status"]
pub type IntErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Interrupt error status"]
    #[inline(always)]
    pub fn int_error_status(&self) -> IntErrorStatusR {
        IntErrorStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacint_error_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacintErrorStatusSpec;
impl crate::RegisterSpec for DmacintErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacint_error_status::R`](R) reader structure"]
impl crate::Readable for DmacintErrorStatusSpec {}
#[doc = "`reset()` method sets DMACIntErrorStatus to value 0"]
impl crate::Resettable for DmacintErrorStatusSpec {}
