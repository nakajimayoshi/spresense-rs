///Register `HPADC_FIFO[%s]` reader
pub type R = crate::R<HpadcFifoSpec>;
///Field `DATA` reader - 10-bit HPADC sample, right-justified
pub type DataR = crate::FieldReader<u16>;
impl R {
    ///Bits 6:15 - 10-bit HPADC sample, right-justified
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
/**HPADC ch%s FIFO Read Port (SEN_AIN0/1, Arduino A4/A5)

You can [`read`](crate::Reg::read) this register and get [`hpadc_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HpadcFifoSpec;
impl crate::RegisterSpec for HpadcFifoSpec {
    type Ux = u32;
}
///`read()` method returns [`hpadc_fifo::R`](R) reader structure
impl crate::Readable for HpadcFifoSpec {}
///`reset()` method sets HPADC_FIFO[%s] to value 0
impl crate::Resettable for HpadcFifoSpec {}
