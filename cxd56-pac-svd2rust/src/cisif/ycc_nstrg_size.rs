///Register `YCC_NSTRG_SIZE` reader
pub type R = crate::R<YccNstrgSizeSpec>;
///Register `YCC_NSTRG_SIZE` writer
pub type W = crate::W<YccNstrgSizeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**YCC data frame memory notice of storage size

You can [`read`](crate::Reg::read) this register and get [`ycc_nstrg_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ycc_nstrg_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct YccNstrgSizeSpec;
impl crate::RegisterSpec for YccNstrgSizeSpec {
    type Ux = u32;
}
///`read()` method returns [`ycc_nstrg_size::R`](R) reader structure
impl crate::Readable for YccNstrgSizeSpec {}
///`write(|w| ..)` method takes [`ycc_nstrg_size::W`](W) writer structure
impl crate::Writable for YccNstrgSizeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets YCC_NSTRG_SIZE to value 0
impl crate::Resettable for YccNstrgSizeSpec {}
