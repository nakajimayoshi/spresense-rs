///Register `STS[%s]` reader
pub type R = crate::R<StsSpec>;
///Field `STATE` reader - Lock state
pub type StateR = crate::FieldReader;
///Field `LOCK_OWNER` reader - CPU id currently holding the lock
pub type LockOwnerR = crate::FieldReader;
///Field `RESV_OWNER` reader - CPU id holding the reservation
pub type ResvOwnerR = crate::FieldReader;
impl R {
    ///Bits 0:3 - Lock state
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:20 - CPU id currently holding the lock
    #[inline(always)]
    pub fn lock_owner(&self) -> LockOwnerR {
        LockOwnerR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - CPU id holding the reservation
    #[inline(always)]
    pub fn resv_owner(&self) -> ResvOwnerR {
        ResvOwnerR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
/**Semaphore status (read-only)

You can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
///`read()` method returns [`sts::R`](R) reader structure
impl crate::Readable for StsSpec {}
///`reset()` method sets STS[%s] to value 0
impl crate::Resettable for StsSpec {}
