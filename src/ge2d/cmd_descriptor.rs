#[doc = "Register `CMD_DESCRIPTOR` writer"]
pub type W = crate::W<CmdDescriptorSpec>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CommandType {
    #[doc = "0: No Operation"]
    Nop = 0,
    #[doc = "1: Normal Descriptor Set and Run"]
    Run = 1,
    #[doc = "3: Stop Descriptor"]
    Stop = 3,
}
impl From<CommandType> for u8 {
    #[inline(always)]
    fn from(variant: CommandType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CommandType {
    type Ux = u8;
}
impl crate::IsEnum for CommandType {}
#[doc = "Field `COMMAND` writer - Command"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 2, CommandType>;
impl<'a, REG> CommandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(CommandType::Nop)
    }
    #[doc = "Normal Descriptor Set and Run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(CommandType::Run)
    }
    #[doc = "Stop Descriptor"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(CommandType::Stop)
    }
}
impl W {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    pub fn command(&mut self) -> CommandW<'_, CmdDescriptorSpec> {
        CommandW::new(self, 0)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_descriptor::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdDescriptorSpec;
impl crate::RegisterSpec for CmdDescriptorSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd_descriptor::W`](W) writer structure"]
impl crate::Writable for CmdDescriptorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD_DESCRIPTOR to value 0"]
impl crate::Resettable for CmdDescriptorSpec {}
