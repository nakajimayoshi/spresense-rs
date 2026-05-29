///Register `PMU_WAKE_TRIG_CPUINTSEL0` reader
pub type R = crate::R<PmuWakeTrigCpuintsel0Spec>;
///Register `PMU_WAKE_TRIG_CPUINTSEL0` writer
pub type W = crate::W<PmuWakeTrigCpuintsel0Spec>;
///Field `SLOT0` reader - Slot 0 route (EXDEVICE_0)
pub type Slot0R = crate::FieldReader;
///Field `SLOT0` writer - Slot 0 route (EXDEVICE_0)
pub type Slot0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT1` reader - Slot 1 route (EXDEVICE_1)
pub type Slot1R = crate::FieldReader;
///Field `SLOT1` writer - Slot 1 route (EXDEVICE_1)
pub type Slot1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT2` reader - Slot 2 route (EXDEVICE_2)
pub type Slot2R = crate::FieldReader;
///Field `SLOT2` writer - Slot 2 route (EXDEVICE_2)
pub type Slot2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLOT3` reader - Slot 3 route (EXDEVICE_3)
pub type Slot3R = crate::FieldReader;
///Field `SLOT3` writer - Slot 3 route (EXDEVICE_3)
pub type Slot3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 16:18 - Slot 0 route (EXDEVICE_0)
    #[inline(always)]
    pub fn slot0(&self) -> Slot0R {
        Slot0R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Slot 1 route (EXDEVICE_1)
    #[inline(always)]
    pub fn slot1(&self) -> Slot1R {
        Slot1R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Slot 2 route (EXDEVICE_2)
    #[inline(always)]
    pub fn slot2(&self) -> Slot2R {
        Slot2R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Slot 3 route (EXDEVICE_3)
    #[inline(always)]
    pub fn slot3(&self) -> Slot3R {
        Slot3R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 16:18 - Slot 0 route (EXDEVICE_0)
    #[inline(always)]
    pub fn slot0(&mut self) -> Slot0W<'_, PmuWakeTrigCpuintsel0Spec> {
        Slot0W::new(self, 16)
    }
    ///Bits 20:22 - Slot 1 route (EXDEVICE_1)
    #[inline(always)]
    pub fn slot1(&mut self) -> Slot1W<'_, PmuWakeTrigCpuintsel0Spec> {
        Slot1W::new(self, 20)
    }
    ///Bits 24:26 - Slot 2 route (EXDEVICE_2)
    #[inline(always)]
    pub fn slot2(&mut self) -> Slot2W<'_, PmuWakeTrigCpuintsel0Spec> {
        Slot2W::new(self, 24)
    }
    ///Bits 28:30 - Slot 3 route (EXDEVICE_3)
    #[inline(always)]
    pub fn slot3(&mut self) -> Slot3W<'_, PmuWakeTrigCpuintsel0Spec> {
        Slot3W::new(self, 28)
    }
}
/**CPU interrupt route select, slots 0–3 (3-bit field per slot at 16+slot*4)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_cpuintsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_cpuintsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrigCpuintsel0Spec;
impl crate::RegisterSpec for PmuWakeTrigCpuintsel0Spec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig_cpuintsel0::R`](R) reader structure
impl crate::Readable for PmuWakeTrigCpuintsel0Spec {}
///`write(|w| ..)` method takes [`pmu_wake_trig_cpuintsel0::W`](W) writer structure
impl crate::Writable for PmuWakeTrigCpuintsel0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMU_WAKE_TRIG_CPUINTSEL0 to value 0
impl crate::Resettable for PmuWakeTrigCpuintsel0Spec {}
