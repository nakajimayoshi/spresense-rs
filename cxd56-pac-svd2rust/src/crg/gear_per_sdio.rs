#[doc = "Register `GEAR_PER_SDIO` reader"]
pub type R = crate::R<GearPerSdioSpec>;
#[doc = "Register `GEAR_PER_SDIO` writer"]
pub type W = crate::W<GearPerSdioSpec>;
#[doc = "Field `GEAR_M_SDIO` reader - "]
pub type GearMSdioR = crate::FieldReader;
#[doc = "Field `GEAR_M_SDIO` writer - "]
pub type GearMSdioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GEAR_N_SDIO` reader - "]
pub type GearNSdioR = crate::BitReader;
#[doc = "Field `GEAR_N_SDIO` writer - "]
pub type GearNSdioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gear_m_sdio(&self) -> GearMSdioR {
        GearMSdioR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gear_n_sdio(&self) -> GearNSdioR {
        GearNSdioR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gear_m_sdio(&mut self) -> GearMSdioW<'_, GearPerSdioSpec> {
        GearMSdioW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gear_n_sdio(&mut self) -> GearNSdioW<'_, GearPerSdioSpec> {
        GearNSdioW::new(self, 16)
    }
}
#[doc = "SDIO clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_per_sdio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_per_sdio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GearPerSdioSpec;
impl crate::RegisterSpec for GearPerSdioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gear_per_sdio::R`](R) reader structure"]
impl crate::Readable for GearPerSdioSpec {}
#[doc = "`write(|w| ..)` method takes [`gear_per_sdio::W`](W) writer structure"]
impl crate::Writable for GearPerSdioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEAR_PER_SDIO to value 0x02"]
impl crate::Resettable for GearPerSdioSpec {
    const RESET_VALUE: u32 = 0x02;
}
