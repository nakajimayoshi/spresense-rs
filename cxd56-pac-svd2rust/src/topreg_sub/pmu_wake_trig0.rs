///Register `PMU_WAKE_TRIG0` reader
pub type R = crate::R<PmuWakeTrig0Spec>;
///Field `SLOT0` reader - Slot 0 masked trigger status (EXDEVICE_0)
pub type Slot0R = crate::BitReader;
///Field `SLOT1` reader - Slot 1 masked trigger status (EXDEVICE_1)
pub type Slot1R = crate::BitReader;
///Field `SLOT2` reader - Slot 2 masked trigger status (EXDEVICE_2)
pub type Slot2R = crate::BitReader;
///Field `SLOT3` reader - Slot 3 masked trigger status (EXDEVICE_3)
pub type Slot3R = crate::BitReader;
///Field `SLOT4` reader - Slot 4 masked trigger status (EXDEVICE_4)
pub type Slot4R = crate::BitReader;
///Field `SLOT5` reader - Slot 5 masked trigger status (EXDEVICE_5)
pub type Slot5R = crate::BitReader;
///Field `SLOT6` reader - Slot 6 masked trigger status (EXDEVICE_6)
pub type Slot6R = crate::BitReader;
///Field `SLOT7` reader - Slot 7 masked trigger status (EXDEVICE_7)
pub type Slot7R = crate::BitReader;
///Field `SLOT8` reader - Slot 8 masked trigger status (EXDEVICE_8)
pub type Slot8R = crate::BitReader;
///Field `SLOT9` reader - Slot 9 masked trigger status (EXDEVICE_9)
pub type Slot9R = crate::BitReader;
///Field `SLOT10` reader - Slot 10 masked trigger status (EXDEVICE_10)
pub type Slot10R = crate::BitReader;
///Field `SLOT11` reader - Slot 11 masked trigger status (EXDEVICE_11)
pub type Slot11R = crate::BitReader;
impl R {
    ///Bit 16 - Slot 0 masked trigger status (EXDEVICE_0)
    #[inline(always)]
    pub fn slot0(&self) -> Slot0R {
        Slot0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Slot 1 masked trigger status (EXDEVICE_1)
    #[inline(always)]
    pub fn slot1(&self) -> Slot1R {
        Slot1R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Slot 2 masked trigger status (EXDEVICE_2)
    #[inline(always)]
    pub fn slot2(&self) -> Slot2R {
        Slot2R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Slot 3 masked trigger status (EXDEVICE_3)
    #[inline(always)]
    pub fn slot3(&self) -> Slot3R {
        Slot3R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Slot 4 masked trigger status (EXDEVICE_4)
    #[inline(always)]
    pub fn slot4(&self) -> Slot4R {
        Slot4R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Slot 5 masked trigger status (EXDEVICE_5)
    #[inline(always)]
    pub fn slot5(&self) -> Slot5R {
        Slot5R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Slot 6 masked trigger status (EXDEVICE_6)
    #[inline(always)]
    pub fn slot6(&self) -> Slot6R {
        Slot6R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Slot 7 masked trigger status (EXDEVICE_7)
    #[inline(always)]
    pub fn slot7(&self) -> Slot7R {
        Slot7R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Slot 8 masked trigger status (EXDEVICE_8)
    #[inline(always)]
    pub fn slot8(&self) -> Slot8R {
        Slot8R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Slot 9 masked trigger status (EXDEVICE_9)
    #[inline(always)]
    pub fn slot9(&self) -> Slot9R {
        Slot9R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Slot 10 masked trigger status (EXDEVICE_10)
    #[inline(always)]
    pub fn slot10(&self) -> Slot10R {
        Slot10R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Slot 11 masked trigger status (EXDEVICE_11)
    #[inline(always)]
    pub fn slot11(&self) -> Slot11R {
        Slot11R::new(((self.bits >> 27) & 1) != 0)
    }
}
/**GPIO interrupt masked status, slots 0–11 (ISR reads this)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrig0Spec;
impl crate::RegisterSpec for PmuWakeTrig0Spec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig0::R`](R) reader structure
impl crate::Readable for PmuWakeTrig0Spec {}
///`reset()` method sets PMU_WAKE_TRIG0 to value 0
impl crate::Resettable for PmuWakeTrig0Spec {}
