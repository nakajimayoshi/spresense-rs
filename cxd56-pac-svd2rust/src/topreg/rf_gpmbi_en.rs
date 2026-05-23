///Register `RF_GPMBI_EN` reader
pub type R = crate::R<RfGpmbiEnSpec>;
///Register `RF_GPMBI_EN` writer
pub type W = crate::W<RfGpmbiEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**RF GP MBI enable

You can [`read`](crate::Reg::read) this register and get [`rf_gpmbi_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_gpmbi_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RfGpmbiEnSpec;
impl crate::RegisterSpec for RfGpmbiEnSpec {
    type Ux = u32;
}
///`read()` method returns [`rf_gpmbi_en::R`](R) reader structure
impl crate::Readable for RfGpmbiEnSpec {}
///`write(|w| ..)` method takes [`rf_gpmbi_en::W`](W) writer structure
impl crate::Writable for RfGpmbiEnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF_GPMBI_EN to value 0
impl crate::Resettable for RfGpmbiEnSpec {}
