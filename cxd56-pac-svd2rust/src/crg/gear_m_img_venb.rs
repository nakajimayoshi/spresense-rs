///Register `GEAR_M_IMG_VENB` reader
pub type R = crate::R<GearMImgVenbSpec>;
///Register `GEAR_M_IMG_VENB` writer
pub type W = crate::W<GearMImgVenbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**VENB_M clock setting

You can [`read`](crate::Reg::read) this register and get [`gear_m_img_venb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_m_img_venb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GearMImgVenbSpec;
impl crate::RegisterSpec for GearMImgVenbSpec {
    type Ux = u32;
}
///`read()` method returns [`gear_m_img_venb::R`](R) reader structure
impl crate::Readable for GearMImgVenbSpec {}
///`write(|w| ..)` method takes [`gear_m_img_venb::W`](W) writer structure
impl crate::Writable for GearMImgVenbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GEAR_M_IMG_VENB to value 0
impl crate::Resettable for GearMImgVenbSpec {}
