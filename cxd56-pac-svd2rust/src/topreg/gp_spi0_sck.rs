///Register `GP_SPI0_SCK` reader
pub type R = crate::R<GpSpi0SckSpec>;
///Register `GP_SPI0_SCK` writer
pub type W = crate::W<GpSpi0SckSpec>;
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
    pub fn in_(&mut self) -> InW<'_, GpSpi0SckSpec> {
        InW::new(self, 0)
    }
    ///Bit 8 - Output data
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, GpSpi0SckSpec> {
        OutW::new(self, 8)
    }
    ///Bit 16 - Output enable, active-low (0 = drive output, 1 = high-Z input)
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, GpSpi0SckSpec> {
        DirW::new(self, 16)
    }
}
/**GPIO SYS pin 18 — SPI0_SCK / UART1_RX (on-board console)

You can [`read`](crate::Reg::read) this register and get [`gp_spi0_sck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_spi0_sck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GpSpi0SckSpec;
impl crate::RegisterSpec for GpSpi0SckSpec {
    type Ux = u32;
}
///`read()` method returns [`gp_spi0_sck::R`](R) reader structure
impl crate::Readable for GpSpi0SckSpec {}
///`write(|w| ..)` method takes [`gp_spi0_sck::W`](W) writer structure
impl crate::Writable for GpSpi0SckSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GP_SPI0_SCK to value 0x0001_0000
impl crate::Resettable for GpSpi0SckSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
