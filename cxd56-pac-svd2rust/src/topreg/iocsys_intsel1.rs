///Register `IOCSYS_INTSEL1` reader
pub type R = crate::R<IocsysIntsel1Spec>;
///Register `IOCSYS_INTSEL1` writer
pub type W = crate::W<IocsysIntsel1Spec>;
///Field `SLOT4` reader - Pin index for SYS slot 4 → EXDEVICE_4 (0x3f = unused)
pub type Slot4R = crate::FieldReader;
///Field `SLOT4` writer - Pin index for SYS slot 4 → EXDEVICE_4 (0x3f = unused)
pub type Slot4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SLOT5` reader - Pin index for SYS slot 5 → EXDEVICE_5 (0x3f = unused)
pub type Slot5R = crate::FieldReader;
///Field `SLOT5` writer - Pin index for SYS slot 5 → EXDEVICE_5 (0x3f = unused)
pub type Slot5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Pin index for SYS slot 4 → EXDEVICE_4 (0x3f = unused)
    #[inline(always)]
    pub fn slot4(&self) -> Slot4R {
        Slot4R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Pin index for SYS slot 5 → EXDEVICE_5 (0x3f = unused)
    #[inline(always)]
    pub fn slot5(&self) -> Slot5R {
        Slot5R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - Pin index for SYS slot 4 → EXDEVICE_4 (0x3f = unused)
    #[inline(always)]
    pub fn slot4(&mut self) -> Slot4W<'_, IocsysIntsel1Spec> {
        Slot4W::new(self, 0)
    }
    ///Bits 8:13 - Pin index for SYS slot 5 → EXDEVICE_5 (0x3f = unused)
    #[inline(always)]
    pub fn slot5(&mut self) -> Slot5W<'_, IocsysIntsel1Spec> {
        Slot5W::new(self, 8)
    }
}
/**SYS-domain GPIO interrupt slot mux, slots 4–5

You can [`read`](crate::Reg::read) this register and get [`iocsys_intsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocsys_intsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IocsysIntsel1Spec;
impl crate::RegisterSpec for IocsysIntsel1Spec {
    type Ux = u32;
}
///`read()` method returns [`iocsys_intsel1::R`](R) reader structure
impl crate::Readable for IocsysIntsel1Spec {}
///`write(|w| ..)` method takes [`iocsys_intsel1::W`](W) writer structure
impl crate::Writable for IocsysIntsel1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCSYS_INTSEL1 to value 0x3f3f
impl crate::Resettable for IocsysIntsel1Spec {
    const RESET_VALUE: u32 = 0x3f3f;
}
