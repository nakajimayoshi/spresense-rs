///Register `PMU_WAKE_TRIG_INTDET1` reader
pub type R = crate::R<PmuWakeTrigIntdet1Spec>;
///Register `PMU_WAKE_TRIG_INTDET1` writer
pub type W = crate::W<PmuWakeTrigIntdet1Spec>;
///Field `SLOT4` reader - Slot 4 polarity (EXDEVICE_4)
pub type Slot4R = crate::FieldReader;
///Field `SLOT4` writer - Slot 4 polarity (EXDEVICE_4)
pub type Slot4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT5` reader - Slot 5 polarity (EXDEVICE_5)
pub type Slot5R = crate::FieldReader;
///Field `SLOT5` writer - Slot 5 polarity (EXDEVICE_5)
pub type Slot5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT6` reader - Slot 6 polarity (EXDEVICE_6)
pub type Slot6R = crate::FieldReader;
///Field `SLOT6` writer - Slot 6 polarity (EXDEVICE_6)
pub type Slot6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT7` reader - Slot 7 polarity (EXDEVICE_7)
pub type Slot7R = crate::FieldReader;
///Field `SLOT7` writer - Slot 7 polarity (EXDEVICE_7)
pub type Slot7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT8` reader - Slot 8 polarity (EXDEVICE_8)
pub type Slot8R = crate::FieldReader;
///Field `SLOT8` writer - Slot 8 polarity (EXDEVICE_8)
pub type Slot8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT9` reader - Slot 9 polarity (EXDEVICE_9)
pub type Slot9R = crate::FieldReader;
///Field `SLOT9` writer - Slot 9 polarity (EXDEVICE_9)
pub type Slot9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT10` reader - Slot 10 polarity (EXDEVICE_10)
pub type Slot10R = crate::FieldReader;
///Field `SLOT10` writer - Slot 10 polarity (EXDEVICE_10)
pub type Slot10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT11` reader - Slot 11 polarity (EXDEVICE_11)
pub type Slot11R = crate::FieldReader;
///Field `SLOT11` writer - Slot 11 polarity (EXDEVICE_11)
pub type Slot11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Slot 4 polarity (EXDEVICE_4)
    #[inline(always)]
    pub fn slot4(&self) -> Slot4R {
        Slot4R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Slot 5 polarity (EXDEVICE_5)
    #[inline(always)]
    pub fn slot5(&self) -> Slot5R {
        Slot5R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Slot 6 polarity (EXDEVICE_6)
    #[inline(always)]
    pub fn slot6(&self) -> Slot6R {
        Slot6R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Slot 7 polarity (EXDEVICE_7)
    #[inline(always)]
    pub fn slot7(&self) -> Slot7R {
        Slot7R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - Slot 8 polarity (EXDEVICE_8)
    #[inline(always)]
    pub fn slot8(&self) -> Slot8R {
        Slot8R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Slot 9 polarity (EXDEVICE_9)
    #[inline(always)]
    pub fn slot9(&self) -> Slot9R {
        Slot9R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Slot 10 polarity (EXDEVICE_10)
    #[inline(always)]
    pub fn slot10(&self) -> Slot10R {
        Slot10R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Slot 11 polarity (EXDEVICE_11)
    #[inline(always)]
    pub fn slot11(&self) -> Slot11R {
        Slot11R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Slot 4 polarity (EXDEVICE_4)
    #[inline(always)]
    pub fn slot4(&mut self) -> Slot4W<'_, PmuWakeTrigIntdet1Spec> {
        Slot4W::new(self, 0)
    }
    ///Bits 4:6 - Slot 5 polarity (EXDEVICE_5)
    #[inline(always)]
    pub fn slot5(&mut self) -> Slot5W<'_, PmuWakeTrigIntdet1Spec> {
        Slot5W::new(self, 4)
    }
    ///Bits 8:10 - Slot 6 polarity (EXDEVICE_6)
    #[inline(always)]
    pub fn slot6(&mut self) -> Slot6W<'_, PmuWakeTrigIntdet1Spec> {
        Slot6W::new(self, 8)
    }
    ///Bits 12:14 - Slot 7 polarity (EXDEVICE_7)
    #[inline(always)]
    pub fn slot7(&mut self) -> Slot7W<'_, PmuWakeTrigIntdet1Spec> {
        Slot7W::new(self, 12)
    }
    ///Bits 16:18 - Slot 8 polarity (EXDEVICE_8)
    #[inline(always)]
    pub fn slot8(&mut self) -> Slot8W<'_, PmuWakeTrigIntdet1Spec> {
        Slot8W::new(self, 16)
    }
    ///Bits 20:22 - Slot 9 polarity (EXDEVICE_9)
    #[inline(always)]
    pub fn slot9(&mut self) -> Slot9W<'_, PmuWakeTrigIntdet1Spec> {
        Slot9W::new(self, 20)
    }
    ///Bits 24:26 - Slot 10 polarity (EXDEVICE_10)
    #[inline(always)]
    pub fn slot10(&mut self) -> Slot10W<'_, PmuWakeTrigIntdet1Spec> {
        Slot10W::new(self, 24)
    }
    ///Bits 28:30 - Slot 11 polarity (EXDEVICE_11)
    #[inline(always)]
    pub fn slot11(&mut self) -> Slot11W<'_, PmuWakeTrigIntdet1Spec> {
        Slot11W::new(self, 28)
    }
}
/**Interrupt detect polarity, slots 4–11 (3-bit field per slot at slot*4)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_intdet1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_intdet1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrigIntdet1Spec;
impl crate::RegisterSpec for PmuWakeTrigIntdet1Spec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig_intdet1::R`](R) reader structure
impl crate::Readable for PmuWakeTrigIntdet1Spec {}
///`write(|w| ..)` method takes [`pmu_wake_trig_intdet1::W`](W) writer structure
impl crate::Writable for PmuWakeTrigIntdet1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMU_WAKE_TRIG_INTDET1 to value 0
impl crate::Resettable for PmuWakeTrigIntdet1Spec {}
