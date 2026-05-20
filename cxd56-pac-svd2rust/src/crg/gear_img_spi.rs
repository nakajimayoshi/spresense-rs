#[doc = "Register `GEAR_IMG_SPI` reader"]
pub type R = crate::R<GearImgSpiSpec>;
#[doc = "Register `GEAR_IMG_SPI` writer"]
pub type W = crate::W<GearImgSpiSpec>;
#[doc = "Field `GEAR_M_SPI` reader - "]
pub type GearMSpiR = crate::FieldReader;
#[doc = "Field `GEAR_M_SPI` writer - "]
pub type GearMSpiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GEAR_N_SPI` reader - "]
pub type GearNSpiR = crate::BitReader;
#[doc = "Field `GEAR_N_SPI` writer - "]
pub type GearNSpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn gear_m_spi(&self) -> GearMSpiR {
        GearMSpiR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gear_n_spi(&self) -> GearNSpiR {
        GearNSpiR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn gear_m_spi(&mut self) -> GearMSpiW<'_, GearImgSpiSpec> {
        GearMSpiW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gear_n_spi(&mut self) -> GearNSpiW<'_, GearImgSpiSpec> {
        GearNSpiW::new(self, 16)
    }
}
#[doc = "IMG SPI clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_img_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_img_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GearImgSpiSpec;
impl crate::RegisterSpec for GearImgSpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gear_img_spi::R`](R) reader structure"]
impl crate::Readable for GearImgSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`gear_img_spi::W`](W) writer structure"]
impl crate::Writable for GearImgSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEAR_IMG_SPI to value 0x04"]
impl crate::Resettable for GearImgSpiSpec {
    const RESET_VALUE: u32 = 0x04;
}
