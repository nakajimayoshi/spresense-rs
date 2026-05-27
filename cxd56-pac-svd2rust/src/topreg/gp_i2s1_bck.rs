///Register `GP_I2S1_BCK` reader
pub type R = crate::R<GpI2s1BckSpec>;
///Register `GP_I2S1_BCK` writer
pub type W = crate::W<GpI2s1BckSpec>;
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
    pub fn in_(&mut self) -> InW<'_, GpI2s1BckSpec> {
        InW::new(self, 0)
    }
    ///Bit 8 - Output data
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, GpI2s1BckSpec> {
        OutW::new(self, 8)
    }
    ///Bit 16 - Output enable, active-low (0 = drive output, 1 = high-Z input)
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, GpI2s1BckSpec> {
        DirW::new(self, 16)
    }
}
/**GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`gp_i2s1_bck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2s1_bck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GpI2s1BckSpec;
impl crate::RegisterSpec for GpI2s1BckSpec {
    type Ux = u32;
}
///`read()` method returns [`gp_i2s1_bck::R`](R) reader structure
impl crate::Readable for GpI2s1BckSpec {}
///`write(|w| ..)` method takes [`gp_i2s1_bck::W`](W) writer structure
impl crate::Writable for GpI2s1BckSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GP_I2S1_BCK to value 0x0001_0000
impl crate::Resettable for GpI2s1BckSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
