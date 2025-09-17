#[doc = "Register `SLAVETYPE` reader"]
pub type R = crate::R<SlavetypeSpec>;
#[doc = "Register `SLAVETYPE` writer"]
pub type W = crate::W<SlavetypeSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SlaveType {
    #[doc = "0: Select CS0"]
    Cs0 = 0,
    #[doc = "1: Select CS1"]
    Cs1 = 1,
    #[doc = "2: Select CS2"]
    Cs2 = 2,
}
impl From<SlaveType> for u8 {
    #[inline(always)]
    fn from(variant: SlaveType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SlaveType {
    type Ux = u8;
}
impl crate::IsEnum for SlaveType {}
#[doc = "Field `SLAVE_TYPE` reader - "]
pub type SlaveTypeR = crate::FieldReader<SlaveType>;
impl SlaveTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SlaveType> {
        match self.bits {
            0 => Some(SlaveType::Cs0),
            1 => Some(SlaveType::Cs1),
            2 => Some(SlaveType::Cs2),
            _ => None,
        }
    }
    #[doc = "Select CS0"]
    #[inline(always)]
    pub fn is_cs0(&self) -> bool {
        *self == SlaveType::Cs0
    }
    #[doc = "Select CS1"]
    #[inline(always)]
    pub fn is_cs1(&self) -> bool {
        *self == SlaveType::Cs1
    }
    #[doc = "Select CS2"]
    #[inline(always)]
    pub fn is_cs2(&self) -> bool {
        *self == SlaveType::Cs2
    }
}
#[doc = "Field `SLAVE_TYPE` writer - "]
pub type SlaveTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SlaveType>;
impl<'a, REG> SlaveTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select CS0"]
    #[inline(always)]
    pub fn cs0(self) -> &'a mut crate::W<REG> {
        self.variant(SlaveType::Cs0)
    }
    #[doc = "Select CS1"]
    #[inline(always)]
    pub fn cs1(self) -> &'a mut crate::W<REG> {
        self.variant(SlaveType::Cs1)
    }
    #[doc = "Select CS2"]
    #[inline(always)]
    pub fn cs2(self) -> &'a mut crate::W<REG> {
        self.variant(SlaveType::Cs2)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn slave_type(&self) -> SlaveTypeR {
        SlaveTypeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn slave_type(&mut self) -> SlaveTypeW<'_, SlavetypeSpec> {
        SlaveTypeW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`slavetype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slavetype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlavetypeSpec;
impl crate::RegisterSpec for SlavetypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slavetype::R`](R) reader structure"]
impl crate::Readable for SlavetypeSpec {}
#[doc = "`write(|w| ..)` method takes [`slavetype::W`](W) writer structure"]
impl crate::Writable for SlavetypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLAVETYPE to value 0"]
impl crate::Resettable for SlavetypeSpec {}
