///Register `CKSEL_ROOT` reader
pub type R = crate::R<CkselRootSpec>;
///Register `CKSEL_ROOT` writer
pub type W = crate::W<CkselRootSpec>;
///Field `ENABLE_RF_PLL1` reader - Enable RF PLL1 as clock source
pub type EnableRfPll1R = crate::BitReader;
///Field `ENABLE_RF_PLL1` writer - Enable RF PLL1 as clock source
pub type EnableRfPll1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_SOURCE_SEL` reader - Enable clock source selection
pub type EnableSourceSelR = crate::BitReader;
///Field `ENABLE_SOURCE_SEL` writer - Enable clock source selection
pub type EnableSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STATUS_RTC` reader - RTC clock source status (read)
pub type StatusRtcR = crate::FieldReader;
///Field `STATUS_RTC` writer - RTC clock source status (read)
pub type StatusRtcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 4 - Enable RF PLL1 as clock source
    #[inline(always)]
    pub fn enable_rf_pll1(&self) -> EnableRfPll1R {
        EnableRfPll1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - Enable clock source selection
    #[inline(always)]
    pub fn enable_source_sel(&self) -> EnableSourceSelR {
        EnableSourceSelR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 30:31 - RTC clock source status (read)
    #[inline(always)]
    pub fn status_rtc(&self) -> StatusRtcR {
        StatusRtcR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bit 4 - Enable RF PLL1 as clock source
    #[inline(always)]
    pub fn enable_rf_pll1(&mut self) -> EnableRfPll1W<'_, CkselRootSpec> {
        EnableRfPll1W::new(self, 4)
    }
    ///Bit 16 - Enable clock source selection
    #[inline(always)]
    pub fn enable_source_sel(&mut self) -> EnableSourceSelW<'_, CkselRootSpec> {
        EnableSourceSelW::new(self, 16)
    }
    ///Bits 30:31 - RTC clock source status (read)
    #[inline(always)]
    pub fn status_rtc(&mut self) -> StatusRtcW<'_, CkselRootSpec> {
        StatusRtcW::new(self, 30)
    }
}
/**Root clock source select and RTC status

You can [`read`](crate::Reg::read) this register and get [`cksel_root::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_root::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkselRootSpec;
impl crate::RegisterSpec for CkselRootSpec {
    type Ux = u32;
}
///`read()` method returns [`cksel_root::R`](R) reader structure
impl crate::Readable for CkselRootSpec {}
///`write(|w| ..)` method takes [`cksel_root::W`](W) writer structure
impl crate::Writable for CkselRootSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKSEL_ROOT to value 0
impl crate::Resettable for CkselRootSpec {}
