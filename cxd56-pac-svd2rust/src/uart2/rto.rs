///Register `RTO` reader
pub type R = crate::R<RtoSpec>;
///Register `RTO` writer
pub type W = crate::W<RtoSpec>;
///Field `TIMEOUT` reader - Recieve timeout
pub type TimeoutR = crate::FieldReader<u16>;
///Field `TIMEOUT` writer - Recieve timeout
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Recieve timeout
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Recieve timeout
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, RtoSpec> {
        TimeoutW::new(self, 0)
    }
}
/**Receive Timeout Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RtoSpec;
impl crate::RegisterSpec for RtoSpec {
    type Ux = u32;
}
///`read()` method returns [`rto::R`](R) reader structure
impl crate::Readable for RtoSpec {}
///`write(|w| ..)` method takes [`rto::W`](W) writer structure
impl crate::Writable for RtoSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTO to value 0
impl crate::Resettable for RtoSpec {}
