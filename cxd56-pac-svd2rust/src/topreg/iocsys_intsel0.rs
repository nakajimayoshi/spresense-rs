///Register `IOCSYS_INTSEL0` reader
pub type R = crate::R<IocsysIntsel0Spec>;
///Register `IOCSYS_INTSEL0` writer
pub type W = crate::W<IocsysIntsel0Spec>;
///Field `SLOT0` reader - Pin index for SYS slot 0 → EXDEVICE_0 (0x3f = unused)
pub type Slot0R = crate::FieldReader;
///Field `SLOT0` writer - Pin index for SYS slot 0 → EXDEVICE_0 (0x3f = unused)
pub type Slot0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SLOT1` reader - Pin index for SYS slot 1 → EXDEVICE_1 (0x3f = unused)
pub type Slot1R = crate::FieldReader;
///Field `SLOT1` writer - Pin index for SYS slot 1 → EXDEVICE_1 (0x3f = unused)
pub type Slot1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SLOT2` reader - Pin index for SYS slot 2 → EXDEVICE_2 (0x3f = unused)
pub type Slot2R = crate::FieldReader;
///Field `SLOT2` writer - Pin index for SYS slot 2 → EXDEVICE_2 (0x3f = unused)
pub type Slot2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SLOT3` reader - Pin index for SYS slot 3 → EXDEVICE_3 (0x3f = unused)
pub type Slot3R = crate::FieldReader;
///Field `SLOT3` writer - Pin index for SYS slot 3 → EXDEVICE_3 (0x3f = unused)
pub type Slot3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Pin index for SYS slot 0 → EXDEVICE_0 (0x3f = unused)
    #[inline(always)]
    pub fn slot0(&self) -> Slot0R {
        Slot0R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Pin index for SYS slot 1 → EXDEVICE_1 (0x3f = unused)
    #[inline(always)]
    pub fn slot1(&self) -> Slot1R {
        Slot1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - Pin index for SYS slot 2 → EXDEVICE_2 (0x3f = unused)
    #[inline(always)]
    pub fn slot2(&self) -> Slot2R {
        Slot2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - Pin index for SYS slot 3 → EXDEVICE_3 (0x3f = unused)
    #[inline(always)]
    pub fn slot3(&self) -> Slot3R {
        Slot3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - Pin index for SYS slot 0 → EXDEVICE_0 (0x3f = unused)
    #[inline(always)]
    pub fn slot0(&mut self) -> Slot0W<'_, IocsysIntsel0Spec> {
        Slot0W::new(self, 0)
    }
    ///Bits 8:13 - Pin index for SYS slot 1 → EXDEVICE_1 (0x3f = unused)
    #[inline(always)]
    pub fn slot1(&mut self) -> Slot1W<'_, IocsysIntsel0Spec> {
        Slot1W::new(self, 8)
    }
    ///Bits 16:21 - Pin index for SYS slot 2 → EXDEVICE_2 (0x3f = unused)
    #[inline(always)]
    pub fn slot2(&mut self) -> Slot2W<'_, IocsysIntsel0Spec> {
        Slot2W::new(self, 16)
    }
    ///Bits 24:29 - Pin index for SYS slot 3 → EXDEVICE_3 (0x3f = unused)
    #[inline(always)]
    pub fn slot3(&mut self) -> Slot3W<'_, IocsysIntsel0Spec> {
        Slot3W::new(self, 24)
    }
}
/**SYS-domain GPIO interrupt slot mux, slots 0–3 (1 byte per slot, pin index 0–63)

You can [`read`](crate::Reg::read) this register and get [`iocsys_intsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocsys_intsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IocsysIntsel0Spec;
impl crate::RegisterSpec for IocsysIntsel0Spec {
    type Ux = u32;
}
///`read()` method returns [`iocsys_intsel0::R`](R) reader structure
impl crate::Readable for IocsysIntsel0Spec {}
///`write(|w| ..)` method takes [`iocsys_intsel0::W`](W) writer structure
impl crate::Writable for IocsysIntsel0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCSYS_INTSEL0 to value 0x3f3f_3f3f
impl crate::Resettable for IocsysIntsel0Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
