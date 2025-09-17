#[doc = "Register `DMACSync` reader"]
pub type R = crate::R<DmacsyncSpec>;
#[doc = "Register `DMACSync` writer"]
pub type W = crate::W<DmacsyncSpec>;
#[doc = "Field `DMACSync` reader - DMA synchronization logic for DMA request signals enabled or disabled"]
pub type DmacsyncR = crate::FieldReader<u16>;
#[doc = "Field `DMACSync` writer - DMA synchronization logic for DMA request signals enabled or disabled"]
pub type DmacsyncW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA synchronization logic for DMA request signals enabled or disabled"]
    #[inline(always)]
    pub fn dmacsync(&self) -> DmacsyncR {
        DmacsyncR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA synchronization logic for DMA request signals enabled or disabled"]
    #[inline(always)]
    pub fn dmacsync(&mut self) -> DmacsyncW<'_, DmacsyncSpec> {
        DmacsyncW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacsyncSpec;
impl crate::RegisterSpec for DmacsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacsync::R`](R) reader structure"]
impl crate::Readable for DmacsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacsync::W`](W) writer structure"]
impl crate::Writable for DmacsyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACSync to value 0xffff"]
impl crate::Resettable for DmacsyncSpec {
    const RESET_VALUE: u32 = 0xffff;
}
