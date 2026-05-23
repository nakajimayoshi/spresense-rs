///Register `GEAR_N_IMG_VENB` reader
pub type R = crate::R<GearNImgVenbSpec>;
///Register `GEAR_N_IMG_VENB` writer
pub type W = crate::W<GearNImgVenbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**VENB_N clock setting

You can [`read`](crate::Reg::read) this register and get [`gear_n_img_venb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_n_img_venb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GearNImgVenbSpec;
impl crate::RegisterSpec for GearNImgVenbSpec {
    type Ux = u32;
}
///`read()` method returns [`gear_n_img_venb::R`](R) reader structure
impl crate::Readable for GearNImgVenbSpec {}
///`write(|w| ..)` method takes [`gear_n_img_venb::W`](W) writer structure
impl crate::Writable for GearNImgVenbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GEAR_N_IMG_VENB to value 0
impl crate::Resettable for GearNImgVenbSpec {}
