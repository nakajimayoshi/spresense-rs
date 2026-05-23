///Register `BUSROM_CKEN` reader
pub type R = crate::R<BusromCkenSpec>;
///Register `BUSROM_CKEN` writer
pub type W = crate::W<BusromCkenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Bus ROM clock enable

You can [`read`](crate::Reg::read) this register and get [`busrom_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busrom_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BusromCkenSpec;
impl crate::RegisterSpec for BusromCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`busrom_cken::R`](R) reader structure
impl crate::Readable for BusromCkenSpec {}
///`write(|w| ..)` method takes [`busrom_cken::W`](W) writer structure
impl crate::Writable for BusromCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSROM_CKEN to value 0
impl crate::Resettable for BusromCkenSpec {}
