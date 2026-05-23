///Register `APP_CKSEL` reader
pub type R = crate::R<AppCkselSpec>;
///Register `APP_CKSEL` writer
pub type W = crate::W<AppCkselSpec>;
///Field `AUD_MCLK` reader - Audio MCLK source select
pub type AudMclkR = crate::FieldReader;
///Field `AUD_MCLK` writer - Audio MCLK source select
pub type AudMclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:17 - Audio MCLK source select
    #[inline(always)]
    pub fn aud_mclk(&self) -> AudMclkR {
        AudMclkR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    ///Bits 16:17 - Audio MCLK source select
    #[inline(always)]
    pub fn aud_mclk(&mut self) -> AudMclkW<'_, AppCkselSpec> {
        AudMclkW::new(self, 16)
    }
}
/**Application domain clock source select

You can [`read`](crate::Reg::read) this register and get [`app_cksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AppCkselSpec;
impl crate::RegisterSpec for AppCkselSpec {
    type Ux = u32;
}
///`read()` method returns [`app_cksel::R`](R) reader structure
impl crate::Readable for AppCkselSpec {}
///`write(|w| ..)` method takes [`app_cksel::W`](W) writer structure
impl crate::Writable for AppCkselSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APP_CKSEL to value 0
impl crate::Resettable for AppCkselSpec {}
