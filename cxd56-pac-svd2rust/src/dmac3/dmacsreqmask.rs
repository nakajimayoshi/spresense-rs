///Register `DMACSREQMask` reader
pub type R = crate::R<DmacsreqmaskSpec>;
///Register `DMACSREQMask` writer
pub type W = crate::W<DmacsreqmaskSpec>;
///Field `DMACSREQMask` reader - Mask SREQ signals between DMAC and peripherals
pub type DmacsreqmaskR = crate::FieldReader<u16>;
///Field `DMACSREQMask` writer - Mask SREQ signals between DMAC and peripherals
pub type DmacsreqmaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Mask SREQ signals between DMAC and peripherals
    #[inline(always)]
    pub fn dmacsreqmask(&self) -> DmacsreqmaskR {
        DmacsreqmaskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Mask SREQ signals between DMAC and peripherals
    #[inline(always)]
    pub fn dmacsreqmask(&mut self) -> DmacsreqmaskW<'_, DmacsreqmaskSpec> {
        DmacsreqmaskW::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacsreqmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsreqmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacsreqmaskSpec;
impl crate::RegisterSpec for DmacsreqmaskSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacsreqmask::R`](R) reader structure
impl crate::Readable for DmacsreqmaskSpec {}
///`write(|w| ..)` method takes [`dmacsreqmask::W`](W) writer structure
impl crate::Writable for DmacsreqmaskSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACSREQMask to value 0xffff
impl crate::Resettable for DmacsreqmaskSpec {
    const RESET_VALUE: u32 = 0xffff;
}
