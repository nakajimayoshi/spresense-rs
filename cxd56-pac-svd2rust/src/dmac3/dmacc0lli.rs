///Register `DMACC0LLI` reader
pub type R = crate::R<Dmacc0lliSpec>;
///Register `DMACC0LLI` writer
pub type W = crate::W<Dmacc0lliSpec>;
/**AHB master select for loading the next LLI

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lm {
    ///0: AHB Master 1
    Ahb1 = 0,
    ///1: AHB Master 2
    Ahb2 = 1,
}
impl From<Lm> for bool {
    #[inline(always)]
    fn from(variant: Lm) -> Self {
        variant as u8 != 0
    }
}
///Field `LM` reader - AHB master select for loading the next LLI
pub type LmR = crate::BitReader<Lm>;
impl LmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lm {
        match self.bits {
            false => Lm::Ahb1,
            true => Lm::Ahb2,
        }
    }
    ///AHB Master 1
    #[inline(always)]
    pub fn is_ahb1(&self) -> bool {
        *self == Lm::Ahb1
    }
    ///AHB Master 2
    #[inline(always)]
    pub fn is_ahb2(&self) -> bool {
        *self == Lm::Ahb2
    }
}
///Field `LM` writer - AHB master select for loading the next LLI
pub type LmW<'a, REG> = crate::BitWriter<'a, REG, Lm>;
impl<'a, REG> LmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB Master 1
    #[inline(always)]
    pub fn ahb1(self) -> &'a mut crate::W<REG> {
        self.variant(Lm::Ahb1)
    }
    ///AHB Master 2
    #[inline(always)]
    pub fn ahb2(self) -> &'a mut crate::W<REG> {
        self.variant(Lm::Ahb2)
    }
}
///Field `LLI` reader - Linked list item
pub type LliR = crate::FieldReader<u32>;
///Field `LLI` writer - Linked list item
pub type LliW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - AHB master select for loading the next LLI
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new((self.bits & 1) != 0)
    }
    ///Bits 2:31 - Linked list item
    #[inline(always)]
    pub fn lli(&self) -> LliR {
        LliR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bit 0 - AHB master select for loading the next LLI
    #[inline(always)]
    pub fn lm(&mut self) -> LmW<'_, Dmacc0lliSpec> {
        LmW::new(self, 0)
    }
    ///Bits 2:31 - Linked list item
    #[inline(always)]
    pub fn lli(&mut self) -> LliW<'_, Dmacc0lliSpec> {
        LliW::new(self, 2)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacc0lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dmacc0lliSpec;
impl crate::RegisterSpec for Dmacc0lliSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacc0lli::R`](R) reader structure
impl crate::Readable for Dmacc0lliSpec {}
///`write(|w| ..)` method takes [`dmacc0lli::W`](W) writer structure
impl crate::Writable for Dmacc0lliSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACC0LLI to value 0
impl crate::Resettable for Dmacc0lliSpec {}
