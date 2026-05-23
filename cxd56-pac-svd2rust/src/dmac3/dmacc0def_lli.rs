///Register `DMACC0DefLLI` reader
pub type R = crate::R<Dmacc0defLliSpec>;
///Register `DMACC0DefLLI` writer
pub type W = crate::W<Dmacc0defLliSpec>;
/**Bus master select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deflm {
    ///0: AHB Master 1
    Ahb1 = 0,
    ///1: AHB Master 2
    Ahb2 = 1,
}
impl From<Deflm> for bool {
    #[inline(always)]
    fn from(variant: Deflm) -> Self {
        variant as u8 != 0
    }
}
///Field `DEFLM` reader - Bus master select
pub type DeflmR = crate::BitReader<Deflm>;
impl DeflmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Deflm {
        match self.bits {
            false => Deflm::Ahb1,
            true => Deflm::Ahb2,
        }
    }
    ///AHB Master 1
    #[inline(always)]
    pub fn is_ahb1(&self) -> bool {
        *self == Deflm::Ahb1
    }
    ///AHB Master 2
    #[inline(always)]
    pub fn is_ahb2(&self) -> bool {
        *self == Deflm::Ahb2
    }
}
///Field `DEFLM` writer - Bus master select
pub type DeflmW<'a, REG> = crate::BitWriter<'a, REG, Deflm>;
impl<'a, REG> DeflmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB Master 1
    #[inline(always)]
    pub fn ahb1(self) -> &'a mut crate::W<REG> {
        self.variant(Deflm::Ahb1)
    }
    ///AHB Master 2
    #[inline(always)]
    pub fn ahb2(self) -> &'a mut crate::W<REG> {
        self.variant(Deflm::Ahb2)
    }
}
/**Enable Default LLI

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Defle {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Defle> for bool {
    #[inline(always)]
    fn from(variant: Defle) -> Self {
        variant as u8 != 0
    }
}
///Field `DEFLE` reader - Enable Default LLI
pub type DefleR = crate::BitReader<Defle>;
impl DefleR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Defle {
        match self.bits {
            false => Defle::Disabled,
            true => Defle::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Defle::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Defle::Enabled
    }
}
///Field `DEFLE` writer - Enable Default LLI
pub type DefleW<'a, REG> = crate::BitWriter<'a, REG, Defle>;
impl<'a, REG> DefleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Defle::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Defle::Enabled)
    }
}
///Field `DEFLLI` reader -
pub type DeflliR = crate::FieldReader<u32>;
///Field `DEFLLI` writer -
pub type DeflliW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - Bus master select
    #[inline(always)]
    pub fn deflm(&self) -> DeflmR {
        DeflmR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Default LLI
    #[inline(always)]
    pub fn defle(&self) -> DefleR {
        DefleR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn deflli(&self) -> DeflliR {
        DeflliR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bit 0 - Bus master select
    #[inline(always)]
    pub fn deflm(&mut self) -> DeflmW<'_, Dmacc0defLliSpec> {
        DeflmW::new(self, 0)
    }
    ///Bit 1 - Enable Default LLI
    #[inline(always)]
    pub fn defle(&mut self) -> DefleW<'_, Dmacc0defLliSpec> {
        DefleW::new(self, 1)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn deflli(&mut self) -> DeflliW<'_, Dmacc0defLliSpec> {
        DeflliW::new(self, 2)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacc0def_lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0def_lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dmacc0defLliSpec;
impl crate::RegisterSpec for Dmacc0defLliSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacc0def_lli::R`](R) reader structure
impl crate::Readable for Dmacc0defLliSpec {}
///`write(|w| ..)` method takes [`dmacc0def_lli::W`](W) writer structure
impl crate::Writable for Dmacc0defLliSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACC0DefLLI to value 0
impl crate::Resettable for Dmacc0defLliSpec {}
