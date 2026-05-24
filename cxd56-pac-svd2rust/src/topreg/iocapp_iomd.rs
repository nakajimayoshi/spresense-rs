///Register `IOCAPP_IOMD` reader
pub type R = crate::R<IocappIomdSpec>;
///Register `IOCAPP_IOMD` writer
pub type W = crate::W<IocappIomdSpec>;
///Field `UART2` reader - Mode select for UART2 TXD/RXD pins (UART2 = Func1)
pub type Uart2R = crate::FieldReader;
///Field `UART2` writer - Mode select for UART2 TXD/RXD pins (UART2 = Func1)
pub type Uart2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 2:3 - Mode select for UART2 TXD/RXD pins (UART2 = Func1)
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    ///Bits 2:3 - Mode select for UART2 TXD/RXD pins (UART2 = Func1)
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, IocappIomdSpec> {
        Uart2W::new(self, 2)
    }
}
/**APP-domain IO-cell mode-mux register

You can [`read`](crate::Reg::read) this register and get [`iocapp_iomd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocapp_iomd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IocappIomdSpec;
impl crate::RegisterSpec for IocappIomdSpec {
    type Ux = u32;
}
///`read()` method returns [`iocapp_iomd::R`](R) reader structure
impl crate::Readable for IocappIomdSpec {}
///`write(|w| ..)` method takes [`iocapp_iomd::W`](W) writer structure
impl crate::Writable for IocappIomdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCAPP_IOMD to value 0
impl crate::Resettable for IocappIomdSpec {}
