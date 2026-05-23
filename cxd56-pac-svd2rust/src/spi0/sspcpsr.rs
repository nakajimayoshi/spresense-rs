///Register `SSPCPSR` reader
pub type R = crate::R<SspcpsrSpec>;
///Register `SSPCPSR` writer
pub type W = crate::W<SspcpsrSpec>;
///Field `CPSDVSR` reader - Clock prescale divisor
pub type CpsdvsrR = crate::FieldReader;
///Field `CPSDVSR` writer - Clock prescale divisor
pub type CpsdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Clock prescale divisor
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CpsdvsrR {
        CpsdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Clock prescale divisor
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CpsdvsrW<'_, SspcpsrSpec> {
        CpsdvsrW::new(self, 0)
    }
}
/**Clock prescale register

You can [`read`](crate::Reg::read) this register and get [`sspcpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SspcpsrSpec;
impl crate::RegisterSpec for SspcpsrSpec {
    type Ux = u32;
}
///`read()` method returns [`sspcpsr::R`](R) reader structure
impl crate::Readable for SspcpsrSpec {}
///`write(|w| ..)` method takes [`sspcpsr::W`](W) writer structure
impl crate::Writable for SspcpsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSPCPSR to value 0
impl crate::Resettable for SspcpsrSpec {}
