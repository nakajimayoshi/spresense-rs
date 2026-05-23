///Register `CRG_INT_CLR1` reader
pub type R = crate::R<CrgIntClr1Spec>;
///Register `CRG_INT_CLR1` writer
pub type W = crate::W<CrgIntClr1Spec>;
///Field `CK_CPU_BUS` reader - CPU bus clock ready
pub type CkCpuBusR = crate::BitReader;
///Field `CK_CPU_BUS` writer - CPU bus clock ready
pub type CkCpuBusW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_CPU_BUS_TO` reader - CPU bus clock timeout
pub type CkCpuBusToR = crate::BitReader;
///Field `CK_CPU_BUS_TO` writer - CPU bus clock timeout
pub type CkCpuBusToW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_RFPLL1` reader - RF PLL1 clock ready
pub type CkRfpll1R = crate::BitReader;
///Field `CK_RFPLL1` writer - RF PLL1 clock ready
pub type CkRfpll1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_RFPLL1_TO` reader - RF PLL1 clock timeout
pub type CkRfpll1ToR = crate::BitReader;
///Field `CK_RFPLL1_TO` writer - RF PLL1 clock timeout
pub type CkRfpll1ToW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_RTC_PRE` reader - RTC pre-divider clock ready
pub type CkRtcPreR = crate::BitReader;
///Field `CK_RTC_PRE` writer - RTC pre-divider clock ready
pub type CkRtcPreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_RTC_PRE_TO` reader - RTC pre-divider clock timeout
pub type CkRtcPreToR = crate::BitReader;
///Field `CK_RTC_PRE_TO` writer - RTC pre-divider clock timeout
pub type CkRtcPreToW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_APP_PRE` reader - APP pre-divider clock ready
pub type CkAppPreR = crate::BitReader;
///Field `CK_APP_PRE` writer - APP pre-divider clock ready
pub type CkAppPreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_APP_PRE_TO` reader - APP pre-divider clock timeout
pub type CkAppPreToR = crate::BitReader;
///Field `CK_APP_PRE_TO` writer - APP pre-divider clock timeout
pub type CkAppPreToW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SEL_SP` reader - SP clock select ready
pub type CkSelSpR = crate::BitReader;
///Field `CK_SEL_SP` writer - SP clock select ready
pub type CkSelSpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SEL_SP_TO` reader - SP clock select timeout
pub type CkSelSpToR = crate::BitReader;
///Field `CK_SEL_SP_TO` writer - SP clock select timeout
pub type CkSelSpToW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SEL_RO_RTC` reader - RO RTC clock select ready
pub type CkSelRoRtcR = crate::BitReader;
///Field `CK_SEL_RO_RTC` writer - RO RTC clock select ready
pub type CkSelRoRtcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRG_FREQFIX_ERR` reader - Frequency fix error
pub type CrgFreqfixErrR = crate::BitReader;
///Field `CRG_FREQFIX_ERR` writer - Frequency fix error
pub type CrgFreqfixErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU bus clock ready
    #[inline(always)]
    pub fn ck_cpu_bus(&self) -> CkCpuBusR {
        CkCpuBusR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU bus clock timeout
    #[inline(always)]
    pub fn ck_cpu_bus_to(&self) -> CkCpuBusToR {
        CkCpuBusToR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RF PLL1 clock ready
    #[inline(always)]
    pub fn ck_rfpll1(&self) -> CkRfpll1R {
        CkRfpll1R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RF PLL1 clock timeout
    #[inline(always)]
    pub fn ck_rfpll1_to(&self) -> CkRfpll1ToR {
        CkRfpll1ToR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RTC pre-divider clock ready
    #[inline(always)]
    pub fn ck_rtc_pre(&self) -> CkRtcPreR {
        CkRtcPreR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RTC pre-divider clock timeout
    #[inline(always)]
    pub fn ck_rtc_pre_to(&self) -> CkRtcPreToR {
        CkRtcPreToR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - APP pre-divider clock ready
    #[inline(always)]
    pub fn ck_app_pre(&self) -> CkAppPreR {
        CkAppPreR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - APP pre-divider clock timeout
    #[inline(always)]
    pub fn ck_app_pre_to(&self) -> CkAppPreToR {
        CkAppPreToR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SP clock select ready
    #[inline(always)]
    pub fn ck_sel_sp(&self) -> CkSelSpR {
        CkSelSpR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SP clock select timeout
    #[inline(always)]
    pub fn ck_sel_sp_to(&self) -> CkSelSpToR {
        CkSelSpToR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RO RTC clock select ready
    #[inline(always)]
    pub fn ck_sel_ro_rtc(&self) -> CkSelRoRtcR {
        CkSelRoRtcR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Frequency fix error
    #[inline(always)]
    pub fn crg_freqfix_err(&self) -> CrgFreqfixErrR {
        CrgFreqfixErrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU bus clock ready
    #[inline(always)]
    pub fn ck_cpu_bus(&mut self) -> CkCpuBusW<'_, CrgIntClr1Spec> {
        CkCpuBusW::new(self, 0)
    }
    ///Bit 1 - CPU bus clock timeout
    #[inline(always)]
    pub fn ck_cpu_bus_to(&mut self) -> CkCpuBusToW<'_, CrgIntClr1Spec> {
        CkCpuBusToW::new(self, 1)
    }
    ///Bit 2 - RF PLL1 clock ready
    #[inline(always)]
    pub fn ck_rfpll1(&mut self) -> CkRfpll1W<'_, CrgIntClr1Spec> {
        CkRfpll1W::new(self, 2)
    }
    ///Bit 3 - RF PLL1 clock timeout
    #[inline(always)]
    pub fn ck_rfpll1_to(&mut self) -> CkRfpll1ToW<'_, CrgIntClr1Spec> {
        CkRfpll1ToW::new(self, 3)
    }
    ///Bit 4 - RTC pre-divider clock ready
    #[inline(always)]
    pub fn ck_rtc_pre(&mut self) -> CkRtcPreW<'_, CrgIntClr1Spec> {
        CkRtcPreW::new(self, 4)
    }
    ///Bit 5 - RTC pre-divider clock timeout
    #[inline(always)]
    pub fn ck_rtc_pre_to(&mut self) -> CkRtcPreToW<'_, CrgIntClr1Spec> {
        CkRtcPreToW::new(self, 5)
    }
    ///Bit 6 - APP pre-divider clock ready
    #[inline(always)]
    pub fn ck_app_pre(&mut self) -> CkAppPreW<'_, CrgIntClr1Spec> {
        CkAppPreW::new(self, 6)
    }
    ///Bit 7 - APP pre-divider clock timeout
    #[inline(always)]
    pub fn ck_app_pre_to(&mut self) -> CkAppPreToW<'_, CrgIntClr1Spec> {
        CkAppPreToW::new(self, 7)
    }
    ///Bit 8 - SP clock select ready
    #[inline(always)]
    pub fn ck_sel_sp(&mut self) -> CkSelSpW<'_, CrgIntClr1Spec> {
        CkSelSpW::new(self, 8)
    }
    ///Bit 9 - SP clock select timeout
    #[inline(always)]
    pub fn ck_sel_sp_to(&mut self) -> CkSelSpToW<'_, CrgIntClr1Spec> {
        CkSelSpToW::new(self, 9)
    }
    ///Bit 10 - RO RTC clock select ready
    #[inline(always)]
    pub fn ck_sel_ro_rtc(&mut self) -> CkSelRoRtcW<'_, CrgIntClr1Spec> {
        CkSelRoRtcW::new(self, 10)
    }
    ///Bit 11 - Frequency fix error
    #[inline(always)]
    pub fn crg_freqfix_err(&mut self) -> CrgFreqfixErrW<'_, CrgIntClr1Spec> {
        CrgFreqfixErrW::new(self, 11)
    }
}
/**TOPREG clock-ready interrupt clear 1 (write 1 to clear)

You can [`read`](crate::Reg::read) this register and get [`crg_int_clr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crg_int_clr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrgIntClr1Spec;
impl crate::RegisterSpec for CrgIntClr1Spec {
    type Ux = u32;
}
///`read()` method returns [`crg_int_clr1::R`](R) reader structure
impl crate::Readable for CrgIntClr1Spec {}
///`write(|w| ..)` method takes [`crg_int_clr1::W`](W) writer structure
impl crate::Writable for CrgIntClr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRG_INT_CLR1 to value 0
impl crate::Resettable for CrgIntClr1Spec {}
