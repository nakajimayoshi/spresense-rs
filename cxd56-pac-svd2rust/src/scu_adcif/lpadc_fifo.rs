///Register `LPADC_FIFO[%s]` reader
pub type R = crate::R<LpadcFifoSpec>;
///Field `DATA` reader - 10-bit ADC result, right-justified (raw bits\[15:6\] of MSB-aligned word; multiply by VDDA_LPADC/1023 for volts)
pub type DataR = crate::FieldReader<u16>;
impl R {
    ///Bits 6:15 - 10-bit ADC result, right-justified (raw bits\[15:6\] of MSB-aligned word; multiply by VDDA_LPADC/1023 for volts)
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
/**LPADC ch%s FIFO Read Port — dequeue one 10-bit sample (MSB-aligned in 16-bit word; DATA = bits\[15:6\] as 0..=1023)

You can [`read`](crate::Reg::read) this register and get [`lpadc_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpadcFifoSpec;
impl crate::RegisterSpec for LpadcFifoSpec {
    type Ux = u32;
}
///`read()` method returns [`lpadc_fifo::R`](R) reader structure
impl crate::Readable for LpadcFifoSpec {}
///`reset()` method sets LPADC_FIFO[%s] to value 0
impl crate::Resettable for LpadcFifoSpec {}
