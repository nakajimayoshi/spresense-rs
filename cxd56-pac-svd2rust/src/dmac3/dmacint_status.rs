///Register `DMACIntStatus` reader
pub type R = crate::R<DmacintStatusSpec>;
///Field `IntStatus` reader - Status of the DMA interrupts after masking
pub type IntStatusR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Status of the DMA interrupts after masking
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 0xff) as u8)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacint_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacintStatusSpec;
impl crate::RegisterSpec for DmacintStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacint_status::R`](R) reader structure
impl crate::Readable for DmacintStatusSpec {}
///`reset()` method sets DMACIntStatus to value 0
impl crate::Resettable for DmacintStatusSpec {}
