///Register `LPADC_D6` reader
pub type R = crate::R<LpadcD6Spec>;
///Register `LPADC_D6` writer
pub type W = crate::W<LpadcD6Spec>;
///Field `FIFO_WATERMARK` reader - FIFO watermark
pub type FifoWatermarkR = crate::FieldReader;
///Field `FIFO_WATERMARK` writer - FIFO watermark
pub type FifoWatermarkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DMA_HS_EN` reader - DMA handshake enable
pub type DmaHsEnR = crate::BitReader;
///Field `DMA_HS_EN` writer - DMA handshake enable
pub type DmaHsEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAMP_RATIO` reader - Frequency divider: 0=÷1 … 15=÷32768
pub type SampRatioR = crate::FieldReader;
///Field `SAMP_RATIO` writer - Frequency divider: 0=÷1 … 15=÷32768
pub type SampRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SAMP_RATIO2` reader - PWM sync enables \[8:0\]
pub type SampRatio2R = crate::FieldReader<u16>;
///Field `SAMP_RATIO2` writer - PWM sync enables \[8:0\]
pub type SampRatio2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:3 - FIFO watermark
    #[inline(always)]
    pub fn fifo_watermark(&self) -> FifoWatermarkR {
        FifoWatermarkR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 5 - DMA handshake enable
    #[inline(always)]
    pub fn dma_hs_en(&self) -> DmaHsEnR {
        DmaHsEnR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:11 - Frequency divider: 0=÷1 … 15=÷32768
    #[inline(always)]
    pub fn samp_ratio(&self) -> SampRatioR {
        SampRatioR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:20 - PWM sync enables \[8:0\]
    #[inline(always)]
    pub fn samp_ratio2(&self) -> SampRatio2R {
        SampRatio2R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 0:3 - FIFO watermark
    #[inline(always)]
    pub fn fifo_watermark(&mut self) -> FifoWatermarkW<'_, LpadcD6Spec> {
        FifoWatermarkW::new(self, 0)
    }
    ///Bit 5 - DMA handshake enable
    #[inline(always)]
    pub fn dma_hs_en(&mut self) -> DmaHsEnW<'_, LpadcD6Spec> {
        DmaHsEnW::new(self, 5)
    }
    ///Bits 8:11 - Frequency divider: 0=÷1 … 15=÷32768
    #[inline(always)]
    pub fn samp_ratio(&mut self) -> SampRatioW<'_, LpadcD6Spec> {
        SampRatioW::new(self, 8)
    }
    ///Bits 12:20 - PWM sync enables \[8:0\]
    #[inline(always)]
    pub fn samp_ratio2(&mut self) -> SampRatio2W<'_, LpadcD6Spec> {
        SampRatio2W::new(self, 12)
    }
}
/**LPADC ch3 (SEN_AIN5 / Arduino A3) sample-rate and watermark

You can [`read`](crate::Reg::read) this register and get [`lpadc_d6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpadcD6Spec;
impl crate::RegisterSpec for LpadcD6Spec {
    type Ux = u32;
}
///`read()` method returns [`lpadc_d6::R`](R) reader structure
impl crate::Readable for LpadcD6Spec {}
///`write(|w| ..)` method takes [`lpadc_d6::W`](W) writer structure
impl crate::Writable for LpadcD6Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPADC_D6 to value 0x08
impl crate::Resettable for LpadcD6Spec {
    const RESET_VALUE: u32 = 0x08;
}
