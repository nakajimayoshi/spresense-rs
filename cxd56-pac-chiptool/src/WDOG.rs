///ARM PrimeCell SP805 watchdog timer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOG {
    ptr: *mut u8,
}
unsafe impl Send for WDOG {}
unsafe impl Sync for WDOG {}
impl WDOG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///Watchdog load register.
    #[inline(always)]
    pub const fn WDOGLOAD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    ///Watchdog current value register (read-only).
    #[inline(always)]
    pub const fn WDOGVALUE(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    ///Watchdog control register.
    #[inline(always)]
    pub const fn WDOGCONTROL(self) -> crate::common::Reg<regs::WDOGCONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    ///Watchdog interrupt clear (write any value to clear and reload).
    #[inline(always)]
    pub const fn WDOGINTCLR(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    ///Watchdog raw interrupt status.
    #[inline(always)]
    pub const fn WDOGRIS(self) -> crate::common::Reg<regs::WDOGRIS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    ///Watchdog masked interrupt status.
    #[inline(always)]
    pub const fn WDOGMIS(self) -> crate::common::Reg<regs::WDOGMIS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    ///Watchdog lock register (write 0x1ACCE551 to unlock).
    #[inline(always)]
    pub const fn WDOGLOCK(self) -> crate::common::Reg<regs::WDOGLOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
}
pub mod regs;
