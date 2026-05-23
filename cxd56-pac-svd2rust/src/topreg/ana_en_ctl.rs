///Register `ANA_EN_CTL` reader
pub type R = crate::R<AnaEnCtlSpec>;
///Register `ANA_EN_CTL` writer
pub type W = crate::W<AnaEnCtlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Analog enable control (paired set/clear bits — use raw write)

You can [`read`](crate::Reg::read) this register and get [`ana_en_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_en_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AnaEnCtlSpec;
impl crate::RegisterSpec for AnaEnCtlSpec {
    type Ux = u32;
}
///`read()` method returns [`ana_en_ctl::R`](R) reader structure
impl crate::Readable for AnaEnCtlSpec {}
///`write(|w| ..)` method takes [`ana_en_ctl::W`](W) writer structure
impl crate::Writable for AnaEnCtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ANA_EN_CTL to value 0
impl crate::Resettable for AnaEnCtlSpec {}
