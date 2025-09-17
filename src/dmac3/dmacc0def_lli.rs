#[doc = "Register `DMACC0DefLLI` reader"]
pub type R = crate::R<Dmacc0defLliSpec>;
#[doc = "Register `DMACC0DefLLI` writer"]
pub type W = crate::W<Dmacc0defLliSpec>;
#[doc = "Bus master select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deflm {
    #[doc = "0: AHB Master 1"]
    Ahb1 = 0,
    #[doc = "1: AHB Master 2"]
    Ahb2 = 1,
}
impl From<Deflm> for bool {
    #[inline(always)]
    fn from(variant: Deflm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEFLM` reader - Bus master select"]
pub type DeflmR = crate::BitReader<Deflm>;
impl DeflmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Deflm {
        match self.bits {
            false => Deflm::Ahb1,
            true => Deflm::Ahb2,
        }
    }
    #[doc = "AHB Master 1"]
    #[inline(always)]
    pub fn is_ahb1(&self) -> bool {
        *self == Deflm::Ahb1
    }
    #[doc = "AHB Master 2"]
    #[inline(always)]
    pub fn is_ahb2(&self) -> bool {
        *self == Deflm::Ahb2
    }
}
#[doc = "Field `DEFLM` writer - Bus master select"]
pub type DeflmW<'a, REG> = crate::BitWriter<'a, REG, Deflm>;
impl<'a, REG> DeflmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB Master 1"]
    #[inline(always)]
    pub fn ahb1(self) -> &'a mut crate::W<REG> {
        self.variant(Deflm::Ahb1)
    }
    #[doc = "AHB Master 2"]
    #[inline(always)]
    pub fn ahb2(self) -> &'a mut crate::W<REG> {
        self.variant(Deflm::Ahb2)
    }
}
#[doc = "Enable Default LLI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Defle {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Defle> for bool {
    #[inline(always)]
    fn from(variant: Defle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEFLE` reader - Enable Default LLI"]
pub type DefleR = crate::BitReader<Defle>;
impl DefleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Defle {
        match self.bits {
            false => Defle::Disabled,
            true => Defle::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Defle::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Defle::Enabled
    }
}
#[doc = "Field `DEFLE` writer - Enable Default LLI"]
pub type DefleW<'a, REG> = crate::BitWriter<'a, REG, Defle>;
impl<'a, REG> DefleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Defle::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Defle::Enabled)
    }
}
#[doc = "Field `DEFLLI` reader - "]
pub type DeflliR = crate::FieldReader<u32>;
#[doc = "Field `DEFLLI` writer - "]
pub type DeflliW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Bus master select"]
    #[inline(always)]
    pub fn deflm(&self) -> DeflmR {
        DeflmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Default LLI"]
    #[inline(always)]
    pub fn defle(&self) -> DefleR {
        DefleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn deflli(&self) -> DeflliR {
        DeflliR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Bus master select"]
    #[inline(always)]
    pub fn deflm(&mut self) -> DeflmW<'_, Dmacc0defLliSpec> {
        DeflmW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Default LLI"]
    #[inline(always)]
    pub fn defle(&mut self) -> DefleW<'_, Dmacc0defLliSpec> {
        DefleW::new(self, 1)
    }
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn deflli(&mut self) -> DeflliW<'_, Dmacc0defLliSpec> {
        DeflliW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0def_lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0def_lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmacc0defLliSpec;
impl crate::RegisterSpec for Dmacc0defLliSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacc0def_lli::R`](R) reader structure"]
impl crate::Readable for Dmacc0defLliSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacc0def_lli::W`](W) writer structure"]
impl crate::Writable for Dmacc0defLliSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACC0DefLLI to value 0"]
impl crate::Resettable for Dmacc0defLliSpec {}
