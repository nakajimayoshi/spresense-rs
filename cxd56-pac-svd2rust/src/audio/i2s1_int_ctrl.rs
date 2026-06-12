///Register `I2S1_INT_CTRL` reader
pub type R = crate::R<I2s1IntCtrlSpec>;
///Register `I2S1_INT_CTRL` writer
pub type W = crate::W<I2s1IntCtrlSpec>;
///Field `DONE_I2SO` reader - TX (out) DMA done
pub type DoneI2soR = crate::BitReader;
///Field `DONE_I2SO` writer - TX (out) DMA done
pub type DoneI2soW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_I2SO` reader - TX (out) DMA error
pub type ErrI2soR = crate::BitReader;
///Field `ERR_I2SO` writer - TX (out) DMA error
pub type ErrI2soW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DONE_I2SI` reader - RX (in) DMA done
pub type DoneI2siR = crate::BitReader;
///Field `DONE_I2SI` writer - RX (in) DMA done
pub type DoneI2siW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_I2SI` reader - RX (in) DMA error
pub type ErrI2siR = crate::BitReader;
///Field `ERR_I2SI` writer - RX (in) DMA error
pub type ErrI2siW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMP_I2S` reader - Sample timing tick
pub type SmpI2sR = crate::BitReader;
///Field `SMP_I2S` writer - Sample timing tick
pub type SmpI2sW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMB_I2S` reader - Command buffer event
pub type CmbI2sR = crate::BitReader;
///Field `CMB_I2S` writer - Command buffer event
pub type CmbI2sW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TX (out) DMA done
    #[inline(always)]
    pub fn done_i2so(&self) -> DoneI2soR {
        DoneI2soR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX (out) DMA error
    #[inline(always)]
    pub fn err_i2so(&self) -> ErrI2soR {
        ErrI2soR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RX (in) DMA done
    #[inline(always)]
    pub fn done_i2si(&self) -> DoneI2siR {
        DoneI2siR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RX (in) DMA error
    #[inline(always)]
    pub fn err_i2si(&self) -> ErrI2siR {
        ErrI2siR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Sample timing tick
    #[inline(always)]
    pub fn smp_i2s(&self) -> SmpI2sR {
        SmpI2sR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Command buffer event
    #[inline(always)]
    pub fn cmb_i2s(&self) -> CmbI2sR {
        CmbI2sR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TX (out) DMA done
    #[inline(always)]
    pub fn done_i2so(&mut self) -> DoneI2soW<'_, I2s1IntCtrlSpec> {
        DoneI2soW::new(self, 0)
    }
    ///Bit 1 - TX (out) DMA error
    #[inline(always)]
    pub fn err_i2so(&mut self) -> ErrI2soW<'_, I2s1IntCtrlSpec> {
        ErrI2soW::new(self, 1)
    }
    ///Bit 2 - RX (in) DMA done
    #[inline(always)]
    pub fn done_i2si(&mut self) -> DoneI2siW<'_, I2s1IntCtrlSpec> {
        DoneI2siW::new(self, 2)
    }
    ///Bit 3 - RX (in) DMA error
    #[inline(always)]
    pub fn err_i2si(&mut self) -> ErrI2siW<'_, I2s1IntCtrlSpec> {
        ErrI2siW::new(self, 3)
    }
    ///Bit 4 - Sample timing tick
    #[inline(always)]
    pub fn smp_i2s(&mut self) -> SmpI2sW<'_, I2s1IntCtrlSpec> {
        SmpI2sW::new(self, 4)
    }
    ///Bit 5 - Command buffer event
    #[inline(always)]
    pub fn cmb_i2s(&mut self) -> CmbI2sW<'_, I2s1IntCtrlSpec> {
        CmbI2sW::new(self, 5)
    }
}
/**I2S0 DMA interrupt status (write 1 to clear)

You can [`read`](crate::Reg::read) this register and get [`i2s1_int_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_int_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1IntCtrlSpec;
impl crate::RegisterSpec for I2s1IntCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_int_ctrl::R`](R) reader structure
impl crate::Readable for I2s1IntCtrlSpec {}
///`write(|w| ..)` method takes [`i2s1_int_ctrl::W`](W) writer structure
impl crate::Writable for I2s1IntCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S1_INT_CTRL to value 0
impl crate::Resettable for I2s1IntCtrlSpec {}
