#[doc = "Register `FIF_PUSH_CMP` writer"]
pub type W = crate::W<FifPushCmpSpec>;
#[doc = "TX data write complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PushCmp {
    #[doc = "0: `0`"]
    DontCare = 0,
    #[doc = "1: `1`"]
    Complete = 1,
}
impl From<PushCmp> for bool {
    #[inline(always)]
    fn from(variant: PushCmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUSH_CMP` writer - TX data write complete"]
pub type PushCmpW<'a, REG> = crate::BitWriter<'a, REG, PushCmp>;
impl<'a, REG> PushCmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dont_care(self) -> &'a mut crate::W<REG> {
        self.variant(PushCmp::DontCare)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(PushCmp::Complete)
    }
}
impl W {
    #[doc = "Bit 0 - TX data write complete"]
    #[inline(always)]
    pub fn push_cmp(&mut self) -> PushCmpW<'_, FifPushCmpSpec> {
        PushCmpW::new(self, 0)
    }
}
#[doc = "TX data write complete\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_push_cmp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifPushCmpSpec;
impl crate::RegisterSpec for FifPushCmpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fif_push_cmp::W`](W) writer structure"]
impl crate::Writable for FifPushCmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIF_PUSH_CMP to value 0"]
impl crate::Resettable for FifPushCmpSpec {}
