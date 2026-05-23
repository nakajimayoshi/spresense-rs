///Register `FIF_PULL_EMP` reader
pub type R = crate::R<FifPullEmpSpec>;
/**RX buffer is empty

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyFlag {
    ///0: `0`
    NotEmpty = 0,
    ///1: `1`
    Empty = 1,
}
impl From<EmptyFlag> for bool {
    #[inline(always)]
    fn from(variant: EmptyFlag) -> Self {
        variant as u8 != 0
    }
}
///Field `EMPTY_FLAG` reader - RX buffer is empty
pub type EmptyFlagR = crate::BitReader<EmptyFlag>;
impl EmptyFlagR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EmptyFlag {
        match self.bits {
            false => EmptyFlag::NotEmpty,
            true => EmptyFlag::Empty,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == EmptyFlag::NotEmpty
    }
    ///`1`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == EmptyFlag::Empty
    }
}
impl R {
    ///Bit 0 - RX buffer is empty
    #[inline(always)]
    pub fn empty_flag(&self) -> EmptyFlagR {
        EmptyFlagR::new((self.bits & 1) != 0)
    }
}
/**RX buffer is empty (=1)

You can [`read`](crate::Reg::read) this register and get [`fif_pull_emp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FifPullEmpSpec;
impl crate::RegisterSpec for FifPullEmpSpec {
    type Ux = u32;
}
///`read()` method returns [`fif_pull_emp::R`](R) reader structure
impl crate::Readable for FifPullEmpSpec {}
///`reset()` method sets FIF_PULL_EMP to value 0
impl crate::Resettable for FifPullEmpSpec {}
