///Register `I2S1_INT_MASK` reader
pub type R = crate::R<I2s1IntMaskSpec>;
///Register `I2S1_INT_MASK` writer
pub type W = crate::W<I2s1IntMaskSpec>;
///Field `DONE_I2SO` reader - Mask TX (out) DMA done interrupt
pub type DoneI2soR = crate::BitReader;
///Field `DONE_I2SO` writer - Mask TX (out) DMA done interrupt
pub type DoneI2soW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_I2SO` reader - Mask TX (out) DMA error interrupt
pub type ErrI2soR = crate::BitReader;
///Field `ERR_I2SO` writer - Mask TX (out) DMA error interrupt
pub type ErrI2soW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DONE_I2SI` reader - Mask RX (in) DMA done interrupt
pub type DoneI2siR = crate::BitReader;
///Field `DONE_I2SI` writer - Mask RX (in) DMA done interrupt
pub type DoneI2siW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_I2SI` reader - Mask RX (in) DMA error interrupt
pub type ErrI2siR = crate::BitReader;
///Field `ERR_I2SI` writer - Mask RX (in) DMA error interrupt
pub type ErrI2siW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMP_I2S` reader - Mask sample timing tick interrupt
pub type SmpI2sR = crate::BitReader;
///Field `SMP_I2S` writer - Mask sample timing tick interrupt
pub type SmpI2sW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMB_I2S` reader - Mask command buffer interrupt
pub type CmbI2sR = crate::BitReader;
///Field `CMB_I2S` writer - Mask command buffer interrupt
pub type CmbI2sW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOSTPMSK_I2S` reader - No-stop mask
pub type NostpmskI2sR = crate::BitReader;
///Field `NOSTPMSK_I2S` writer - No-stop mask
pub type NostpmskI2sW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRST_I2S` reader - I2S0 DMA soft reset
pub type SrstI2sR = crate::BitReader;
///Field `SRST_I2S` writer - I2S0 DMA soft reset
pub type SrstI2sW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Mask TX (out) DMA done interrupt
    #[inline(always)]
    pub fn done_i2so(&self) -> DoneI2soR {
        DoneI2soR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mask TX (out) DMA error interrupt
    #[inline(always)]
    pub fn err_i2so(&self) -> ErrI2soR {
        ErrI2soR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mask RX (in) DMA done interrupt
    #[inline(always)]
    pub fn done_i2si(&self) -> DoneI2siR {
        DoneI2siR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Mask RX (in) DMA error interrupt
    #[inline(always)]
    pub fn err_i2si(&self) -> ErrI2siR {
        ErrI2siR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Mask sample timing tick interrupt
    #[inline(always)]
    pub fn smp_i2s(&self) -> SmpI2sR {
        SmpI2sR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mask command buffer interrupt
    #[inline(always)]
    pub fn cmb_i2s(&self) -> CmbI2sR {
        CmbI2sR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 30 - No-stop mask
    #[inline(always)]
    pub fn nostpmsk_i2s(&self) -> NostpmskI2sR {
        NostpmskI2sR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - I2S0 DMA soft reset
    #[inline(always)]
    pub fn srst_i2s(&self) -> SrstI2sR {
        SrstI2sR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Mask TX (out) DMA done interrupt
    #[inline(always)]
    pub fn done_i2so(&mut self) -> DoneI2soW<'_, I2s1IntMaskSpec> {
        DoneI2soW::new(self, 0)
    }
    ///Bit 1 - Mask TX (out) DMA error interrupt
    #[inline(always)]
    pub fn err_i2so(&mut self) -> ErrI2soW<'_, I2s1IntMaskSpec> {
        ErrI2soW::new(self, 1)
    }
    ///Bit 2 - Mask RX (in) DMA done interrupt
    #[inline(always)]
    pub fn done_i2si(&mut self) -> DoneI2siW<'_, I2s1IntMaskSpec> {
        DoneI2siW::new(self, 2)
    }
    ///Bit 3 - Mask RX (in) DMA error interrupt
    #[inline(always)]
    pub fn err_i2si(&mut self) -> ErrI2siW<'_, I2s1IntMaskSpec> {
        ErrI2siW::new(self, 3)
    }
    ///Bit 4 - Mask sample timing tick interrupt
    #[inline(always)]
    pub fn smp_i2s(&mut self) -> SmpI2sW<'_, I2s1IntMaskSpec> {
        SmpI2sW::new(self, 4)
    }
    ///Bit 5 - Mask command buffer interrupt
    #[inline(always)]
    pub fn cmb_i2s(&mut self) -> CmbI2sW<'_, I2s1IntMaskSpec> {
        CmbI2sW::new(self, 5)
    }
    ///Bit 30 - No-stop mask
    #[inline(always)]
    pub fn nostpmsk_i2s(&mut self) -> NostpmskI2sW<'_, I2s1IntMaskSpec> {
        NostpmskI2sW::new(self, 30)
    }
    ///Bit 31 - I2S0 DMA soft reset
    #[inline(always)]
    pub fn srst_i2s(&mut self) -> SrstI2sW<'_, I2s1IntMaskSpec> {
        SrstI2sW::new(self, 31)
    }
}
/**I2S0 DMA interrupt mask (1=masked)

You can [`read`](crate::Reg::read) this register and get [`i2s1_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1IntMaskSpec;
impl crate::RegisterSpec for I2s1IntMaskSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_int_mask::R`](R) reader structure
impl crate::Readable for I2s1IntMaskSpec {}
///`write(|w| ..)` method takes [`i2s1_int_mask::W`](W) writer structure
impl crate::Writable for I2s1IntMaskSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S1_INT_MASK to value 0x3f
impl crate::Resettable for I2s1IntMaskSpec {
    const RESET_VALUE: u32 = 0x3f;
}
