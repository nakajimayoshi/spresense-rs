#[doc = "Register `POL` reader"]
pub type R = crate::R<PolSpec>;
#[doc = "Register `POL` writer"]
pub type W = crate::W<PolSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hpol {
    #[doc = "0: Hsync low active"]
    Lowactive = 0,
    #[doc = "1: Hsync high active"]
    Highactive = 1,
}
impl From<Hpol> for bool {
    #[inline(always)]
    fn from(variant: Hpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hpol` reader - "]
pub type HpolR = crate::BitReader<Hpol>;
impl HpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hpol {
        match self.bits {
            false => Hpol::Lowactive,
            true => Hpol::Highactive,
        }
    }
    #[doc = "Hsync low active"]
    #[inline(always)]
    pub fn is_lowactive(&self) -> bool {
        *self == Hpol::Lowactive
    }
    #[doc = "Hsync high active"]
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Hpol::Highactive
    }
}
#[doc = "Field `hpol` writer - "]
pub type HpolW<'a, REG> = crate::BitWriter<'a, REG, Hpol>;
impl<'a, REG> HpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hsync low active"]
    #[inline(always)]
    pub fn lowactive(self) -> &'a mut crate::W<REG> {
        self.variant(Hpol::Lowactive)
    }
    #[doc = "Hsync high active"]
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Hpol::Highactive)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vpol {
    #[doc = "0: Vsync low active"]
    Lowactive = 0,
    #[doc = "1: Vsync high active"]
    Highactive = 1,
}
impl From<Vpol> for bool {
    #[inline(always)]
    fn from(variant: Vpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vpol` reader - "]
pub type VpolR = crate::BitReader<Vpol>;
impl VpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vpol {
        match self.bits {
            false => Vpol::Lowactive,
            true => Vpol::Highactive,
        }
    }
    #[doc = "Vsync low active"]
    #[inline(always)]
    pub fn is_lowactive(&self) -> bool {
        *self == Vpol::Lowactive
    }
    #[doc = "Vsync high active"]
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Vpol::Highactive
    }
}
#[doc = "Field `vpol` writer - "]
pub type VpolW<'a, REG> = crate::BitWriter<'a, REG, Vpol>;
impl<'a, REG> VpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Vsync low active"]
    #[inline(always)]
    pub fn lowactive(self) -> &'a mut crate::W<REG> {
        self.variant(Vpol::Lowactive)
    }
    #[doc = "Vsync high active"]
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Vpol::Highactive)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hpol(&self) -> HpolR {
        HpolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vpol(&self) -> VpolR {
        VpolR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hpol(&mut self) -> HpolW<'_, PolSpec> {
        HpolW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vpol(&mut self) -> VpolW<'_, PolSpec> {
        VpolW::new(self, 1)
    }
}
#[doc = "CIS input Vsync/Hsync polarity setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolSpec;
impl crate::RegisterSpec for PolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pol::R`](R) reader structure"]
impl crate::Readable for PolSpec {}
#[doc = "`write(|w| ..)` method takes [`pol::W`](W) writer structure"]
impl crate::Writable for PolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POL to value 0"]
impl crate::Resettable for PolSpec {}
