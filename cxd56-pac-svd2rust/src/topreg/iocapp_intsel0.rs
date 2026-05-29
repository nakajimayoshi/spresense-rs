///Register `IOCAPP_INTSEL0` reader
pub type R = crate::R<IocappIntsel0Spec>;
///Register `IOCAPP_INTSEL0` writer
pub type W = crate::W<IocappIntsel0Spec>;
///Field `SLOT6` reader - Pin index for APP slot 6 → EXDEVICE_6 (0x3f = unused)
pub type Slot6R = crate::FieldReader;
///Field `SLOT6` writer - Pin index for APP slot 6 → EXDEVICE_6 (0x3f = unused)
pub type Slot6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SLOT7` reader - Pin index for APP slot 7 → EXDEVICE_7 (0x3f = unused)
pub type Slot7R = crate::FieldReader;
///Field `SLOT7` writer - Pin index for APP slot 7 → EXDEVICE_7 (0x3f = unused)
pub type Slot7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SLOT8` reader - Pin index for APP slot 8 → EXDEVICE_8 (0x3f = unused)
pub type Slot8R = crate::FieldReader;
///Field `SLOT8` writer - Pin index for APP slot 8 → EXDEVICE_8 (0x3f = unused)
pub type Slot8W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SLOT9` reader - Pin index for APP slot 9 → EXDEVICE_9 (0x3f = unused)
pub type Slot9R = crate::FieldReader;
///Field `SLOT9` writer - Pin index for APP slot 9 → EXDEVICE_9 (0x3f = unused)
pub type Slot9W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Pin index for APP slot 6 → EXDEVICE_6 (0x3f = unused)
    #[inline(always)]
    pub fn slot6(&self) -> Slot6R {
        Slot6R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Pin index for APP slot 7 → EXDEVICE_7 (0x3f = unused)
    #[inline(always)]
    pub fn slot7(&self) -> Slot7R {
        Slot7R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - Pin index for APP slot 8 → EXDEVICE_8 (0x3f = unused)
    #[inline(always)]
    pub fn slot8(&self) -> Slot8R {
        Slot8R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - Pin index for APP slot 9 → EXDEVICE_9 (0x3f = unused)
    #[inline(always)]
    pub fn slot9(&self) -> Slot9R {
        Slot9R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - Pin index for APP slot 6 → EXDEVICE_6 (0x3f = unused)
    #[inline(always)]
    pub fn slot6(&mut self) -> Slot6W<'_, IocappIntsel0Spec> {
        Slot6W::new(self, 0)
    }
    ///Bits 8:13 - Pin index for APP slot 7 → EXDEVICE_7 (0x3f = unused)
    #[inline(always)]
    pub fn slot7(&mut self) -> Slot7W<'_, IocappIntsel0Spec> {
        Slot7W::new(self, 8)
    }
    ///Bits 16:21 - Pin index for APP slot 8 → EXDEVICE_8 (0x3f = unused)
    #[inline(always)]
    pub fn slot8(&mut self) -> Slot8W<'_, IocappIntsel0Spec> {
        Slot8W::new(self, 16)
    }
    ///Bits 24:29 - Pin index for APP slot 9 → EXDEVICE_9 (0x3f = unused)
    #[inline(always)]
    pub fn slot9(&mut self) -> Slot9W<'_, IocappIntsel0Spec> {
        Slot9W::new(self, 24)
    }
}
/**APP-domain GPIO interrupt slot mux, slots 6–9 (1 byte per slot, pin index 0–63)

You can [`read`](crate::Reg::read) this register and get [`iocapp_intsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocapp_intsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IocappIntsel0Spec;
impl crate::RegisterSpec for IocappIntsel0Spec {
    type Ux = u32;
}
///`read()` method returns [`iocapp_intsel0::R`](R) reader structure
impl crate::Readable for IocappIntsel0Spec {}
///`write(|w| ..)` method takes [`iocapp_intsel0::W`](W) writer structure
impl crate::Writable for IocappIntsel0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCAPP_INTSEL0 to value 0x3f3f_3f3f
impl crate::Resettable for IocappIntsel0Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
