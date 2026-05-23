///CMOS Image Sensor IF.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CISIF {
    ptr: *mut u8,
}
unsafe impl Send for CISIF {}
unsafe impl Sync for CISIF {}
impl CISIF {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///Interrupt status register.
    #[inline(always)]
    pub const fn INTR_STAT(self) -> crate::common::Reg<regs::INTR_STAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    ///Input data enable register.
    #[inline(always)]
    pub const fn DIN_ENABLE(self) -> crate::common::Reg<regs::DIN_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    ///CIS input activa area size setting register.
    #[inline(always)]
    pub const fn CIS_SIZE(self) -> crate::common::Reg<regs::CIS_SIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    ///Active area position setting register.
    #[inline(always)]
    pub const fn ACT_POS(self) -> crate::common::Reg<regs::ACT_POS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    ///Active area size setting register.
    #[inline(always)]
    pub const fn ACT_SIZE(self) -> crate::common::Reg<regs::ACT_SIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    ///CIS input mode register.
    #[inline(always)]
    pub const fn MODE(self) -> crate::common::Reg<regs::MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    ///CIS input in line code setting register.
    #[inline(always)]
    pub const fn ILCODE(self) -> crate::common::Reg<regs::ILCODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    ///CIS input data format setting register.
    #[inline(always)]
    pub const fn FORMAT(self) -> crate::common::Reg<regs::FORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    ///CIS input Vsync/Hsync polarity setting register.
    #[inline(always)]
    pub const fn POL(self) -> crate::common::Reg<regs::POL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    ///YCC data frame memory start address.
    #[inline(always)]
    pub const fn YCC_START_ADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    ///YCC data frame memory area size.
    #[inline(always)]
    pub const fn YCC_DAREA_SIZE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    ///YCC data frame memory notice of storage size.
    #[inline(always)]
    pub const fn YCC_NSTRG_SIZE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    ///YCC data frame memory storage size counter.
    #[inline(always)]
    pub const fn YCC_DSTRG_CONT(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    ///YCC data frame memory read counter.
    #[inline(always)]
    pub const fn YCC_DREAD_CONT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    ///JPEG data frame memory start address.
    #[inline(always)]
    pub const fn JPG_START_ADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    ///JPEG data frame memory area size.
    #[inline(always)]
    pub const fn JPG_DAREA_SIZE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    ///JPEG data frame memory notice of storage size.
    #[inline(always)]
    pub const fn JPG_NSTRG_SIZE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    ///JPEG data frame memory storage size counter.
    #[inline(always)]
    pub const fn JPG_DSTRG_CONT(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    ///JPEG data frame memory read counter.
    #[inline(always)]
    pub const fn JPG_DREAD_CONT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    ///Execution command register.
    #[inline(always)]
    pub const fn EXE_CMD(self) -> crate::common::Reg<regs::EXE_CMD, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
}
pub mod regs;
pub mod vals;
