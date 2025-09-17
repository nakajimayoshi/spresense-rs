#[doc = "Register `GEAR_IMG_WSPI` reader"]
pub type R = crate::R<GearImgWspiSpec>;
#[doc = "Register `GEAR_IMG_WSPI` writer"]
pub type W = crate::W<GearImgWspiSpec>;
#[doc = "Field `GEAR_M_IMG_WSPI` reader - "]
pub type GearMImgWspiR = crate::FieldReader;
#[doc = "Field `GEAR_M_IMG_WSPI` writer - "]
pub type GearMImgWspiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GEAR_N_IMG_WSPI` reader - "]
pub type GearNImgWspiR = crate::BitReader;
#[doc = "Field `GEAR_N_IMG_WSPI` writer - "]
pub type GearNImgWspiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gear_m_img_wspi(&self) -> GearMImgWspiR {
        GearMImgWspiR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gear_n_img_wspi(&self) -> GearNImgWspiR {
        GearNImgWspiR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gear_m_img_wspi(&mut self) -> GearMImgWspiW<'_, GearImgWspiSpec> {
        GearMImgWspiW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gear_n_img_wspi(&mut self) -> GearNImgWspiW<'_, GearImgWspiSpec> {
        GearNImgWspiW::new(self, 16)
    }
}
#[doc = "IMG WSPI clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_img_wspi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_img_wspi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GearImgWspiSpec;
impl crate::RegisterSpec for GearImgWspiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gear_img_wspi::R`](R) reader structure"]
impl crate::Readable for GearImgWspiSpec {}
#[doc = "`write(|w| ..)` method takes [`gear_img_wspi::W`](W) writer structure"]
impl crate::Writable for GearImgWspiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEAR_IMG_WSPI to value 0x04"]
impl crate::Resettable for GearImgWspiSpec {
    const RESET_VALUE: u32 = 0x04;
}
