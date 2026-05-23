///ARM PrimeCell SP804 dual-input timer 0.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIMER0 {
    ptr: *mut u8,
}
unsafe impl Send for TIMER0 {}
unsafe impl Sync for TIMER0 {}
impl TIMER0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///Counter reload value.
    #[inline(always)]
    pub const fn LOAD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    ///Current counter value (read-only).
    #[inline(always)]
    pub const fn VALUE(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    ///Timer mode, prescaler, and interrupt control.
    #[inline(always)]
    pub const fn CONTROL(self) -> crate::common::Reg<regs::CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    ///Interrupt clear (write any value to clear).
    #[inline(always)]
    pub const fn INTCLR(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    ///Raw interrupt status.
    #[inline(always)]
    pub const fn RIS(self) -> crate::common::Reg<regs::RIS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    ///Masked interrupt status.
    #[inline(always)]
    pub const fn MIS(self) -> crate::common::Reg<regs::MIS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    ///Background load register (no immediate counter restart).
    #[inline(always)]
    pub const fn BGLOAD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs;
