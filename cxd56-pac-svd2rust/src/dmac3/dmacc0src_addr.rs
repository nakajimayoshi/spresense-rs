#[doc = "Register `DMACC0SrcAddr` reader"]
pub type R = crate::R<Dmacc0srcAddrSpec>;
#[doc = "Register `DMACC0SrcAddr` writer"]
pub type W = crate::W<Dmacc0srcAddrSpec>;
#[doc = "Field `SrcAddr` reader - DMA source address"]
pub type SrcAddrR = crate::FieldReader<u32>;
#[doc = "Field `SrcAddr` writer - DMA source address"]
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA source address"]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA source address"]
    #[inline(always)]
    pub fn src_addr(&mut self) -> SrcAddrW<'_, Dmacc0srcAddrSpec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0src_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0src_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmacc0srcAddrSpec;
impl crate::RegisterSpec for Dmacc0srcAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacc0src_addr::R`](R) reader structure"]
impl crate::Readable for Dmacc0srcAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacc0src_addr::W`](W) writer structure"]
impl crate::Writable for Dmacc0srcAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACC0SrcAddr to value 0"]
impl crate::Resettable for Dmacc0srcAddrSpec {}
