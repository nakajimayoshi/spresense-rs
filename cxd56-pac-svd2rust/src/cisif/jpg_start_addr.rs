#[doc = "Register `JPG_START_ADDR` reader"]
pub type R = crate::R<JpgStartAddrSpec>;
#[doc = "Register `JPG_START_ADDR` writer"]
pub type W = crate::W<JpgStartAddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "JPEG data frame memory start address\n\nYou can [`read`](crate::Reg::read) this register and get [`jpg_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpg_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JpgStartAddrSpec;
impl crate::RegisterSpec for JpgStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jpg_start_addr::R`](R) reader structure"]
impl crate::Readable for JpgStartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`jpg_start_addr::W`](W) writer structure"]
impl crate::Writable for JpgStartAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JPG_START_ADDR to value 0"]
impl crate::Resettable for JpgStartAddrSpec {}
