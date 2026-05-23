///Hardware 2D Graphics Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GE2D {
    ptr: *mut u8,
}
unsafe impl Send for GE2D {}
unsafe impl Sync for GE2D {}
impl GE2D {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///2D Graphics Engine Interrupt Control.
    #[inline(always)]
    pub const fn INTR_ENABLE(self) -> crate::common::Reg<regs::INTR_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    ///Descriptor Address Set Register.
    #[inline(always)]
    pub const fn ADDRESS_DESCRIPTOR_START(
        self,
    ) -> crate::common::Reg<regs::ADDRESS_DESCRIPTOR_START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    ///Command Register.
    #[inline(always)]
    pub const fn CMD_DESCRIPTOR(
        self,
    ) -> crate::common::Reg<regs::CMD_DESCRIPTOR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    ///Status Register.
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    ///Normal Descriptor Address Register.
    #[inline(always)]
    pub const fn STAT_NORMAL_DESCRIPTOR_ADDRESS(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    ///Current Descriptor Address Register.
    #[inline(always)]
    pub const fn STAT_CURRENT_DESCRIPTOR_ADDRESS(
        self,
    ) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs;
pub mod vals;
