///Register `GEAR_PER_USB` reader
pub type R = crate::R<GearPerUsbSpec>;
///Register `GEAR_PER_USB` writer
pub type W = crate::W<GearPerUsbSpec>;
///Field `GEAR_M_USB` reader -
pub type GearMUsbR = crate::FieldReader<u16>;
///Field `GEAR_M_USB` writer -
pub type GearMUsbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `GEAR_N_USB` reader -
pub type GearNUsbR = crate::BitReader;
///Field `GEAR_N_USB` writer -
pub type GearNUsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn gear_m_usb(&self) -> GearMUsbR {
        GearMUsbR::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 16
    #[inline(always)]
    pub fn gear_n_usb(&self) -> GearNUsbR {
        GearNUsbR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    pub fn gear_m_usb(&mut self) -> GearMUsbW<'_, GearPerUsbSpec> {
        GearMUsbW::new(self, 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn gear_n_usb(&mut self) -> GearNUsbW<'_, GearPerUsbSpec> {
        GearNUsbW::new(self, 16)
    }
}
/**USB clock setting

You can [`read`](crate::Reg::read) this register and get [`gear_per_usb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_per_usb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GearPerUsbSpec;
impl crate::RegisterSpec for GearPerUsbSpec {
    type Ux = u32;
}
///`read()` method returns [`gear_per_usb::R`](R) reader structure
impl crate::Readable for GearPerUsbSpec {}
///`write(|w| ..)` method takes [`gear_per_usb::W`](W) writer structure
impl crate::Writable for GearPerUsbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GEAR_PER_USB to value 0x02
impl crate::Resettable for GearPerUsbSpec {
    const RESET_VALUE: u32 = 0x02;
}
