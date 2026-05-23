///Register `SET_DIRECTION` reader
pub type R = crate::R<SetDirectionSpec>;
///Register `SET_DIRECTION` writer
pub type W = crate::W<SetDirectionSpec>;
/**Rotation Angle

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rot {
    ///0: `0`
    NoRotation = 0,
    ///1: `1`
    Right90degrees = 1,
    ///2: `10`
    Right180degrees = 2,
    ///4: `100`
    Right270degrees = 4,
}
impl From<Rot> for u8 {
    #[inline(always)]
    fn from(variant: Rot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rot {
    type Ux = u8;
}
impl crate::IsEnum for Rot {}
///Field `ROT` reader - Rotation Angle
pub type RotR = crate::FieldReader<Rot>;
impl RotR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rot> {
        match self.bits {
            0 => Some(Rot::NoRotation),
            1 => Some(Rot::Right90degrees),
            2 => Some(Rot::Right180degrees),
            4 => Some(Rot::Right270degrees),
            _ => None,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_no_rotation(&self) -> bool {
        *self == Rot::NoRotation
    }
    ///`1`
    #[inline(always)]
    pub fn is_right90degrees(&self) -> bool {
        *self == Rot::Right90degrees
    }
    ///`10`
    #[inline(always)]
    pub fn is_right180degrees(&self) -> bool {
        *self == Rot::Right180degrees
    }
    ///`100`
    #[inline(always)]
    pub fn is_right270degrees(&self) -> bool {
        *self == Rot::Right270degrees
    }
}
///Field `ROT` writer - Rotation Angle
pub type RotW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rot>;
impl<'a, REG> RotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///`0`
    #[inline(always)]
    pub fn no_rotation(self) -> &'a mut crate::W<REG> {
        self.variant(Rot::NoRotation)
    }
    ///`1`
    #[inline(always)]
    pub fn right90degrees(self) -> &'a mut crate::W<REG> {
        self.variant(Rot::Right90degrees)
    }
    ///`10`
    #[inline(always)]
    pub fn right180degrees(self) -> &'a mut crate::W<REG> {
        self.variant(Rot::Right180degrees)
    }
    ///`100`
    #[inline(always)]
    pub fn right270degrees(self) -> &'a mut crate::W<REG> {
        self.variant(Rot::Right270degrees)
    }
}
impl R {
    ///Bits 0:2 - Rotation Angle
    #[inline(always)]
    pub fn rot(&self) -> RotR {
        RotR::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Rotation Angle
    #[inline(always)]
    pub fn rot(&mut self) -> RotW<'_, SetDirectionSpec> {
        RotW::new(self, 0)
    }
}
/**Set Rotation Angle

You can [`read`](crate::Reg::read) this register and get [`set_direction::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_direction::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SetDirectionSpec;
impl crate::RegisterSpec for SetDirectionSpec {
    type Ux = u32;
}
///`read()` method returns [`set_direction::R`](R) reader structure
impl crate::Readable for SetDirectionSpec {}
///`write(|w| ..)` method takes [`set_direction::W`](W) writer structure
impl crate::Writable for SetDirectionSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SET_DIRECTION to value 0
impl crate::Resettable for SetDirectionSpec {}
