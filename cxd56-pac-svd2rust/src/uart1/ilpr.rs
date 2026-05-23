///Register `ILPR` reader
pub type R = crate::R<IlprSpec>;
///Register `ILPR` writer
pub type W = crate::W<IlprSpec>;
///Field `ILPDVSR` reader - 8-bit low-power divisor value
pub type IlpdvsrR = crate::FieldReader;
///Field `ILPDVSR` writer - 8-bit low-power divisor value
pub type IlpdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - 8-bit low-power divisor value
    #[inline(always)]
    pub fn ilpdvsr(&self) -> IlpdvsrR {
        IlpdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - 8-bit low-power divisor value
    #[inline(always)]
    pub fn ilpdvsr(&mut self) -> IlpdvsrW<'_, IlprSpec> {
        IlpdvsrW::new(self, 0)
    }
}
/**IrDA Low-Power Counter Register

You can [`read`](crate::Reg::read) this register and get [`ilpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IlprSpec;
impl crate::RegisterSpec for IlprSpec {
    type Ux = u32;
}
///`read()` method returns [`ilpr::R`](R) reader structure
impl crate::Readable for IlprSpec {}
///`write(|w| ..)` method takes [`ilpr::W`](W) writer structure
impl crate::Writable for IlprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ILPR to value 0
impl crate::Resettable for IlprSpec {}
