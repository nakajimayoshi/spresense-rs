///Register `APP_DIV` reader
pub type R = crate::R<AppDivSpec>;
///Register `APP_DIV` writer
pub type W = crate::W<AppDivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Application domain clock divider

You can [`read`](crate::Reg::read) this register and get [`app_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AppDivSpec;
impl crate::RegisterSpec for AppDivSpec {
    type Ux = u32;
}
///`read()` method returns [`app_div::R`](R) reader structure
impl crate::Readable for AppDivSpec {}
///`write(|w| ..)` method takes [`app_div::W`](W) writer structure
impl crate::Writable for AppDivSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APP_DIV to value 0
impl crate::Resettable for AppDivSpec {}
