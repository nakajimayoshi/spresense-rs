///Register `SSPCS` reader
pub type R = crate::R<SspcsSpec>;
///Register `SSPCS` writer
pub type W = crate::W<SspcsSpec>;
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SspCs {
    ///0: Output low to the CS
    Low = 0,
    ///1: Output high to the CS
    High = 1,
}
impl From<SspCs> for bool {
    #[inline(always)]
    fn from(variant: SspCs) -> Self {
        variant as u8 != 0
    }
}
///Field `SSP_CS` reader -
pub type SspCsR = crate::BitReader<SspCs>;
impl SspCsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SspCs {
        match self.bits {
            false => SspCs::Low,
            true => SspCs::High,
        }
    }
    ///Output low to the CS
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SspCs::Low
    }
    ///Output high to the CS
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SspCs::High
    }
}
///Field `SSP_CS` writer -
pub type SspCsW<'a, REG> = crate::BitWriter<'a, REG, SspCs>;
impl<'a, REG> SspCsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low to the CS
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SspCs::Low)
    }
    ///Output high to the CS
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SspCs::High)
    }
}
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn ssp_cs(&self) -> SspCsR {
        SspCsR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn ssp_cs(&mut self) -> SspCsW<'_, SspcsSpec> {
        SspCsW::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`sspcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SspcsSpec;
impl crate::RegisterSpec for SspcsSpec {
    type Ux = u32;
}
///`read()` method returns [`sspcs::R`](R) reader structure
impl crate::Readable for SspcsSpec {}
///`write(|w| ..)` method takes [`sspcs::W`](W) writer structure
impl crate::Writable for SspcsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSPCS to value 0
impl crate::Resettable for SspcsSpec {}
