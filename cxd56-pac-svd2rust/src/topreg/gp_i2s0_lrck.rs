///Register `GP_I2S0_LRCK` reader
pub type R = crate::R<GpI2s0LrckSpec>;
///Register `GP_I2S0_LRCK` writer
pub type W = crate::W<GpI2s0LrckSpec>;
///Field `IN` reader - Sampled pin level (read)
pub type InR = crate::BitReader;
///Field `IN` writer - Sampled pin level (read)
pub type InW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT` reader - Output data
pub type OutR = crate::BitReader;
///Field `OUT` writer - Output data
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - Output enable, active-low (0 = drive output, 1 = high-Z input)
pub type DirR = crate::BitReader;
///Field `DIR` writer - Output enable, active-low (0 = drive output, 1 = high-Z input)
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sampled pin level (read)
    #[inline(always)]
    pub fn in_(&self) -> InR {
        InR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Output data
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Output enable, active-low (0 = drive output, 1 = high-Z input)
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Sampled pin level (read)
    #[inline(always)]
    pub fn in_(&mut self) -> InW<'_, GpI2s0LrckSpec> {
        InW::new(self, 0)
    }
    ///Bit 8 - Output data
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, GpI2s0LrckSpec> {
        OutW::new(self, 8)
    }
    ///Bit 16 - Output enable, active-low (0 = drive output, 1 = high-Z input)
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, GpI2s0LrckSpec> {
        DirW::new(self, 16)
    }
}
/**GPIO APP pin 94 — I2S0_LRCK / Arduino D25 (JP1)

You can [`read`](crate::Reg::read) this register and get [`gp_i2s0_lrck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2s0_lrck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GpI2s0LrckSpec;
impl crate::RegisterSpec for GpI2s0LrckSpec {
    type Ux = u32;
}
///`read()` method returns [`gp_i2s0_lrck::R`](R) reader structure
impl crate::Readable for GpI2s0LrckSpec {}
///`write(|w| ..)` method takes [`gp_i2s0_lrck::W`](W) writer structure
impl crate::Writable for GpI2s0LrckSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GP_I2S0_LRCK to value 0x0001_0000
impl crate::Resettable for GpI2s0LrckSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
