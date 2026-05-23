///Register `SWRESET_SCU` reader
pub type R = crate::R<SwresetScuSpec>;
///Register `SWRESET_SCU` writer
pub type W = crate::W<SwresetScuSpec>;
///Field `XRST_SCU_HPADC` reader - SCU high-precision ADC reset
pub type XrstScuHpadcR = crate::BitReader;
///Field `XRST_SCU_HPADC` writer - SCU high-precision ADC reset
pub type XrstScuHpadcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_SCU_LPADC` reader - SCU low-power ADC reset
pub type XrstScuLpadcR = crate::BitReader;
///Field `XRST_SCU_LPADC` writer - SCU low-power ADC reset
pub type XrstScuLpadcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_SCU_I2C2` reader - SCU I2C2 reset
pub type XrstScuI2c2R = crate::BitReader;
///Field `XRST_SCU_I2C2` writer - SCU I2C2 reset
pub type XrstScuI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_SCU_I2C1` reader - SCU I2C1 reset
pub type XrstScuI2c1R = crate::BitReader;
///Field `XRST_SCU_I2C1` writer - SCU I2C1 reset
pub type XrstScuI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_SCU_ISOP` reader - SCU ISOP reset
pub type XrstScuIsopR = crate::BitReader;
///Field `XRST_SCU_ISOP` writer - SCU ISOP reset
pub type XrstScuIsopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST_SCU_SPI` reader - SCU SPI reset
pub type XrstScuSpiR = crate::BitReader;
///Field `XRST_SCU_SPI` writer - SCU SPI reset
pub type XrstScuSpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - SCU high-precision ADC reset
    #[inline(always)]
    pub fn xrst_scu_hpadc(&self) -> XrstScuHpadcR {
        XrstScuHpadcR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SCU low-power ADC reset
    #[inline(always)]
    pub fn xrst_scu_lpadc(&self) -> XrstScuLpadcR {
        XrstScuLpadcR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SCU I2C2 reset
    #[inline(always)]
    pub fn xrst_scu_i2c2(&self) -> XrstScuI2c2R {
        XrstScuI2c2R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SCU I2C1 reset
    #[inline(always)]
    pub fn xrst_scu_i2c1(&self) -> XrstScuI2c1R {
        XrstScuI2c1R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SCU ISOP reset
    #[inline(always)]
    pub fn xrst_scu_isop(&self) -> XrstScuIsopR {
        XrstScuIsopR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SCU SPI reset
    #[inline(always)]
    pub fn xrst_scu_spi(&self) -> XrstScuSpiR {
        XrstScuSpiR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - SCU high-precision ADC reset
    #[inline(always)]
    pub fn xrst_scu_hpadc(&mut self) -> XrstScuHpadcW<'_, SwresetScuSpec> {
        XrstScuHpadcW::new(self, 2)
    }
    ///Bit 4 - SCU low-power ADC reset
    #[inline(always)]
    pub fn xrst_scu_lpadc(&mut self) -> XrstScuLpadcW<'_, SwresetScuSpec> {
        XrstScuLpadcW::new(self, 4)
    }
    ///Bit 5 - SCU I2C2 reset
    #[inline(always)]
    pub fn xrst_scu_i2c2(&mut self) -> XrstScuI2c2W<'_, SwresetScuSpec> {
        XrstScuI2c2W::new(self, 5)
    }
    ///Bit 6 - SCU I2C1 reset
    #[inline(always)]
    pub fn xrst_scu_i2c1(&mut self) -> XrstScuI2c1W<'_, SwresetScuSpec> {
        XrstScuI2c1W::new(self, 6)
    }
    ///Bit 7 - SCU ISOP reset
    #[inline(always)]
    pub fn xrst_scu_isop(&mut self) -> XrstScuIsopW<'_, SwresetScuSpec> {
        XrstScuIsopW::new(self, 7)
    }
    ///Bit 8 - SCU SPI reset
    #[inline(always)]
    pub fn xrst_scu_spi(&mut self) -> XrstScuSpiW<'_, SwresetScuSpec> {
        XrstScuSpiW::new(self, 8)
    }
}
/**SCU peripheral software reset (0 = held in reset, 1 = released)

You can [`read`](crate::Reg::read) this register and get [`swreset_scu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset_scu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SwresetScuSpec;
impl crate::RegisterSpec for SwresetScuSpec {
    type Ux = u32;
}
///`read()` method returns [`swreset_scu::R`](R) reader structure
impl crate::Readable for SwresetScuSpec {}
///`write(|w| ..)` method takes [`swreset_scu::W`](W) writer structure
impl crate::Writable for SwresetScuSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWRESET_SCU to value 0
impl crate::Resettable for SwresetScuSpec {}
