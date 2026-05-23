///Register `YCC_DAREA_SIZE` reader
pub type R = crate::R<YccDareaSizeSpec>;
///Register `YCC_DAREA_SIZE` writer
pub type W = crate::W<YccDareaSizeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**YCC data frame memory area size

You can [`read`](crate::Reg::read) this register and get [`ycc_darea_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ycc_darea_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct YccDareaSizeSpec;
impl crate::RegisterSpec for YccDareaSizeSpec {
    type Ux = u32;
}
///`read()` method returns [`ycc_darea_size::R`](R) reader structure
impl crate::Readable for YccDareaSizeSpec {}
///`write(|w| ..)` method takes [`ycc_darea_size::W`](W) writer structure
impl crate::Writable for YccDareaSizeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets YCC_DAREA_SIZE to value 0
impl crate::Resettable for YccDareaSizeSpec {}
