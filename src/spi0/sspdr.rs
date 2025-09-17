#[doc = "Register `SSPDR` reader"]
pub type R = crate::R<SspdrSpec>;
#[doc = "Register `SSPDR` writer"]
pub type W = crate::W<SspdrSpec>;
#[doc = "Field `DATA` reader - Transmit/Receive FIFO\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Transmit/Receive FIFO"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit/Receive FIFO"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit/Receive FIFO"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, SspdrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspdrSpec;
impl crate::RegisterSpec for SspdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspdr::R`](R) reader structure"]
impl crate::Readable for SspdrSpec {}
#[doc = "`write(|w| ..)` method takes [`sspdr::W`](W) writer structure"]
impl crate::Writable for SspdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSPDR to value 0"]
impl crate::Resettable for SspdrSpec {}
