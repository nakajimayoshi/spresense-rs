#[doc = "Register `FIF_PULL_CMP` writer"]
pub type W = crate::W<FifPullCmpSpec>;
#[doc = "RX data read complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PullCmp {
    #[doc = "0: `0`"]
    Dontcare = 0,
    #[doc = "1: `1`"]
    Complete = 1,
}
impl From<PullCmp> for bool {
    #[inline(always)]
    fn from(variant: PullCmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULL_CMP` writer - RX data read complete"]
pub type PullCmpW<'a, REG> = crate::BitWriter<'a, REG, PullCmp>;
impl<'a, REG> PullCmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dontcare(self) -> &'a mut crate::W<REG> {
        self.variant(PullCmp::Dontcare)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(PullCmp::Complete)
    }
}
impl W {
    #[doc = "Bit 0 - RX data read complete"]
    #[inline(always)]
    pub fn pull_cmp(&mut self) -> PullCmpW<'_, FifPullCmpSpec> {
        PullCmpW::new(self, 0)
    }
}
#[doc = "RX data read complete\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_pull_cmp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifPullCmpSpec;
impl crate::RegisterSpec for FifPullCmpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fif_pull_cmp::W`](W) writer structure"]
impl crate::Writable for FifPullCmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIF_PULL_CMP to value 0"]
impl crate::Resettable for FifPullCmpSpec {}
