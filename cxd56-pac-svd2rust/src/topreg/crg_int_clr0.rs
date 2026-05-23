///Register `CRG_INT_CLR0` reader
pub type R = crate::R<CrgIntClr0Spec>;
///Register `CRG_INT_CLR0` writer
pub type W = crate::W<CrgIntClr0Spec>;
///Field `CK_PCLK_UART0` reader - UART0 PCLK ready
pub type CkPclkUart0R = crate::BitReader;
///Field `CK_PCLK_UART0` writer - UART0 PCLK ready
pub type CkPclkUart0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_UART0` reader - UART0 clock ready
pub type CkUart0R = crate::BitReader;
///Field `CK_UART0` writer - UART0 clock ready
pub type CkUart0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_BRG_HOST` reader - Host bridge clock ready
pub type CkBrgHostR = crate::BitReader;
///Field `CK_BRG_HOST` writer - Host bridge clock ready
pub type CkBrgHostW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_PCLK_HOSTIFC` reader - Host interface PCLK ready
pub type CkPclkHostifcR = crate::BitReader;
///Field `CK_PCLK_HOSTIFC` writer - Host interface PCLK ready
pub type CkPclkHostifcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_HOSTIFC_SEQ` reader - Host interface sequencer clock ready
pub type CkHostifcSeqR = crate::BitReader;
///Field `CK_HOSTIFC_SEQ` writer - Host interface sequencer clock ready
pub type CkHostifcSeqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_I2CS` reader - I2C slave clock ready
pub type CkI2csR = crate::BitReader;
///Field `CK_I2CS` writer - I2C slave clock ready
pub type CkI2csW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_RTC_ORG` reader - RTC origin clock ready
pub type CkRtcOrgR = crate::BitReader;
///Field `CK_RTC_ORG` writer - RTC origin clock ready
pub type CkRtcOrgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SYSIOP_RTC` reader - SYSIOP RTC clock ready
pub type CkSysiopRtcR = crate::BitReader;
///Field `CK_SYSIOP_RTC` writer - SYSIOP RTC clock ready
pub type CkSysiopRtcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_BRG_SCU` reader - SCU bridge clock ready
pub type CkBrgScuR = crate::BitReader;
///Field `CK_BRG_SCU` writer - SCU bridge clock ready
pub type CkBrgScuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SCU` reader - SCU clock ready
pub type CkScuR = crate::BitReader;
///Field `CK_SCU` writer - SCU clock ready
pub type CkScuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SCU_SPI` reader - SCU SPI clock ready
pub type CkScuSpiR = crate::BitReader;
///Field `CK_SCU_SPI` writer - SCU SPI clock ready
pub type CkScuSpiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SCU_I2C0` reader - SCU I2C0 clock ready
pub type CkScuI2c0R = crate::BitReader;
///Field `CK_SCU_I2C0` writer - SCU I2C0 clock ready
pub type CkScuI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SCU_I2C1` reader - SCU I2C1 clock ready
pub type CkScuI2c1R = crate::BitReader;
///Field `CK_SCU_I2C1` writer - SCU I2C1 clock ready
pub type CkScuI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SCU_SEQ` reader - SCU sequencer clock ready
pub type CkScuSeqR = crate::BitReader;
///Field `CK_SCU_SEQ` writer - SCU sequencer clock ready
pub type CkScuSeqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SCU_SC` reader - SCU SC clock ready
pub type CkScuScR = crate::BitReader;
///Field `CK_SCU_SC` writer - SCU SC clock ready
pub type CkScuScW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_32K` reader - 32 kHz clock ready
pub type Ck32kR = crate::BitReader;
///Field `CK_32K` writer - 32 kHz clock ready
pub type Ck32kW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_U32KH` reader - U32K high clock ready
pub type CkU32khR = crate::BitReader;
///Field `CK_U32KH` writer - U32K high clock ready
pub type CkU32khW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_U32KL` reader - U32K low clock ready
pub type CkU32klR = crate::BitReader;
///Field `CK_U32KL` writer - U32K low clock ready
pub type CkU32klW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_TADC` reader - Temperature ADC clock ready
pub type CkTadcR = crate::BitReader;
///Field `CK_TADC` writer - Temperature ADC clock ready
pub type CkTadcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_RTC_PCLK` reader - RTC PCLK ready
pub type CkRtcPclkR = crate::BitReader;
///Field `CK_RTC_PCLK` writer - RTC PCLK ready
pub type CkRtcPclkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_PMU_RTC_PCLK` reader - PMU RTC PCLK ready
pub type CkPmuRtcPclkR = crate::BitReader;
///Field `CK_PMU_RTC_PCLK` writer - PMU RTC PCLK ready
pub type CkPmuRtcPclkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_APP` reader - APP domain clock ready
pub type CkAppR = crate::BitReader;
///Field `CK_APP` writer - APP domain clock ready
pub type CkAppW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UART0 PCLK ready
    #[inline(always)]
    pub fn ck_pclk_uart0(&self) -> CkPclkUart0R {
        CkPclkUart0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UART0 clock ready
    #[inline(always)]
    pub fn ck_uart0(&self) -> CkUart0R {
        CkUart0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Host bridge clock ready
    #[inline(always)]
    pub fn ck_brg_host(&self) -> CkBrgHostR {
        CkBrgHostR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Host interface PCLK ready
    #[inline(always)]
    pub fn ck_pclk_hostifc(&self) -> CkPclkHostifcR {
        CkPclkHostifcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Host interface sequencer clock ready
    #[inline(always)]
    pub fn ck_hostifc_seq(&self) -> CkHostifcSeqR {
        CkHostifcSeqR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I2C slave clock ready
    #[inline(always)]
    pub fn ck_i2cs(&self) -> CkI2csR {
        CkI2csR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RTC origin clock ready
    #[inline(always)]
    pub fn ck_rtc_org(&self) -> CkRtcOrgR {
        CkRtcOrgR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SYSIOP RTC clock ready
    #[inline(always)]
    pub fn ck_sysiop_rtc(&self) -> CkSysiopRtcR {
        CkSysiopRtcR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SCU bridge clock ready
    #[inline(always)]
    pub fn ck_brg_scu(&self) -> CkBrgScuR {
        CkBrgScuR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SCU clock ready
    #[inline(always)]
    pub fn ck_scu(&self) -> CkScuR {
        CkScuR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SCU SPI clock ready
    #[inline(always)]
    pub fn ck_scu_spi(&self) -> CkScuSpiR {
        CkScuSpiR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SCU I2C0 clock ready
    #[inline(always)]
    pub fn ck_scu_i2c0(&self) -> CkScuI2c0R {
        CkScuI2c0R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SCU I2C1 clock ready
    #[inline(always)]
    pub fn ck_scu_i2c1(&self) -> CkScuI2c1R {
        CkScuI2c1R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SCU sequencer clock ready
    #[inline(always)]
    pub fn ck_scu_seq(&self) -> CkScuSeqR {
        CkScuSeqR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SCU SC clock ready
    #[inline(always)]
    pub fn ck_scu_sc(&self) -> CkScuScR {
        CkScuScR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - 32 kHz clock ready
    #[inline(always)]
    pub fn ck_32k(&self) -> Ck32kR {
        Ck32kR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - U32K high clock ready
    #[inline(always)]
    pub fn ck_u32kh(&self) -> CkU32khR {
        CkU32khR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - U32K low clock ready
    #[inline(always)]
    pub fn ck_u32kl(&self) -> CkU32klR {
        CkU32klR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Temperature ADC clock ready
    #[inline(always)]
    pub fn ck_tadc(&self) -> CkTadcR {
        CkTadcR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RTC PCLK ready
    #[inline(always)]
    pub fn ck_rtc_pclk(&self) -> CkRtcPclkR {
        CkRtcPclkR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PMU RTC PCLK ready
    #[inline(always)]
    pub fn ck_pmu_rtc_pclk(&self) -> CkPmuRtcPclkR {
        CkPmuRtcPclkR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - APP domain clock ready
    #[inline(always)]
    pub fn ck_app(&self) -> CkAppR {
        CkAppR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - UART0 PCLK ready
    #[inline(always)]
    pub fn ck_pclk_uart0(&mut self) -> CkPclkUart0W<'_, CrgIntClr0Spec> {
        CkPclkUart0W::new(self, 0)
    }
    ///Bit 1 - UART0 clock ready
    #[inline(always)]
    pub fn ck_uart0(&mut self) -> CkUart0W<'_, CrgIntClr0Spec> {
        CkUart0W::new(self, 1)
    }
    ///Bit 2 - Host bridge clock ready
    #[inline(always)]
    pub fn ck_brg_host(&mut self) -> CkBrgHostW<'_, CrgIntClr0Spec> {
        CkBrgHostW::new(self, 2)
    }
    ///Bit 3 - Host interface PCLK ready
    #[inline(always)]
    pub fn ck_pclk_hostifc(&mut self) -> CkPclkHostifcW<'_, CrgIntClr0Spec> {
        CkPclkHostifcW::new(self, 3)
    }
    ///Bit 4 - Host interface sequencer clock ready
    #[inline(always)]
    pub fn ck_hostifc_seq(&mut self) -> CkHostifcSeqW<'_, CrgIntClr0Spec> {
        CkHostifcSeqW::new(self, 4)
    }
    ///Bit 5 - I2C slave clock ready
    #[inline(always)]
    pub fn ck_i2cs(&mut self) -> CkI2csW<'_, CrgIntClr0Spec> {
        CkI2csW::new(self, 5)
    }
    ///Bit 6 - RTC origin clock ready
    #[inline(always)]
    pub fn ck_rtc_org(&mut self) -> CkRtcOrgW<'_, CrgIntClr0Spec> {
        CkRtcOrgW::new(self, 6)
    }
    ///Bit 7 - SYSIOP RTC clock ready
    #[inline(always)]
    pub fn ck_sysiop_rtc(&mut self) -> CkSysiopRtcW<'_, CrgIntClr0Spec> {
        CkSysiopRtcW::new(self, 7)
    }
    ///Bit 8 - SCU bridge clock ready
    #[inline(always)]
    pub fn ck_brg_scu(&mut self) -> CkBrgScuW<'_, CrgIntClr0Spec> {
        CkBrgScuW::new(self, 8)
    }
    ///Bit 9 - SCU clock ready
    #[inline(always)]
    pub fn ck_scu(&mut self) -> CkScuW<'_, CrgIntClr0Spec> {
        CkScuW::new(self, 9)
    }
    ///Bit 10 - SCU SPI clock ready
    #[inline(always)]
    pub fn ck_scu_spi(&mut self) -> CkScuSpiW<'_, CrgIntClr0Spec> {
        CkScuSpiW::new(self, 10)
    }
    ///Bit 11 - SCU I2C0 clock ready
    #[inline(always)]
    pub fn ck_scu_i2c0(&mut self) -> CkScuI2c0W<'_, CrgIntClr0Spec> {
        CkScuI2c0W::new(self, 11)
    }
    ///Bit 12 - SCU I2C1 clock ready
    #[inline(always)]
    pub fn ck_scu_i2c1(&mut self) -> CkScuI2c1W<'_, CrgIntClr0Spec> {
        CkScuI2c1W::new(self, 12)
    }
    ///Bit 13 - SCU sequencer clock ready
    #[inline(always)]
    pub fn ck_scu_seq(&mut self) -> CkScuSeqW<'_, CrgIntClr0Spec> {
        CkScuSeqW::new(self, 13)
    }
    ///Bit 14 - SCU SC clock ready
    #[inline(always)]
    pub fn ck_scu_sc(&mut self) -> CkScuScW<'_, CrgIntClr0Spec> {
        CkScuScW::new(self, 14)
    }
    ///Bit 15 - 32 kHz clock ready
    #[inline(always)]
    pub fn ck_32k(&mut self) -> Ck32kW<'_, CrgIntClr0Spec> {
        Ck32kW::new(self, 15)
    }
    ///Bit 16 - U32K high clock ready
    #[inline(always)]
    pub fn ck_u32kh(&mut self) -> CkU32khW<'_, CrgIntClr0Spec> {
        CkU32khW::new(self, 16)
    }
    ///Bit 17 - U32K low clock ready
    #[inline(always)]
    pub fn ck_u32kl(&mut self) -> CkU32klW<'_, CrgIntClr0Spec> {
        CkU32klW::new(self, 17)
    }
    ///Bit 18 - Temperature ADC clock ready
    #[inline(always)]
    pub fn ck_tadc(&mut self) -> CkTadcW<'_, CrgIntClr0Spec> {
        CkTadcW::new(self, 18)
    }
    ///Bit 19 - RTC PCLK ready
    #[inline(always)]
    pub fn ck_rtc_pclk(&mut self) -> CkRtcPclkW<'_, CrgIntClr0Spec> {
        CkRtcPclkW::new(self, 19)
    }
    ///Bit 20 - PMU RTC PCLK ready
    #[inline(always)]
    pub fn ck_pmu_rtc_pclk(&mut self) -> CkPmuRtcPclkW<'_, CrgIntClr0Spec> {
        CkPmuRtcPclkW::new(self, 20)
    }
    ///Bit 21 - APP domain clock ready
    #[inline(always)]
    pub fn ck_app(&mut self) -> CkAppW<'_, CrgIntClr0Spec> {
        CkAppW::new(self, 21)
    }
}
/**TOPREG clock-ready interrupt clear 0 (write 1 to clear)

You can [`read`](crate::Reg::read) this register and get [`crg_int_clr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crg_int_clr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrgIntClr0Spec;
impl crate::RegisterSpec for CrgIntClr0Spec {
    type Ux = u32;
}
///`read()` method returns [`crg_int_clr0::R`](R) reader structure
impl crate::Readable for CrgIntClr0Spec {}
///`write(|w| ..)` method takes [`crg_int_clr0::W`](W) writer structure
impl crate::Writable for CrgIntClr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRG_INT_CLR0 to value 0
impl crate::Resettable for CrgIntClr0Spec {}
