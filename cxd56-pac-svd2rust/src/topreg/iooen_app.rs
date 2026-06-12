///Register `IOOEN_APP` reader
pub type R = crate::R<IooenAppSpec>;
///Register `IOOEN_APP` writer
pub type W = crate::W<IooenAppSpec>;
///Field `I2S0_BCK` reader - I2S0_BCK pad output (0=enable, 1=disable)
pub type I2s0BckR = crate::BitReader;
///Field `I2S0_BCK` writer - I2S0_BCK pad output (0=enable, 1=disable)
pub type I2s0BckW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S0_LRCK` reader - I2S0_LRCK pad output (0=enable, 1=disable)
pub type I2s0LrckR = crate::BitReader;
///Field `I2S0_LRCK` writer - I2S0_LRCK pad output (0=enable, 1=disable)
pub type I2s0LrckW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S1_BCK` reader - I2S1_BCK pad output (0=enable, 1=disable)
pub type I2s1BckR = crate::BitReader;
///Field `I2S1_BCK` writer - I2S1_BCK pad output (0=enable, 1=disable)
pub type I2s1BckW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S1_LRCK` reader - I2S1_LRCK pad output (0=enable, 1=disable)
pub type I2s1LrckR = crate::BitReader;
///Field `I2S1_LRCK` writer - I2S1_LRCK pad output (0=enable, 1=disable)
pub type I2s1LrckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2S0_BCK pad output (0=enable, 1=disable)
    #[inline(always)]
    pub fn i2s0_bck(&self) -> I2s0BckR {
        I2s0BckR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2S0_LRCK pad output (0=enable, 1=disable)
    #[inline(always)]
    pub fn i2s0_lrck(&self) -> I2s0LrckR {
        I2s0LrckR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - I2S1_BCK pad output (0=enable, 1=disable)
    #[inline(always)]
    pub fn i2s1_bck(&self) -> I2s1BckR {
        I2s1BckR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I2S1_LRCK pad output (0=enable, 1=disable)
    #[inline(always)]
    pub fn i2s1_lrck(&self) -> I2s1LrckR {
        I2s1LrckR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I2S0_BCK pad output (0=enable, 1=disable)
    #[inline(always)]
    pub fn i2s0_bck(&mut self) -> I2s0BckW<'_, IooenAppSpec> {
        I2s0BckW::new(self, 0)
    }
    ///Bit 1 - I2S0_LRCK pad output (0=enable, 1=disable)
    #[inline(always)]
    pub fn i2s0_lrck(&mut self) -> I2s0LrckW<'_, IooenAppSpec> {
        I2s0LrckW::new(self, 1)
    }
    ///Bit 4 - I2S1_BCK pad output (0=enable, 1=disable)
    #[inline(always)]
    pub fn i2s1_bck(&mut self) -> I2s1BckW<'_, IooenAppSpec> {
        I2s1BckW::new(self, 4)
    }
    ///Bit 5 - I2S1_LRCK pad output (0=enable, 1=disable)
    #[inline(always)]
    pub fn i2s1_lrck(&mut self) -> I2s1LrckW<'_, IooenAppSpec> {
        I2s1LrckW::new(self, 5)
    }
}
/**I2S BCK/LRCK pad output enable (active-low: 0=enable, 1=disable)

You can [`read`](crate::Reg::read) this register and get [`iooen_app::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iooen_app::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IooenAppSpec;
impl crate::RegisterSpec for IooenAppSpec {
    type Ux = u32;
}
///`read()` method returns [`iooen_app::R`](R) reader structure
impl crate::Readable for IooenAppSpec {}
///`write(|w| ..)` method takes [`iooen_app::W`](W) writer structure
impl crate::Writable for IooenAppSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOOEN_APP to value 0x33
impl crate::Resettable for IooenAppSpec {
    const RESET_VALUE: u32 = 0x33;
}
