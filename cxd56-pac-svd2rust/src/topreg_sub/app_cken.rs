///Register `APP_CKEN` reader
pub type R = crate::R<AppCkenSpec>;
///Register `APP_CKEN` writer
pub type W = crate::W<AppCkenSpec>;
///Field `APP_CPU` reader - APP CPU clock enable
pub type AppCpuR = crate::BitReader;
///Field `APP_CPU` writer - APP CPU clock enable
pub type AppCpuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_MCLK` reader - APP MCLK enable
pub type AppMclkR = crate::BitReader;
///Field `APP_MCLK` writer - APP MCLK enable
pub type AppMclkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_AHB` reader - APP AHB clock enable
pub type AppAhbR = crate::BitReader;
///Field `APP_AHB` writer - APP AHB clock enable
pub type AppAhbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - APP CPU clock enable
    #[inline(always)]
    pub fn app_cpu(&self) -> AppCpuR {
        AppCpuR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - APP MCLK enable
    #[inline(always)]
    pub fn app_mclk(&self) -> AppMclkR {
        AppMclkR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - APP AHB clock enable
    #[inline(always)]
    pub fn app_ahb(&self) -> AppAhbR {
        AppAhbR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - APP CPU clock enable
    #[inline(always)]
    pub fn app_cpu(&mut self) -> AppCpuW<'_, AppCkenSpec> {
        AppCpuW::new(self, 0)
    }
    ///Bit 1 - APP MCLK enable
    #[inline(always)]
    pub fn app_mclk(&mut self) -> AppMclkW<'_, AppCkenSpec> {
        AppMclkW::new(self, 1)
    }
    ///Bit 3 - APP AHB clock enable
    #[inline(always)]
    pub fn app_ahb(&mut self) -> AppAhbW<'_, AppCkenSpec> {
        AppAhbW::new(self, 3)
    }
}
/**Application domain clock enables

You can [`read`](crate::Reg::read) this register and get [`app_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AppCkenSpec;
impl crate::RegisterSpec for AppCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`app_cken::R`](R) reader structure
impl crate::Readable for AppCkenSpec {}
///`write(|w| ..)` method takes [`app_cken::W`](W) writer structure
impl crate::Writable for AppCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APP_CKEN to value 0
impl crate::Resettable for AppCkenSpec {}
