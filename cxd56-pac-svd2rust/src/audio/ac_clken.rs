///Register `AC_CLKEN` reader
pub type R = crate::R<AcClkenSpec>;
///Register `AC_CLKEN` writer
pub type W = crate::W<AcClkenSpec>;
///Field `FS_FS` reader - FS mode (mic decimation)
pub type FsFsR = crate::BitReader;
///Field `FS_FS` writer - FS mode (mic decimation)
pub type FsFsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DECIM0_EN` reader - Decimation filter 0 enable
pub type Decim0EnR = crate::BitReader;
///Field `DECIM0_EN` writer - Decimation filter 0 enable
pub type Decim0EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DECIM1_EN` reader - Decimation filter 1 enable
pub type Decim1EnR = crate::BitReader;
///Field `DECIM1_EN` writer - Decimation filter 1 enable
pub type Decim1EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDES_EN` reader - SDES enable
pub type SdesEnR = crate::BitReader;
///Field `SDES_EN` writer - SDES enable
pub type SdesEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCK_AHBMSTR_EN` reader - AHB-master (audio DMA) clock enable
pub type MckAhbmstrEnR = crate::BitReader;
///Field `MCK_AHBMSTR_EN` writer - AHB-master (audio DMA) clock enable
pub type MckAhbmstrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FS mode (mic decimation)
    #[inline(always)]
    pub fn fs_fs(&self) -> FsFsR {
        FsFsR::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Decimation filter 0 enable
    #[inline(always)]
    pub fn decim0_en(&self) -> Decim0EnR {
        Decim0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Decimation filter 1 enable
    #[inline(always)]
    pub fn decim1_en(&self) -> Decim1EnR {
        Decim1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SDES enable
    #[inline(always)]
    pub fn sdes_en(&self) -> SdesEnR {
        SdesEnR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - AHB-master (audio DMA) clock enable
    #[inline(always)]
    pub fn mck_ahbmstr_en(&self) -> MckAhbmstrEnR {
        MckAhbmstrEnR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FS mode (mic decimation)
    #[inline(always)]
    pub fn fs_fs(&mut self) -> FsFsW<'_, AcClkenSpec> {
        FsFsW::new(self, 0)
    }
    ///Bit 16 - Decimation filter 0 enable
    #[inline(always)]
    pub fn decim0_en(&mut self) -> Decim0EnW<'_, AcClkenSpec> {
        Decim0EnW::new(self, 16)
    }
    ///Bit 17 - Decimation filter 1 enable
    #[inline(always)]
    pub fn decim1_en(&mut self) -> Decim1EnW<'_, AcClkenSpec> {
        Decim1EnW::new(self, 17)
    }
    ///Bit 18 - SDES enable
    #[inline(always)]
    pub fn sdes_en(&mut self) -> SdesEnW<'_, AcClkenSpec> {
        SdesEnW::new(self, 18)
    }
    ///Bit 19 - AHB-master (audio DMA) clock enable
    #[inline(always)]
    pub fn mck_ahbmstr_en(&mut self) -> MckAhbmstrEnW<'_, AcClkenSpec> {
        MckAhbmstrEnW::new(self, 19)
    }
}
/**Audio block clock enables

You can [`read`](crate::Reg::read) this register and get [`ac_clken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ac_clken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcClkenSpec;
impl crate::RegisterSpec for AcClkenSpec {
    type Ux = u32;
}
///`read()` method returns [`ac_clken::R`](R) reader structure
impl crate::Readable for AcClkenSpec {}
///`write(|w| ..)` method takes [`ac_clken::W`](W) writer structure
impl crate::Writable for AcClkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AC_CLKEN to value 0
impl crate::Resettable for AcClkenSpec {}
