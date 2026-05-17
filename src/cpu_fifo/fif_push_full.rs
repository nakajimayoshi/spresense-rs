#[doc = "Register `FIF_PUSH_FULL` reader"]
pub type R = crate::R<FifPushFullSpec>;
#[doc = "TX buffer is full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FullFlag {
    #[doc = "0: `0`"]
    NotFull = 0,
    #[doc = "1: `1`"]
    Full = 1,
}
impl From<FullFlag> for bool {
    #[inline(always)]
    fn from(variant: FullFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FULL_FLAG` reader - TX buffer is full"]
pub type FullFlagR = crate::BitReader<FullFlag>;
impl FullFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FullFlag {
        match self.bits {
            false => FullFlag::NotFull,
            true => FullFlag::Full,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == FullFlag::NotFull
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FullFlag::Full
    }
}
impl R {
    #[doc = "Bit 0 - TX buffer is full"]
    #[inline(always)]
    pub fn full_flag(&self) -> FullFlagR {
        FullFlagR::new((self.bits & 1) != 0)
    }
}
#[doc = "TX buffer is full (=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_push_full::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifPushFullSpec;
impl crate::RegisterSpec for FifPushFullSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fif_push_full::R`](R) reader structure"]
impl crate::Readable for FifPushFullSpec {}
#[doc = "`reset()` method sets FIF_PUSH_FULL to value 0"]
impl crate::Resettable for FifPushFullSpec {}
