///Register `PMU_WAKE_TRIG0_CLR` reader
pub type R = crate::R<PmuWakeTrig0ClrSpec>;
///Register `PMU_WAKE_TRIG0_CLR` writer
pub type W = crate::W<PmuWakeTrig0ClrSpec>;
///Field `SLOT0` reader - Clear slot 0 latched interrupt (EXDEVICE_0)
pub type Slot0R = crate::BitReader;
///Field `SLOT0` writer - Clear slot 0 latched interrupt (EXDEVICE_0)
pub type Slot0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT1` reader - Clear slot 1 latched interrupt (EXDEVICE_1)
pub type Slot1R = crate::BitReader;
///Field `SLOT1` writer - Clear slot 1 latched interrupt (EXDEVICE_1)
pub type Slot1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT2` reader - Clear slot 2 latched interrupt (EXDEVICE_2)
pub type Slot2R = crate::BitReader;
///Field `SLOT2` writer - Clear slot 2 latched interrupt (EXDEVICE_2)
pub type Slot2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT3` reader - Clear slot 3 latched interrupt (EXDEVICE_3)
pub type Slot3R = crate::BitReader;
///Field `SLOT3` writer - Clear slot 3 latched interrupt (EXDEVICE_3)
pub type Slot3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT4` reader - Clear slot 4 latched interrupt (EXDEVICE_4)
pub type Slot4R = crate::BitReader;
///Field `SLOT4` writer - Clear slot 4 latched interrupt (EXDEVICE_4)
pub type Slot4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT5` reader - Clear slot 5 latched interrupt (EXDEVICE_5)
pub type Slot5R = crate::BitReader;
///Field `SLOT5` writer - Clear slot 5 latched interrupt (EXDEVICE_5)
pub type Slot5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT6` reader - Clear slot 6 latched interrupt (EXDEVICE_6)
pub type Slot6R = crate::BitReader;
///Field `SLOT6` writer - Clear slot 6 latched interrupt (EXDEVICE_6)
pub type Slot6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT7` reader - Clear slot 7 latched interrupt (EXDEVICE_7)
pub type Slot7R = crate::BitReader;
///Field `SLOT7` writer - Clear slot 7 latched interrupt (EXDEVICE_7)
pub type Slot7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT8` reader - Clear slot 8 latched interrupt (EXDEVICE_8)
pub type Slot8R = crate::BitReader;
///Field `SLOT8` writer - Clear slot 8 latched interrupt (EXDEVICE_8)
pub type Slot8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT9` reader - Clear slot 9 latched interrupt (EXDEVICE_9)
pub type Slot9R = crate::BitReader;
///Field `SLOT9` writer - Clear slot 9 latched interrupt (EXDEVICE_9)
pub type Slot9W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT10` reader - Clear slot 10 latched interrupt (EXDEVICE_10)
pub type Slot10R = crate::BitReader;
///Field `SLOT10` writer - Clear slot 10 latched interrupt (EXDEVICE_10)
pub type Slot10W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT11` reader - Clear slot 11 latched interrupt (EXDEVICE_11)
pub type Slot11R = crate::BitReader;
///Field `SLOT11` writer - Clear slot 11 latched interrupt (EXDEVICE_11)
pub type Slot11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - Clear slot 0 latched interrupt (EXDEVICE_0)
    #[inline(always)]
    pub fn slot0(&self) -> Slot0R {
        Slot0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Clear slot 1 latched interrupt (EXDEVICE_1)
    #[inline(always)]
    pub fn slot1(&self) -> Slot1R {
        Slot1R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Clear slot 2 latched interrupt (EXDEVICE_2)
    #[inline(always)]
    pub fn slot2(&self) -> Slot2R {
        Slot2R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clear slot 3 latched interrupt (EXDEVICE_3)
    #[inline(always)]
    pub fn slot3(&self) -> Slot3R {
        Slot3R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Clear slot 4 latched interrupt (EXDEVICE_4)
    #[inline(always)]
    pub fn slot4(&self) -> Slot4R {
        Slot4R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Clear slot 5 latched interrupt (EXDEVICE_5)
    #[inline(always)]
    pub fn slot5(&self) -> Slot5R {
        Slot5R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Clear slot 6 latched interrupt (EXDEVICE_6)
    #[inline(always)]
    pub fn slot6(&self) -> Slot6R {
        Slot6R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Clear slot 7 latched interrupt (EXDEVICE_7)
    #[inline(always)]
    pub fn slot7(&self) -> Slot7R {
        Slot7R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Clear slot 8 latched interrupt (EXDEVICE_8)
    #[inline(always)]
    pub fn slot8(&self) -> Slot8R {
        Slot8R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Clear slot 9 latched interrupt (EXDEVICE_9)
    #[inline(always)]
    pub fn slot9(&self) -> Slot9R {
        Slot9R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Clear slot 10 latched interrupt (EXDEVICE_10)
    #[inline(always)]
    pub fn slot10(&self) -> Slot10R {
        Slot10R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Clear slot 11 latched interrupt (EXDEVICE_11)
    #[inline(always)]
    pub fn slot11(&self) -> Slot11R {
        Slot11R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - Clear slot 0 latched interrupt (EXDEVICE_0)
    #[inline(always)]
    pub fn slot0(&mut self) -> Slot0W<'_, PmuWakeTrig0ClrSpec> {
        Slot0W::new(self, 16)
    }
    ///Bit 17 - Clear slot 1 latched interrupt (EXDEVICE_1)
    #[inline(always)]
    pub fn slot1(&mut self) -> Slot1W<'_, PmuWakeTrig0ClrSpec> {
        Slot1W::new(self, 17)
    }
    ///Bit 18 - Clear slot 2 latched interrupt (EXDEVICE_2)
    #[inline(always)]
    pub fn slot2(&mut self) -> Slot2W<'_, PmuWakeTrig0ClrSpec> {
        Slot2W::new(self, 18)
    }
    ///Bit 19 - Clear slot 3 latched interrupt (EXDEVICE_3)
    #[inline(always)]
    pub fn slot3(&mut self) -> Slot3W<'_, PmuWakeTrig0ClrSpec> {
        Slot3W::new(self, 19)
    }
    ///Bit 20 - Clear slot 4 latched interrupt (EXDEVICE_4)
    #[inline(always)]
    pub fn slot4(&mut self) -> Slot4W<'_, PmuWakeTrig0ClrSpec> {
        Slot4W::new(self, 20)
    }
    ///Bit 21 - Clear slot 5 latched interrupt (EXDEVICE_5)
    #[inline(always)]
    pub fn slot5(&mut self) -> Slot5W<'_, PmuWakeTrig0ClrSpec> {
        Slot5W::new(self, 21)
    }
    ///Bit 22 - Clear slot 6 latched interrupt (EXDEVICE_6)
    #[inline(always)]
    pub fn slot6(&mut self) -> Slot6W<'_, PmuWakeTrig0ClrSpec> {
        Slot6W::new(self, 22)
    }
    ///Bit 23 - Clear slot 7 latched interrupt (EXDEVICE_7)
    #[inline(always)]
    pub fn slot7(&mut self) -> Slot7W<'_, PmuWakeTrig0ClrSpec> {
        Slot7W::new(self, 23)
    }
    ///Bit 24 - Clear slot 8 latched interrupt (EXDEVICE_8)
    #[inline(always)]
    pub fn slot8(&mut self) -> Slot8W<'_, PmuWakeTrig0ClrSpec> {
        Slot8W::new(self, 24)
    }
    ///Bit 25 - Clear slot 9 latched interrupt (EXDEVICE_9)
    #[inline(always)]
    pub fn slot9(&mut self) -> Slot9W<'_, PmuWakeTrig0ClrSpec> {
        Slot9W::new(self, 25)
    }
    ///Bit 26 - Clear slot 10 latched interrupt (EXDEVICE_10)
    #[inline(always)]
    pub fn slot10(&mut self) -> Slot10W<'_, PmuWakeTrig0ClrSpec> {
        Slot10W::new(self, 26)
    }
    ///Bit 27 - Clear slot 11 latched interrupt (EXDEVICE_11)
    #[inline(always)]
    pub fn slot11(&mut self) -> Slot11W<'_, PmuWakeTrig0ClrSpec> {
        Slot11W::new(self, 27)
    }
}
/**GPIO interrupt clear register, slots 0–11 (write 1<<(16+slot) to clear)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig0_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig0_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuWakeTrig0ClrSpec;
impl crate::RegisterSpec for PmuWakeTrig0ClrSpec {
    type Ux = u32;
}
///`read()` method returns [`pmu_wake_trig0_clr::R`](R) reader structure
impl crate::Readable for PmuWakeTrig0ClrSpec {}
///`write(|w| ..)` method takes [`pmu_wake_trig0_clr::W`](W) writer structure
impl crate::Writable for PmuWakeTrig0ClrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMU_WAKE_TRIG0_CLR to value 0
impl crate::Resettable for PmuWakeTrig0ClrSpec {}
