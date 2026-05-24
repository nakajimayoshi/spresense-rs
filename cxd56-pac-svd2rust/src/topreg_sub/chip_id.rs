///Register `CHIP_ID` reader
pub type R = crate::R<ChipIdSpec>;
///Field `ID` reader - Chip ID value
pub type IdR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Chip ID value
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits)
    }
}
/**Chip identification register (read-only)

You can [`read`](crate::Reg::read) this register and get [`chip_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ChipIdSpec;
impl crate::RegisterSpec for ChipIdSpec {
    type Ux = u32;
}
///`read()` method returns [`chip_id::R`](R) reader structure
impl crate::Readable for ChipIdSpec {}
///`reset()` method sets CHIP_ID to value 0
impl crate::Resettable for ChipIdSpec {}
