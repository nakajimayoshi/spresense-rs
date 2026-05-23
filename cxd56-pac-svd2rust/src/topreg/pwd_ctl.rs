///Register `PWD_CTL` reader
pub type R = crate::R<PwdCtlSpec>;
///Register `PWD_CTL` writer
pub type W = crate::W<PwdCtlSpec>;
///Field `SCU` reader - Sensor Control Unit domain
pub type ScuR = crate::BitReader;
///Field `SCU` writer - Sensor Control Unit domain
pub type ScuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE` reader - Core domain
pub type CoreR = crate::BitReader;
///Field `CORE` writer - Core domain
pub type CoreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSIOP` reader - SYSIOP domain
pub type SysiopR = crate::BitReader;
///Field `SYSIOP` writer - SYSIOP domain
pub type SysiopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSIOP_SUB` reader - SYSIOP sub-domain
pub type SysiopSubR = crate::BitReader;
///Field `SYSIOP_SUB` writer - SYSIOP sub-domain
pub type SysiopSubW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP` reader - Application domain
pub type AppR = crate::BitReader;
///Field `APP` writer - Application domain
pub type AppW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_DSP` reader - Application DSP domain
pub type AppDspR = crate::BitReader;
///Field `APP_DSP` writer - Application DSP domain
pub type AppDspW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_SUB` reader - Application sub-domain
pub type AppSubR = crate::BitReader;
///Field `APP_SUB` writer - Application sub-domain
pub type AppSubW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GNSS_ITP` reader - GNSS ITP domain
pub type GnssItpR = crate::BitReader;
///Field `GNSS_ITP` writer - GNSS ITP domain
pub type GnssItpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GNSS` reader - GNSS domain
pub type GnssR = crate::BitReader;
///Field `GNSS` writer - GNSS domain
pub type GnssW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_AUDIO` reader - Application audio domain
pub type AppAudioR = crate::BitReader;
///Field `APP_AUDIO` writer - Application audio domain
pub type AppAudioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sensor Control Unit domain
    #[inline(always)]
    pub fn scu(&self) -> ScuR {
        ScuR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Core domain
    #[inline(always)]
    pub fn core(&self) -> CoreR {
        CoreR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYSIOP domain
    #[inline(always)]
    pub fn sysiop(&self) -> SysiopR {
        SysiopR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SYSIOP sub-domain
    #[inline(always)]
    pub fn sysiop_sub(&self) -> SysiopSubR {
        SysiopSubR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Application domain
    #[inline(always)]
    pub fn app(&self) -> AppR {
        AppR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Application DSP domain
    #[inline(always)]
    pub fn app_dsp(&self) -> AppDspR {
        AppDspR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Application sub-domain
    #[inline(always)]
    pub fn app_sub(&self) -> AppSubR {
        AppSubR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - GNSS ITP domain
    #[inline(always)]
    pub fn gnss_itp(&self) -> GnssItpR {
        GnssItpR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GNSS domain
    #[inline(always)]
    pub fn gnss(&self) -> GnssR {
        GnssR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Application audio domain
    #[inline(always)]
    pub fn app_audio(&self) -> AppAudioR {
        AppAudioR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Sensor Control Unit domain
    #[inline(always)]
    pub fn scu(&mut self) -> ScuW<'_, PwdCtlSpec> {
        ScuW::new(self, 0)
    }
    ///Bit 4 - Core domain
    #[inline(always)]
    pub fn core(&mut self) -> CoreW<'_, PwdCtlSpec> {
        CoreW::new(self, 4)
    }
    ///Bit 5 - SYSIOP domain
    #[inline(always)]
    pub fn sysiop(&mut self) -> SysiopW<'_, PwdCtlSpec> {
        SysiopW::new(self, 5)
    }
    ///Bit 6 - SYSIOP sub-domain
    #[inline(always)]
    pub fn sysiop_sub(&mut self) -> SysiopSubW<'_, PwdCtlSpec> {
        SysiopSubW::new(self, 6)
    }
    ///Bit 8 - Application domain
    #[inline(always)]
    pub fn app(&mut self) -> AppW<'_, PwdCtlSpec> {
        AppW::new(self, 8)
    }
    ///Bit 9 - Application DSP domain
    #[inline(always)]
    pub fn app_dsp(&mut self) -> AppDspW<'_, PwdCtlSpec> {
        AppDspW::new(self, 9)
    }
    ///Bit 10 - Application sub-domain
    #[inline(always)]
    pub fn app_sub(&mut self) -> AppSubW<'_, PwdCtlSpec> {
        AppSubW::new(self, 10)
    }
    ///Bit 12 - GNSS ITP domain
    #[inline(always)]
    pub fn gnss_itp(&mut self) -> GnssItpW<'_, PwdCtlSpec> {
        GnssItpW::new(self, 12)
    }
    ///Bit 13 - GNSS domain
    #[inline(always)]
    pub fn gnss(&mut self) -> GnssW<'_, PwdCtlSpec> {
        GnssW::new(self, 13)
    }
    ///Bit 14 - Application audio domain
    #[inline(always)]
    pub fn app_audio(&mut self) -> AppAudioW<'_, PwdCtlSpec> {
        AppAudioW::new(self, 14)
    }
}
/**Power-domain control (1 = powered on)

You can [`read`](crate::Reg::read) this register and get [`pwd_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PwdCtlSpec;
impl crate::RegisterSpec for PwdCtlSpec {
    type Ux = u32;
}
///`read()` method returns [`pwd_ctl::R`](R) reader structure
impl crate::Readable for PwdCtlSpec {}
///`write(|w| ..)` method takes [`pwd_ctl::W`](W) writer structure
impl crate::Writable for PwdCtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWD_CTL to value 0
impl crate::Resettable for PwdCtlSpec {}
