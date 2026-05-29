///Register `IOCAPP_INTSEL1` reader
pub type R = crate::R<IocappIntsel1Spec>;
///Register `IOCAPP_INTSEL1` writer
pub type W = crate::W<IocappIntsel1Spec>;
///Field `SLOT10` reader - Pin index for APP slot 10 → EXDEVICE_10 (0x3f = unused)
pub type Slot10R = crate::FieldReader;
///Field `SLOT10` writer - Pin index for APP slot 10 → EXDEVICE_10 (0x3f = unused)
pub type Slot10W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SLOT11` reader - Pin index for APP slot 11 → EXDEVICE_11 (0x3f = unused)
pub type Slot11R = crate::FieldReader;
///Field `SLOT11` writer - Pin index for APP slot 11 → EXDEVICE_11 (0x3f = unused)
pub type Slot11W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Pin index for APP slot 10 → EXDEVICE_10 (0x3f = unused)
    #[inline(always)]
    pub fn slot10(&self) -> Slot10R {
        Slot10R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Pin index for APP slot 11 → EXDEVICE_11 (0x3f = unused)
    #[inline(always)]
    pub fn slot11(&self) -> Slot11R {
        Slot11R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - Pin index for APP slot 10 → EXDEVICE_10 (0x3f = unused)
    #[inline(always)]
    pub fn slot10(&mut self) -> Slot10W<'_, IocappIntsel1Spec> {
        Slot10W::new(self, 0)
    }
    ///Bits 8:13 - Pin index for APP slot 11 → EXDEVICE_11 (0x3f = unused)
    #[inline(always)]
    pub fn slot11(&mut self) -> Slot11W<'_, IocappIntsel1Spec> {
        Slot11W::new(self, 8)
    }
}
/**APP-domain GPIO interrupt slot mux, slots 10–11

You can [`read`](crate::Reg::read) this register and get [`iocapp_intsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocapp_intsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IocappIntsel1Spec;
impl crate::RegisterSpec for IocappIntsel1Spec {
    type Ux = u32;
}
///`read()` method returns [`iocapp_intsel1::R`](R) reader structure
impl crate::Readable for IocappIntsel1Spec {}
///`write(|w| ..)` method takes [`iocapp_intsel1::W`](W) writer structure
impl crate::Writable for IocappIntsel1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCAPP_INTSEL1 to value 0x3f3f
impl crate::Resettable for IocappIntsel1Spec {
    const RESET_VALUE: u32 = 0x3f3f;
}
