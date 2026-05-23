///Register `COMMAND` writer
pub type W = crate::W<CommandSpec>;
/**Start rotation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd {
    ///1: `1`
    Start = 1,
}
impl From<Cmd> for bool {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as u8 != 0
    }
}
///Field `CMD` writer - Start rotation
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG, Cmd>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///`1`
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Start)
    }
}
impl W {
    ///Bit 0 - Start rotation
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CommandSpec> {
        CmdW::new(self, 0)
    }
}
/**Start rotation processing

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`command::W`](W) writer structure
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMMAND to value 0
impl crate::Resettable for CommandSpec {}
