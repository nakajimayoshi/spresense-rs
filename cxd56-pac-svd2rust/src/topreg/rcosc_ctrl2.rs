///Register `RCOSC_CTRL2` reader
pub type R = crate::R<RcoscCtrl2Spec>;
///Register `RCOSC_CTRL2` writer
pub type W = crate::W<RcoscCtrl2Spec>;
///Field `DISABLE_LOGICLK` reader - Disable RCOSC logic clock output (1 = disabled)
pub type DisableLogiclkR = crate::BitReader;
///Field `DISABLE_LOGICLK` writer - Disable RCOSC logic clock output (1 = disabled)
pub type DisableLogiclkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISABLE_SENSCLK` reader - Disable RCOSC sensor clock output (1 = disabled)
pub type DisableSensclkR = crate::BitReader;
///Field `DISABLE_SENSCLK` writer - Disable RCOSC sensor clock output (1 = disabled)
pub type DisableSensclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - Disable RCOSC logic clock output (1 = disabled)
    #[inline(always)]
    pub fn disable_logiclk(&self) -> DisableLogiclkR {
        DisableLogiclkR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Disable RCOSC sensor clock output (1 = disabled)
    #[inline(always)]
    pub fn disable_sensclk(&self) -> DisableSensclkR {
        DisableSensclkR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 13 - Disable RCOSC logic clock output (1 = disabled)
    #[inline(always)]
    pub fn disable_logiclk(&mut self) -> DisableLogiclkW<'_, RcoscCtrl2Spec> {
        DisableLogiclkW::new(self, 13)
    }
    ///Bit 14 - Disable RCOSC sensor clock output (1 = disabled)
    #[inline(always)]
    pub fn disable_sensclk(&mut self) -> DisableSensclkW<'_, RcoscCtrl2Spec> {
        DisableSensclkW::new(self, 14)
    }
}
/**RC oscillator control 2

You can [`read`](crate::Reg::read) this register and get [`rcosc_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcosc_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RcoscCtrl2Spec;
impl crate::RegisterSpec for RcoscCtrl2Spec {
    type Ux = u32;
}
///`read()` method returns [`rcosc_ctrl2::R`](R) reader structure
impl crate::Readable for RcoscCtrl2Spec {}
///`write(|w| ..)` method takes [`rcosc_ctrl2::W`](W) writer structure
impl crate::Writable for RcoscCtrl2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCOSC_CTRL2 to value 0
impl crate::Resettable for RcoscCtrl2Spec {}
