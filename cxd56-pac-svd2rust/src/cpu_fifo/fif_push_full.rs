///Register `FIF_PUSH_FULL` reader
pub type R = crate::R<FifPushFullSpec>;
/**TX buffer is full

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FullFlag {
    ///0: `0`
    NotFull = 0,
    ///1: `1`
    Full = 1,
}
impl From<FullFlag> for bool {
    #[inline(always)]
    fn from(variant: FullFlag) -> Self {
        variant as u8 != 0
    }
}
///Field `FULL_FLAG` reader - TX buffer is full
pub type FullFlagR = crate::BitReader<FullFlag>;
impl FullFlagR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FullFlag {
        match self.bits {
            false => FullFlag::NotFull,
            true => FullFlag::Full,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == FullFlag::NotFull
    }
    ///`1`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FullFlag::Full
    }
}
impl R {
    ///Bit 0 - TX buffer is full
    #[inline(always)]
    pub fn full_flag(&self) -> FullFlagR {
        FullFlagR::new((self.bits & 1) != 0)
    }
}
/**TX buffer is full (=1)

You can [`read`](crate::Reg::read) this register and get [`fif_push_full::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FifPushFullSpec;
impl crate::RegisterSpec for FifPushFullSpec {
    type Ux = u32;
}
///`read()` method returns [`fif_push_full::R`](R) reader structure
impl crate::Readable for FifPushFullSpec {}
///`reset()` method sets FIF_PUSH_FULL to value 0
impl crate::Resettable for FifPushFullSpec {}
