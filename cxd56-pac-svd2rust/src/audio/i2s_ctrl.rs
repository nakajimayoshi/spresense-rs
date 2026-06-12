///Register `I2S_CTRL` reader
pub type R = crate::R<I2sCtrlSpec>;
///Register `I2S_CTRL` writer
pub type W = crate::W<I2sCtrlSpec>;
///Field `DSR_RATE` reader - Codec DSP downsampling rate
pub type DsrRateR = crate::FieldReader;
///Field `DSR_RATE` writer - Codec DSP downsampling rate
pub type DsrRateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIGSFT` reader - Digital soft-ramp enable
pub type DigsftR = crate::BitReader;
///Field `DIGSFT` writer - Digital soft-ramp enable
pub type DigsftW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRC1` reader - I2S0 SRC frequency band: 0=prohibited, 1=8-48kHz, 2=48-96kHz, 3=96-192kHz
pub type Src1R = crate::FieldReader;
///Field `SRC1` writer - I2S0 SRC frequency band: 0=prohibited, 1=8-48kHz, 2=48-96kHz, 3=96-192kHz
pub type Src1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SRC1IN_SEL` reader - I2S0 SRC input source select (0=I2S0 DMA via AU_DAT_SEL1)
pub type Src1inSelR = crate::FieldReader;
///Field `SRC1IN_SEL` writer - I2S0 SRC input source select (0=I2S0 DMA via AU_DAT_SEL1)
pub type Src1inSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2S0_FMT` reader - I2S0 data format (DIF1): 0=I2S, 1=Left-justified
pub type I2s0FmtR = crate::BitReader;
///Field `I2S0_FMT` writer - I2S0 data format (DIF1): 0=I2S, 1=Left-justified
pub type I2s0FmtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD1MASTER` reader - I2S0 clock mode: 0=slave, 1=master
pub type Sd1masterR = crate::BitReader;
///Field `SD1MASTER` writer - I2S0 clock mode: 0=slave, 1=master
pub type Sd1masterW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDCK_OUTENX` reader - Master BCK/LRCK pad output (active-low): 0=output enable, 1=output disable
pub type SdckOutenxR = crate::BitReader;
///Field `SDCK_OUTENX` writer - Master BCK/LRCK pad output (active-low): 0=output enable, 1=output disable
pub type SdckOutenxW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HI_RES_MODE` reader - Resolution: 0=normal (up to 48kHz), 1=hi-res (192kHz)
pub type HiResModeR = crate::BitReader;
///Field `HI_RES_MODE` writer - Resolution: 0=normal (up to 48kHz), 1=hi-res (192kHz)
pub type HiResModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Codec DSP downsampling rate
    #[inline(always)]
    pub fn dsr_rate(&self) -> DsrRateR {
        DsrRateR::new((self.bits & 7) as u8)
    }
    ///Bit 12 - Digital soft-ramp enable
    #[inline(always)]
    pub fn digsft(&self) -> DigsftR {
        DigsftR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:17 - I2S0 SRC frequency band: 0=prohibited, 1=8-48kHz, 2=48-96kHz, 3=96-192kHz
    #[inline(always)]
    pub fn src1(&self) -> Src1R {
        Src1R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - I2S0 SRC input source select (0=I2S0 DMA via AU_DAT_SEL1)
    #[inline(always)]
    pub fn src1in_sel(&self) -> Src1inSelR {
        Src1inSelR::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 27 - I2S0 data format (DIF1): 0=I2S, 1=Left-justified
    #[inline(always)]
    pub fn i2s0_fmt(&self) -> I2s0FmtR {
        I2s0FmtR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - I2S0 clock mode: 0=slave, 1=master
    #[inline(always)]
    pub fn sd1master(&self) -> Sd1masterR {
        Sd1masterR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Master BCK/LRCK pad output (active-low): 0=output enable, 1=output disable
    #[inline(always)]
    pub fn sdck_outenx(&self) -> SdckOutenxR {
        SdckOutenxR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Resolution: 0=normal (up to 48kHz), 1=hi-res (192kHz)
    #[inline(always)]
    pub fn hi_res_mode(&self) -> HiResModeR {
        HiResModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Codec DSP downsampling rate
    #[inline(always)]
    pub fn dsr_rate(&mut self) -> DsrRateW<'_, I2sCtrlSpec> {
        DsrRateW::new(self, 0)
    }
    ///Bit 12 - Digital soft-ramp enable
    #[inline(always)]
    pub fn digsft(&mut self) -> DigsftW<'_, I2sCtrlSpec> {
        DigsftW::new(self, 12)
    }
    ///Bits 16:17 - I2S0 SRC frequency band: 0=prohibited, 1=8-48kHz, 2=48-96kHz, 3=96-192kHz
    #[inline(always)]
    pub fn src1(&mut self) -> Src1W<'_, I2sCtrlSpec> {
        Src1W::new(self, 16)
    }
    ///Bits 18:19 - I2S0 SRC input source select (0=I2S0 DMA via AU_DAT_SEL1)
    #[inline(always)]
    pub fn src1in_sel(&mut self) -> Src1inSelW<'_, I2sCtrlSpec> {
        Src1inSelW::new(self, 18)
    }
    ///Bit 27 - I2S0 data format (DIF1): 0=I2S, 1=Left-justified
    #[inline(always)]
    pub fn i2s0_fmt(&mut self) -> I2s0FmtW<'_, I2sCtrlSpec> {
        I2s0FmtW::new(self, 27)
    }
    ///Bit 29 - I2S0 clock mode: 0=slave, 1=master
    #[inline(always)]
    pub fn sd1master(&mut self) -> Sd1masterW<'_, I2sCtrlSpec> {
        Sd1masterW::new(self, 29)
    }
    ///Bit 30 - Master BCK/LRCK pad output (active-low): 0=output enable, 1=output disable
    #[inline(always)]
    pub fn sdck_outenx(&mut self) -> SdckOutenxW<'_, I2sCtrlSpec> {
        SdckOutenxW::new(self, 30)
    }
    ///Bit 31 - Resolution: 0=normal (up to 48kHz), 1=hi-res (192kHz)
    #[inline(always)]
    pub fn hi_res_mode(&mut self) -> HiResModeW<'_, I2sCtrlSpec> {
        HiResModeW::new(self, 31)
    }
}
/**Audio Control Register 0 — I2S0 (SD1) master/slave, format and SRC routing

You can [`read`](crate::Reg::read) this register and get [`i2s_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2sCtrlSpec;
impl crate::RegisterSpec for I2sCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s_ctrl::R`](R) reader structure
impl crate::Readable for I2sCtrlSpec {}
///`write(|w| ..)` method takes [`i2s_ctrl::W`](W) writer structure
impl crate::Writable for I2sCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S_CTRL to value 0x0c99_1000
impl crate::Resettable for I2sCtrlSpec {
    const RESET_VALUE: u32 = 0x0c99_1000;
}
