#[doc = "Register `YCC_START_ADDR` reader"]
pub type R = crate::R<YccStartAddrSpec>;
#[doc = "Register `YCC_START_ADDR` writer"]
pub type W = crate::W<YccStartAddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "YCC data frame memory start address\n\nYou can [`read`](crate::Reg::read) this register and get [`ycc_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ycc_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YccStartAddrSpec;
impl crate::RegisterSpec for YccStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ycc_start_addr::R`](R) reader structure"]
impl crate::Readable for YccStartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ycc_start_addr::W`](W) writer structure"]
impl crate::Writable for YccStartAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets YCC_START_ADDR to value 0"]
impl crate::Resettable for YccStartAddrSpec {}
