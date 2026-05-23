///Register `SYS_PLL_CTRL1` reader
pub type R = crate::R<SysPllCtrl1Spec>;
///Register `SYS_PLL_CTRL1` writer
pub type W = crate::W<SysPllCtrl1Spec>;
///Field `ENABLE_GPADCLK` reader - Enable GP ADC clock from PLL
pub type EnableGpadclkR = crate::BitReader;
///Field `ENABLE_GPADCLK` writer - Enable GP ADC clock from PLL
pub type EnableGpadclkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_DSPCLK` reader - Enable DSP clock from PLL
pub type EnableDspclkR = crate::BitReader;
///Field `ENABLE_DSPCLK` writer - Enable DSP clock from PLL
pub type EnableDspclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Enable GP ADC clock from PLL
    #[inline(always)]
    pub fn enable_gpadclk(&self) -> EnableGpadclkR {
        EnableGpadclkR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Enable DSP clock from PLL
    #[inline(always)]
    pub fn enable_dspclk(&self) -> EnableDspclkR {
        EnableDspclkR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Enable GP ADC clock from PLL
    #[inline(always)]
    pub fn enable_gpadclk(&mut self) -> EnableGpadclkW<'_, SysPllCtrl1Spec> {
        EnableGpadclkW::new(self, 1)
    }
    ///Bit 3 - Enable DSP clock from PLL
    #[inline(always)]
    pub fn enable_dspclk(&mut self) -> EnableDspclkW<'_, SysPllCtrl1Spec> {
        EnableDspclkW::new(self, 3)
    }
}
/**System PLL control 1

You can [`read`](crate::Reg::read) this register and get [`sys_pll_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_pll_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SysPllCtrl1Spec;
impl crate::RegisterSpec for SysPllCtrl1Spec {
    type Ux = u32;
}
///`read()` method returns [`sys_pll_ctrl1::R`](R) reader structure
impl crate::Readable for SysPllCtrl1Spec {}
///`write(|w| ..)` method takes [`sys_pll_ctrl1::W`](W) writer structure
impl crate::Writable for SysPllCtrl1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYS_PLL_CTRL1 to value 0
impl crate::Resettable for SysPllCtrl1Spec {}
