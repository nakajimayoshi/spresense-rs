///Register `IO_EMMC_DATA0` reader
pub type R = crate::R<IoEmmcData0Spec>;
///Register `IO_EMMC_DATA0` writer
pub type W = crate::W<IoEmmcData0Spec>;
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
    pub fn enzi(&mut self) -> EnziW<'_, IoEmmcData0Spec> {
        EnziW::new(self, 0)
    }
    ///Bit 8 - Pullup: 0=pullup enabled, 1=normal (off)
    #[inline(always)]
    pub fn pun(&mut self) -> PunW<'_, IoEmmcData0Spec> {
        PunW::new(self, 8)
    }
    ///Bit 16 - Pulldown: 0=pulldown enabled, 1=normal (off)
    #[inline(always)]
    pub fn pdn(&mut self) -> PdnW<'_, IoEmmcData0Spec> {
        PdnW::new(self, 16)
    }
    ///Bit 24 - Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz)
    #[inline(always)]
    pub fn lowemi(&mut self) -> LowemiW<'_, IoEmmcData0Spec> {
        LowemiW::new(self, 24)
    }
}
/**IOCELL control for EMMC_DATA0 / SPI5_MOSI pin

You can [`read`](crate::Reg::read) this register and get [`io_emmc_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_emmc_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IoEmmcData0Spec;
impl crate::RegisterSpec for IoEmmcData0Spec {
    type Ux = u32;
}
///`read()` method returns [`io_emmc_data0::R`](R) reader structure
impl crate::Readable for IoEmmcData0Spec {}
///`write(|w| ..)` method takes [`io_emmc_data0::W`](W) writer structure
impl crate::Writable for IoEmmcData0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IO_EMMC_DATA0 to value 0x0101_0100
impl crate::Resettable for IoEmmcData0Spec {
    const RESET_VALUE: u32 = 0x0101_0100;
}
