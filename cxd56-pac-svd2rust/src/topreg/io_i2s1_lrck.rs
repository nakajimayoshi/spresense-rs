///Register `IO_I2S1_LRCK` reader
pub type R = crate::R<IoI2s1LrckSpec>;
///Register `IO_I2S1_LRCK` writer
pub type W = crate::W<IoI2s1LrckSpec>;
///Field `ENZI` reader - Input enable: 0=disabled, 1=enabled
pub type EnziR = crate::BitReader;
///Field `ENZI` writer - Input enable: 0=disabled, 1=enabled
pub type EnziW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PUN` reader - Pullup: 0=pullup enabled, 1=normal (off)
pub type PunR = crate::BitReader;
///Field `PUN` writer - Pullup: 0=pullup enabled, 1=normal (off)
pub type PunW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN` reader - Pulldown: 0=pulldown enabled, 1=normal (off)
pub type PdnR = crate::BitReader;
///Field `PDN` writer - Pulldown: 0=pulldown enabled, 1=normal (off)
pub type PdnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOWEMI` reader - Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz)
pub type LowemiR = crate::BitReader;
///Field `LOWEMI` writer - Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz)
pub type LowemiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Input enable: 0=disabled, 1=enabled
    #[inline(always)]
    pub fn enzi(&self) -> EnziR {
        EnziR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Pullup: 0=pullup enabled, 1=normal (off)
    #[inline(always)]
    pub fn pun(&self) -> PunR {
        PunR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Pulldown: 0=pulldown enabled, 1=normal (off)
    #[inline(always)]
    pub fn pdn(&self) -> PdnR {
        PdnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz)
    #[inline(always)]
    pub fn lowemi(&self) -> LowemiR {
        LowemiR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Input enable: 0=disabled, 1=enabled
    #[inline(always)]
    pub fn enzi(&mut self) -> EnziW<'_, IoI2s1LrckSpec> {
        EnziW::new(self, 0)
    }
    ///Bit 8 - Pullup: 0=pullup enabled, 1=normal (off)
    #[inline(always)]
    pub fn pun(&mut self) -> PunW<'_, IoI2s1LrckSpec> {
        PunW::new(self, 8)
    }
    ///Bit 16 - Pulldown: 0=pulldown enabled, 1=normal (off)
    #[inline(always)]
    pub fn pdn(&mut self) -> PdnW<'_, IoI2s1LrckSpec> {
        PdnW::new(self, 16)
    }
    ///Bit 24 - Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz)
    #[inline(always)]
    pub fn lowemi(&mut self) -> LowemiW<'_, IoI2s1LrckSpec> {
        LowemiW::new(self, 24)
    }
}
/**IOCELL control for I2S1_LRCK pin

You can [`read`](crate::Reg::read) this register and get [`io_i2s1_lrck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_i2s1_lrck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IoI2s1LrckSpec;
impl crate::RegisterSpec for IoI2s1LrckSpec {
    type Ux = u32;
}
///`read()` method returns [`io_i2s1_lrck::R`](R) reader structure
impl crate::Readable for IoI2s1LrckSpec {}
///`write(|w| ..)` method takes [`io_i2s1_lrck::W`](W) writer structure
impl crate::Writable for IoI2s1LrckSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IO_I2S1_LRCK to value 0x0101_0100
impl crate::Resettable for IoI2s1LrckSpec {
    const RESET_VALUE: u32 = 0x0101_0100;
}
