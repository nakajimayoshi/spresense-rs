///Register `SYSIOP_CKEN` reader
pub type R = crate::R<SysiopCkenSpec>;
///Register `SYSIOP_CKEN` writer
pub type W = crate::W<SysiopCkenSpec>;
///Field `CKEN_UART0` reader - UART0 clock enable
pub type CkenUart0R = crate::BitReader;
///Field `CKEN_UART0` writer - UART0 clock enable
pub type CkenUart0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_PCLK_UART0` reader - UART0 PCLK enable
pub type CkenPclkUart0R = crate::BitReader;
///Field `CKEN_PCLK_UART0` writer - UART0 PCLK enable
pub type CkenPclkUart0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_PCLK_HOSTIFC` reader - Host interface PCLK enable
pub type CkenPclkHostifcR = crate::BitReader;
///Field `CKEN_PCLK_HOSTIFC` writer - Host interface PCLK enable
pub type CkenPclkHostifcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_I2CS` reader - I2C slave clock enable
pub type CkenI2csR = crate::BitReader;
///Field `CKEN_I2CS` writer - I2C slave clock enable
pub type CkenI2csW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_BRG_HOST` reader - Host bridge clock enable
pub type CkenBrgHostR = crate::BitReader;
///Field `CKEN_BRG_HOST` writer - Host bridge clock enable
pub type CkenBrgHostW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_AHB_DMAC0` reader - AHB DMAC0 clock enable
pub type CkenAhbDmac0R = crate::BitReader;
///Field `CKEN_AHB_DMAC0` writer - AHB DMAC0 clock enable
pub type CkenAhbDmac0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_AHB_DMAC1` reader - AHB DMAC1 clock enable
pub type CkenAhbDmac1R = crate::BitReader;
///Field `CKEN_AHB_DMAC1` writer - AHB DMAC1 clock enable
pub type CkenAhbDmac1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_AHB_DMAC2` reader - AHB DMAC2 clock enable
pub type CkenAhbDmac2R = crate::BitReader;
///Field `CKEN_AHB_DMAC2` writer - AHB DMAC2 clock enable
pub type CkenAhbDmac2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_APB` reader - APB clock enable
pub type CkenApbR = crate::BitReader;
///Field `CKEN_APB` writer - APB clock enable
pub type CkenApbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_FREQDIS` reader - Frequency discriminator clock enable
pub type CkenFreqdisR = crate::BitReader;
///Field `CKEN_FREQDIS` writer - Frequency discriminator clock enable
pub type CkenFreqdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_RTC_ORG` reader - RTC origin clock enable
pub type CkenRtcOrgR = crate::BitReader;
///Field `CKEN_RTC_ORG` writer - RTC origin clock enable
pub type CkenRtcOrgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_AP_CLK` reader - AP clock enable
pub type CkenApClkR = crate::BitReader;
///Field `CKEN_AP_CLK` writer - AP clock enable
pub type CkenApClkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_RCOSC_OUT` reader - RCOSC output clock enable
pub type CkenRcoscOutR = crate::BitReader;
///Field `CKEN_RCOSC_OUT` writer - RCOSC output clock enable
pub type CkenRcoscOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_SYSIOP_RTC` reader - SYSIOP RTC clock enable
pub type CkenSysiopRtcR = crate::BitReader;
///Field `CKEN_SYSIOP_RTC` writer - SYSIOP RTC clock enable
pub type CkenSysiopRtcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_BRG_SCU` reader - SCU bridge clock enable
pub type CkenBrgScuR = crate::BitReader;
///Field `CKEN_BRG_SCU` writer - SCU bridge clock enable
pub type CkenBrgScuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_HOSTIFC_SEQ` reader - Host interface sequencer clock enable
pub type CkenHostifcSeqR = crate::BitReader;
///Field `CKEN_HOSTIFC_SEQ` writer - Host interface sequencer clock enable
pub type CkenHostifcSeqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_HOSI2C` reader - Host I2C clock enable
pub type CkenHosi2cR = crate::BitReader;
///Field `CKEN_HOSI2C` writer - Host I2C clock enable
pub type CkenHosi2cW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN_HOSSPI` reader - Host SPI clock enable
pub type CkenHosspiR = crate::BitReader;
///Field `CKEN_HOSSPI` writer - Host SPI clock enable
pub type CkenHosspiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UART0 clock enable
    #[inline(always)]
    pub fn cken_uart0(&self) -> CkenUart0R {
        CkenUart0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UART0 PCLK enable
    #[inline(always)]
    pub fn cken_pclk_uart0(&self) -> CkenPclkUart0R {
        CkenPclkUart0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Host interface PCLK enable
    #[inline(always)]
    pub fn cken_pclk_hostifc(&self) -> CkenPclkHostifcR {
        CkenPclkHostifcR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C slave clock enable
    #[inline(always)]
    pub fn cken_i2cs(&self) -> CkenI2csR {
        CkenI2csR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Host bridge clock enable
    #[inline(always)]
    pub fn cken_brg_host(&self) -> CkenBrgHostR {
        CkenBrgHostR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AHB DMAC0 clock enable
    #[inline(always)]
    pub fn cken_ahb_dmac0(&self) -> CkenAhbDmac0R {
        CkenAhbDmac0R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AHB DMAC1 clock enable
    #[inline(always)]
    pub fn cken_ahb_dmac1(&self) -> CkenAhbDmac1R {
        CkenAhbDmac1R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AHB DMAC2 clock enable
    #[inline(always)]
    pub fn cken_ahb_dmac2(&self) -> CkenAhbDmac2R {
        CkenAhbDmac2R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - APB clock enable
    #[inline(always)]
    pub fn cken_apb(&self) -> CkenApbR {
        CkenApbR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Frequency discriminator clock enable
    #[inline(always)]
    pub fn cken_freqdis(&self) -> CkenFreqdisR {
        CkenFreqdisR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RTC origin clock enable
    #[inline(always)]
    pub fn cken_rtc_org(&self) -> CkenRtcOrgR {
        CkenRtcOrgR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AP clock enable
    #[inline(always)]
    pub fn cken_ap_clk(&self) -> CkenApClkR {
        CkenApClkR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RCOSC output clock enable
    #[inline(always)]
    pub fn cken_rcosc_out(&self) -> CkenRcoscOutR {
        CkenRcoscOutR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SYSIOP RTC clock enable
    #[inline(always)]
    pub fn cken_sysiop_rtc(&self) -> CkenSysiopRtcR {
        CkenSysiopRtcR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SCU bridge clock enable
    #[inline(always)]
    pub fn cken_brg_scu(&self) -> CkenBrgScuR {
        CkenBrgScuR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Host interface sequencer clock enable
    #[inline(always)]
    pub fn cken_hostifc_seq(&self) -> CkenHostifcSeqR {
        CkenHostifcSeqR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Host I2C clock enable
    #[inline(always)]
    pub fn cken_hosi2c(&self) -> CkenHosi2cR {
        CkenHosi2cR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Host SPI clock enable
    #[inline(always)]
    pub fn cken_hosspi(&self) -> CkenHosspiR {
        CkenHosspiR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - UART0 clock enable
    #[inline(always)]
    pub fn cken_uart0(&mut self) -> CkenUart0W<'_, SysiopCkenSpec> {
        CkenUart0W::new(self, 0)
    }
    ///Bit 1 - UART0 PCLK enable
    #[inline(always)]
    pub fn cken_pclk_uart0(&mut self) -> CkenPclkUart0W<'_, SysiopCkenSpec> {
        CkenPclkUart0W::new(self, 1)
    }
    ///Bit 2 - Host interface PCLK enable
    #[inline(always)]
    pub fn cken_pclk_hostifc(&mut self) -> CkenPclkHostifcW<'_, SysiopCkenSpec> {
        CkenPclkHostifcW::new(self, 2)
    }
    ///Bit 3 - I2C slave clock enable
    #[inline(always)]
    pub fn cken_i2cs(&mut self) -> CkenI2csW<'_, SysiopCkenSpec> {
        CkenI2csW::new(self, 3)
    }
    ///Bit 4 - Host bridge clock enable
    #[inline(always)]
    pub fn cken_brg_host(&mut self) -> CkenBrgHostW<'_, SysiopCkenSpec> {
        CkenBrgHostW::new(self, 4)
    }
    ///Bit 5 - AHB DMAC0 clock enable
    #[inline(always)]
    pub fn cken_ahb_dmac0(&mut self) -> CkenAhbDmac0W<'_, SysiopCkenSpec> {
        CkenAhbDmac0W::new(self, 5)
    }
    ///Bit 6 - AHB DMAC1 clock enable
    #[inline(always)]
    pub fn cken_ahb_dmac1(&mut self) -> CkenAhbDmac1W<'_, SysiopCkenSpec> {
        CkenAhbDmac1W::new(self, 6)
    }
    ///Bit 7 - AHB DMAC2 clock enable
    #[inline(always)]
    pub fn cken_ahb_dmac2(&mut self) -> CkenAhbDmac2W<'_, SysiopCkenSpec> {
        CkenAhbDmac2W::new(self, 7)
    }
    ///Bit 8 - APB clock enable
    #[inline(always)]
    pub fn cken_apb(&mut self) -> CkenApbW<'_, SysiopCkenSpec> {
        CkenApbW::new(self, 8)
    }
    ///Bit 9 - Frequency discriminator clock enable
    #[inline(always)]
    pub fn cken_freqdis(&mut self) -> CkenFreqdisW<'_, SysiopCkenSpec> {
        CkenFreqdisW::new(self, 9)
    }
    ///Bit 10 - RTC origin clock enable
    #[inline(always)]
    pub fn cken_rtc_org(&mut self) -> CkenRtcOrgW<'_, SysiopCkenSpec> {
        CkenRtcOrgW::new(self, 10)
    }
    ///Bit 11 - AP clock enable
    #[inline(always)]
    pub fn cken_ap_clk(&mut self) -> CkenApClkW<'_, SysiopCkenSpec> {
        CkenApClkW::new(self, 11)
    }
    ///Bit 12 - RCOSC output clock enable
    #[inline(always)]
    pub fn cken_rcosc_out(&mut self) -> CkenRcoscOutW<'_, SysiopCkenSpec> {
        CkenRcoscOutW::new(self, 12)
    }
    ///Bit 13 - SYSIOP RTC clock enable
    #[inline(always)]
    pub fn cken_sysiop_rtc(&mut self) -> CkenSysiopRtcW<'_, SysiopCkenSpec> {
        CkenSysiopRtcW::new(self, 13)
    }
    ///Bit 14 - SCU bridge clock enable
    #[inline(always)]
    pub fn cken_brg_scu(&mut self) -> CkenBrgScuW<'_, SysiopCkenSpec> {
        CkenBrgScuW::new(self, 14)
    }
    ///Bit 15 - Host interface sequencer clock enable
    #[inline(always)]
    pub fn cken_hostifc_seq(&mut self) -> CkenHostifcSeqW<'_, SysiopCkenSpec> {
        CkenHostifcSeqW::new(self, 15)
    }
    ///Bit 16 - Host I2C clock enable
    #[inline(always)]
    pub fn cken_hosi2c(&mut self) -> CkenHosi2cW<'_, SysiopCkenSpec> {
        CkenHosi2cW::new(self, 16)
    }
    ///Bit 17 - Host SPI clock enable
    #[inline(always)]
    pub fn cken_hosspi(&mut self) -> CkenHosspiW<'_, SysiopCkenSpec> {
        CkenHosspiW::new(self, 17)
    }
}
/**SYSIOP peripheral clock enables

You can [`read`](crate::Reg::read) this register and get [`sysiop_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysiop_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SysiopCkenSpec;
impl crate::RegisterSpec for SysiopCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`sysiop_cken::R`](R) reader structure
impl crate::Readable for SysiopCkenSpec {}
///`write(|w| ..)` method takes [`sysiop_cken::W`](W) writer structure
impl crate::Writable for SysiopCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSIOP_CKEN to value 0
impl crate::Resettable for SysiopCkenSpec {}
