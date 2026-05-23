///Real-time clock 0 (always-on, 32.768 kHz, 47-bit dual counter, 3 alarms).
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTC0 {
    ptr: *mut u8,
}
unsafe impl Send for RTC0 {}
unsafe impl Sync for RTC0 {}
impl RTC0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///Write post-counter (32-bit integer seconds to set).
    #[inline(always)]
    pub const fn WRREGPOSTCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    ///Write pre-counter (15-bit fractional 32 kHz ticks to set).
    #[inline(always)]
    pub const fn WRREGPRECNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    ///Counter write request and busy status.
    #[inline(always)]
    pub const fn WRREGREQ(self) -> crate::common::Reg<regs::WRREGREQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    ///Write interrupt post-counter compare value.
    #[inline(always)]
    pub const fn WRINTPOSTCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    ///Write interrupt pre-counter compare value.
    #[inline(always)]
    pub const fn WRINTPRECNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    ///Write interrupt control.
    #[inline(always)]
    pub const fn WRINTCTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    ///Write interrupt clear.
    #[inline(always)]
    pub const fn WRINTCLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    ///Frequency offset correction value.
    #[inline(always)]
    pub const fn OFFSETVAL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    ///Frequency offset correction request and busy.
    #[inline(always)]
    pub const fn OFFSETREQ(self) -> crate::common::Reg<regs::OFFSETREQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    ///Read request (triggers a counter capture).
    #[inline(always)]
    pub const fn RDREQ(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    ///Captured post-counter value (integer seconds).
    #[inline(always)]
    pub const fn RDPOSTCNT(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    ///Captured pre-counter value (15-bit fractional ticks).
    #[inline(always)]
    pub const fn RDPRECNT(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    ///Real-time post-counter (free-running read).
    #[inline(always)]
    pub const fn RTPOSTCNT(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    ///Real-time pre-counter (free-running read).
    #[inline(always)]
    pub const fn RTPRECNT(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    ///Alarm 0 post-counter compare value.
    #[inline(always)]
    pub const fn SETALMPOSTCNT0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    ///Alarm 0 pre-counter compare value.
    #[inline(always)]
    pub const fn SETALMPRECNT0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    ///Alarm 1 post-counter compare value.
    #[inline(always)]
    pub const fn SETALMPOSTCNT1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    ///Alarm 1 pre-counter compare value.
    #[inline(always)]
    pub const fn SETALMPRECNT1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    ///Alarm 2 post-counter compare value.
    #[inline(always)]
    pub const fn SETALMPOSTCNT2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    ///Alarm 2 pre-counter compare value.
    #[inline(always)]
    pub const fn SETALMPRECNT2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    ///Alarm flag clear register.
    #[inline(always)]
    pub const fn ALMCLR(self) -> crate::common::Reg<regs::ALMCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    ///Alarm 0 output enable and busy status.
    #[inline(always)]
    pub const fn ALMOUTEN0(self) -> crate::common::Reg<regs::ALMOUTEN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    ///Alarm 1 output enable and busy status.
    #[inline(always)]
    pub const fn ALMOUTEN1(self) -> crate::common::Reg<regs::ALMOUTEN1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    ///Alarm 2 output enable and busy status.
    #[inline(always)]
    pub const fn ALMOUTEN2(self) -> crate::common::Reg<regs::ALMOUTEN2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    ///Alarm flag status (read-only).
    #[inline(always)]
    pub const fn ALMFLG(self) -> crate::common::Reg<regs::ALMFLG, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    ///Debug alarm 0 post-counter compare.
    #[inline(always)]
    pub const fn DBGSETALMPOSTCNT0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    ///Debug alarm 0 pre-counter compare.
    #[inline(always)]
    pub const fn DBGSETALMPRECNT0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    ///Debug alarm 1 post-counter compare.
    #[inline(always)]
    pub const fn DBGSETALMPOSTCNT1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    ///Debug alarm 1 pre-counter compare.
    #[inline(always)]
    pub const fn DBGSETALMPRECNT1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    ///Debug alarm 2 post-counter compare.
    #[inline(always)]
    pub const fn DBGSETALMPOSTCNT2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    ///Debug alarm 2 pre-counter compare.
    #[inline(always)]
    pub const fn DBGSETALMPRECNT2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
}
pub mod regs;
