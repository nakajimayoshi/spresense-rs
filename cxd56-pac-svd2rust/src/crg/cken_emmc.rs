///Register `CKEN_EMMC` reader
pub type R = crate::R<CkenEmmcSpec>;
///Register `CKEN_EMMC` writer
pub type W = crate::W<CkenEmmcSpec>;
/**Enable eMMC clock

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkin {
    ///0: Disable
    Disable = 0,
    ///1: Enable
    Enable = 1,
}
impl From<Clkin> for bool {
    #[inline(always)]
    fn from(variant: Clkin) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKIN` reader - Enable eMMC clock
pub type ClkinR = crate::BitReader<Clkin>;
impl ClkinR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Clkin {
        match self.bits {
            false => Clkin::Disable,
            true => Clkin::Enable,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Clkin::Disable
    }
    ///Enable
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Clkin::Enable
    }
}
///Field `CLKIN` writer - Enable eMMC clock
pub type ClkinW<'a, REG> = crate::BitWriter<'a, REG, Clkin>;
impl<'a, REG> ClkinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Disable)
    }
    ///Enable
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Enable)
    }
}
///Enable DRV clock
pub use Clkin as Drv;
///Enable eMMC sampling clock
pub use Clkin as Smp;
///Field `DRV` reader - Enable DRV clock
pub use ClkinR as DrvR;
///Field `SMP` reader - Enable eMMC sampling clock
pub use ClkinR as SmpR;
///Field `DRV` writer - Enable DRV clock
pub use ClkinW as DrvW;
///Field `SMP` writer - Enable eMMC sampling clock
pub use ClkinW as SmpW;
impl R {
    ///Bit 0 - Enable eMMC clock
    #[inline(always)]
    pub fn clkin(&self) -> ClkinR {
        ClkinR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable DRV clock
    #[inline(always)]
    pub fn drv(&self) -> DrvR {
        DrvR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable eMMC sampling clock
    #[inline(always)]
    pub fn smp(&self) -> SmpR {
        SmpR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable eMMC clock
    #[inline(always)]
    pub fn clkin(&mut self) -> ClkinW<'_, CkenEmmcSpec> {
        ClkinW::new(self, 0)
    }
    ///Bit 1 - Enable DRV clock
    #[inline(always)]
    pub fn drv(&mut self) -> DrvW<'_, CkenEmmcSpec> {
        DrvW::new(self, 1)
    }
    ///Bit 2 - Enable eMMC sampling clock
    #[inline(always)]
    pub fn smp(&mut self) -> SmpW<'_, CkenEmmcSpec> {
        SmpW::new(self, 2)
    }
}
/**eMMC clock setting

You can [`read`](crate::Reg::read) this register and get [`cken_emmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cken_emmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkenEmmcSpec;
impl crate::RegisterSpec for CkenEmmcSpec {
    type Ux = u32;
}
///`read()` method returns [`cken_emmc::R`](R) reader structure
impl crate::Readable for CkenEmmcSpec {}
///`write(|w| ..)` method takes [`cken_emmc::W`](W) writer structure
impl crate::Writable for CkenEmmcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKEN_EMMC to value 0
impl crate::Resettable for CkenEmmcSpec {}
