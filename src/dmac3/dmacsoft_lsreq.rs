#[doc = "Register `DMACSoftLSReq` reader"]
pub type R = crate::R<DmacsoftLsreqSpec>;
#[doc = "Register `DMACSoftLSReq` writer"]
pub type W = crate::W<DmacsoftLsreqSpec>;
#[doc = "Field `SoftLSReq` reader - Software last single request"]
pub type SoftLsreqR = crate::FieldReader<u16>;
#[doc = "Field `SoftLSReq` writer - Software last single request"]
pub type SoftLsreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Software last single request"]
    #[inline(always)]
    pub fn soft_lsreq(&self) -> SoftLsreqR {
        SoftLsreqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Software last single request"]
    #[inline(always)]
    pub fn soft_lsreq(&mut self) -> SoftLsreqW<'_, DmacsoftLsreqSpec> {
        SoftLsreqW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsoft_lsreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_lsreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacsoftLsreqSpec;
impl crate::RegisterSpec for DmacsoftLsreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacsoft_lsreq::R`](R) reader structure"]
impl crate::Readable for DmacsoftLsreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacsoft_lsreq::W`](W) writer structure"]
impl crate::Writable for DmacsoftLsreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACSoftLSReq to value 0"]
impl crate::Resettable for DmacsoftLsreqSpec {}
