///Register `LR_SWAP` reader
pub type R = crate::R<LrSwapSpec>;
///Register `LR_SWAP` writer
pub type W = crate::W<LrSwapSpec>;
///Field `LR_SWAP1` reader - I2S0 (SD1) L/R swap
pub type LrSwap1R = crate::BitReader;
///Field `LR_SWAP1` writer - I2S0 (SD1) L/R swap
pub type LrSwap1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LR_SWAP2` reader - I2S1 (SD2) L/R swap
pub type LrSwap2R = crate::BitReader;
///Field `LR_SWAP2` writer - I2S1 (SD2) L/R swap
pub type LrSwap2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2S0 (SD1) L/R swap
    #[inline(always)]
    pub fn lr_swap1(&self) -> LrSwap1R {
        LrSwap1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2S1 (SD2) L/R swap
    #[inline(always)]
    pub fn lr_swap2(&self) -> LrSwap2R {
        LrSwap2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I2S0 (SD1) L/R swap
    #[inline(always)]
    pub fn lr_swap1(&mut self) -> LrSwap1W<'_, LrSwapSpec> {
        LrSwap1W::new(self, 0)
    }
    ///Bit 1 - I2S1 (SD2) L/R swap
    #[inline(always)]
    pub fn lr_swap2(&mut self) -> LrSwap2W<'_, LrSwapSpec> {
        LrSwap2W::new(self, 1)
    }
}
/**I2S L/R channel swap (set together with left-justified format)

You can [`read`](crate::Reg::read) this register and get [`lr_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LrSwapSpec;
impl crate::RegisterSpec for LrSwapSpec {
    type Ux = u32;
}
///`read()` method returns [`lr_swap::R`](R) reader structure
impl crate::Readable for LrSwapSpec {}
///`write(|w| ..)` method takes [`lr_swap::W`](W) writer structure
impl crate::Writable for LrSwapSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LR_SWAP to value 0
impl crate::Resettable for LrSwapSpec {}
