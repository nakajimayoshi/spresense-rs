///Register `DMACC0DestAddr` reader
pub type R = crate::R<Dmacc0destAddrSpec>;
///Register `DMACC0DestAddr` writer
pub type W = crate::W<Dmacc0destAddrSpec>;
///Field `DestAddr` reader - DMA destination address
pub type DestAddrR = crate::FieldReader<u32>;
///Field `DestAddr` writer - DMA destination address
pub type DestAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMA destination address
    #[inline(always)]
    pub fn dest_addr(&self) -> DestAddrR {
        DestAddrR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - DMA destination address
    #[inline(always)]
    pub fn dest_addr(&mut self) -> DestAddrW<'_, Dmacc0destAddrSpec> {
        DestAddrW::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacc0dest_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0dest_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dmacc0destAddrSpec;
impl crate::RegisterSpec for Dmacc0destAddrSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacc0dest_addr::R`](R) reader structure
impl crate::Readable for Dmacc0destAddrSpec {}
///`write(|w| ..)` method takes [`dmacc0dest_addr::W`](W) writer structure
impl crate::Writable for Dmacc0destAddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACC0DestAddr to value 0
impl crate::Resettable for Dmacc0destAddrSpec {}
