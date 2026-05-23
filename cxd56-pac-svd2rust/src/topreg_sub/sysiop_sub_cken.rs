///Register `SYSIOP_SUB_CKEN` reader
pub type R = crate::R<SysiopSubCkenSpec>;
///Register `SYSIOP_SUB_CKEN` writer
pub type W = crate::W<SysiopSubCkenSpec>;
///Field `CK_AHB_BRG_COMIF` reader - AHB bridge COMIF clock enable
pub type CkAhbBrgComifR = crate::BitReader;
///Field `CK_AHB_BRG_COMIF` writer - AHB bridge COMIF clock enable
pub type CkAhbBrgComifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_COM_BRG` reader - COM bridge clock enable
pub type CkComBrgR = crate::BitReader;
///Field `CK_COM_BRG` writer - COM bridge clock enable
pub type CkComBrgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_AHB_DMAC3` reader - AHB DMAC3 clock enable
pub type CkAhbDmac3R = crate::BitReader;
///Field `CK_AHB_DMAC3` writer - AHB DMAC3 clock enable
pub type CkAhbDmac3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_UART1` reader - UART1 clock enable
pub type CkUart1R = crate::BitReader;
///Field `CK_UART1` writer - UART1 clock enable
pub type CkUart1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SPIM` reader - SPI master clock enable
pub type CkSpimR = crate::BitReader;
///Field `CK_SPIM` writer - SPI master clock enable
pub type CkSpimW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_I2CM` reader - I2C master clock enable
pub type CkI2cmR = crate::BitReader;
///Field `CK_I2CM` writer - I2C master clock enable
pub type CkI2cmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_HCLK_SAKE` reader - SAKE HCLK enable
pub type CkHclkSakeR = crate::BitReader;
///Field `CK_HCLK_SAKE` writer - SAKE HCLK enable
pub type CkHclkSakeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SFC_HCLK` reader - Serial flash controller HCLK enable
pub type CkSfcHclkR = crate::BitReader;
///Field `CK_SFC_HCLK` writer - Serial flash controller HCLK enable
pub type CkSfcHclkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SFC_SFCLK` reader - Serial flash controller SFCLK enable
pub type CkSfcSfclkR = crate::BitReader;
///Field `CK_SFC_SFCLK` writer - Serial flash controller SFCLK enable
pub type CkSfcSfclkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_SFC_HCLK_LOW` reader - Serial flash controller HCLK low-speed enable
pub type CkSfcHclkLowR = crate::BitReader;
///Field `CK_SFC_HCLK_LOW` writer - Serial flash controller HCLK low-speed enable
pub type CkSfcHclkLowW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_COM_UART_PCLK` reader - COM UART PCLK enable
pub type CkComUartPclkR = crate::BitReader;
///Field `CK_COM_UART_PCLK` writer - COM UART PCLK enable
pub type CkComUartPclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AHB bridge COMIF clock enable
    #[inline(always)]
    pub fn ck_ahb_brg_comif(&self) -> CkAhbBrgComifR {
        CkAhbBrgComifR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - COM bridge clock enable
    #[inline(always)]
    pub fn ck_com_brg(&self) -> CkComBrgR {
        CkComBrgR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHB DMAC3 clock enable
    #[inline(always)]
    pub fn ck_ahb_dmac3(&self) -> CkAhbDmac3R {
        CkAhbDmac3R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - UART1 clock enable
    #[inline(always)]
    pub fn ck_uart1(&self) -> CkUart1R {
        CkUart1R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SPI master clock enable
    #[inline(always)]
    pub fn ck_spim(&self) -> CkSpimR {
        CkSpimR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I2C master clock enable
    #[inline(always)]
    pub fn ck_i2cm(&self) -> CkI2cmR {
        CkI2cmR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SAKE HCLK enable
    #[inline(always)]
    pub fn ck_hclk_sake(&self) -> CkHclkSakeR {
        CkHclkSakeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Serial flash controller HCLK enable
    #[inline(always)]
    pub fn ck_sfc_hclk(&self) -> CkSfcHclkR {
        CkSfcHclkR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Serial flash controller SFCLK enable
    #[inline(always)]
    pub fn ck_sfc_sfclk(&self) -> CkSfcSfclkR {
        CkSfcSfclkR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Serial flash controller HCLK low-speed enable
    #[inline(always)]
    pub fn ck_sfc_hclk_low(&self) -> CkSfcHclkLowR {
        CkSfcHclkLowR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - COM UART PCLK enable
    #[inline(always)]
    pub fn ck_com_uart_pclk(&self) -> CkComUartPclkR {
        CkComUartPclkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AHB bridge COMIF clock enable
    #[inline(always)]
    pub fn ck_ahb_brg_comif(&mut self) -> CkAhbBrgComifW<'_, SysiopSubCkenSpec> {
        CkAhbBrgComifW::new(self, 0)
    }
    ///Bit 1 - COM bridge clock enable
    #[inline(always)]
    pub fn ck_com_brg(&mut self) -> CkComBrgW<'_, SysiopSubCkenSpec> {
        CkComBrgW::new(self, 1)
    }
    ///Bit 2 - AHB DMAC3 clock enable
    #[inline(always)]
    pub fn ck_ahb_dmac3(&mut self) -> CkAhbDmac3W<'_, SysiopSubCkenSpec> {
        CkAhbDmac3W::new(self, 2)
    }
    ///Bit 3 - UART1 clock enable
    #[inline(always)]
    pub fn ck_uart1(&mut self) -> CkUart1W<'_, SysiopSubCkenSpec> {
        CkUart1W::new(self, 3)
    }
    ///Bit 4 - SPI master clock enable
    #[inline(always)]
    pub fn ck_spim(&mut self) -> CkSpimW<'_, SysiopSubCkenSpec> {
        CkSpimW::new(self, 4)
    }
    ///Bit 5 - I2C master clock enable
    #[inline(always)]
    pub fn ck_i2cm(&mut self) -> CkI2cmW<'_, SysiopSubCkenSpec> {
        CkI2cmW::new(self, 5)
    }
    ///Bit 6 - SAKE HCLK enable
    #[inline(always)]
    pub fn ck_hclk_sake(&mut self) -> CkHclkSakeW<'_, SysiopSubCkenSpec> {
        CkHclkSakeW::new(self, 6)
    }
    ///Bit 7 - Serial flash controller HCLK enable
    #[inline(always)]
    pub fn ck_sfc_hclk(&mut self) -> CkSfcHclkW<'_, SysiopSubCkenSpec> {
        CkSfcHclkW::new(self, 7)
    }
    ///Bit 8 - Serial flash controller SFCLK enable
    #[inline(always)]
    pub fn ck_sfc_sfclk(&mut self) -> CkSfcSfclkW<'_, SysiopSubCkenSpec> {
        CkSfcSfclkW::new(self, 8)
    }
    ///Bit 9 - Serial flash controller HCLK low-speed enable
    #[inline(always)]
    pub fn ck_sfc_hclk_low(&mut self) -> CkSfcHclkLowW<'_, SysiopSubCkenSpec> {
        CkSfcHclkLowW::new(self, 9)
    }
    ///Bit 16 - COM UART PCLK enable
    #[inline(always)]
    pub fn ck_com_uart_pclk(&mut self) -> CkComUartPclkW<'_, SysiopSubCkenSpec> {
        CkComUartPclkW::new(self, 16)
    }
}
/**SYSIOP sub-domain peripheral clock enables

You can [`read`](crate::Reg::read) this register and get [`sysiop_sub_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysiop_sub_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SysiopSubCkenSpec;
impl crate::RegisterSpec for SysiopSubCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`sysiop_sub_cken::R`](R) reader structure
impl crate::Readable for SysiopSubCkenSpec {}
///`write(|w| ..)` method takes [`sysiop_sub_cken::W`](W) writer structure
impl crate::Writable for SysiopSubCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSIOP_SUB_CKEN to value 0
impl crate::Resettable for SysiopSubCkenSpec {}
