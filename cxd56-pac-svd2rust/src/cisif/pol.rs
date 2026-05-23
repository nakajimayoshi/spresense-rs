///Register `POL` reader
pub type R = crate::R<PolSpec>;
///Register `POL` writer
pub type W = crate::W<PolSpec>;
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hpol {
    ///0: Hsync low active
    Lowactive = 0,
    ///1: Hsync high active
    Highactive = 1,
}
impl From<Hpol> for bool {
    #[inline(always)]
    fn from(variant: Hpol) -> Self {
        variant as u8 != 0
    }
}
///Field `hpol` reader -
pub type HpolR = crate::BitReader<Hpol>;
impl HpolR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hpol {
        match self.bits {
            false => Hpol::Lowactive,
            true => Hpol::Highactive,
        }
    }
    ///Hsync low active
    #[inline(always)]
    pub fn is_lowactive(&self) -> bool {
        *self == Hpol::Lowactive
    }
    ///Hsync high active
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Hpol::Highactive
    }
}
///Field `hpol` writer -
pub type HpolW<'a, REG> = crate::BitWriter<'a, REG, Hpol>;
impl<'a, REG> HpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hsync low active
    #[inline(always)]
    pub fn lowactive(self) -> &'a mut crate::W<REG> {
        self.variant(Hpol::Lowactive)
    }
    ///Hsync high active
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Hpol::Highactive)
    }
}
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vpol {
    ///0: Vsync low active
    Lowactive = 0,
    ///1: Vsync high active
    Highactive = 1,
}
impl From<Vpol> for bool {
    #[inline(always)]
    fn from(variant: Vpol) -> Self {
        variant as u8 != 0
    }
}
///Field `vpol` reader -
pub type VpolR = crate::BitReader<Vpol>;
impl VpolR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vpol {
        match self.bits {
            false => Vpol::Lowactive,
            true => Vpol::Highactive,
        }
    }
    ///Vsync low active
    #[inline(always)]
    pub fn is_lowactive(&self) -> bool {
        *self == Vpol::Lowactive
    }
    ///Vsync high active
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Vpol::Highactive
    }
}
///Field `vpol` writer -
pub type VpolW<'a, REG> = crate::BitWriter<'a, REG, Vpol>;
impl<'a, REG> VpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Vsync low active
    #[inline(always)]
    pub fn lowactive(self) -> &'a mut crate::W<REG> {
        self.variant(Vpol::Lowactive)
    }
    ///Vsync high active
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Vpol::Highactive)
    }
}
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn hpol(&self) -> HpolR {
        HpolR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn vpol(&self) -> VpolR {
        VpolR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn hpol(&mut self) -> HpolW<'_, PolSpec> {
        HpolW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn vpol(&mut self) -> VpolW<'_, PolSpec> {
        VpolW::new(self, 1)
    }
}
/**CIS input Vsync/Hsync polarity setting register

You can [`read`](crate::Reg::read) this register and get [`pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PolSpec;
impl crate::RegisterSpec for PolSpec {
    type Ux = u32;
}
///`read()` method returns [`pol::R`](R) reader structure
impl crate::Readable for PolSpec {}
///`write(|w| ..)` method takes [`pol::W`](W) writer structure
impl crate::Writable for PolSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POL to value 0
impl crate::Resettable for PolSpec {}
