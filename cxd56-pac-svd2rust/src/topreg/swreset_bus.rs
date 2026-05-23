///Register `SWRESET_BUS` reader
pub type R = crate::R<SwresetBusSpec>;
///Register `SWRESET_BUS` writer
pub type W = crate::W<SwresetBusSpec>;
///Field `XRST_SPIM` reader - SPI master reset
pub type XrstSpimR = crate::BitReader;
///Field `XRST_SPIM` writer - SPI master reset
pub type XrstSpimW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_SFC` reader - Serial flash controller reset
pub type XrstSfcR = crate::BitReader;
///Field `XRST_SFC` writer - Serial flash controller reset
pub type XrstSfcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_SAKE` reader - SAKE reset
pub type XrstSakeR = crate::BitReader;
///Field `XRST_SAKE` writer - SAKE reset
pub type XrstSakeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_UART1` reader - UART1 reset
pub type XrstUart1R = crate::BitReader;
///Field `XRST_UART1` writer - UART1 reset
pub type XrstUart1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_KAKI` reader - KAKI reset
pub type XrstKakiR = crate::BitReader;
///Field `XRST_KAKI` writer - KAKI reset
pub type XrstKakiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_HOSTIFC` reader - Host interface reset
pub type XrstHostifcR = crate::BitReader;
///Field `XRST_HOSTIFC` writer - Host interface reset
pub type XrstHostifcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_HOSTIFC_ISOP` reader - Host interface ISOP reset
pub type XrstHostifcIsopR = crate::BitReader;
///Field `XRST_HOSTIFC_ISOP` writer - Host interface ISOP reset
pub type XrstHostifcIsopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_UART0` reader - UART0 reset
pub type XrstUart0R = crate::BitReader;
///Field `XRST_UART0` writer - UART0 reset
pub type XrstUart0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_I2CM` reader - I2C master reset
pub type XrstI2cmR = crate::BitReader;
///Field `XRST_I2CM` writer - I2C master reset
pub type XrstI2cmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_PMU_I2CM` reader - PMU I2C master reset
pub type XrstPmuI2cmR = crate::BitReader;
///Field `XRST_PMU_I2CM` writer - PMU I2C master reset
pub type XrstPmuI2cmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPI master reset
    #[inline(always)]
    pub fn xrst_spim(&self) -> XrstSpimR {
        XrstSpimR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Serial flash controller reset
    #[inline(always)]
    pub fn xrst_sfc(&self) -> XrstSfcR {
        XrstSfcR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SAKE reset
    #[inline(always)]
    pub fn xrst_sake(&self) -> XrstSakeR {
        XrstSakeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - UART1 reset
    #[inline(always)]
    pub fn xrst_uart1(&self) -> XrstUart1R {
        XrstUart1R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - KAKI reset
    #[inline(always)]
    pub fn xrst_kaki(&self) -> XrstKakiR {
        XrstKakiR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Host interface reset
    #[inline(always)]
    pub fn xrst_hostifc(&self) -> XrstHostifcR {
        XrstHostifcR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Host interface ISOP reset
    #[inline(always)]
    pub fn xrst_hostifc_isop(&self) -> XrstHostifcIsopR {
        XrstHostifcIsopR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - UART0 reset
    #[inline(always)]
    pub fn xrst_uart0(&self) -> XrstUart0R {
        XrstUart0R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I2C master reset
    #[inline(always)]
    pub fn xrst_i2cm(&self) -> XrstI2cmR {
        XrstI2cmR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - PMU I2C master reset
    #[inline(always)]
    pub fn xrst_pmu_i2cm(&self) -> XrstPmuI2cmR {
        XrstPmuI2cmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SPI master reset
    #[inline(always)]
    pub fn xrst_spim(&mut self) -> XrstSpimW<'_, SwresetBusSpec> {
        XrstSpimW::new(self, 0)
    }
    ///Bit 1 - Serial flash controller reset
    #[inline(always)]
    pub fn xrst_sfc(&mut self) -> XrstSfcW<'_, SwresetBusSpec> {
        XrstSfcW::new(self, 1)
    }
    ///Bit 2 - SAKE reset
    #[inline(always)]
    pub fn xrst_sake(&mut self) -> XrstSakeW<'_, SwresetBusSpec> {
        XrstSakeW::new(self, 2)
    }
    ///Bit 5 - UART1 reset
    #[inline(always)]
    pub fn xrst_uart1(&mut self) -> XrstUart1W<'_, SwresetBusSpec> {
        XrstUart1W::new(self, 5)
    }
    ///Bit 6 - KAKI reset
    #[inline(always)]
    pub fn xrst_kaki(&mut self) -> XrstKakiW<'_, SwresetBusSpec> {
        XrstKakiW::new(self, 6)
    }
    ///Bit 8 - Host interface reset
    #[inline(always)]
    pub fn xrst_hostifc(&mut self) -> XrstHostifcW<'_, SwresetBusSpec> {
        XrstHostifcW::new(self, 8)
    }
    ///Bit 9 - Host interface ISOP reset
    #[inline(always)]
    pub fn xrst_hostifc_isop(&mut self) -> XrstHostifcIsopW<'_, SwresetBusSpec> {
        XrstHostifcIsopW::new(self, 9)
    }
    ///Bit 10 - UART0 reset
    #[inline(always)]
    pub fn xrst_uart0(&mut self) -> XrstUart0W<'_, SwresetBusSpec> {
        XrstUart0W::new(self, 10)
    }
    ///Bit 11 - I2C master reset
    #[inline(always)]
    pub fn xrst_i2cm(&mut self) -> XrstI2cmW<'_, SwresetBusSpec> {
        XrstI2cmW::new(self, 11)
    }
    ///Bit 16 - PMU I2C master reset
    #[inline(always)]
    pub fn xrst_pmu_i2cm(&mut self) -> XrstPmuI2cmW<'_, SwresetBusSpec> {
        XrstPmuI2cmW::new(self, 16)
    }
}
/**Bus peripheral software reset (0 = held in reset, 1 = released)

You can [`read`](crate::Reg::read) this register and get [`swreset_bus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset_bus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SwresetBusSpec;
impl crate::RegisterSpec for SwresetBusSpec {
    type Ux = u32;
}
///`read()` method returns [`swreset_bus::R`](R) reader structure
impl crate::Readable for SwresetBusSpec {}
///`write(|w| ..)` method takes [`swreset_bus::W`](W) writer structure
impl crate::Writable for SwresetBusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWRESET_BUS to value 0
impl crate::Resettable for SwresetBusSpec {}
