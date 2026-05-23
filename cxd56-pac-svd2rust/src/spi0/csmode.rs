///Register `CSMODE` reader
pub type R = crate::R<CsmodeSpec>;
///Register `CSMODE` writer
pub type W = crate::W<CsmodeSpec>;
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsMode {
    ///0: CS controled by controller
    Auto = 0,
    ///1: CS controled by CS register
    Manual = 1,
}
impl From<CsMode> for bool {
    #[inline(always)]
    fn from(variant: CsMode) -> Self {
        variant as u8 != 0
    }
}
///Field `CS_MODE` reader -
pub type CsModeR = crate::BitReader<CsMode>;
impl CsModeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CsMode {
        match self.bits {
            false => CsMode::Auto,
            true => CsMode::Manual,
        }
    }
    ///CS controled by controller
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == CsMode::Auto
    }
    ///CS controled by CS register
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == CsMode::Manual
    }
}
///Field `CS_MODE` writer -
pub type CsModeW<'a, REG> = crate::BitWriter<'a, REG, CsMode>;
impl<'a, REG> CsModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CS controled by controller
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(CsMode::Auto)
    }
    ///CS controled by CS register
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(CsMode::Manual)
    }
}
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn cs_mode(&self) -> CsModeR {
        CsModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn cs_mode(&mut self) -> CsModeW<'_, CsmodeSpec> {
        CsModeW::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`csmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CsmodeSpec;
impl crate::RegisterSpec for CsmodeSpec {
    type Ux = u32;
}
///`read()` method returns [`csmode::R`](R) reader structure
impl crate::Readable for CsmodeSpec {}
///`write(|w| ..)` method takes [`csmode::W`](W) writer structure
impl crate::Writable for CsmodeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSMODE to value 0
impl crate::Resettable for CsmodeSpec {}
