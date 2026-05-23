///Register `GNSS_DIV` reader
pub type R = crate::R<GnssDivSpec>;
///Register `GNSS_DIV` writer
pub type W = crate::W<GnssDivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**GNSS clock divider

You can [`read`](crate::Reg::read) this register and get [`gnss_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnss_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GnssDivSpec;
impl crate::RegisterSpec for GnssDivSpec {
    type Ux = u32;
}
///`read()` method returns [`gnss_div::R`](R) reader structure
impl crate::Readable for GnssDivSpec {}
///`write(|w| ..)` method takes [`gnss_div::W`](W) writer structure
impl crate::Writable for GnssDivSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GNSS_DIV to value 0
impl crate::Resettable for GnssDivSpec {}
