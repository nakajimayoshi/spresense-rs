///Register `STATUS` reader
pub type R = crate::R<StatusSpec>;
/**Running Status (1 = running)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    ///0: `0`
    Stop = 0,
    ///1: `1`
    Running = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
///Field `STATUS` reader - Running Status (1 = running)
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Stop,
            true => Status::Running,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Status::Stop
    }
    ///`1`
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Status::Running
    }
}
impl R {
    ///Bit 0 - Running Status (1 = running)
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
/**Running Status

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for StatusSpec {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for StatusSpec {}
