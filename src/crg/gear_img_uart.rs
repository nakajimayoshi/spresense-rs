#[doc = "Register `GEAR_IMG_UART` reader"]
pub type R = crate::R<GearImgUartSpec>;
#[doc = "Register `GEAR_IMG_UART` writer"]
pub type W = crate::W<GearImgUartSpec>;
#[doc = "Field `GEAR_M_UART` reader - "]
pub type GearMUartR = crate::FieldReader;
#[doc = "Field `GEAR_M_UART` writer - "]
pub type GearMUartW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GEAR_N_UART` reader - "]
pub type GearNUartR = crate::BitReader;
#[doc = "Field `GEAR_N_UART` writer - "]
pub type GearNUartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn gear_m_uart(&self) -> GearMUartR {
        GearMUartR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gear_n_uart(&self) -> GearNUartR {
        GearNUartR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn gear_m_uart(&mut self) -> GearMUartW<'_, GearImgUartSpec> {
        GearMUartW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gear_n_uart(&mut self) -> GearNUartW<'_, GearImgUartSpec> {
        GearNUartW::new(self, 16)
    }
}
#[doc = "IMG UART clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_img_uart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_img_uart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GearImgUartSpec;
impl crate::RegisterSpec for GearImgUartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gear_img_uart::R`](R) reader structure"]
impl crate::Readable for GearImgUartSpec {}
#[doc = "`write(|w| ..)` method takes [`gear_img_uart::W`](W) writer structure"]
impl crate::Writable for GearImgUartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEAR_IMG_UART to value 0x04"]
impl crate::Resettable for GearImgUartSpec {
    const RESET_VALUE: u32 = 0x04;
}
