///Register `PWD_STAT` reader
pub type R = crate::R<PwdStatSpec>;
///Field `SCU` reader - Sensor Control Unit domain powered on
pub type ScuR = crate::BitReader;
///Field `CORE` reader - Core domain powered on
pub type CoreR = crate::BitReader;
///Field `SYSIOP` reader - SYSIOP domain powered on
pub type SysiopR = crate::BitReader;
///Field `SYSIOP_SUB` reader - SYSIOP sub-domain powered on
pub type SysiopSubR = crate::BitReader;
///Field `APP` reader - Application domain powered on
pub type AppR = crate::BitReader;
///Field `APP_DSP` reader - Application DSP domain powered on
pub type AppDspR = crate::BitReader;
///Field `APP_SUB` reader - Application sub-domain powered on
pub type AppSubR = crate::BitReader;
///Field `GNSS_ITP` reader - GNSS ITP domain powered on
pub type GnssItpR = crate::BitReader;
///Field `GNSS` reader - GNSS domain powered on
pub type GnssR = crate::BitReader;
///Field `APP_AUDIO` reader - Application audio domain powered on
pub type AppAudioR = crate::BitReader;
impl R {
    ///Bit 0 - Sensor Control Unit domain powered on
    #[inline(always)]
    pub fn scu(&self) -> ScuR {
        ScuR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Core domain powered on
    #[inline(always)]
    pub fn core(&self) -> CoreR {
        CoreR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYSIOP domain powered on
    #[inline(always)]
    pub fn sysiop(&self) -> SysiopR {
        SysiopR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SYSIOP sub-domain powered on
    #[inline(always)]
    pub fn sysiop_sub(&self) -> SysiopSubR {
        SysiopSubR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Application domain powered on
    #[inline(always)]
    pub fn app(&self) -> AppR {
        AppR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Application DSP domain powered on
    #[inline(always)]
    pub fn app_dsp(&self) -> AppDspR {
        AppDspR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Application sub-domain powered on
    #[inline(always)]
    pub fn app_sub(&self) -> AppSubR {
        AppSubR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - GNSS ITP domain powered on
    #[inline(always)]
    pub fn gnss_itp(&self) -> GnssItpR {
        GnssItpR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GNSS domain powered on
    #[inline(always)]
    pub fn gnss(&self) -> GnssR {
        GnssR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Application audio domain powered on
    #[inline(always)]
    pub fn app_audio(&self) -> AppAudioR {
        AppAudioR::new(((self.bits >> 14) & 1) != 0)
    }
}
/**Power-domain status (read-only mirror of PWD_CTL)

You can [`read`](crate::Reg::read) this register and get [`pwd_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PwdStatSpec;
impl crate::RegisterSpec for PwdStatSpec {
    type Ux = u32;
}
///`read()` method returns [`pwd_stat::R`](R) reader structure
impl crate::Readable for PwdStatSpec {}
///`reset()` method sets PWD_STAT to value 0
impl crate::Resettable for PwdStatSpec {}
