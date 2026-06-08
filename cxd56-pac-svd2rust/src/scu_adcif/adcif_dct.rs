///Register `ADCIF_DCT` reader
pub type R = crate::R<AdcifDctSpec>;
///Field `REVISION` reader - Revision code (always 0xADC1F000)
pub type RevisionR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Revision code (always 0xADC1F000)
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(self.bits)
    }
}
/**ADCIF register-map revision code (RO; reads 0xADC1F000)

You can [`read`](crate::Reg::read) this register and get [`adcif_dct::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcifDctSpec;
impl crate::RegisterSpec for AdcifDctSpec {
    type Ux = u32;
}
///`read()` method returns [`adcif_dct::R`](R) reader structure
impl crate::Readable for AdcifDctSpec {}
///`reset()` method sets ADCIF_DCT to value 0xadc1_f000
impl crate::Resettable for AdcifDctSpec {
    const RESET_VALUE: u32 = 0xadc1_f000;
}
