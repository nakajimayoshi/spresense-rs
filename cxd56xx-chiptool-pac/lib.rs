#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (828b7b8 2025-09-01))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "1 - CRG interrupt"]
    CRG = 1,
    #[doc = "27 - UART1 interrupt"]
    UART1 = 27,
    #[doc = "32 - SPI3 interrupt"]
    SPI3 = 32,
    #[doc = "90 - SPI0 interrupt"]
    SPI0 = 90,
    #[doc = "106 - 2D interrupt"]
    _2D = 106,
    #[doc = "107 - Rotation interrupt"]
    ROT = 107,
    #[doc = "124 - CISIF interrupt"]
    CISIF = 124,
    #[doc = "125 - SPI5 interrupt"]
    SPI5 = 125,
    #[doc = "126 - DMAC3 interrupt"]
    DMAC3 = 126,
    #[doc = "127 - UART2 interrupt"]
    UART2 = 127,
    #[doc = "129 - SPI4 interrupt"]
    SPI4 = 129,
    #[doc = "134 - DMAC1 interrupt"]
    DMAC1 = 134,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn CRG();
        fn UART1();
        fn SPI3();
        fn SPI0();
        fn _2D();
        fn ROT();
        fn CISIF();
        fn SPI5();
        fn DMAC3();
        fn UART2();
        fn SPI4();
        fn DMAC1();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 135] = [
        Vector { _reserved: 0 },
        Vector { _handler: CRG },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: UART1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: _2D },
        Vector { _handler: ROT },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CISIF },
        Vector { _handler: SPI5 },
        Vector { _handler: DMAC3 },
        Vector { _handler: UART2 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMAC1 },
    ];
}
#[doc = "SRAM Control"]
pub const SMP_RAM_CTRL: smp_ram_ctrl::SmpRamCtrl =
    unsafe { smp_ram_ctrl::SmpRamCtrl::from_ptr(0x0200_1000usize as _) };
#[doc = "Clock / Reset / Gating"]
pub const CRG: crg::Crg = unsafe { crg::Crg::from_ptr(0x0201_1000usize as _) };
#[doc = "Address Converter"]
pub const ADDRCONV: addrconv::Addrconv =
    unsafe { addrconv::Addrconv::from_ptr(0x0201_2000usize as _) };
#[doc = "DMA controller (ADMAC)"]
pub const DMAC1: dmac1::Dmac1 = unsafe { dmac1::Dmac1::from_ptr(0x0202_0000usize as _) };
#[doc = "CMOS Image Sensor IF"]
pub const CISIF: cisif::Cisif = unsafe { cisif::Cisif::from_ptr(0x0210_0000usize as _) };
#[doc = "Hardware 2D Graphics Engine"]
pub const GE2D: ge2d::Ge2d = unsafe { ge2d::Ge2d::from_ptr(0x0210_1000usize as _) };
#[doc = "Image rotation"]
pub const ROT: rot::Rot = unsafe { rot::Rot::from_ptr(0x0210_1400usize as _) };
#[doc = "DMA controller (IDMAC)"]
pub const DMAC3: dmac3::Dmac3 = unsafe { dmac3::Dmac3::from_ptr(0x0210_2000usize as _) };
#[doc = "UART"]
pub const UART2: uart2::Uart2 = unsafe { uart2::Uart2::from_ptr(0x0210_3000usize as _) };
#[doc = "Synchronous Serial Port Controller (IMG SPI)"]
pub const SPI4: spi4::Spi4 = unsafe { spi4::Spi4::from_ptr(0x0210_3400usize as _) };
#[doc = "Synchronous Serial Port Controller (IMG WSPI)"]
pub const SPI5: spi4::Spi4 = unsafe { spi4::Spi4::from_ptr(0x0210_3c00usize as _) };
#[doc = "Synchronous Serial Port Controller (SCU SPI)"]
pub const SPI3: spi0::Spi0 = unsafe { spi0::Spi0::from_ptr(0x0418_d000usize as _) };
#[doc = "Synchronous Serial Port Controller (SPIM)"]
pub const SPI0: spi0::Spi0 = unsafe { spi0::Spi0::from_ptr(0x041a_a000usize as _) };
#[doc = "UART"]
pub const UART1: uart1::Uart1 = unsafe { uart1::Uart1::from_ptr(0x041a_c000usize as _) };
#[doc = "CPU FIFO Control"]
pub const CPU_FIFO: cpu_fifo::CpuFifo =
    unsafe { cpu_fifo::CpuFifo::from_ptr(0x4600_c400usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod addrconv {
    #[doc = "Address Converter"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrconv {
        ptr: *mut u8,
    }
    unsafe impl Send for Addrconv {}
    unsafe impl Sync for Addrconv {}
    impl Addrconv {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "CPU 0 address conversion area 0/1"]
        #[inline(always)]
        pub const fn acnv_p0_dst0(self) -> crate::common::Reg<regs::AcnvP0Dst0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "CPU 0 address conversion area 2/3"]
        #[inline(always)]
        pub const fn acnv_p0_dst1(self) -> crate::common::Reg<regs::AcnvP0Dst1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "CPU 0 address conversion area 4/5"]
        #[inline(always)]
        pub const fn acnv_p0_dst2(self) -> crate::common::Reg<regs::AcnvP0Dst2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "CPU 0 address conversion area 6/7"]
        #[inline(always)]
        pub const fn acnv_p0_dst3(self) -> crate::common::Reg<regs::AcnvP0Dst3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "CPU 0 address conversion area 8/9"]
        #[inline(always)]
        pub const fn acnv_p0_dst4(self) -> crate::common::Reg<regs::AcnvP0Dst4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "CPU 0 address conversion area A/B"]
        #[inline(always)]
        pub const fn acnv_p0_dst5(self) -> crate::common::Reg<regs::AcnvP0Dst5, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "CPU 0 address conversion area C/D"]
        #[inline(always)]
        pub const fn acnv_p0_dst6(self) -> crate::common::Reg<regs::AcnvP0Dst6, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "CPU 0 address conversion area E/F"]
        #[inline(always)]
        pub const fn acnv_p0_dst7(self) -> crate::common::Reg<regs::AcnvP0Dst7, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "CPU 0 address conversion area 0/1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AcnvP0Dst0(pub u32);
        impl AcnvP0Dst0 {
            #[doc = "0x00000 .. 0x10000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_0(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x00000 .. 0x10000"]
            #[inline(always)]
            pub const fn set_area_0(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "0x10000 .. 0x20000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_1(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x10000 .. 0x20000"]
            #[inline(always)]
            pub const fn set_area_1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
            }
        }
        impl Default for AcnvP0Dst0 {
            #[inline(always)]
            fn default() -> AcnvP0Dst0 {
                AcnvP0Dst0(0)
            }
        }
        impl core::fmt::Debug for AcnvP0Dst0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AcnvP0Dst0")
                    .field("area_0", &self.area_0())
                    .field("area_1", &self.area_1())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AcnvP0Dst0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AcnvP0Dst0 {{ area_0: {=u16:?}, area_1: {=u16:?} }}",
                    self.area_0(),
                    self.area_1()
                )
            }
        }
        #[doc = "CPU 0 address conversion area 2/3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AcnvP0Dst1(pub u32);
        impl AcnvP0Dst1 {
            #[doc = "0x20000 .. 0x30000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_2(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x20000 .. 0x30000"]
            #[inline(always)]
            pub const fn set_area_2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "0x30000 .. 0x40000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_3(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x30000 .. 0x40000"]
            #[inline(always)]
            pub const fn set_area_3(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
            }
        }
        impl Default for AcnvP0Dst1 {
            #[inline(always)]
            fn default() -> AcnvP0Dst1 {
                AcnvP0Dst1(0)
            }
        }
        impl core::fmt::Debug for AcnvP0Dst1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AcnvP0Dst1")
                    .field("area_2", &self.area_2())
                    .field("area_3", &self.area_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AcnvP0Dst1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AcnvP0Dst1 {{ area_2: {=u16:?}, area_3: {=u16:?} }}",
                    self.area_2(),
                    self.area_3()
                )
            }
        }
        #[doc = "CPU 0 address conversion area 4/5"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AcnvP0Dst2(pub u32);
        impl AcnvP0Dst2 {
            #[doc = "0x40000 .. 0x50000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_4(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x40000 .. 0x50000"]
            #[inline(always)]
            pub const fn set_area_4(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "0x50000 .. 0x60000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_5(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x50000 .. 0x60000"]
            #[inline(always)]
            pub const fn set_area_5(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
            }
        }
        impl Default for AcnvP0Dst2 {
            #[inline(always)]
            fn default() -> AcnvP0Dst2 {
                AcnvP0Dst2(0)
            }
        }
        impl core::fmt::Debug for AcnvP0Dst2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AcnvP0Dst2")
                    .field("area_4", &self.area_4())
                    .field("area_5", &self.area_5())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AcnvP0Dst2 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AcnvP0Dst2 {{ area_4: {=u16:?}, area_5: {=u16:?} }}",
                    self.area_4(),
                    self.area_5()
                )
            }
        }
        #[doc = "CPU 0 address conversion area 6/7"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AcnvP0Dst3(pub u32);
        impl AcnvP0Dst3 {
            #[doc = "0x60000 .. 0x70000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_6(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x60000 .. 0x70000"]
            #[inline(always)]
            pub const fn set_area_6(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "0x70000 .. 0x80000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_7(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x70000 .. 0x80000"]
            #[inline(always)]
            pub const fn set_area_7(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
            }
        }
        impl Default for AcnvP0Dst3 {
            #[inline(always)]
            fn default() -> AcnvP0Dst3 {
                AcnvP0Dst3(0)
            }
        }
        impl core::fmt::Debug for AcnvP0Dst3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AcnvP0Dst3")
                    .field("area_6", &self.area_6())
                    .field("area_7", &self.area_7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AcnvP0Dst3 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AcnvP0Dst3 {{ area_6: {=u16:?}, area_7: {=u16:?} }}",
                    self.area_6(),
                    self.area_7()
                )
            }
        }
        #[doc = "CPU 0 address conversion area 8/9"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AcnvP0Dst4(pub u32);
        impl AcnvP0Dst4 {
            #[doc = "0x80000 .. 0x90000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_8(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x80000 .. 0x90000"]
            #[inline(always)]
            pub const fn set_area_8(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "0x90000 .. 0xa0000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_9(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x07ff;
                val as u16
            }
            #[doc = "0x90000 .. 0xa0000"]
            #[inline(always)]
            pub const fn set_area_9(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
            }
        }
        impl Default for AcnvP0Dst4 {
            #[inline(always)]
            fn default() -> AcnvP0Dst4 {
                AcnvP0Dst4(0)
            }
        }
        impl core::fmt::Debug for AcnvP0Dst4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AcnvP0Dst4")
                    .field("area_8", &self.area_8())
                    .field("area_9", &self.area_9())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AcnvP0Dst4 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AcnvP0Dst4 {{ area_8: {=u16:?}, area_9: {=u16:?} }}",
                    self.area_8(),
                    self.area_9()
                )
            }
        }
        #[doc = "CPU 0 address conversion area A/B"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AcnvP0Dst5(pub u32);
        impl AcnvP0Dst5 {
            #[doc = "0xa0000 .. 0xb0000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "0xa0000 .. 0xb0000"]
            #[inline(always)]
            pub const fn set_area_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "0xb0000 .. 0xc0000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_b(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x07ff;
                val as u16
            }
            #[doc = "0xb0000 .. 0xc0000"]
            #[inline(always)]
            pub const fn set_area_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
            }
        }
        impl Default for AcnvP0Dst5 {
            #[inline(always)]
            fn default() -> AcnvP0Dst5 {
                AcnvP0Dst5(0)
            }
        }
        impl core::fmt::Debug for AcnvP0Dst5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AcnvP0Dst5")
                    .field("area_a", &self.area_a())
                    .field("area_b", &self.area_b())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AcnvP0Dst5 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AcnvP0Dst5 {{ area_a: {=u16:?}, area_b: {=u16:?} }}",
                    self.area_a(),
                    self.area_b()
                )
            }
        }
        #[doc = "CPU 0 address conversion area C/D"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AcnvP0Dst6(pub u32);
        impl AcnvP0Dst6 {
            #[doc = "0xc0000 .. 0xd0000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_c(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "0xc0000 .. 0xd0000"]
            #[inline(always)]
            pub const fn set_area_c(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "0xd0000 .. 0xe0000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_d(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x07ff;
                val as u16
            }
            #[doc = "0xd0000 .. 0xe0000"]
            #[inline(always)]
            pub const fn set_area_d(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
            }
        }
        impl Default for AcnvP0Dst6 {
            #[inline(always)]
            fn default() -> AcnvP0Dst6 {
                AcnvP0Dst6(0)
            }
        }
        impl core::fmt::Debug for AcnvP0Dst6 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AcnvP0Dst6")
                    .field("area_c", &self.area_c())
                    .field("area_d", &self.area_d())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AcnvP0Dst6 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AcnvP0Dst6 {{ area_c: {=u16:?}, area_d: {=u16:?} }}",
                    self.area_c(),
                    self.area_d()
                )
            }
        }
        #[doc = "CPU 0 address conversion area E/F"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AcnvP0Dst7(pub u32);
        impl AcnvP0Dst7 {
            #[doc = "0xe0000 .. 0xf0000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_e(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "0xe0000 .. 0xf0000"]
            #[inline(always)]
            pub const fn set_area_e(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "0xf0000 .. 0x100000"]
            #[must_use]
            #[inline(always)]
            pub const fn area_f(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x07ff;
                val as u16
            }
            #[doc = "0xf0000 .. 0x100000"]
            #[inline(always)]
            pub const fn set_area_f(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
            }
        }
        impl Default for AcnvP0Dst7 {
            #[inline(always)]
            fn default() -> AcnvP0Dst7 {
                AcnvP0Dst7(0)
            }
        }
        impl core::fmt::Debug for AcnvP0Dst7 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AcnvP0Dst7")
                    .field("area_e", &self.area_e())
                    .field("area_f", &self.area_f())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AcnvP0Dst7 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AcnvP0Dst7 {{ area_e: {=u16:?}, area_f: {=u16:?} }}",
                    self.area_e(),
                    self.area_f()
                )
            }
        }
    }
}
pub mod cisif {
    #[doc = "CMOS Image Sensor IF"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cisif {
        ptr: *mut u8,
    }
    unsafe impl Send for Cisif {}
    unsafe impl Sync for Cisif {}
    impl Cisif {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Interrupt status register"]
        #[inline(always)]
        pub const fn intr_stat(self) -> crate::common::Reg<regs::IntrStat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Input data enable register"]
        #[inline(always)]
        pub const fn din_enable(self) -> crate::common::Reg<regs::DinEnable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "CIS input activa area size setting register"]
        #[inline(always)]
        pub const fn cis_size(self) -> crate::common::Reg<regs::CisSize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "Active area position setting register"]
        #[inline(always)]
        pub const fn act_pos(self) -> crate::common::Reg<regs::ActPos, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "Active area size setting register"]
        #[inline(always)]
        pub const fn act_size(self) -> crate::common::Reg<regs::ActSize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
        }
        #[doc = "CIS input mode register"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[doc = "CIS input in line code setting register"]
        #[inline(always)]
        pub const fn ilcode(self) -> crate::common::Reg<regs::Ilcode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
        }
        #[doc = "CIS input data format setting register"]
        #[inline(always)]
        pub const fn format(self) -> crate::common::Reg<regs::Format, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
        }
        #[doc = "CIS input Vsync/Hsync polarity setting register"]
        #[inline(always)]
        pub const fn pol(self) -> crate::common::Reg<regs::Pol, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
        }
        #[doc = "YCC data frame memory start address"]
        #[inline(always)]
        pub const fn ycc_start_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
        }
        #[doc = "YCC data frame memory area size"]
        #[inline(always)]
        pub const fn ycc_darea_size(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
        }
        #[doc = "YCC data frame memory notice of storage size"]
        #[inline(always)]
        pub const fn ycc_nstrg_size(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
        }
        #[doc = "YCC data frame memory storage size counter"]
        #[inline(always)]
        pub const fn ycc_dstrg_cont(self) -> crate::common::Reg<u16, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
        }
        #[doc = "YCC data frame memory read counter"]
        #[inline(always)]
        pub const fn ycc_dread_cont(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
        }
        #[doc = "JPEG data frame memory start address"]
        #[inline(always)]
        pub const fn jpg_start_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
        }
        #[doc = "JPEG data frame memory area size"]
        #[inline(always)]
        pub const fn jpg_darea_size(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
        }
        #[doc = "JPEG data frame memory notice of storage size"]
        #[inline(always)]
        pub const fn jpg_nstrg_size(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
        }
        #[doc = "JPEG data frame memory storage size counter"]
        #[inline(always)]
        pub const fn jpg_dstrg_cont(self) -> crate::common::Reg<u16, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
        }
        #[doc = "JPEG data frame memory read counter"]
        #[inline(always)]
        pub const fn jpg_dread_cont(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
        }
        #[doc = "Execution command register"]
        #[inline(always)]
        pub const fn exe_cmd(self) -> crate::common::Reg<regs::ExeCmd, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Active area position setting register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ActPos(pub u32);
        impl ActPos {
            #[must_use]
            #[inline(always)]
            pub const fn act_hst(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_act_hst(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn act_vst(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_act_vst(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
        }
        impl Default for ActPos {
            #[inline(always)]
            fn default() -> ActPos {
                ActPos(0)
            }
        }
        impl core::fmt::Debug for ActPos {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ActPos")
                    .field("act_hst", &self.act_hst())
                    .field("act_vst", &self.act_vst())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ActPos {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "ActPos {{ act_hst: {=u16:?}, act_vst: {=u16:?} }}",
                    self.act_hst(),
                    self.act_vst()
                )
            }
        }
        #[doc = "Active area size setting register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ActSize(pub u32);
        impl ActSize {
            #[must_use]
            #[inline(always)]
            pub const fn act_hsz(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_act_hsz(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn act_vsz(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x01ff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_act_vsz(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
            }
        }
        impl Default for ActSize {
            #[inline(always)]
            fn default() -> ActSize {
                ActSize(0)
            }
        }
        impl core::fmt::Debug for ActSize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ActSize")
                    .field("act_hsz", &self.act_hsz())
                    .field("act_vsz", &self.act_vsz())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ActSize {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "ActSize {{ act_hsz: {=u16:?}, act_vsz: {=u16:?} }}",
                    self.act_hsz(),
                    self.act_vsz()
                )
            }
        }
        #[doc = "CIS input activa area size setting register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CisSize(pub u32);
        impl CisSize {
            #[must_use]
            #[inline(always)]
            pub const fn cis_hst(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_cis_hst(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn cis_vst(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_cis_vst(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
        }
        impl Default for CisSize {
            #[inline(always)]
            fn default() -> CisSize {
                CisSize(0)
            }
        }
        impl core::fmt::Debug for CisSize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CisSize")
                    .field("cis_hst", &self.cis_hst())
                    .field("cis_vst", &self.cis_vst())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CisSize {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "CisSize {{ cis_hst: {=u16:?}, cis_vst: {=u16:?} }}",
                    self.cis_hst(),
                    self.cis_vst()
                )
            }
        }
        #[doc = "Input data enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DinEnable(pub u32);
        impl DinEnable {
            #[must_use]
            #[inline(always)]
            pub const fn ycin_enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycin_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for DinEnable {
            #[inline(always)]
            fn default() -> DinEnable {
                DinEnable(0)
            }
        }
        impl core::fmt::Debug for DinEnable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DinEnable")
                    .field("ycin_enable", &self.ycin_enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DinEnable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DinEnable {{ ycin_enable: {=bool:?} }}",
                    self.ycin_enable()
                )
            }
        }
        #[doc = "Execution command register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ExeCmd(pub u32);
        impl ExeCmd {
            #[must_use]
            #[inline(always)]
            pub const fn exe_cmd(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_exe_cmd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for ExeCmd {
            #[inline(always)]
            fn default() -> ExeCmd {
                ExeCmd(0)
            }
        }
        impl core::fmt::Debug for ExeCmd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ExeCmd")
                    .field("exe_cmd", &self.exe_cmd())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ExeCmd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "ExeCmd {{ exe_cmd: {=bool:?} }}", self.exe_cmd())
            }
        }
        #[doc = "CIS input data format setting register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Format(pub u32);
        impl Format {
            #[must_use]
            #[inline(always)]
            pub const fn yc_order(&self) -> super::vals::YcOrder {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::YcOrder::from_bits(val as u8)
            }
            #[inline(always)]
            pub const fn set_yc_order(&mut self, val: super::vals::YcOrder) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Format {
            #[inline(always)]
            fn default() -> Format {
                Format(0)
            }
        }
        impl core::fmt::Debug for Format {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Format")
                    .field("yc_order", &self.yc_order())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Format {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Format {{ yc_order: {:?} }}", self.yc_order())
            }
        }
        #[doc = "CIS input in line code setting register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ilcode(pub u32);
        impl Ilcode {
            #[must_use]
            #[inline(always)]
            pub const fn sosi(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[inline(always)]
            pub const fn set_sosi(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn eosi(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[inline(always)]
            pub const fn set_eosi(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn soy(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[inline(always)]
            pub const fn set_soy(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn eoy(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[inline(always)]
            pub const fn set_eoy(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Ilcode {
            #[inline(always)]
            fn default() -> Ilcode {
                Ilcode(0)
            }
        }
        impl core::fmt::Debug for Ilcode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ilcode")
                    .field("sosi", &self.sosi())
                    .field("eosi", &self.eosi())
                    .field("soy", &self.soy())
                    .field("eoy", &self.eoy())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ilcode {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ilcode {{ sosi: {=u8:?}, eosi: {=u8:?}, soy: {=u8:?}, eoy: {=u8:?} }}",
                    self.sosi(),
                    self.eosi(),
                    self.soy(),
                    self.eoy()
                )
            }
        }
        #[doc = "Interrupt status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct IntrStat(pub u32);
        impl IntrStat {
            #[must_use]
            #[inline(always)]
            pub const fn vs_int(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_vs_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn eoy_int(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_eoy_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn soy_int(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_soy_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn eoi_int(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_eoi_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn soi_int(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_soi_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_vact_end_int(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_vact_end_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_vact_end_int(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_vact_end_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_axi_trdn_int(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_axi_trdn_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_nstorage_int(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_nstorage_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_darea_end_int(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_darea_end_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_axi_trdn_int(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_axi_trdn_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_nstorage_int(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_nstorage_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_darea_end_int(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_darea_end_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn vlatch_int(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_vlatch_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn size_over_int(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_size_over_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn size_under_int(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_size_under_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_marker_err_int(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_marker_err_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_axi_trerr__int(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_axi_trerr__int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_fifo_ovf_int(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_fifo_ovf_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_mem_ovf_int(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_mem_ovf_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_marker_err_int(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_marker_err_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_axi_trerr_int(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_axi_trerr_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_fifo_ovf_int(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_fifo_ovf_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_mem_ovf_int(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_mem_ovf_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_err_status_int(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_err_status_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
        }
        impl Default for IntrStat {
            #[inline(always)]
            fn default() -> IntrStat {
                IntrStat(0)
            }
        }
        impl core::fmt::Debug for IntrStat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("IntrStat")
                    .field("vs_int", &self.vs_int())
                    .field("eoy_int", &self.eoy_int())
                    .field("soy_int", &self.soy_int())
                    .field("eoi_int", &self.eoi_int())
                    .field("soi_int", &self.soi_int())
                    .field("ycc_vact_end_int", &self.ycc_vact_end_int())
                    .field("jpg_vact_end_int", &self.jpg_vact_end_int())
                    .field("ycc_axi_trdn_int", &self.ycc_axi_trdn_int())
                    .field("ycc_nstorage_int", &self.ycc_nstorage_int())
                    .field("ycc_darea_end_int", &self.ycc_darea_end_int())
                    .field("jpg_axi_trdn_int", &self.jpg_axi_trdn_int())
                    .field("jpg_nstorage_int", &self.jpg_nstorage_int())
                    .field("jpg_darea_end_int", &self.jpg_darea_end_int())
                    .field("vlatch_int", &self.vlatch_int())
                    .field("size_over_int", &self.size_over_int())
                    .field("size_under_int", &self.size_under_int())
                    .field("ycc_marker_err_int", &self.ycc_marker_err_int())
                    .field("ycc_axi_trerr__int", &self.ycc_axi_trerr__int())
                    .field("ycc_fifo_ovf_int", &self.ycc_fifo_ovf_int())
                    .field("ycc_mem_ovf_int", &self.ycc_mem_ovf_int())
                    .field("jpg_marker_err_int", &self.jpg_marker_err_int())
                    .field("jpg_axi_trerr_int", &self.jpg_axi_trerr_int())
                    .field("jpg_fifo_ovf_int", &self.jpg_fifo_ovf_int())
                    .field("jpg_mem_ovf_int", &self.jpg_mem_ovf_int())
                    .field("jpg_err_status_int", &self.jpg_err_status_int())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for IntrStat {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "IntrStat {{ vs_int: {=bool:?}, eoy_int: {=bool:?}, soy_int: {=bool:?}, eoi_int: {=bool:?}, soi_int: {=bool:?}, ycc_vact_end_int: {=bool:?}, jpg_vact_end_int: {=bool:?}, ycc_axi_trdn_int: {=bool:?}, ycc_nstorage_int: {=bool:?}, ycc_darea_end_int: {=bool:?}, jpg_axi_trdn_int: {=bool:?}, jpg_nstorage_int: {=bool:?}, jpg_darea_end_int: {=bool:?}, vlatch_int: {=bool:?}, size_over_int: {=bool:?}, size_under_int: {=bool:?}, ycc_marker_err_int: {=bool:?}, ycc_axi_trerr__int: {=bool:?}, ycc_fifo_ovf_int: {=bool:?}, ycc_mem_ovf_int: {=bool:?}, jpg_marker_err_int: {=bool:?}, jpg_axi_trerr_int: {=bool:?}, jpg_fifo_ovf_int: {=bool:?}, jpg_mem_ovf_int: {=bool:?}, jpg_err_status_int: {=bool:?} }}" , self . vs_int () , self . eoy_int () , self . soy_int () , self . eoi_int () , self . soi_int () , self . ycc_vact_end_int () , self . jpg_vact_end_int () , self . ycc_axi_trdn_int () , self . ycc_nstorage_int () , self . ycc_darea_end_int () , self . jpg_axi_trdn_int () , self . jpg_nstorage_int () , self . jpg_darea_end_int () , self . vlatch_int () , self . size_over_int () , self . size_under_int () , self . ycc_marker_err_int () , self . ycc_axi_trerr__int () , self . ycc_fifo_ovf_int () , self . ycc_mem_ovf_int () , self . jpg_marker_err_int () , self . jpg_axi_trerr_int () , self . jpg_fifo_ovf_int () , self . jpg_mem_ovf_int () , self . jpg_err_status_int ())
            }
        }
        #[doc = "CIS input mode register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[must_use]
            #[inline(always)]
            pub const fn cis_mode(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[inline(always)]
            pub const fn set_cis_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ycc_trns_en(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ycc_trns_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_trns_en(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_trns_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn jpg_cap_mode(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_jpg_cap_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode")
                    .field("cis_mode", &self.cis_mode())
                    .field("ycc_trns_en", &self.ycc_trns_en())
                    .field("jpg_trns_en", &self.jpg_trns_en())
                    .field("jpg_cap_mode", &self.jpg_cap_mode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Mode {{ cis_mode: {=u8:?}, ycc_trns_en: {=bool:?}, jpg_trns_en: {=bool:?}, jpg_cap_mode: {=bool:?} }}" , self . cis_mode () , self . ycc_trns_en () , self . jpg_trns_en () , self . jpg_cap_mode ())
            }
        }
        #[doc = "CIS input Vsync/Hsync polarity setting register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pol(pub u32);
        impl Pol {
            #[must_use]
            #[inline(always)]
            pub const fn hpol(&self) -> super::vals::Hpol {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Hpol::from_bits(val as u8)
            }
            #[inline(always)]
            pub const fn set_hpol(&mut self, val: super::vals::Hpol) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn vpol(&self) -> super::vals::Vpol {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Vpol::from_bits(val as u8)
            }
            #[inline(always)]
            pub const fn set_vpol(&mut self, val: super::vals::Vpol) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Pol {
            #[inline(always)]
            fn default() -> Pol {
                Pol(0)
            }
        }
        impl core::fmt::Debug for Pol {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pol")
                    .field("hpol", &self.hpol())
                    .field("vpol", &self.vpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pol {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Pol {{ hpol: {:?}, vpol: {:?} }}",
                    self.hpol(),
                    self.vpol()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Hpol {
            #[doc = "Hsync low active"]
            LOWACTIVE = 0x0,
            #[doc = "Hsync high active"]
            HIGHACTIVE = 0x01,
        }
        impl Hpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Hpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Hpol {
            #[inline(always)]
            fn from(val: u8) -> Hpol {
                Hpol::from_bits(val)
            }
        }
        impl From<Hpol> for u8 {
            #[inline(always)]
            fn from(val: Hpol) -> u8 {
                Hpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Vpol {
            #[doc = "Vsync low active"]
            LOWACTIVE = 0x0,
            #[doc = "Vsync high active"]
            HIGHACTIVE = 0x01,
        }
        impl Vpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Vpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Vpol {
            #[inline(always)]
            fn from(val: u8) -> Vpol {
                Vpol::from_bits(val)
            }
        }
        impl From<Vpol> for u8 {
            #[inline(always)]
            fn from(val: Vpol) -> u8 {
                Vpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum YcOrder {
            #[doc = "Y/Cb/Y/Cr"]
            YCB_YCR = 0x0,
            #[doc = "Y/Cr/Y/Cb"]
            YCR_YCB = 0x01,
            #[doc = "Cb/Y/Cr/Y"]
            CB_YCR_Y = 0x02,
            #[doc = "Cr/Y/Cb/Y"]
            CR_YCB_Y = 0x03,
        }
        impl YcOrder {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> YcOrder {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for YcOrder {
            #[inline(always)]
            fn from(val: u8) -> YcOrder {
                YcOrder::from_bits(val)
            }
        }
        impl From<YcOrder> for u8 {
            #[inline(always)]
            fn from(val: YcOrder) -> u8 {
                YcOrder::to_bits(val)
            }
        }
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write(&self, f: impl FnOnce(&mut T)) {
            let mut val = Default::default();
            f(&mut val);
            self.write_value(val);
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify(&self, f: impl FnOnce(&mut T)) {
            let mut val = self.read();
            f(&mut val);
            self.write_value(val);
        }
    }
}
pub mod cpu_fifo {
    #[doc = "CPU FIFO Control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuFifo {
        ptr: *mut u8,
    }
    unsafe impl Send for CpuFifo {}
    unsafe impl Sync for CpuFifo {}
    impl CpuFifo {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "TX buffer is full (=1)"]
        #[inline(always)]
        pub const fn fif_push_full(
            self,
        ) -> crate::common::Reg<regs::FifPushFull, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "TX data word 0"]
        #[inline(always)]
        pub const fn fif_push_wrd0(
            self,
        ) -> crate::common::Reg<regs::FifPushWrd0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "TX data word 1"]
        #[inline(always)]
        pub const fn fif_push_wrd1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "TX data write complete"]
        #[inline(always)]
        pub const fn fif_push_cmp(self) -> crate::common::Reg<regs::FifPushCmp, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "RX buffer is empty (=1)"]
        #[inline(always)]
        pub const fn fif_pull_emp(self) -> crate::common::Reg<regs::FifPullEmp, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "RX data word 0"]
        #[inline(always)]
        pub const fn fif_pull_wrd0(
            self,
        ) -> crate::common::Reg<regs::FifPullWrd0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "RX data word 1"]
        #[inline(always)]
        pub const fn fif_pull_wrd1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "RX data read complete"]
        #[inline(always)]
        pub const fn fif_pull_cmp(self) -> crate::common::Reg<regs::FifPullCmp, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "RX data read complete"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FifPullCmp(pub u32);
        impl FifPullCmp {
            #[doc = "RX data read complete"]
            #[must_use]
            #[inline(always)]
            pub const fn pull_cmp(&self) -> super::vals::PullCmp {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::PullCmp::from_bits(val as u8)
            }
            #[doc = "RX data read complete"]
            #[inline(always)]
            pub const fn set_pull_cmp(&mut self, val: super::vals::PullCmp) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for FifPullCmp {
            #[inline(always)]
            fn default() -> FifPullCmp {
                FifPullCmp(0)
            }
        }
        impl core::fmt::Debug for FifPullCmp {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FifPullCmp")
                    .field("pull_cmp", &self.pull_cmp())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FifPullCmp {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FifPullCmp {{ pull_cmp: {:?} }}", self.pull_cmp())
            }
        }
        #[doc = "RX buffer is empty (=1)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FifPullEmp(pub u32);
        impl FifPullEmp {
            #[doc = "RX buffer is empty"]
            #[must_use]
            #[inline(always)]
            pub const fn empty_flag(&self) -> super::vals::EmptyFlag {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::EmptyFlag::from_bits(val as u8)
            }
            #[doc = "RX buffer is empty"]
            #[inline(always)]
            pub const fn set_empty_flag(&mut self, val: super::vals::EmptyFlag) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for FifPullEmp {
            #[inline(always)]
            fn default() -> FifPullEmp {
                FifPullEmp(0)
            }
        }
        impl core::fmt::Debug for FifPullEmp {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FifPullEmp")
                    .field("empty_flag", &self.empty_flag())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FifPullEmp {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FifPullEmp {{ empty_flag: {:?} }}", self.empty_flag())
            }
        }
        #[doc = "RX data word 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FifPullWrd0(pub u32);
        impl FifPullWrd0 {
            #[doc = "RX data word 0"]
            #[must_use]
            #[inline(always)]
            pub const fn data_0(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x0fff_ffff;
                val as u32
            }
            #[doc = "RX data word 0"]
            #[inline(always)]
            pub const fn set_data_0(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
            }
            #[doc = "Sender ID"]
            #[must_use]
            #[inline(always)]
            pub const fn sender_id(&self) -> u8 {
                let val = (self.0 >> 28usize) & 0x0f;
                val as u8
            }
            #[doc = "Sender ID"]
            #[inline(always)]
            pub const fn set_sender_id(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
            }
        }
        impl Default for FifPullWrd0 {
            #[inline(always)]
            fn default() -> FifPullWrd0 {
                FifPullWrd0(0)
            }
        }
        impl core::fmt::Debug for FifPullWrd0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FifPullWrd0")
                    .field("data_0", &self.data_0())
                    .field("sender_id", &self.sender_id())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FifPullWrd0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "FifPullWrd0 {{ data_0: {=u32:?}, sender_id: {=u8:?} }}",
                    self.data_0(),
                    self.sender_id()
                )
            }
        }
        #[doc = "TX data write complete"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FifPushCmp(pub u32);
        impl FifPushCmp {
            #[doc = "TX data write complete"]
            #[must_use]
            #[inline(always)]
            pub const fn push_cmp(&self) -> super::vals::PushCmp {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::PushCmp::from_bits(val as u8)
            }
            #[doc = "TX data write complete"]
            #[inline(always)]
            pub const fn set_push_cmp(&mut self, val: super::vals::PushCmp) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for FifPushCmp {
            #[inline(always)]
            fn default() -> FifPushCmp {
                FifPushCmp(0)
            }
        }
        impl core::fmt::Debug for FifPushCmp {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FifPushCmp")
                    .field("push_cmp", &self.push_cmp())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FifPushCmp {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FifPushCmp {{ push_cmp: {:?} }}", self.push_cmp())
            }
        }
        #[doc = "TX buffer is full (=1)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FifPushFull(pub u32);
        impl FifPushFull {
            #[doc = "TX buffer is full"]
            #[must_use]
            #[inline(always)]
            pub const fn full_flag(&self) -> super::vals::FullFlag {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::FullFlag::from_bits(val as u8)
            }
            #[doc = "TX buffer is full"]
            #[inline(always)]
            pub const fn set_full_flag(&mut self, val: super::vals::FullFlag) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for FifPushFull {
            #[inline(always)]
            fn default() -> FifPushFull {
                FifPushFull(0)
            }
        }
        impl core::fmt::Debug for FifPushFull {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FifPushFull")
                    .field("full_flag", &self.full_flag())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FifPushFull {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FifPushFull {{ full_flag: {:?} }}", self.full_flag())
            }
        }
        #[doc = "TX data word 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FifPushWrd0(pub u32);
        impl FifPushWrd0 {
            #[doc = "TX data word 0"]
            #[must_use]
            #[inline(always)]
            pub const fn data_0(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x0fff_ffff;
                val as u32
            }
            #[doc = "TX data word 0"]
            #[inline(always)]
            pub const fn set_data_0(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
            }
            #[doc = "Target ID"]
            #[must_use]
            #[inline(always)]
            pub const fn receiver_id(&self) -> u8 {
                let val = (self.0 >> 28usize) & 0x0f;
                val as u8
            }
            #[doc = "Target ID"]
            #[inline(always)]
            pub const fn set_receiver_id(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
            }
        }
        impl Default for FifPushWrd0 {
            #[inline(always)]
            fn default() -> FifPushWrd0 {
                FifPushWrd0(0)
            }
        }
        impl core::fmt::Debug for FifPushWrd0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FifPushWrd0")
                    .field("data_0", &self.data_0())
                    .field("receiver_id", &self.receiver_id())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FifPushWrd0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "FifPushWrd0 {{ data_0: {=u32:?}, receiver_id: {=u8:?} }}",
                    self.data_0(),
                    self.receiver_id()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum EmptyFlag {
            NOT_EMPTY = 0x0,
            EMPTY = 0x01,
        }
        impl EmptyFlag {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> EmptyFlag {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for EmptyFlag {
            #[inline(always)]
            fn from(val: u8) -> EmptyFlag {
                EmptyFlag::from_bits(val)
            }
        }
        impl From<EmptyFlag> for u8 {
            #[inline(always)]
            fn from(val: EmptyFlag) -> u8 {
                EmptyFlag::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum FullFlag {
            NOT_FULL = 0x0,
            FULL = 0x01,
        }
        impl FullFlag {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> FullFlag {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for FullFlag {
            #[inline(always)]
            fn from(val: u8) -> FullFlag {
                FullFlag::from_bits(val)
            }
        }
        impl From<FullFlag> for u8 {
            #[inline(always)]
            fn from(val: FullFlag) -> u8 {
                FullFlag::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum PullCmp {
            DONT_CARE = 0x0,
            COMPLETE = 0x01,
        }
        impl PullCmp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> PullCmp {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for PullCmp {
            #[inline(always)]
            fn from(val: u8) -> PullCmp {
                PullCmp::from_bits(val)
            }
        }
        impl From<PullCmp> for u8 {
            #[inline(always)]
            fn from(val: PullCmp) -> u8 {
                PullCmp::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum PushCmp {
            DONT_CARE = 0x0,
            COMPLETE = 0x01,
        }
        impl PushCmp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> PushCmp {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for PushCmp {
            #[inline(always)]
            fn from(val: u8) -> PushCmp {
                PushCmp::from_bits(val)
            }
        }
        impl From<PushCmp> for u8 {
            #[inline(always)]
            fn from(val: PushCmp) -> u8 {
                PushCmp::to_bits(val)
            }
        }
    }
}
pub mod crg {
    #[doc = "Clock / Reset / Gating"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crg {
        ptr: *mut u8,
    }
    unsafe impl Send for Crg {}
    unsafe impl Sync for Crg {}
    impl Crg {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Gear ratio (n/m) for AHB"]
        #[inline(always)]
        pub const fn gear_ahb(self) -> crate::common::Reg<regs::GearAhb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "IMG UART clock setting"]
        #[inline(always)]
        pub const fn gear_img_uart(
            self,
        ) -> crate::common::Reg<regs::GearImgUart, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "IMG SPI clock setting"]
        #[inline(always)]
        pub const fn gear_img_spi(self) -> crate::common::Reg<regs::GearImgSpi, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "SDIO clock setting"]
        #[inline(always)]
        pub const fn gear_per_sdio(
            self,
        ) -> crate::common::Reg<regs::GearPerSdio, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "USB clock setting"]
        #[inline(always)]
        pub const fn gear_per_usb(self) -> crate::common::Reg<regs::GearPerUsb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "VENB_M clock setting"]
        #[inline(always)]
        pub const fn gear_m_img_venb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "VENB_N clock setting"]
        #[inline(always)]
        pub const fn gear_n_img_venb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "IMG WSPI clock setting"]
        #[inline(always)]
        pub const fn gear_img_wspi(
            self,
        ) -> crate::common::Reg<regs::GearImgWspi, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "eMMC clock setting"]
        #[inline(always)]
        pub const fn cken_emmc(self) -> crate::common::Reg<regs::CkenEmmc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Reset control"]
        #[inline(always)]
        pub const fn reset(self) -> crate::common::Reg<regs::Reset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[doc = "CPU/Peripheral clock gating control"]
        #[inline(always)]
        pub const fn ck_gate_ahb(self) -> crate::common::Reg<regs::CkGateAhb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "CPU/Peripheral clock gating control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CkGateAhb(pub u32);
        impl CkGateAhb {
            #[doc = "0=Gated, 1=Ungated"]
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_aud(&self) -> super::vals::CkGateAud {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::CkGateAud::from_bits(val as u8)
            }
            #[doc = "0=Gated, 1=Ungated"]
            #[inline(always)]
            pub const fn set_ck_gate_aud(&mut self, val: super::vals::CkGateAud) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_img(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_img(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_usb(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_usb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_sdio(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_sdio(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_mmc(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_mmc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_dsp0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_dsp0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_dsp1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_dsp1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_dsp2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_dsp2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_dsp3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_dsp3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_dsp4(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_dsp4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_dsp5(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_dsp5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn ck_gate_dmac(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_ck_gate_dmac(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
        }
        impl Default for CkGateAhb {
            #[inline(always)]
            fn default() -> CkGateAhb {
                CkGateAhb(0)
            }
        }
        impl core::fmt::Debug for CkGateAhb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CkGateAhb")
                    .field("ck_gate_aud", &self.ck_gate_aud())
                    .field("ck_gate_img", &self.ck_gate_img())
                    .field("ck_gate_usb", &self.ck_gate_usb())
                    .field("ck_gate_sdio", &self.ck_gate_sdio())
                    .field("ck_gate_mmc", &self.ck_gate_mmc())
                    .field("ck_gate_dsp0", &self.ck_gate_dsp0())
                    .field("ck_gate_dsp1", &self.ck_gate_dsp1())
                    .field("ck_gate_dsp2", &self.ck_gate_dsp2())
                    .field("ck_gate_dsp3", &self.ck_gate_dsp3())
                    .field("ck_gate_dsp4", &self.ck_gate_dsp4())
                    .field("ck_gate_dsp5", &self.ck_gate_dsp5())
                    .field("ck_gate_dmac", &self.ck_gate_dmac())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CkGateAhb {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "CkGateAhb {{ ck_gate_aud: {:?}, ck_gate_img: {=bool:?}, ck_gate_usb: {=bool:?}, ck_gate_sdio: {=bool:?}, ck_gate_mmc: {=bool:?}, ck_gate_dsp0: {=bool:?}, ck_gate_dsp1: {=bool:?}, ck_gate_dsp2: {=bool:?}, ck_gate_dsp3: {=bool:?}, ck_gate_dsp4: {=bool:?}, ck_gate_dsp5: {=bool:?}, ck_gate_dmac: {=bool:?} }}" , self . ck_gate_aud () , self . ck_gate_img () , self . ck_gate_usb () , self . ck_gate_sdio () , self . ck_gate_mmc () , self . ck_gate_dsp0 () , self . ck_gate_dsp1 () , self . ck_gate_dsp2 () , self . ck_gate_dsp3 () , self . ck_gate_dsp4 () , self . ck_gate_dsp5 () , self . ck_gate_dmac ())
            }
        }
        #[doc = "eMMC clock setting"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CkenEmmc(pub u32);
        impl CkenEmmc {
            #[doc = "Enable eMMC clock"]
            #[must_use]
            #[inline(always)]
            pub const fn clkin(&self) -> super::vals::Clkin {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Clkin::from_bits(val as u8)
            }
            #[doc = "Enable eMMC clock"]
            #[inline(always)]
            pub const fn set_clkin(&mut self, val: super::vals::Clkin) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable DRV clock"]
            #[must_use]
            #[inline(always)]
            pub const fn drv(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable DRV clock"]
            #[inline(always)]
            pub const fn set_drv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable eMMC sampling clock"]
            #[must_use]
            #[inline(always)]
            pub const fn smp(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable eMMC sampling clock"]
            #[inline(always)]
            pub const fn set_smp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for CkenEmmc {
            #[inline(always)]
            fn default() -> CkenEmmc {
                CkenEmmc(0)
            }
        }
        impl core::fmt::Debug for CkenEmmc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CkenEmmc")
                    .field("clkin", &self.clkin())
                    .field("drv", &self.drv())
                    .field("smp", &self.smp())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CkenEmmc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "CkenEmmc {{ clkin: {:?}, drv: {=bool:?}, smp: {=bool:?} }}",
                    self.clkin(),
                    self.drv(),
                    self.smp()
                )
            }
        }
        #[doc = "Gear ratio (n/m) for AHB"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GearAhb(pub u32);
        impl GearAhb {
            #[must_use]
            #[inline(always)]
            pub const fn gear_m_ahb(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[inline(always)]
            pub const fn set_gear_m_ahb(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn gear_n_ahb(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x7f;
                val as u8
            }
            #[inline(always)]
            pub const fn set_gear_n_ahb(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
            }
        }
        impl Default for GearAhb {
            #[inline(always)]
            fn default() -> GearAhb {
                GearAhb(0)
            }
        }
        impl core::fmt::Debug for GearAhb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GearAhb")
                    .field("gear_m_ahb", &self.gear_m_ahb())
                    .field("gear_n_ahb", &self.gear_n_ahb())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GearAhb {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "GearAhb {{ gear_m_ahb: {=u8:?}, gear_n_ahb: {=u8:?} }}",
                    self.gear_m_ahb(),
                    self.gear_n_ahb()
                )
            }
        }
        #[doc = "IMG SPI clock setting"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GearImgSpi(pub u32);
        impl GearImgSpi {
            #[must_use]
            #[inline(always)]
            pub const fn gear_m_spi(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[inline(always)]
            pub const fn set_gear_m_spi(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn gear_n_spi(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_gear_n_spi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for GearImgSpi {
            #[inline(always)]
            fn default() -> GearImgSpi {
                GearImgSpi(0)
            }
        }
        impl core::fmt::Debug for GearImgSpi {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GearImgSpi")
                    .field("gear_m_spi", &self.gear_m_spi())
                    .field("gear_n_spi", &self.gear_n_spi())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GearImgSpi {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "GearImgSpi {{ gear_m_spi: {=u8:?}, gear_n_spi: {=bool:?} }}",
                    self.gear_m_spi(),
                    self.gear_n_spi()
                )
            }
        }
        #[doc = "IMG UART clock setting"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GearImgUart(pub u32);
        impl GearImgUart {
            #[must_use]
            #[inline(always)]
            pub const fn gear_m_uart(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[inline(always)]
            pub const fn set_gear_m_uart(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn gear_n_uart(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_gear_n_uart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for GearImgUart {
            #[inline(always)]
            fn default() -> GearImgUart {
                GearImgUart(0)
            }
        }
        impl core::fmt::Debug for GearImgUart {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GearImgUart")
                    .field("gear_m_uart", &self.gear_m_uart())
                    .field("gear_n_uart", &self.gear_n_uart())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GearImgUart {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "GearImgUart {{ gear_m_uart: {=u8:?}, gear_n_uart: {=bool:?} }}",
                    self.gear_m_uart(),
                    self.gear_n_uart()
                )
            }
        }
        #[doc = "IMG WSPI clock setting"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GearImgWspi(pub u32);
        impl GearImgWspi {
            #[must_use]
            #[inline(always)]
            pub const fn gear_m_img_wspi(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[inline(always)]
            pub const fn set_gear_m_img_wspi(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn gear_n_img_wspi(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_gear_n_img_wspi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for GearImgWspi {
            #[inline(always)]
            fn default() -> GearImgWspi {
                GearImgWspi(0)
            }
        }
        impl core::fmt::Debug for GearImgWspi {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GearImgWspi")
                    .field("gear_m_img_wspi", &self.gear_m_img_wspi())
                    .field("gear_n_img_wspi", &self.gear_n_img_wspi())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GearImgWspi {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "GearImgWspi {{ gear_m_img_wspi: {=u8:?}, gear_n_img_wspi: {=bool:?} }}",
                    self.gear_m_img_wspi(),
                    self.gear_n_img_wspi()
                )
            }
        }
        #[doc = "SDIO clock setting"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GearPerSdio(pub u32);
        impl GearPerSdio {
            #[must_use]
            #[inline(always)]
            pub const fn gear_m_sdio(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[inline(always)]
            pub const fn set_gear_m_sdio(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn gear_n_sdio(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_gear_n_sdio(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for GearPerSdio {
            #[inline(always)]
            fn default() -> GearPerSdio {
                GearPerSdio(0)
            }
        }
        impl core::fmt::Debug for GearPerSdio {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GearPerSdio")
                    .field("gear_m_sdio", &self.gear_m_sdio())
                    .field("gear_n_sdio", &self.gear_n_sdio())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GearPerSdio {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "GearPerSdio {{ gear_m_sdio: {=u8:?}, gear_n_sdio: {=bool:?} }}",
                    self.gear_m_sdio(),
                    self.gear_n_sdio()
                )
            }
        }
        #[doc = "USB clock setting"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GearPerUsb(pub u32);
        impl GearPerUsb {
            #[must_use]
            #[inline(always)]
            pub const fn gear_m_usb(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_gear_m_usb(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn gear_n_usb(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_gear_n_usb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for GearPerUsb {
            #[inline(always)]
            fn default() -> GearPerUsb {
                GearPerUsb(0)
            }
        }
        impl core::fmt::Debug for GearPerUsb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GearPerUsb")
                    .field("gear_m_usb", &self.gear_m_usb())
                    .field("gear_n_usb", &self.gear_n_usb())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GearPerUsb {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "GearPerUsb {{ gear_m_usb: {=u16:?}, gear_n_usb: {=bool:?} }}",
                    self.gear_m_usb(),
                    self.gear_n_usb()
                )
            }
        }
        #[doc = "Reset control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reset(pub u32);
        impl Reset {
            #[doc = "0=reset, 1=active"]
            #[must_use]
            #[inline(always)]
            pub const fn xrs_aud(&self) -> super::vals::XrsAud {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::XrsAud::from_bits(val as u8)
            }
            #[doc = "0=reset, 1=active"]
            #[inline(always)]
            pub const fn set_xrs_aud(&mut self, val: super::vals::XrsAud) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_img(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_img(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_usb(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_usb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_sdio(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_sdio(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_mmc(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_mmc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_mmc_crg(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_mmc_crg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_dsp0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_dsp0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_dsp1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_dsp1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_dsp2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_dsp2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_dsp3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_dsp3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_dsp4(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_dsp4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_dsp5(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_dsp5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn xrs_dsp_gen(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_xrs_dsp_gen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Reset {
            #[inline(always)]
            fn default() -> Reset {
                Reset(0)
            }
        }
        impl core::fmt::Debug for Reset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Reset")
                    .field("xrs_aud", &self.xrs_aud())
                    .field("xrs_img", &self.xrs_img())
                    .field("xrs_usb", &self.xrs_usb())
                    .field("xrs_sdio", &self.xrs_sdio())
                    .field("xrs_mmc", &self.xrs_mmc())
                    .field("xrs_mmc_crg", &self.xrs_mmc_crg())
                    .field("xrs_dsp0", &self.xrs_dsp0())
                    .field("xrs_dsp1", &self.xrs_dsp1())
                    .field("xrs_dsp2", &self.xrs_dsp2())
                    .field("xrs_dsp3", &self.xrs_dsp3())
                    .field("xrs_dsp4", &self.xrs_dsp4())
                    .field("xrs_dsp5", &self.xrs_dsp5())
                    .field("xrs_dsp_gen", &self.xrs_dsp_gen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Reset {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Reset {{ xrs_aud: {:?}, xrs_img: {=bool:?}, xrs_usb: {=bool:?}, xrs_sdio: {=bool:?}, xrs_mmc: {=bool:?}, xrs_mmc_crg: {=bool:?}, xrs_dsp0: {=bool:?}, xrs_dsp1: {=bool:?}, xrs_dsp2: {=bool:?}, xrs_dsp3: {=bool:?}, xrs_dsp4: {=bool:?}, xrs_dsp5: {=bool:?}, xrs_dsp_gen: {=bool:?} }}" , self . xrs_aud () , self . xrs_img () , self . xrs_usb () , self . xrs_sdio () , self . xrs_mmc () , self . xrs_mmc_crg () , self . xrs_dsp0 () , self . xrs_dsp1 () , self . xrs_dsp2 () , self . xrs_dsp3 () , self . xrs_dsp4 () , self . xrs_dsp5 () , self . xrs_dsp_gen ())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum CkGateAud {
            #[doc = "Gated"]
            GATED = 0x0,
            #[doc = "Ungated"]
            UNGATED = 0x01,
        }
        impl CkGateAud {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> CkGateAud {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for CkGateAud {
            #[inline(always)]
            fn from(val: u8) -> CkGateAud {
                CkGateAud::from_bits(val)
            }
        }
        impl From<CkGateAud> for u8 {
            #[inline(always)]
            fn from(val: CkGateAud) -> u8 {
                CkGateAud::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Clkin {
            #[doc = "Disable"]
            DISABLE = 0x0,
            #[doc = "Enable"]
            ENABLE = 0x01,
        }
        impl Clkin {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Clkin {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Clkin {
            #[inline(always)]
            fn from(val: u8) -> Clkin {
                Clkin::from_bits(val)
            }
        }
        impl From<Clkin> for u8 {
            #[inline(always)]
            fn from(val: Clkin) -> u8 {
                Clkin::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum XrsAud {
            #[doc = "Reset"]
            RESET = 0x0,
            #[doc = "Active"]
            ACTIVE = 0x01,
        }
        impl XrsAud {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> XrsAud {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for XrsAud {
            #[inline(always)]
            fn from(val: u8) -> XrsAud {
                XrsAud::from_bits(val)
            }
        }
        impl From<XrsAud> for u8 {
            #[inline(always)]
            fn from(val: XrsAud) -> u8 {
                XrsAud::to_bits(val)
            }
        }
    }
}
pub mod dmac1 {
    #[doc = "DMA controller (ADMAC)"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmac1 {
        ptr: *mut u8,
    }
    unsafe impl Send for Dmac1 {}
    unsafe impl Sync for Dmac1 {}
    impl Dmac1 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[inline(always)]
        pub const fn dmacint_status(
            self,
        ) -> crate::common::Reg<regs::DmacintStatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacint_tcstatus(
            self,
        ) -> crate::common::Reg<regs::DmacintTcstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacint_tcclear(
            self,
        ) -> crate::common::Reg<regs::DmacintTcclear, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacint_error_status(
            self,
        ) -> crate::common::Reg<regs::DmacintErrorStatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[inline(always)]
        pub const fn dmacint_err_clr(
            self,
        ) -> crate::common::Reg<regs::DmacintErrClr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacraw_int_tcstatus(
            self,
        ) -> crate::common::Reg<regs::DmacrawIntTcstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacraw_int_error_status(
            self,
        ) -> crate::common::Reg<regs::DmacrawIntErrorStatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacenbld_chns(
            self,
        ) -> crate::common::Reg<regs::DmacenbldChns, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsoft_breq(
            self,
        ) -> crate::common::Reg<regs::DmacsoftBreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsoft_sreq(
            self,
        ) -> crate::common::Reg<regs::DmacsoftSreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsoft_lbreq(
            self,
        ) -> crate::common::Reg<regs::DmacsoftLbreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsoft_lsreq(
            self,
        ) -> crate::common::Reg<regs::DmacsoftLsreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
        }
        #[inline(always)]
        pub const fn dmacconfiguration(
            self,
        ) -> crate::common::Reg<regs::Dmacconfiguration, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsync(self) -> crate::common::Reg<regs::Dmacsync, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0src_addr(
            self,
        ) -> crate::common::Reg<regs::Dmacc0srcAddr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0dest_addr(
            self,
        ) -> crate::common::Reg<regs::Dmacc0destAddr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0lli(self) -> crate::common::Reg<regs::Dmacc0lli, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0control(
            self,
        ) -> crate::common::Reg<regs::Dmacc0control, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0configuration(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
        }
    }
    pub mod regs {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0control(pub u32);
        impl Dmacc0control {
            #[doc = "Transfer size"]
            #[must_use]
            #[inline(always)]
            pub const fn transfer_size(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Transfer size"]
            #[inline(always)]
            pub const fn set_transfer_size(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[doc = "Source burst size"]
            #[must_use]
            #[inline(always)]
            pub const fn sbsize(&self) -> u8 {
                let val = (self.0 >> 12usize) & 0x07;
                val as u8
            }
            #[doc = "Source burst size"]
            #[inline(always)]
            pub const fn set_sbsize(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
            }
            #[doc = "Destination burst size"]
            #[must_use]
            #[inline(always)]
            pub const fn dbsize(&self) -> super::vals::Dbsize {
                let val = (self.0 >> 15usize) & 0x07;
                super::vals::Dbsize::from_bits(val as u8)
            }
            #[doc = "Destination burst size"]
            #[inline(always)]
            pub const fn set_dbsize(&mut self, val: super::vals::Dbsize) {
                self.0 =
                    (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
            }
            #[doc = "Source transfer width"]
            #[must_use]
            #[inline(always)]
            pub const fn swidth(&self) -> u8 {
                let val = (self.0 >> 18usize) & 0x07;
                val as u8
            }
            #[doc = "Source transfer width"]
            #[inline(always)]
            pub const fn set_swidth(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
            }
            #[doc = "Destination transfer width"]
            #[must_use]
            #[inline(always)]
            pub const fn dwidth(&self) -> super::vals::Dwidth {
                let val = (self.0 >> 21usize) & 0x07;
                super::vals::Dwidth::from_bits(val as u8)
            }
            #[doc = "Destination transfer width"]
            #[inline(always)]
            pub const fn set_dwidth(&mut self, val: super::vals::Dwidth) {
                self.0 =
                    (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
            }
            #[doc = "Source AHB master select"]
            #[must_use]
            #[inline(always)]
            pub const fn s(&self) -> super::vals::S {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::S::from_bits(val as u8)
            }
            #[doc = "Source AHB master select"]
            #[inline(always)]
            pub const fn set_s(&mut self, val: super::vals::S) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
            #[doc = "Destination AHB master select"]
            #[must_use]
            #[inline(always)]
            pub const fn d(&self) -> super::vals::D {
                let val = (self.0 >> 25usize) & 0x01;
                super::vals::D::from_bits(val as u8)
            }
            #[doc = "Destination AHB master select"]
            #[inline(always)]
            pub const fn set_d(&mut self, val: super::vals::D) {
                self.0 =
                    (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
            }
            #[doc = "Source increment"]
            #[must_use]
            #[inline(always)]
            pub const fn si(&self) -> super::vals::Si {
                let val = (self.0 >> 26usize) & 0x01;
                super::vals::Si::from_bits(val as u8)
            }
            #[doc = "Source increment"]
            #[inline(always)]
            pub const fn set_si(&mut self, val: super::vals::Si) {
                self.0 =
                    (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
            }
            #[doc = "Destination increment"]
            #[must_use]
            #[inline(always)]
            pub const fn di(&self) -> super::vals::Di {
                let val = (self.0 >> 27usize) & 0x01;
                super::vals::Di::from_bits(val as u8)
            }
            #[doc = "Destination increment"]
            #[inline(always)]
            pub const fn set_di(&mut self, val: super::vals::Di) {
                self.0 =
                    (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
            }
            #[doc = "Protection"]
            #[must_use]
            #[inline(always)]
            pub const fn prot(&self) -> u8 {
                let val = (self.0 >> 28usize) & 0x07;
                val as u8
            }
            #[doc = "Protection"]
            #[inline(always)]
            pub const fn set_prot(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
            }
            #[doc = "Terminal count interrupt enable"]
            #[must_use]
            #[inline(always)]
            pub const fn i(&self) -> super::vals::I {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::I::from_bits(val as u8)
            }
            #[doc = "Terminal count interrupt enable"]
            #[inline(always)]
            pub const fn set_i(&mut self, val: super::vals::I) {
                self.0 =
                    (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Dmacc0control {
            #[inline(always)]
            fn default() -> Dmacc0control {
                Dmacc0control(0)
            }
        }
        impl core::fmt::Debug for Dmacc0control {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0control")
                    .field("transfer_size", &self.transfer_size())
                    .field("sbsize", &self.sbsize())
                    .field("dbsize", &self.dbsize())
                    .field("swidth", &self.swidth())
                    .field("dwidth", &self.dwidth())
                    .field("s", &self.s())
                    .field("d", &self.d())
                    .field("si", &self.si())
                    .field("di", &self.di())
                    .field("prot", &self.prot())
                    .field("i", &self.i())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0control {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Dmacc0control {{ transfer_size: {=u16:?}, sbsize: {=u8:?}, dbsize: {:?}, swidth: {=u8:?}, dwidth: {:?}, s: {:?}, d: {:?}, si: {:?}, di: {:?}, prot: {=u8:?}, i: {:?} }}" , self . transfer_size () , self . sbsize () , self . dbsize () , self . swidth () , self . dwidth () , self . s () , self . d () , self . si () , self . di () , self . prot () , self . i ())
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0destAddr(pub u32);
        impl Dmacc0destAddr {
            #[doc = "DMA destination address"]
            #[must_use]
            #[inline(always)]
            pub const fn dest_addr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "DMA destination address"]
            #[inline(always)]
            pub const fn set_dest_addr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Dmacc0destAddr {
            #[inline(always)]
            fn default() -> Dmacc0destAddr {
                Dmacc0destAddr(0)
            }
        }
        impl core::fmt::Debug for Dmacc0destAddr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0destAddr")
                    .field("dest_addr", &self.dest_addr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0destAddr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacc0destAddr {{ dest_addr: {=u32:?} }}",
                    self.dest_addr()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0lli(pub u32);
        impl Dmacc0lli {
            #[doc = "AHB master select for loading the next LLI"]
            #[must_use]
            #[inline(always)]
            pub const fn lm(&self) -> super::vals::Lm {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Lm::from_bits(val as u8)
            }
            #[doc = "AHB master select for loading the next LLI"]
            #[inline(always)]
            pub const fn set_lm(&mut self, val: super::vals::Lm) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Linked list item"]
            #[must_use]
            #[inline(always)]
            pub const fn lli(&self) -> u32 {
                let val = (self.0 >> 2usize) & 0x3fff_ffff;
                val as u32
            }
            #[doc = "Linked list item"]
            #[inline(always)]
            pub const fn set_lli(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
            }
        }
        impl Default for Dmacc0lli {
            #[inline(always)]
            fn default() -> Dmacc0lli {
                Dmacc0lli(0)
            }
        }
        impl core::fmt::Debug for Dmacc0lli {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0lli")
                    .field("lm", &self.lm())
                    .field("lli", &self.lli())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0lli {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacc0lli {{ lm: {:?}, lli: {=u32:?} }}",
                    self.lm(),
                    self.lli()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0srcAddr(pub u32);
        impl Dmacc0srcAddr {
            #[doc = "DMA source address"]
            #[must_use]
            #[inline(always)]
            pub const fn src_addr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "DMA source address"]
            #[inline(always)]
            pub const fn set_src_addr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Dmacc0srcAddr {
            #[inline(always)]
            fn default() -> Dmacc0srcAddr {
                Dmacc0srcAddr(0)
            }
        }
        impl core::fmt::Debug for Dmacc0srcAddr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0srcAddr")
                    .field("src_addr", &self.src_addr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0srcAddr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dmacc0srcAddr {{ src_addr: {=u32:?} }}", self.src_addr())
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacconfiguration(pub u32);
        impl Dmacconfiguration {
            #[doc = "DMAC enable"]
            #[must_use]
            #[inline(always)]
            pub const fn e(&self) -> super::vals::E {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::E::from_bits(val as u8)
            }
            #[doc = "DMAC enable"]
            #[inline(always)]
            pub const fn set_e(&mut self, val: super::vals::E) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "AHB Master 1 endianess configuration"]
            #[must_use]
            #[inline(always)]
            pub const fn m1(&self) -> super::vals::M1 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::M1::from_bits(val as u8)
            }
            #[doc = "AHB Master 1 endianess configuration"]
            #[inline(always)]
            pub const fn set_m1(&mut self, val: super::vals::M1) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "AHB Master 2 endianess configuration"]
            #[must_use]
            #[inline(always)]
            pub const fn m2(&self) -> super::vals::M2 {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::M2::from_bits(val as u8)
            }
            #[doc = "AHB Master 2 endianess configuration"]
            #[inline(always)]
            pub const fn set_m2(&mut self, val: super::vals::M2) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Dmacconfiguration {
            #[inline(always)]
            fn default() -> Dmacconfiguration {
                Dmacconfiguration(0)
            }
        }
        impl core::fmt::Debug for Dmacconfiguration {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacconfiguration")
                    .field("e", &self.e())
                    .field("m1", &self.m1())
                    .field("m2", &self.m2())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacconfiguration {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacconfiguration {{ e: {:?}, m1: {:?}, m2: {:?} }}",
                    self.e(),
                    self.m1(),
                    self.m2()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacenbldChns(pub u32);
        impl DmacenbldChns {
            #[doc = "Channel enable status"]
            #[must_use]
            #[inline(always)]
            pub const fn enabled_channels(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Channel enable status"]
            #[inline(always)]
            pub const fn set_enabled_channels(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacenbldChns {
            #[inline(always)]
            fn default() -> DmacenbldChns {
                DmacenbldChns(0)
            }
        }
        impl core::fmt::Debug for DmacenbldChns {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacenbldChns")
                    .field("enabled_channels", &self.enabled_channels())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacenbldChns {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacenbldChns {{ enabled_channels: {=u8:?} }}",
                    self.enabled_channels()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintErrClr(pub u32);
        impl DmacintErrClr {
            #[doc = "Interrupt error clear"]
            #[must_use]
            #[inline(always)]
            pub const fn int_err_clr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Interrupt error clear"]
            #[inline(always)]
            pub const fn set_int_err_clr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintErrClr {
            #[inline(always)]
            fn default() -> DmacintErrClr {
                DmacintErrClr(0)
            }
        }
        impl core::fmt::Debug for DmacintErrClr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintErrClr")
                    .field("int_err_clr", &self.int_err_clr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintErrClr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintErrClr {{ int_err_clr: {=u8:?} }}",
                    self.int_err_clr()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintErrorStatus(pub u32);
        impl DmacintErrorStatus {
            #[doc = "Interrupt error status"]
            #[must_use]
            #[inline(always)]
            pub const fn int_error_status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Interrupt error status"]
            #[inline(always)]
            pub const fn set_int_error_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintErrorStatus {
            #[inline(always)]
            fn default() -> DmacintErrorStatus {
                DmacintErrorStatus(0)
            }
        }
        impl core::fmt::Debug for DmacintErrorStatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintErrorStatus")
                    .field("int_error_status", &self.int_error_status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintErrorStatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintErrorStatus {{ int_error_status: {=u8:?} }}",
                    self.int_error_status()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintStatus(pub u32);
        impl DmacintStatus {
            #[doc = "Status of the DMA interrupts after masking"]
            #[must_use]
            #[inline(always)]
            pub const fn int_status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Status of the DMA interrupts after masking"]
            #[inline(always)]
            pub const fn set_int_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintStatus {
            #[inline(always)]
            fn default() -> DmacintStatus {
                DmacintStatus(0)
            }
        }
        impl core::fmt::Debug for DmacintStatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintStatus")
                    .field("int_status", &self.int_status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintStatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintStatus {{ int_status: {=u8:?} }}",
                    self.int_status()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintTcclear(pub u32);
        impl DmacintTcclear {
            #[doc = "Terminal count request clear"]
            #[must_use]
            #[inline(always)]
            pub const fn int_tcclear(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Terminal count request clear"]
            #[inline(always)]
            pub const fn set_int_tcclear(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintTcclear {
            #[inline(always)]
            fn default() -> DmacintTcclear {
                DmacintTcclear(0)
            }
        }
        impl core::fmt::Debug for DmacintTcclear {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintTcclear")
                    .field("int_tcclear", &self.int_tcclear())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintTcclear {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintTcclear {{ int_tcclear: {=u8:?} }}",
                    self.int_tcclear()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintTcstatus(pub u32);
        impl DmacintTcstatus {
            #[doc = "Interrupt terminal count request status"]
            #[must_use]
            #[inline(always)]
            pub const fn int_tcstatus(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Interrupt terminal count request status"]
            #[inline(always)]
            pub const fn set_int_tcstatus(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintTcstatus {
            #[inline(always)]
            fn default() -> DmacintTcstatus {
                DmacintTcstatus(0)
            }
        }
        impl core::fmt::Debug for DmacintTcstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintTcstatus")
                    .field("int_tcstatus", &self.int_tcstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintTcstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintTcstatus {{ int_tcstatus: {=u8:?} }}",
                    self.int_tcstatus()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacrawIntErrorStatus(pub u32);
        impl DmacrawIntErrorStatus {
            #[doc = "Status of the error interrupt prior to masking"]
            #[must_use]
            #[inline(always)]
            pub const fn raw_int_error_status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Status of the error interrupt prior to masking"]
            #[inline(always)]
            pub const fn set_raw_int_error_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacrawIntErrorStatus {
            #[inline(always)]
            fn default() -> DmacrawIntErrorStatus {
                DmacrawIntErrorStatus(0)
            }
        }
        impl core::fmt::Debug for DmacrawIntErrorStatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacrawIntErrorStatus")
                    .field("raw_int_error_status", &self.raw_int_error_status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacrawIntErrorStatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacrawIntErrorStatus {{ raw_int_error_status: {=u8:?} }}",
                    self.raw_int_error_status()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacrawIntTcstatus(pub u32);
        impl DmacrawIntTcstatus {
            #[doc = "Status of the terminal count interrupt prior to masking"]
            #[must_use]
            #[inline(always)]
            pub const fn raw_int_tcstatus(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Status of the terminal count interrupt prior to masking"]
            #[inline(always)]
            pub const fn set_raw_int_tcstatus(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacrawIntTcstatus {
            #[inline(always)]
            fn default() -> DmacrawIntTcstatus {
                DmacrawIntTcstatus(0)
            }
        }
        impl core::fmt::Debug for DmacrawIntTcstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacrawIntTcstatus")
                    .field("raw_int_tcstatus", &self.raw_int_tcstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacrawIntTcstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacrawIntTcstatus {{ raw_int_tcstatus: {=u8:?} }}",
                    self.raw_int_tcstatus()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacsoftBreq(pub u32);
        impl DmacsoftBreq {
            #[doc = "Software burst request"]
            #[must_use]
            #[inline(always)]
            pub const fn soft_breq(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Software burst request"]
            #[inline(always)]
            pub const fn set_soft_breq(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for DmacsoftBreq {
            #[inline(always)]
            fn default() -> DmacsoftBreq {
                DmacsoftBreq(0)
            }
        }
        impl core::fmt::Debug for DmacsoftBreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacsoftBreq")
                    .field("soft_breq", &self.soft_breq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacsoftBreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacsoftBreq {{ soft_breq: {=u16:?} }}",
                    self.soft_breq()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacsoftLbreq(pub u32);
        impl DmacsoftLbreq {
            #[doc = "Software last burst request"]
            #[must_use]
            #[inline(always)]
            pub const fn soft_lbreq(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Software last burst request"]
            #[inline(always)]
            pub const fn set_soft_lbreq(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for DmacsoftLbreq {
            #[inline(always)]
            fn default() -> DmacsoftLbreq {
                DmacsoftLbreq(0)
            }
        }
        impl core::fmt::Debug for DmacsoftLbreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacsoftLbreq")
                    .field("soft_lbreq", &self.soft_lbreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacsoftLbreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacsoftLbreq {{ soft_lbreq: {=u16:?} }}",
                    self.soft_lbreq()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacsoftLsreq(pub u32);
        impl DmacsoftLsreq {
            #[doc = "Software last single request"]
            #[must_use]
            #[inline(always)]
            pub const fn soft_lsreq(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Software last single request"]
            #[inline(always)]
            pub const fn set_soft_lsreq(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for DmacsoftLsreq {
            #[inline(always)]
            fn default() -> DmacsoftLsreq {
                DmacsoftLsreq(0)
            }
        }
        impl core::fmt::Debug for DmacsoftLsreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacsoftLsreq")
                    .field("soft_lsreq", &self.soft_lsreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacsoftLsreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacsoftLsreq {{ soft_lsreq: {=u16:?} }}",
                    self.soft_lsreq()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacsoftSreq(pub u32);
        impl DmacsoftSreq {
            #[doc = "Software single request"]
            #[must_use]
            #[inline(always)]
            pub const fn soft_sreq(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Software single request"]
            #[inline(always)]
            pub const fn set_soft_sreq(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for DmacsoftSreq {
            #[inline(always)]
            fn default() -> DmacsoftSreq {
                DmacsoftSreq(0)
            }
        }
        impl core::fmt::Debug for DmacsoftSreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacsoftSreq")
                    .field("soft_sreq", &self.soft_sreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacsoftSreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacsoftSreq {{ soft_sreq: {=u16:?} }}",
                    self.soft_sreq()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacsync(pub u32);
        impl Dmacsync {
            #[doc = "DMA synchronization logic for DMA request signals enabled or disabled"]
            #[must_use]
            #[inline(always)]
            pub const fn dmacsync(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "DMA synchronization logic for DMA request signals enabled or disabled"]
            #[inline(always)]
            pub const fn set_dmacsync(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dmacsync {
            #[inline(always)]
            fn default() -> Dmacsync {
                Dmacsync(0)
            }
        }
        impl core::fmt::Debug for Dmacsync {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacsync")
                    .field("dmacsync", &self.dmacsync())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacsync {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dmacsync {{ dmacsync: {=u16:?} }}", self.dmacsync())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum D {
            #[doc = "AHB Master 1"]
            AHB1 = 0x0,
            #[doc = "AHB Master 2"]
            AHB2 = 0x01,
        }
        impl D {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> D {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for D {
            #[inline(always)]
            fn from(val: u8) -> D {
                D::from_bits(val)
            }
        }
        impl From<D> for u8 {
            #[inline(always)]
            fn from(val: D) -> u8 {
                D::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dbsize {
            #[doc = "1"]
            _1BYTE = 0x0,
            #[doc = "4"]
            _4BYTES = 0x01,
            #[doc = "8"]
            _8BYTES = 0x02,
            #[doc = "16"]
            _16BYTES = 0x03,
            #[doc = "32"]
            _32BYTES = 0x04,
            #[doc = "64"]
            _64BYTES = 0x05,
            #[doc = "128"]
            _128BYTES = 0x06,
            #[doc = "256"]
            _256BYTES = 0x07,
        }
        impl Dbsize {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dbsize {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dbsize {
            #[inline(always)]
            fn from(val: u8) -> Dbsize {
                Dbsize::from_bits(val)
            }
        }
        impl From<Dbsize> for u8 {
            #[inline(always)]
            fn from(val: Dbsize) -> u8 {
                Dbsize::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Di {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Di {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Di {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Di {
            #[inline(always)]
            fn from(val: u8) -> Di {
                Di::from_bits(val)
            }
        }
        impl From<Di> for u8 {
            #[inline(always)]
            fn from(val: Di) -> u8 {
                Di::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dwidth {
            #[doc = "Byte, 8-bit"]
            BYTE = 0x0,
            #[doc = "Halfword, 16-bit"]
            HALFWORD = 0x01,
            #[doc = "Word, 32-bit"]
            WORD = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Dwidth {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dwidth {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dwidth {
            #[inline(always)]
            fn from(val: u8) -> Dwidth {
                Dwidth::from_bits(val)
            }
        }
        impl From<Dwidth> for u8 {
            #[inline(always)]
            fn from(val: Dwidth) -> u8 {
                Dwidth::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum E {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl E {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> E {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for E {
            #[inline(always)]
            fn from(val: u8) -> E {
                E::from_bits(val)
            }
        }
        impl From<E> for u8 {
            #[inline(always)]
            fn from(val: E) -> u8 {
                E::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum I {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl I {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> I {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for I {
            #[inline(always)]
            fn from(val: u8) -> I {
                I::from_bits(val)
            }
        }
        impl From<I> for u8 {
            #[inline(always)]
            fn from(val: I) -> u8 {
                I::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lm {
            #[doc = "AHB Master 1"]
            AHB1 = 0x0,
            #[doc = "AHB Master 2"]
            AHB2 = 0x01,
        }
        impl Lm {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lm {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lm {
            #[inline(always)]
            fn from(val: u8) -> Lm {
                Lm::from_bits(val)
            }
        }
        impl From<Lm> for u8 {
            #[inline(always)]
            fn from(val: Lm) -> u8 {
                Lm::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum M1 {
            #[doc = "Little-endian mode"]
            LE = 0x0,
            #[doc = "Big-endian mode"]
            BE = 0x01,
        }
        impl M1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> M1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for M1 {
            #[inline(always)]
            fn from(val: u8) -> M1 {
                M1::from_bits(val)
            }
        }
        impl From<M1> for u8 {
            #[inline(always)]
            fn from(val: M1) -> u8 {
                M1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum M2 {
            #[doc = "Little-endian mode"]
            LE = 0x0,
            #[doc = "Big-endian mode"]
            BE = 0x01,
        }
        impl M2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> M2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for M2 {
            #[inline(always)]
            fn from(val: u8) -> M2 {
                M2::from_bits(val)
            }
        }
        impl From<M2> for u8 {
            #[inline(always)]
            fn from(val: M2) -> u8 {
                M2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum S {
            #[doc = "AHB Master 1"]
            AHB1 = 0x0,
            #[doc = "AHB Master 2"]
            AHB2 = 0x01,
        }
        impl S {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> S {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for S {
            #[inline(always)]
            fn from(val: u8) -> S {
                S::from_bits(val)
            }
        }
        impl From<S> for u8 {
            #[inline(always)]
            fn from(val: S) -> u8 {
                S::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Si {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Si {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Si {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Si {
            #[inline(always)]
            fn from(val: u8) -> Si {
                Si::from_bits(val)
            }
        }
        impl From<Si> for u8 {
            #[inline(always)]
            fn from(val: Si) -> u8 {
                Si::to_bits(val)
            }
        }
    }
}
pub mod dmac3 {
    #[doc = "DMA controller (IDMAC)"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmac3 {
        ptr: *mut u8,
    }
    unsafe impl Send for Dmac3 {}
    unsafe impl Sync for Dmac3 {}
    impl Dmac3 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[inline(always)]
        pub const fn dmacint_status(
            self,
        ) -> crate::common::Reg<regs::DmacintStatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacint_tcstatus(
            self,
        ) -> crate::common::Reg<regs::DmacintTcstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacint_tcclear(
            self,
        ) -> crate::common::Reg<regs::DmacintTcclear, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacint_error_status(
            self,
        ) -> crate::common::Reg<regs::DmacintErrorStatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[inline(always)]
        pub const fn dmacint_err_clr(
            self,
        ) -> crate::common::Reg<regs::DmacintErrClr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacraw_int_tcstatus(
            self,
        ) -> crate::common::Reg<regs::DmacrawIntTcstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacraw_int_error_status(
            self,
        ) -> crate::common::Reg<regs::DmacrawIntErrorStatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacenbld_chns(
            self,
        ) -> crate::common::Reg<regs::DmacenbldChns, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsoft_breq(
            self,
        ) -> crate::common::Reg<regs::DmacsoftBreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsoft_sreq(
            self,
        ) -> crate::common::Reg<regs::DmacsoftSreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsoft_lbreq(
            self,
        ) -> crate::common::Reg<regs::DmacsoftLbreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsoft_lsreq(
            self,
        ) -> crate::common::Reg<regs::DmacsoftLsreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
        }
        #[inline(always)]
        pub const fn dmacconfiguration(
            self,
        ) -> crate::common::Reg<regs::Dmacconfiguration, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsync(self) -> crate::common::Reg<regs::Dmacsync, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacsreqmask(
            self,
        ) -> crate::common::Reg<regs::Dmacsreqmask, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0src_addr(
            self,
        ) -> crate::common::Reg<regs::Dmacc0srcAddr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0dest_addr(
            self,
        ) -> crate::common::Reg<regs::Dmacc0destAddr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0lli(self) -> crate::common::Reg<regs::Dmacc0lli, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0control(
            self,
        ) -> crate::common::Reg<regs::Dmacc0control, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0configuration(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
        }
        #[inline(always)]
        pub const fn dmacc0def_lli(
            self,
        ) -> crate::common::Reg<regs::Dmacc0defLli, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
        }
    }
    pub mod regs {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0control(pub u32);
        impl Dmacc0control {
            #[doc = "Transfer size"]
            #[must_use]
            #[inline(always)]
            pub const fn transfer_size(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x0007_ffff;
                val as u32
            }
            #[doc = "Transfer size"]
            #[inline(always)]
            pub const fn set_transfer_size(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
            }
            #[doc = "Source burst size"]
            #[must_use]
            #[inline(always)]
            pub const fn sbsize(&self) -> u8 {
                let val = (self.0 >> 19usize) & 0x03;
                val as u8
            }
            #[doc = "Source burst size"]
            #[inline(always)]
            pub const fn set_sbsize(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
            }
            #[doc = "Destination burst size"]
            #[must_use]
            #[inline(always)]
            pub const fn dbsize(&self) -> super::vals::Dbsize {
                let val = (self.0 >> 21usize) & 0x03;
                super::vals::Dbsize::from_bits(val as u8)
            }
            #[doc = "Destination burst size"]
            #[inline(always)]
            pub const fn set_dbsize(&mut self, val: super::vals::Dbsize) {
                self.0 =
                    (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
            }
            #[doc = "Source transfer width"]
            #[must_use]
            #[inline(always)]
            pub const fn swidth(&self) -> u8 {
                let val = (self.0 >> 23usize) & 0x03;
                val as u8
            }
            #[doc = "Source transfer width"]
            #[inline(always)]
            pub const fn set_swidth(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
            }
            #[doc = "Destination transfer width"]
            #[must_use]
            #[inline(always)]
            pub const fn dwidth(&self) -> super::vals::Dwidth {
                let val = (self.0 >> 25usize) & 0x03;
                super::vals::Dwidth::from_bits(val as u8)
            }
            #[doc = "Destination transfer width"]
            #[inline(always)]
            pub const fn set_dwidth(&mut self, val: super::vals::Dwidth) {
                self.0 =
                    (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
            }
            #[doc = "Source AHB master select"]
            #[must_use]
            #[inline(always)]
            pub const fn s(&self) -> super::vals::S {
                let val = (self.0 >> 27usize) & 0x01;
                super::vals::S::from_bits(val as u8)
            }
            #[doc = "Source AHB master select"]
            #[inline(always)]
            pub const fn set_s(&mut self, val: super::vals::S) {
                self.0 =
                    (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
            }
            #[doc = "Destination AHB master select"]
            #[must_use]
            #[inline(always)]
            pub const fn d(&self) -> super::vals::D {
                let val = (self.0 >> 28usize) & 0x01;
                super::vals::D::from_bits(val as u8)
            }
            #[doc = "Destination AHB master select"]
            #[inline(always)]
            pub const fn set_d(&mut self, val: super::vals::D) {
                self.0 =
                    (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
            }
            #[doc = "Source increment"]
            #[must_use]
            #[inline(always)]
            pub const fn si(&self) -> super::vals::Si {
                let val = (self.0 >> 29usize) & 0x01;
                super::vals::Si::from_bits(val as u8)
            }
            #[doc = "Source increment"]
            #[inline(always)]
            pub const fn set_si(&mut self, val: super::vals::Si) {
                self.0 =
                    (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
            }
            #[doc = "Destination increment"]
            #[must_use]
            #[inline(always)]
            pub const fn di(&self) -> super::vals::Di {
                let val = (self.0 >> 30usize) & 0x01;
                super::vals::Di::from_bits(val as u8)
            }
            #[doc = "Destination increment"]
            #[inline(always)]
            pub const fn set_di(&mut self, val: super::vals::Di) {
                self.0 =
                    (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
            }
            #[doc = "Terminal count interrupt enable"]
            #[must_use]
            #[inline(always)]
            pub const fn i(&self) -> super::vals::I {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::I::from_bits(val as u8)
            }
            #[doc = "Terminal count interrupt enable"]
            #[inline(always)]
            pub const fn set_i(&mut self, val: super::vals::I) {
                self.0 =
                    (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Dmacc0control {
            #[inline(always)]
            fn default() -> Dmacc0control {
                Dmacc0control(0)
            }
        }
        impl core::fmt::Debug for Dmacc0control {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0control")
                    .field("transfer_size", &self.transfer_size())
                    .field("sbsize", &self.sbsize())
                    .field("dbsize", &self.dbsize())
                    .field("swidth", &self.swidth())
                    .field("dwidth", &self.dwidth())
                    .field("s", &self.s())
                    .field("d", &self.d())
                    .field("si", &self.si())
                    .field("di", &self.di())
                    .field("i", &self.i())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0control {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Dmacc0control {{ transfer_size: {=u32:?}, sbsize: {=u8:?}, dbsize: {:?}, swidth: {=u8:?}, dwidth: {:?}, s: {:?}, d: {:?}, si: {:?}, di: {:?}, i: {:?} }}" , self . transfer_size () , self . sbsize () , self . dbsize () , self . swidth () , self . dwidth () , self . s () , self . d () , self . si () , self . di () , self . i ())
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0defLli(pub u32);
        impl Dmacc0defLli {
            #[doc = "Bus master select"]
            #[must_use]
            #[inline(always)]
            pub const fn deflm(&self) -> super::vals::Deflm {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Deflm::from_bits(val as u8)
            }
            #[doc = "Bus master select"]
            #[inline(always)]
            pub const fn set_deflm(&mut self, val: super::vals::Deflm) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable Default LLI"]
            #[must_use]
            #[inline(always)]
            pub const fn defle(&self) -> super::vals::Defle {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Defle::from_bits(val as u8)
            }
            #[doc = "Enable Default LLI"]
            #[inline(always)]
            pub const fn set_defle(&mut self, val: super::vals::Defle) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn deflli(&self) -> u32 {
                let val = (self.0 >> 2usize) & 0x3fff_ffff;
                val as u32
            }
            #[inline(always)]
            pub const fn set_deflli(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
            }
        }
        impl Default for Dmacc0defLli {
            #[inline(always)]
            fn default() -> Dmacc0defLli {
                Dmacc0defLli(0)
            }
        }
        impl core::fmt::Debug for Dmacc0defLli {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0defLli")
                    .field("deflm", &self.deflm())
                    .field("defle", &self.defle())
                    .field("deflli", &self.deflli())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0defLli {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacc0defLli {{ deflm: {:?}, defle: {:?}, deflli: {=u32:?} }}",
                    self.deflm(),
                    self.defle(),
                    self.deflli()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0destAddr(pub u32);
        impl Dmacc0destAddr {
            #[doc = "DMA destination address"]
            #[must_use]
            #[inline(always)]
            pub const fn dest_addr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "DMA destination address"]
            #[inline(always)]
            pub const fn set_dest_addr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Dmacc0destAddr {
            #[inline(always)]
            fn default() -> Dmacc0destAddr {
                Dmacc0destAddr(0)
            }
        }
        impl core::fmt::Debug for Dmacc0destAddr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0destAddr")
                    .field("dest_addr", &self.dest_addr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0destAddr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacc0destAddr {{ dest_addr: {=u32:?} }}",
                    self.dest_addr()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0lli(pub u32);
        impl Dmacc0lli {
            #[doc = "AHB master select for loading the next LLI"]
            #[must_use]
            #[inline(always)]
            pub const fn lm(&self) -> super::vals::Lm {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Lm::from_bits(val as u8)
            }
            #[doc = "AHB master select for loading the next LLI"]
            #[inline(always)]
            pub const fn set_lm(&mut self, val: super::vals::Lm) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Linked list item"]
            #[must_use]
            #[inline(always)]
            pub const fn lli(&self) -> u32 {
                let val = (self.0 >> 2usize) & 0x3fff_ffff;
                val as u32
            }
            #[doc = "Linked list item"]
            #[inline(always)]
            pub const fn set_lli(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
            }
        }
        impl Default for Dmacc0lli {
            #[inline(always)]
            fn default() -> Dmacc0lli {
                Dmacc0lli(0)
            }
        }
        impl core::fmt::Debug for Dmacc0lli {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0lli")
                    .field("lm", &self.lm())
                    .field("lli", &self.lli())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0lli {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacc0lli {{ lm: {:?}, lli: {=u32:?} }}",
                    self.lm(),
                    self.lli()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacc0srcAddr(pub u32);
        impl Dmacc0srcAddr {
            #[doc = "DMA source address"]
            #[must_use]
            #[inline(always)]
            pub const fn src_addr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "DMA source address"]
            #[inline(always)]
            pub const fn set_src_addr(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Dmacc0srcAddr {
            #[inline(always)]
            fn default() -> Dmacc0srcAddr {
                Dmacc0srcAddr(0)
            }
        }
        impl core::fmt::Debug for Dmacc0srcAddr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacc0srcAddr")
                    .field("src_addr", &self.src_addr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacc0srcAddr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dmacc0srcAddr {{ src_addr: {=u32:?} }}", self.src_addr())
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacconfiguration(pub u32);
        impl Dmacconfiguration {
            #[doc = "DMAC enable"]
            #[must_use]
            #[inline(always)]
            pub const fn e(&self) -> super::vals::E {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::E::from_bits(val as u8)
            }
            #[doc = "DMAC enable"]
            #[inline(always)]
            pub const fn set_e(&mut self, val: super::vals::E) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "AHB Master 1 endianess configuration"]
            #[must_use]
            #[inline(always)]
            pub const fn m1(&self) -> super::vals::M1 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::M1::from_bits(val as u8)
            }
            #[doc = "AHB Master 1 endianess configuration"]
            #[inline(always)]
            pub const fn set_m1(&mut self, val: super::vals::M1) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "AHB Master 2 endianess configuration"]
            #[must_use]
            #[inline(always)]
            pub const fn m2(&self) -> super::vals::M2 {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::M2::from_bits(val as u8)
            }
            #[doc = "AHB Master 2 endianess configuration"]
            #[inline(always)]
            pub const fn set_m2(&mut self, val: super::vals::M2) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Transfer Size Extended"]
            #[must_use]
            #[inline(always)]
            pub const fn ts(&self) -> super::vals::Ts {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Ts::from_bits(val as u8)
            }
            #[doc = "Transfer Size Extended"]
            #[inline(always)]
            pub const fn set_ts(&mut self, val: super::vals::Ts) {
                self.0 =
                    (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "Default LLI enabled"]
            #[must_use]
            #[inline(always)]
            pub const fn dlli(&self) -> super::vals::Dlli {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Dlli::from_bits(val as u8)
            }
            #[doc = "Default LLI enabled"]
            #[inline(always)]
            pub const fn set_dlli(&mut self, val: super::vals::Dlli) {
                self.0 =
                    (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
            #[doc = "Trigger function enabled"]
            #[must_use]
            #[inline(always)]
            pub const fn tr(&self) -> super::vals::Tr {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Tr::from_bits(val as u8)
            }
            #[doc = "Trigger function enabled"]
            #[inline(always)]
            pub const fn set_tr(&mut self, val: super::vals::Tr) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
            #[doc = "DMAC Arbitration logic"]
            #[must_use]
            #[inline(always)]
            pub const fn arb(&self) -> super::vals::Arb {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Arb::from_bits(val as u8)
            }
            #[doc = "DMAC Arbitration logic"]
            #[inline(always)]
            pub const fn set_arb(&mut self, val: super::vals::Arb) {
                self.0 =
                    (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
            }
            #[doc = "Transfer Size Extended"]
            #[must_use]
            #[inline(always)]
            pub const fn fsize(&self) -> super::vals::Fsize {
                let val = (self.0 >> 14usize) & 0x03;
                super::vals::Fsize::from_bits(val as u8)
            }
            #[doc = "Transfer Size Extended"]
            #[inline(always)]
            pub const fn set_fsize(&mut self, val: super::vals::Fsize) {
                self.0 =
                    (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
            }
        }
        impl Default for Dmacconfiguration {
            #[inline(always)]
            fn default() -> Dmacconfiguration {
                Dmacconfiguration(0)
            }
        }
        impl core::fmt::Debug for Dmacconfiguration {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacconfiguration")
                    .field("e", &self.e())
                    .field("m1", &self.m1())
                    .field("m2", &self.m2())
                    .field("ts", &self.ts())
                    .field("dlli", &self.dlli())
                    .field("tr", &self.tr())
                    .field("arb", &self.arb())
                    .field("fsize", &self.fsize())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacconfiguration {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Dmacconfiguration {{ e: {:?}, m1: {:?}, m2: {:?}, ts: {:?}, dlli: {:?}, tr: {:?}, arb: {:?}, fsize: {:?} }}" , self . e () , self . m1 () , self . m2 () , self . ts () , self . dlli () , self . tr () , self . arb () , self . fsize ())
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacenbldChns(pub u32);
        impl DmacenbldChns {
            #[doc = "Channel enable status"]
            #[must_use]
            #[inline(always)]
            pub const fn enabled_channels(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Channel enable status"]
            #[inline(always)]
            pub const fn set_enabled_channels(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacenbldChns {
            #[inline(always)]
            fn default() -> DmacenbldChns {
                DmacenbldChns(0)
            }
        }
        impl core::fmt::Debug for DmacenbldChns {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacenbldChns")
                    .field("enabled_channels", &self.enabled_channels())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacenbldChns {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacenbldChns {{ enabled_channels: {=u8:?} }}",
                    self.enabled_channels()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintErrClr(pub u32);
        impl DmacintErrClr {
            #[doc = "Interrupt error clear"]
            #[must_use]
            #[inline(always)]
            pub const fn int_err_clr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Interrupt error clear"]
            #[inline(always)]
            pub const fn set_int_err_clr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintErrClr {
            #[inline(always)]
            fn default() -> DmacintErrClr {
                DmacintErrClr(0)
            }
        }
        impl core::fmt::Debug for DmacintErrClr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintErrClr")
                    .field("int_err_clr", &self.int_err_clr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintErrClr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintErrClr {{ int_err_clr: {=u8:?} }}",
                    self.int_err_clr()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintErrorStatus(pub u32);
        impl DmacintErrorStatus {
            #[doc = "Interrupt error status"]
            #[must_use]
            #[inline(always)]
            pub const fn int_error_status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Interrupt error status"]
            #[inline(always)]
            pub const fn set_int_error_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintErrorStatus {
            #[inline(always)]
            fn default() -> DmacintErrorStatus {
                DmacintErrorStatus(0)
            }
        }
        impl core::fmt::Debug for DmacintErrorStatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintErrorStatus")
                    .field("int_error_status", &self.int_error_status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintErrorStatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintErrorStatus {{ int_error_status: {=u8:?} }}",
                    self.int_error_status()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintStatus(pub u32);
        impl DmacintStatus {
            #[doc = "Status of the DMA interrupts after masking"]
            #[must_use]
            #[inline(always)]
            pub const fn int_status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Status of the DMA interrupts after masking"]
            #[inline(always)]
            pub const fn set_int_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintStatus {
            #[inline(always)]
            fn default() -> DmacintStatus {
                DmacintStatus(0)
            }
        }
        impl core::fmt::Debug for DmacintStatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintStatus")
                    .field("int_status", &self.int_status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintStatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintStatus {{ int_status: {=u8:?} }}",
                    self.int_status()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintTcclear(pub u32);
        impl DmacintTcclear {
            #[doc = "Terminal count request clear"]
            #[must_use]
            #[inline(always)]
            pub const fn int_tcclear(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Terminal count request clear"]
            #[inline(always)]
            pub const fn set_int_tcclear(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintTcclear {
            #[inline(always)]
            fn default() -> DmacintTcclear {
                DmacintTcclear(0)
            }
        }
        impl core::fmt::Debug for DmacintTcclear {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintTcclear")
                    .field("int_tcclear", &self.int_tcclear())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintTcclear {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintTcclear {{ int_tcclear: {=u8:?} }}",
                    self.int_tcclear()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacintTcstatus(pub u32);
        impl DmacintTcstatus {
            #[doc = "Interrupt terminal count request status"]
            #[must_use]
            #[inline(always)]
            pub const fn int_tcstatus(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Interrupt terminal count request status"]
            #[inline(always)]
            pub const fn set_int_tcstatus(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacintTcstatus {
            #[inline(always)]
            fn default() -> DmacintTcstatus {
                DmacintTcstatus(0)
            }
        }
        impl core::fmt::Debug for DmacintTcstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacintTcstatus")
                    .field("int_tcstatus", &self.int_tcstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacintTcstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacintTcstatus {{ int_tcstatus: {=u8:?} }}",
                    self.int_tcstatus()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacrawIntErrorStatus(pub u32);
        impl DmacrawIntErrorStatus {
            #[doc = "Status of the error interrupt prior to masking"]
            #[must_use]
            #[inline(always)]
            pub const fn raw_int_error_status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Status of the error interrupt prior to masking"]
            #[inline(always)]
            pub const fn set_raw_int_error_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacrawIntErrorStatus {
            #[inline(always)]
            fn default() -> DmacrawIntErrorStatus {
                DmacrawIntErrorStatus(0)
            }
        }
        impl core::fmt::Debug for DmacrawIntErrorStatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacrawIntErrorStatus")
                    .field("raw_int_error_status", &self.raw_int_error_status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacrawIntErrorStatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacrawIntErrorStatus {{ raw_int_error_status: {=u8:?} }}",
                    self.raw_int_error_status()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacrawIntTcstatus(pub u32);
        impl DmacrawIntTcstatus {
            #[doc = "Status of the terminal count interrupt prior to masking"]
            #[must_use]
            #[inline(always)]
            pub const fn raw_int_tcstatus(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Status of the terminal count interrupt prior to masking"]
            #[inline(always)]
            pub const fn set_raw_int_tcstatus(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for DmacrawIntTcstatus {
            #[inline(always)]
            fn default() -> DmacrawIntTcstatus {
                DmacrawIntTcstatus(0)
            }
        }
        impl core::fmt::Debug for DmacrawIntTcstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacrawIntTcstatus")
                    .field("raw_int_tcstatus", &self.raw_int_tcstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacrawIntTcstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacrawIntTcstatus {{ raw_int_tcstatus: {=u8:?} }}",
                    self.raw_int_tcstatus()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacsoftBreq(pub u32);
        impl DmacsoftBreq {
            #[doc = "Software burst request"]
            #[must_use]
            #[inline(always)]
            pub const fn soft_breq(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Software burst request"]
            #[inline(always)]
            pub const fn set_soft_breq(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for DmacsoftBreq {
            #[inline(always)]
            fn default() -> DmacsoftBreq {
                DmacsoftBreq(0)
            }
        }
        impl core::fmt::Debug for DmacsoftBreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacsoftBreq")
                    .field("soft_breq", &self.soft_breq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacsoftBreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacsoftBreq {{ soft_breq: {=u16:?} }}",
                    self.soft_breq()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacsoftLbreq(pub u32);
        impl DmacsoftLbreq {
            #[doc = "Software last burst request"]
            #[must_use]
            #[inline(always)]
            pub const fn soft_lbreq(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Software last burst request"]
            #[inline(always)]
            pub const fn set_soft_lbreq(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for DmacsoftLbreq {
            #[inline(always)]
            fn default() -> DmacsoftLbreq {
                DmacsoftLbreq(0)
            }
        }
        impl core::fmt::Debug for DmacsoftLbreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacsoftLbreq")
                    .field("soft_lbreq", &self.soft_lbreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacsoftLbreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacsoftLbreq {{ soft_lbreq: {=u16:?} }}",
                    self.soft_lbreq()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacsoftLsreq(pub u32);
        impl DmacsoftLsreq {
            #[doc = "Software last single request"]
            #[must_use]
            #[inline(always)]
            pub const fn soft_lsreq(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Software last single request"]
            #[inline(always)]
            pub const fn set_soft_lsreq(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for DmacsoftLsreq {
            #[inline(always)]
            fn default() -> DmacsoftLsreq {
                DmacsoftLsreq(0)
            }
        }
        impl core::fmt::Debug for DmacsoftLsreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacsoftLsreq")
                    .field("soft_lsreq", &self.soft_lsreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacsoftLsreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacsoftLsreq {{ soft_lsreq: {=u16:?} }}",
                    self.soft_lsreq()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacsoftSreq(pub u32);
        impl DmacsoftSreq {
            #[doc = "Software single request"]
            #[must_use]
            #[inline(always)]
            pub const fn soft_sreq(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Software single request"]
            #[inline(always)]
            pub const fn set_soft_sreq(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for DmacsoftSreq {
            #[inline(always)]
            fn default() -> DmacsoftSreq {
                DmacsoftSreq(0)
            }
        }
        impl core::fmt::Debug for DmacsoftSreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DmacsoftSreq")
                    .field("soft_sreq", &self.soft_sreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DmacsoftSreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DmacsoftSreq {{ soft_sreq: {=u16:?} }}",
                    self.soft_sreq()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacsreqmask(pub u32);
        impl Dmacsreqmask {
            #[doc = "Mask SREQ signals between DMAC and peripherals"]
            #[must_use]
            #[inline(always)]
            pub const fn dmacsreqmask(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Mask SREQ signals between DMAC and peripherals"]
            #[inline(always)]
            pub const fn set_dmacsreqmask(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dmacsreqmask {
            #[inline(always)]
            fn default() -> Dmacsreqmask {
                Dmacsreqmask(0)
            }
        }
        impl core::fmt::Debug for Dmacsreqmask {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacsreqmask")
                    .field("dmacsreqmask", &self.dmacsreqmask())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacsreqmask {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacsreqmask {{ dmacsreqmask: {=u16:?} }}",
                    self.dmacsreqmask()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacsync(pub u32);
        impl Dmacsync {
            #[doc = "DMA synchronization logic for DMA request signals enabled or disabled"]
            #[must_use]
            #[inline(always)]
            pub const fn dmacsync(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "DMA synchronization logic for DMA request signals enabled or disabled"]
            #[inline(always)]
            pub const fn set_dmacsync(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dmacsync {
            #[inline(always)]
            fn default() -> Dmacsync {
                Dmacsync(0)
            }
        }
        impl core::fmt::Debug for Dmacsync {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacsync")
                    .field("dmacsync", &self.dmacsync())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacsync {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dmacsync {{ dmacsync: {=u16:?} }}", self.dmacsync())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Arb {
            #[doc = "Static"]
            STATIC = 0x0,
            #[doc = "Round Robin"]
            ROUNDROBIN = 0x01,
        }
        impl Arb {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Arb {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Arb {
            #[inline(always)]
            fn from(val: u8) -> Arb {
                Arb::from_bits(val)
            }
        }
        impl From<Arb> for u8 {
            #[inline(always)]
            fn from(val: Arb) -> u8 {
                Arb::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum D {
            #[doc = "AHB Master 1"]
            AHB1 = 0x0,
            #[doc = "AHB Master 2"]
            AHB2 = 0x01,
        }
        impl D {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> D {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for D {
            #[inline(always)]
            fn from(val: u8) -> D {
                D::from_bits(val)
            }
        }
        impl From<D> for u8 {
            #[inline(always)]
            fn from(val: D) -> u8 {
                D::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dbsize {
            #[doc = "1"]
            _1BYTE = 0x0,
            #[doc = "4"]
            _4BYTES = 0x01,
            #[doc = "8"]
            _8BYTES = 0x02,
            #[doc = "16"]
            _16BYTES = 0x03,
        }
        impl Dbsize {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dbsize {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dbsize {
            #[inline(always)]
            fn from(val: u8) -> Dbsize {
                Dbsize::from_bits(val)
            }
        }
        impl From<Dbsize> for u8 {
            #[inline(always)]
            fn from(val: Dbsize) -> u8 {
                Dbsize::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Defle {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Defle {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Defle {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Defle {
            #[inline(always)]
            fn from(val: u8) -> Defle {
                Defle::from_bits(val)
            }
        }
        impl From<Defle> for u8 {
            #[inline(always)]
            fn from(val: Defle) -> u8 {
                Defle::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Deflm {
            #[doc = "AHB Master 1"]
            AHB1 = 0x0,
            #[doc = "AHB Master 2"]
            AHB2 = 0x01,
        }
        impl Deflm {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Deflm {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Deflm {
            #[inline(always)]
            fn from(val: u8) -> Deflm {
                Deflm::from_bits(val)
            }
        }
        impl From<Deflm> for u8 {
            #[inline(always)]
            fn from(val: Deflm) -> u8 {
                Deflm::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Di {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Di {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Di {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Di {
            #[inline(always)]
            fn from(val: u8) -> Di {
                Di::from_bits(val)
            }
        }
        impl From<Di> for u8 {
            #[inline(always)]
            fn from(val: Di) -> u8 {
                Di::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dlli {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Dlli {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dlli {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dlli {
            #[inline(always)]
            fn from(val: u8) -> Dlli {
                Dlli::from_bits(val)
            }
        }
        impl From<Dlli> for u8 {
            #[inline(always)]
            fn from(val: Dlli) -> u8 {
                Dlli::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dwidth {
            #[doc = "Byte, 8-bit"]
            BYTE = 0x0,
            #[doc = "Halfword, 16-bit"]
            HALFWORD = 0x01,
            #[doc = "Word, 32-bit"]
            WORD = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Dwidth {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dwidth {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dwidth {
            #[inline(always)]
            fn from(val: u8) -> Dwidth {
                Dwidth::from_bits(val)
            }
        }
        impl From<Dwidth> for u8 {
            #[inline(always)]
            fn from(val: Dwidth) -> u8 {
                Dwidth::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum E {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl E {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> E {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for E {
            #[inline(always)]
            fn from(val: u8) -> E {
                E::from_bits(val)
            }
        }
        impl From<E> for u8 {
            #[inline(always)]
            fn from(val: E) -> u8 {
                E::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Fsize {
            #[doc = "16 bytes"]
            _16 = 0x0,
            #[doc = "32 bytes"]
            _32 = 0x01,
            #[doc = "64 bytes"]
            _64 = 0x02,
            #[doc = "128 bytes"]
            _128 = 0x03,
        }
        impl Fsize {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Fsize {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Fsize {
            #[inline(always)]
            fn from(val: u8) -> Fsize {
                Fsize::from_bits(val)
            }
        }
        impl From<Fsize> for u8 {
            #[inline(always)]
            fn from(val: Fsize) -> u8 {
                Fsize::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum I {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl I {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> I {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for I {
            #[inline(always)]
            fn from(val: u8) -> I {
                I::from_bits(val)
            }
        }
        impl From<I> for u8 {
            #[inline(always)]
            fn from(val: I) -> u8 {
                I::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lm {
            #[doc = "AHB Master 1"]
            AHB1 = 0x0,
            #[doc = "AHB Master 2"]
            AHB2 = 0x01,
        }
        impl Lm {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lm {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lm {
            #[inline(always)]
            fn from(val: u8) -> Lm {
                Lm::from_bits(val)
            }
        }
        impl From<Lm> for u8 {
            #[inline(always)]
            fn from(val: Lm) -> u8 {
                Lm::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum M1 {
            #[doc = "Little-endian mode"]
            LE = 0x0,
            #[doc = "Big-endian mode"]
            BE = 0x01,
        }
        impl M1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> M1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for M1 {
            #[inline(always)]
            fn from(val: u8) -> M1 {
                M1::from_bits(val)
            }
        }
        impl From<M1> for u8 {
            #[inline(always)]
            fn from(val: M1) -> u8 {
                M1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum M2 {
            #[doc = "Little-endian mode"]
            LE = 0x0,
            #[doc = "Big-endian mode"]
            BE = 0x01,
        }
        impl M2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> M2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for M2 {
            #[inline(always)]
            fn from(val: u8) -> M2 {
                M2::from_bits(val)
            }
        }
        impl From<M2> for u8 {
            #[inline(always)]
            fn from(val: M2) -> u8 {
                M2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum S {
            #[doc = "AHB Master 1"]
            AHB1 = 0x0,
            #[doc = "AHB Master 2"]
            AHB2 = 0x01,
        }
        impl S {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> S {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for S {
            #[inline(always)]
            fn from(val: u8) -> S {
                S::from_bits(val)
            }
        }
        impl From<S> for u8 {
            #[inline(always)]
            fn from(val: S) -> u8 {
                S::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Si {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Si {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Si {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Si {
            #[inline(always)]
            fn from(val: u8) -> Si {
                Si::from_bits(val)
            }
        }
        impl From<Si> for u8 {
            #[inline(always)]
            fn from(val: Si) -> u8 {
                Si::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tr {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Tr {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tr {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tr {
            #[inline(always)]
            fn from(val: u8) -> Tr {
                Tr::from_bits(val)
            }
        }
        impl From<Tr> for u8 {
            #[inline(always)]
            fn from(val: Tr) -> u8 {
                Tr::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ts {
            #[doc = "Not extended"]
            NOEXTENDED = 0x0,
            #[doc = "Extended"]
            EXTENDED = 0x01,
        }
        impl Ts {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ts {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ts {
            #[inline(always)]
            fn from(val: u8) -> Ts {
                Ts::from_bits(val)
            }
        }
        impl From<Ts> for u8 {
            #[inline(always)]
            fn from(val: Ts) -> u8 {
                Ts::to_bits(val)
            }
        }
    }
}
pub mod ge2d {
    #[doc = "Hardware 2D Graphics Engine"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ge2d {
        ptr: *mut u8,
    }
    unsafe impl Send for Ge2d {}
    unsafe impl Sync for Ge2d {}
    impl Ge2d {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "2D Graphics Engine Interrupt Control"]
        #[inline(always)]
        pub const fn intr_enable(self) -> crate::common::Reg<regs::IntrEnable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Descriptor Address Set Register"]
        #[inline(always)]
        pub const fn address_descriptor_start(
            self,
        ) -> crate::common::Reg<regs::AddressDescriptorStart, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Command Register"]
        #[inline(always)]
        pub const fn cmd_descriptor(
            self,
        ) -> crate::common::Reg<regs::CmdDescriptor, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Status Register"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Normal Descriptor Address Register"]
        #[inline(always)]
        pub const fn stat_normal_descriptor_address(
            self,
        ) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Current Descriptor Address Register"]
        #[inline(always)]
        pub const fn stat_current_descriptor_address(
            self,
        ) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Descriptor Address Set Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AddressDescriptorStart(pub u32);
        impl AddressDescriptorStart {
            #[doc = "must be 1"]
            #[must_use]
            #[inline(always)]
            pub const fn msel(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "must be 1"]
            #[inline(always)]
            pub const fn set_msel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Descriptor Address"]
            #[must_use]
            #[inline(always)]
            pub const fn da(&self) -> u32 {
                let val = (self.0 >> 4usize) & 0x0fff_ffff;
                val as u32
            }
            #[doc = "Descriptor Address"]
            #[inline(always)]
            pub const fn set_da(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
            }
        }
        impl Default for AddressDescriptorStart {
            #[inline(always)]
            fn default() -> AddressDescriptorStart {
                AddressDescriptorStart(0)
            }
        }
        impl core::fmt::Debug for AddressDescriptorStart {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AddressDescriptorStart")
                    .field("msel", &self.msel())
                    .field("da", &self.da())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AddressDescriptorStart {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "AddressDescriptorStart {{ msel: {=bool:?}, da: {=u32:?} }}",
                    self.msel(),
                    self.da()
                )
            }
        }
        #[doc = "Command Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CmdDescriptor(pub u32);
        impl CmdDescriptor {
            #[doc = "Command"]
            #[must_use]
            #[inline(always)]
            pub const fn command(&self) -> super::vals::Command {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Command::from_bits(val as u8)
            }
            #[doc = "Command"]
            #[inline(always)]
            pub const fn set_command(&mut self, val: super::vals::Command) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for CmdDescriptor {
            #[inline(always)]
            fn default() -> CmdDescriptor {
                CmdDescriptor(0)
            }
        }
        impl core::fmt::Debug for CmdDescriptor {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CmdDescriptor")
                    .field("command", &self.command())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CmdDescriptor {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "CmdDescriptor {{ command: {:?} }}", self.command())
            }
        }
        #[doc = "2D Graphics Engine Interrupt Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct IntrEnable(pub u32);
        impl IntrEnable {
            #[doc = "Normal Descriptor Command Update Interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn hpu(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Normal Descriptor Command Update Interrupt"]
            #[inline(always)]
            pub const fn set_hpu(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Normal Descriptor Command Finished Interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn ndf(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Normal Descriptor Command Finished Interrupt"]
            #[inline(always)]
            pub const fn set_ndf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Normal Descriptor Command Branch Interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn ndb(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Normal Descriptor Command Branch Interrupt"]
            #[inline(always)]
            pub const fn set_ndb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Normal Descriptor Command Error Interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn nde(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Normal Descriptor Command Error Interrupt"]
            #[inline(always)]
            pub const fn set_nde(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Descriptor STOP Command Done Interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn dsd(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Descriptor STOP Command Done Interrupt"]
            #[inline(always)]
            pub const fn set_dsd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Read Error Interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn rd_err(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Read Error Interrupt"]
            #[inline(always)]
            pub const fn set_rd_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Write Error Interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn wr_err(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write Error Interrupt"]
            #[inline(always)]
            pub const fn set_wr_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for IntrEnable {
            #[inline(always)]
            fn default() -> IntrEnable {
                IntrEnable(0)
            }
        }
        impl core::fmt::Debug for IntrEnable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("IntrEnable")
                    .field("hpu", &self.hpu())
                    .field("ndf", &self.ndf())
                    .field("ndb", &self.ndb())
                    .field("nde", &self.nde())
                    .field("dsd", &self.dsd())
                    .field("rd_err", &self.rd_err())
                    .field("wr_err", &self.wr_err())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for IntrEnable {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "IntrEnable {{ hpu: {=bool:?}, ndf: {=bool:?}, ndb: {=bool:?}, nde: {=bool:?}, dsd: {=bool:?}, rd_err: {=bool:?}, wr_err: {=bool:?} }}" , self . hpu () , self . ndf () , self . ndb () , self . nde () , self . dsd () , self . rd_err () , self . wr_err ())
            }
        }
        #[doc = "Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "Requesting Normal Descriptor"]
            #[must_use]
            #[inline(always)]
            pub const fn nreq(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Requesting Normal Descriptor"]
            #[inline(always)]
            pub const fn set_nreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Reqesting Descriptor Stop"]
            #[must_use]
            #[inline(always)]
            pub const fn sreq(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Reqesting Descriptor Stop"]
            #[inline(always)]
            pub const fn set_sreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Normal Descriptor Command Running Status"]
            #[must_use]
            #[inline(always)]
            pub const fn ndcr(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Normal Descriptor Command Running Status"]
            #[inline(always)]
            pub const fn set_ndcr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "ISE Running Status"]
            #[must_use]
            #[inline(always)]
            pub const fn iser(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "ISE Running Status"]
            #[inline(always)]
            pub const fn set_iser(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        impl core::fmt::Debug for Status {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Status")
                    .field("nreq", &self.nreq())
                    .field("sreq", &self.sreq())
                    .field("ndcr", &self.ndcr())
                    .field("iser", &self.iser())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Status {{ nreq: {=bool:?}, sreq: {=bool:?}, ndcr: {=bool:?}, iser: {=bool:?} }}" , self . nreq () , self . sreq () , self . ndcr () , self . iser ())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Command {
            #[doc = "No Operation"]
            NOP = 0x0,
            #[doc = "Normal Descriptor Set and Run"]
            RUN = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Stop Descriptor"]
            STOP = 0x03,
        }
        impl Command {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Command {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Command {
            #[inline(always)]
            fn from(val: u8) -> Command {
                Command::from_bits(val)
            }
        }
        impl From<Command> for u8 {
            #[inline(always)]
            fn from(val: Command) -> u8 {
                Command::to_bits(val)
            }
        }
    }
}
pub mod rot {
    #[doc = "Image rotation"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rot {
        ptr: *mut u8,
    }
    unsafe impl Send for Rot {}
    unsafe impl Sync for Rot {}
    impl Rot {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Interrupt Status"]
        #[inline(always)]
        pub const fn intr_status(self) -> crate::common::Reg<regs::IntrStatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Interrupt Enable"]
        #[inline(always)]
        pub const fn intr_enable(self) -> crate::common::Reg<regs::IntrEnable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Interrupt Disable"]
        #[inline(always)]
        pub const fn intr_disable(self) -> crate::common::Reg<regs::IntrDisable, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Interrupt Clear"]
        #[inline(always)]
        pub const fn intr_clear(self) -> crate::common::Reg<regs::IntrClear, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Start rotation processing"]
        #[inline(always)]
        pub const fn command(self) -> crate::common::Reg<regs::Command, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Set Rotation Angle"]
        #[inline(always)]
        pub const fn set_direction(
            self,
        ) -> crate::common::Reg<regs::SetDirection, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "Source Image Horizontal Size (Actual size - 1)"]
        #[inline(always)]
        pub const fn set_src_hsize(
            self,
        ) -> crate::common::Reg<regs::SetSrcHsize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "Source Image Vertical Size (Actual size - 1)"]
        #[inline(always)]
        pub const fn set_src_vsize(
            self,
        ) -> crate::common::Reg<regs::SetSrcVsize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Source Image Address"]
        #[inline(always)]
        pub const fn set_src_address(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Source Image Pitch (Actual size - 1)"]
        #[inline(always)]
        pub const fn set_src_pitch(
            self,
        ) -> crate::common::Reg<regs::SetSrcPitch, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "Destination Address"]
        #[inline(always)]
        pub const fn set_dst_address(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "Destination Image Pitch (Actual size - 1)"]
        #[inline(always)]
        pub const fn set_dst_pitch(
            self,
        ) -> crate::common::Reg<regs::SetDstPitch, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
        }
        #[doc = "Running Status"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[doc = "Color Convertion Control"]
        #[inline(always)]
        pub const fn conv_ctrl(self) -> crate::common::Reg<regs::ConvCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
        }
        #[doc = "RGB format selector"]
        #[inline(always)]
        pub const fn rgb_alignment(
            self,
        ) -> crate::common::Reg<regs::RgbAlignment, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Start rotation processing"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Command(pub u32);
        impl Command {
            #[doc = "Start rotation"]
            #[must_use]
            #[inline(always)]
            pub const fn cmd(&self) -> super::vals::Cmd {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Cmd::from_bits(val as u8)
            }
            #[doc = "Start rotation"]
            #[inline(always)]
            pub const fn set_cmd(&mut self, val: super::vals::Cmd) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Command {
            #[inline(always)]
            fn default() -> Command {
                Command(0)
            }
        }
        impl core::fmt::Debug for Command {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Command").field("cmd", &self.cmd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Command {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Command {{ cmd: {:?} }}", self.cmd())
            }
        }
        #[doc = "Color Convertion Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ConvCtrl(pub u32);
        impl ConvCtrl {
            #[doc = "Convert RGB565 to YCbCr422"]
            #[must_use]
            #[inline(always)]
            pub const fn conv_format(&self) -> super::vals::ConvFormat {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::ConvFormat::from_bits(val as u8)
            }
            #[doc = "Convert RGB565 to YCbCr422"]
            #[inline(always)]
            pub const fn set_conv_format(&mut self, val: super::vals::ConvFormat) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Scale factor for Cb and Cr"]
            #[must_use]
            #[inline(always)]
            pub const fn conv_calc_sel(&self) -> super::vals::ConvCalcSel {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::ConvCalcSel::from_bits(val as u8)
            }
            #[doc = "Scale factor for Cb and Cr"]
            #[inline(always)]
            pub const fn set_conv_calc_sel(&mut self, val: super::vals::ConvCalcSel) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
        }
        impl Default for ConvCtrl {
            #[inline(always)]
            fn default() -> ConvCtrl {
                ConvCtrl(0)
            }
        }
        impl core::fmt::Debug for ConvCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ConvCtrl")
                    .field("conv_format", &self.conv_format())
                    .field("conv_calc_sel", &self.conv_calc_sel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ConvCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "ConvCtrl {{ conv_format: {:?}, conv_calc_sel: {:?} }}",
                    self.conv_format(),
                    self.conv_calc_sel()
                )
            }
        }
        #[doc = "Interrupt Clear"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct IntrClear(pub u32);
        impl IntrClear {
            #[doc = "Done Interrupt Clear"]
            #[must_use]
            #[inline(always)]
            pub const fn end_clr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Done Interrupt Clear"]
            #[inline(always)]
            pub const fn set_end_clr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Read Error Clear"]
            #[must_use]
            #[inline(always)]
            pub const fn rd_err_clr(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Read Error Clear"]
            #[inline(always)]
            pub const fn set_rd_err_clr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Write Error Clear"]
            #[must_use]
            #[inline(always)]
            pub const fn wr_err_clr(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write Error Clear"]
            #[inline(always)]
            pub const fn set_wr_err_clr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
        }
        impl Default for IntrClear {
            #[inline(always)]
            fn default() -> IntrClear {
                IntrClear(0)
            }
        }
        impl core::fmt::Debug for IntrClear {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("IntrClear")
                    .field("end_clr", &self.end_clr())
                    .field("rd_err_clr", &self.rd_err_clr())
                    .field("wr_err_clr", &self.wr_err_clr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for IntrClear {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "IntrClear {{ end_clr: {=bool:?}, rd_err_clr: {=bool:?}, wr_err_clr: {=bool:?} }}" , self . end_clr () , self . rd_err_clr () , self . wr_err_clr ())
            }
        }
        #[doc = "Interrupt Disable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct IntrDisable(pub u32);
        impl IntrDisable {
            #[doc = "Done Interrupt Disable"]
            #[must_use]
            #[inline(always)]
            pub const fn end_dis(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Done Interrupt Disable"]
            #[inline(always)]
            pub const fn set_end_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Read Error Disable"]
            #[must_use]
            #[inline(always)]
            pub const fn rd_err_dis(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Read Error Disable"]
            #[inline(always)]
            pub const fn set_rd_err_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Write Error Disable"]
            #[must_use]
            #[inline(always)]
            pub const fn wr_err_dis(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write Error Disable"]
            #[inline(always)]
            pub const fn set_wr_err_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
        }
        impl Default for IntrDisable {
            #[inline(always)]
            fn default() -> IntrDisable {
                IntrDisable(0)
            }
        }
        impl core::fmt::Debug for IntrDisable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("IntrDisable")
                    .field("end_dis", &self.end_dis())
                    .field("rd_err_dis", &self.rd_err_dis())
                    .field("wr_err_dis", &self.wr_err_dis())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for IntrDisable {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "IntrDisable {{ end_dis: {=bool:?}, rd_err_dis: {=bool:?}, wr_err_dis: {=bool:?} }}" , self . end_dis () , self . rd_err_dis () , self . wr_err_dis ())
            }
        }
        #[doc = "Interrupt Enable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct IntrEnable(pub u32);
        impl IntrEnable {
            #[doc = "Done Interrupt Enable"]
            #[must_use]
            #[inline(always)]
            pub const fn end_enb(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Done Interrupt Enable"]
            #[inline(always)]
            pub const fn set_end_enb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Read Error"]
            #[must_use]
            #[inline(always)]
            pub const fn rd_err_enb(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Read Error"]
            #[inline(always)]
            pub const fn set_rd_err_enb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Write Error Enable"]
            #[must_use]
            #[inline(always)]
            pub const fn wr_err_enb(&self) -> super::vals::WrErrEnb {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::WrErrEnb::from_bits(val as u8)
            }
            #[doc = "Write Error Enable"]
            #[inline(always)]
            pub const fn set_wr_err_enb(&mut self, val: super::vals::WrErrEnb) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
        }
        impl Default for IntrEnable {
            #[inline(always)]
            fn default() -> IntrEnable {
                IntrEnable(0)
            }
        }
        impl core::fmt::Debug for IntrEnable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("IntrEnable")
                    .field("end_enb", &self.end_enb())
                    .field("rd_err_enb", &self.rd_err_enb())
                    .field("wr_err_enb", &self.wr_err_enb())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for IntrEnable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "IntrEnable {{ end_enb: {=bool:?}, rd_err_enb: {=bool:?}, wr_err_enb: {:?} }}",
                    self.end_enb(),
                    self.rd_err_enb(),
                    self.wr_err_enb()
                )
            }
        }
        #[doc = "Interrupt Status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct IntrStatus(pub u32);
        impl IntrStatus {
            #[doc = "Done"]
            #[must_use]
            #[inline(always)]
            pub const fn end_sts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Done"]
            #[inline(always)]
            pub const fn set_end_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Read Error"]
            #[must_use]
            #[inline(always)]
            pub const fn rd_err_sts(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Read Error"]
            #[inline(always)]
            pub const fn set_rd_err_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Write Error"]
            #[must_use]
            #[inline(always)]
            pub const fn wr_err_sts(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write Error"]
            #[inline(always)]
            pub const fn set_wr_err_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
        }
        impl Default for IntrStatus {
            #[inline(always)]
            fn default() -> IntrStatus {
                IntrStatus(0)
            }
        }
        impl core::fmt::Debug for IntrStatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("IntrStatus")
                    .field("end_sts", &self.end_sts())
                    .field("rd_err_sts", &self.rd_err_sts())
                    .field("wr_err_sts", &self.wr_err_sts())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for IntrStatus {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "IntrStatus {{ end_sts: {=bool:?}, rd_err_sts: {=bool:?}, wr_err_sts: {=bool:?} }}" , self . end_sts () , self . rd_err_sts () , self . wr_err_sts ())
            }
        }
        #[doc = "RGB format selector"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RgbAlignment(pub u32);
        impl RgbAlignment {
            #[doc = "RGB Format"]
            #[must_use]
            #[inline(always)]
            pub const fn format(&self) -> super::vals::Format {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Format::from_bits(val as u8)
            }
            #[doc = "RGB Format"]
            #[inline(always)]
            pub const fn set_format(&mut self, val: super::vals::Format) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for RgbAlignment {
            #[inline(always)]
            fn default() -> RgbAlignment {
                RgbAlignment(0)
            }
        }
        impl core::fmt::Debug for RgbAlignment {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RgbAlignment")
                    .field("format", &self.format())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RgbAlignment {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RgbAlignment {{ format: {:?} }}", self.format())
            }
        }
        #[doc = "Set Rotation Angle"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SetDirection(pub u32);
        impl SetDirection {
            #[doc = "Rotation Angle"]
            #[must_use]
            #[inline(always)]
            pub const fn rot(&self) -> super::vals::Rot {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Rot::from_bits(val as u8)
            }
            #[doc = "Rotation Angle"]
            #[inline(always)]
            pub const fn set_rot(&mut self, val: super::vals::Rot) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for SetDirection {
            #[inline(always)]
            fn default() -> SetDirection {
                SetDirection(0)
            }
        }
        impl core::fmt::Debug for SetDirection {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SetDirection")
                    .field("rot", &self.rot())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SetDirection {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SetDirection {{ rot: {:?} }}", self.rot())
            }
        }
        #[doc = "Destination Image Pitch (Actual size - 1)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SetDstPitch(pub u32);
        impl SetDstPitch {
            #[must_use]
            #[inline(always)]
            pub const fn pitch(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_pitch(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for SetDstPitch {
            #[inline(always)]
            fn default() -> SetDstPitch {
                SetDstPitch(0)
            }
        }
        impl core::fmt::Debug for SetDstPitch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SetDstPitch")
                    .field("pitch", &self.pitch())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SetDstPitch {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SetDstPitch {{ pitch: {=u16:?} }}", self.pitch())
            }
        }
        #[doc = "Source Image Horizontal Size (Actual size - 1)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SetSrcHsize(pub u32);
        impl SetSrcHsize {
            #[must_use]
            #[inline(always)]
            pub const fn size(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_size(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for SetSrcHsize {
            #[inline(always)]
            fn default() -> SetSrcHsize {
                SetSrcHsize(0)
            }
        }
        impl core::fmt::Debug for SetSrcHsize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SetSrcHsize")
                    .field("size", &self.size())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SetSrcHsize {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SetSrcHsize {{ size: {=u16:?} }}", self.size())
            }
        }
        #[doc = "Source Image Pitch (Actual size - 1)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SetSrcPitch(pub u32);
        impl SetSrcPitch {
            #[must_use]
            #[inline(always)]
            pub const fn pitch(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_pitch(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for SetSrcPitch {
            #[inline(always)]
            fn default() -> SetSrcPitch {
                SetSrcPitch(0)
            }
        }
        impl core::fmt::Debug for SetSrcPitch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SetSrcPitch")
                    .field("pitch", &self.pitch())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SetSrcPitch {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SetSrcPitch {{ pitch: {=u16:?} }}", self.pitch())
            }
        }
        #[doc = "Source Image Vertical Size (Actual size - 1)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SetSrcVsize(pub u32);
        impl SetSrcVsize {
            #[must_use]
            #[inline(always)]
            pub const fn size(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[inline(always)]
            pub const fn set_size(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for SetSrcVsize {
            #[inline(always)]
            fn default() -> SetSrcVsize {
                SetSrcVsize(0)
            }
        }
        impl core::fmt::Debug for SetSrcVsize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SetSrcVsize")
                    .field("size", &self.size())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SetSrcVsize {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SetSrcVsize {{ size: {=u16:?} }}", self.size())
            }
        }
        #[doc = "Running Status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "Running Status (1 = running)"]
            #[must_use]
            #[inline(always)]
            pub const fn status(&self) -> super::vals::Status {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Status::from_bits(val as u8)
            }
            #[doc = "Running Status (1 = running)"]
            #[inline(always)]
            pub const fn set_status(&mut self, val: super::vals::Status) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        impl core::fmt::Debug for Status {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Status")
                    .field("status", &self.status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Status {{ status: {:?} }}", self.status())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cmd {
            _RESERVED_0 = 0x0,
            START = 0x01,
        }
        impl Cmd {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cmd {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cmd {
            #[inline(always)]
            fn from(val: u8) -> Cmd {
                Cmd::from_bits(val)
            }
        }
        impl From<Cmd> for u8 {
            #[inline(always)]
            fn from(val: Cmd) -> u8 {
                Cmd::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum ConvCalcSel {
            FROM16TO240RANGE = 0x0,
            FROM_127TO127RANGE = 0x01,
        }
        impl ConvCalcSel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ConvCalcSel {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ConvCalcSel {
            #[inline(always)]
            fn from(val: u8) -> ConvCalcSel {
                ConvCalcSel::from_bits(val)
            }
        }
        impl From<ConvCalcSel> for u8 {
            #[inline(always)]
            fn from(val: ConvCalcSel) -> u8 {
                ConvCalcSel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum ConvFormat {
            NOCONVERT = 0x0,
            YCBCR422_RGB565 = 0x01,
            RGB565_YCBCR422 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl ConvFormat {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ConvFormat {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ConvFormat {
            #[inline(always)]
            fn from(val: u8) -> ConvFormat {
                ConvFormat::from_bits(val)
            }
        }
        impl From<ConvFormat> for u8 {
            #[inline(always)]
            fn from(val: ConvFormat) -> u8 {
                ConvFormat::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Format {
            RGB = 0x0,
            BGR = 0x01,
        }
        impl Format {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Format {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Format {
            #[inline(always)]
            fn from(val: u8) -> Format {
                Format::from_bits(val)
            }
        }
        impl From<Format> for u8 {
            #[inline(always)]
            fn from(val: Format) -> u8 {
                Format::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rot {
            NO_ROTATION = 0x0,
            RIGHT_90_DEGREES = 0x01,
            RIGHT_180_DEGREES = 0x02,
            _RESERVED_3 = 0x03,
            RIGHT_270_DEGREES = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Rot {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rot {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rot {
            #[inline(always)]
            fn from(val: u8) -> Rot {
                Rot::from_bits(val)
            }
        }
        impl From<Rot> for u8 {
            #[inline(always)]
            fn from(val: Rot) -> u8 {
                Rot::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Status {
            STOP = 0x0,
            RUNNING = 0x01,
        }
        impl Status {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Status {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Status {
            #[inline(always)]
            fn from(val: u8) -> Status {
                Status::from_bits(val)
            }
        }
        impl From<Status> for u8 {
            #[inline(always)]
            fn from(val: Status) -> u8 {
                Status::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum WrErrEnb {
            DISABLE = 0x0,
            ENABLE = 0x01,
        }
        impl WrErrEnb {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> WrErrEnb {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for WrErrEnb {
            #[inline(always)]
            fn from(val: u8) -> WrErrEnb {
                WrErrEnb::from_bits(val)
            }
        }
        impl From<WrErrEnb> for u8 {
            #[inline(always)]
            fn from(val: WrErrEnb) -> u8 {
                WrErrEnb::to_bits(val)
            }
        }
    }
}
pub mod smp_ram_ctrl {
    #[doc = "SRAM Control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmpRamCtrl {
        ptr: *mut u8,
    }
    unsafe impl Send for SmpRamCtrl {}
    unsafe impl Sync for SmpRamCtrl {}
    impl SmpRamCtrl {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "SRAM tile (128KB) clock gating control (0 = gated)"]
        #[inline(always)]
        pub const fn app_tile_clk_gating_enb(
            self,
        ) -> crate::common::Reg<regs::AppTileClkGatingEnb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "SRAM tile (128KB) clock gating control (0 = gated)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AppTileClkGatingEnb(pub u32);
        impl AppTileClkGatingEnb {
            #[doc = "Enable clock gating"]
            #[must_use]
            #[inline(always)]
            pub const fn tile_clk_gating_enb(&self, n: usize) -> super::vals::TileClkGatingEnb {
                assert!(n < 12usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::TileClkGatingEnb::from_bits(val as u8)
            }
            #[doc = "Enable clock gating"]
            #[inline(always)]
            pub const fn set_tile_clk_gating_enb(
                &mut self,
                n: usize,
                val: super::vals::TileClkGatingEnb,
            ) {
                assert!(n < 12usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
            }
        }
        impl Default for AppTileClkGatingEnb {
            #[inline(always)]
            fn default() -> AppTileClkGatingEnb {
                AppTileClkGatingEnb(0)
            }
        }
        impl core::fmt::Debug for AppTileClkGatingEnb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AppTileClkGatingEnb")
                    .field("tile_clk_gating_enb[0]", &self.tile_clk_gating_enb(0usize))
                    .field("tile_clk_gating_enb[1]", &self.tile_clk_gating_enb(1usize))
                    .field("tile_clk_gating_enb[2]", &self.tile_clk_gating_enb(2usize))
                    .field("tile_clk_gating_enb[3]", &self.tile_clk_gating_enb(3usize))
                    .field("tile_clk_gating_enb[4]", &self.tile_clk_gating_enb(4usize))
                    .field("tile_clk_gating_enb[5]", &self.tile_clk_gating_enb(5usize))
                    .field("tile_clk_gating_enb[6]", &self.tile_clk_gating_enb(6usize))
                    .field("tile_clk_gating_enb[7]", &self.tile_clk_gating_enb(7usize))
                    .field("tile_clk_gating_enb[8]", &self.tile_clk_gating_enb(8usize))
                    .field("tile_clk_gating_enb[9]", &self.tile_clk_gating_enb(9usize))
                    .field(
                        "tile_clk_gating_enb[10]",
                        &self.tile_clk_gating_enb(10usize),
                    )
                    .field(
                        "tile_clk_gating_enb[11]",
                        &self.tile_clk_gating_enb(11usize),
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AppTileClkGatingEnb {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "AppTileClkGatingEnb {{ tile_clk_gating_enb[0]: {:?}, tile_clk_gating_enb[1]: {:?}, tile_clk_gating_enb[2]: {:?}, tile_clk_gating_enb[3]: {:?}, tile_clk_gating_enb[4]: {:?}, tile_clk_gating_enb[5]: {:?}, tile_clk_gating_enb[6]: {:?}, tile_clk_gating_enb[7]: {:?}, tile_clk_gating_enb[8]: {:?}, tile_clk_gating_enb[9]: {:?}, tile_clk_gating_enb[10]: {:?}, tile_clk_gating_enb[11]: {:?} }}" , self . tile_clk_gating_enb (0usize) , self . tile_clk_gating_enb (1usize) , self . tile_clk_gating_enb (2usize) , self . tile_clk_gating_enb (3usize) , self . tile_clk_gating_enb (4usize) , self . tile_clk_gating_enb (5usize) , self . tile_clk_gating_enb (6usize) , self . tile_clk_gating_enb (7usize) , self . tile_clk_gating_enb (8usize) , self . tile_clk_gating_enb (9usize) , self . tile_clk_gating_enb (10usize) , self . tile_clk_gating_enb (11usize))
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TileClkGatingEnb {
            GATED = 0x0,
            UNGATED = 0x01,
        }
        impl TileClkGatingEnb {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TileClkGatingEnb {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TileClkGatingEnb {
            #[inline(always)]
            fn from(val: u8) -> TileClkGatingEnb {
                TileClkGatingEnb::from_bits(val)
            }
        }
        impl From<TileClkGatingEnb> for u8 {
            #[inline(always)]
            fn from(val: TileClkGatingEnb) -> u8 {
                TileClkGatingEnb::to_bits(val)
            }
        }
    }
}
pub mod spi0 {
    #[doc = "Synchronous Serial Port Controller (SPIM)"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spi0 {
        ptr: *mut u8,
    }
    unsafe impl Send for Spi0 {}
    unsafe impl Sync for Spi0 {}
    impl Spi0 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Control register 0"]
        #[inline(always)]
        pub const fn sspcr0(self) -> crate::common::Reg<regs::Sspcr0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Control register 1"]
        #[inline(always)]
        pub const fn sspcr1(self) -> crate::common::Reg<regs::Sspcr1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Data register"]
        #[inline(always)]
        pub const fn sspdr(self) -> crate::common::Reg<regs::Sspdr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Status register"]
        #[inline(always)]
        pub const fn sspsr(self) -> crate::common::Reg<regs::Sspsr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Clock prescale register"]
        #[inline(always)]
        pub const fn sspcpsr(self) -> crate::common::Reg<regs::Sspcpsr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Interrupt mask set or clear register"]
        #[inline(always)]
        pub const fn sspimsc(self) -> crate::common::Reg<regs::Sspimsc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "Raw interrupt status register"]
        #[inline(always)]
        pub const fn sspris(self) -> crate::common::Reg<regs::Sspris, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "Masked interrupt status register"]
        #[inline(always)]
        pub const fn sspmis(self) -> crate::common::Reg<regs::Sspmis, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Interrupt clear register"]
        #[inline(always)]
        pub const fn sspicr(self) -> crate::common::Reg<regs::Sspicr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "DMA control register"]
        #[inline(always)]
        pub const fn sspdmacr(self) -> crate::common::Reg<regs::Sspdmacr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[inline(always)]
        pub const fn csmode(self) -> crate::common::Reg<regs::Csmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
        }
        #[inline(always)]
        pub const fn sspcs(self) -> crate::common::Reg<regs::Sspcs, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
        }
        #[inline(always)]
        pub const fn slavetype(self) -> crate::common::Reg<regs::Slavetype, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
        }
    }
    pub mod regs {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Csmode(pub u32);
        impl Csmode {
            #[must_use]
            #[inline(always)]
            pub const fn cs_mode(&self) -> super::vals::CsMode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::CsMode::from_bits(val as u8)
            }
            #[inline(always)]
            pub const fn set_cs_mode(&mut self, val: super::vals::CsMode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Csmode {
            #[inline(always)]
            fn default() -> Csmode {
                Csmode(0)
            }
        }
        impl core::fmt::Debug for Csmode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Csmode")
                    .field("cs_mode", &self.cs_mode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Csmode {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Csmode {{ cs_mode: {:?} }}", self.cs_mode())
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Slavetype(pub u32);
        impl Slavetype {
            #[must_use]
            #[inline(always)]
            pub const fn slave_type(&self) -> super::vals::SlaveType {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::SlaveType::from_bits(val as u8)
            }
            #[inline(always)]
            pub const fn set_slave_type(&mut self, val: super::vals::SlaveType) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Slavetype {
            #[inline(always)]
            fn default() -> Slavetype {
                Slavetype(0)
            }
        }
        impl core::fmt::Debug for Slavetype {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Slavetype")
                    .field("slave_type", &self.slave_type())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Slavetype {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Slavetype {{ slave_type: {:?} }}", self.slave_type())
            }
        }
        #[doc = "Clock prescale register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspcpsr(pub u32);
        impl Sspcpsr {
            #[doc = "Clock prescale divisor"]
            #[must_use]
            #[inline(always)]
            pub const fn cpsdvsr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Clock prescale divisor"]
            #[inline(always)]
            pub const fn set_cpsdvsr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Sspcpsr {
            #[inline(always)]
            fn default() -> Sspcpsr {
                Sspcpsr(0)
            }
        }
        impl core::fmt::Debug for Sspcpsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspcpsr")
                    .field("cpsdvsr", &self.cpsdvsr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspcpsr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sspcpsr {{ cpsdvsr: {=u8:?} }}", self.cpsdvsr())
            }
        }
        #[doc = "Control register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspcr0(pub u32);
        impl Sspcr0 {
            #[doc = "Data Size Select"]
            #[must_use]
            #[inline(always)]
            pub const fn dss(&self) -> super::vals::Dss {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Dss::from_bits(val as u8)
            }
            #[doc = "Data Size Select"]
            #[inline(always)]
            pub const fn set_dss(&mut self, val: super::vals::Dss) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[doc = "Frame format"]
            #[must_use]
            #[inline(always)]
            pub const fn frf(&self) -> super::vals::Frf {
                let val = (self.0 >> 4usize) & 0x03;
                super::vals::Frf::from_bits(val as u8)
            }
            #[doc = "Frame format"]
            #[inline(always)]
            pub const fn set_frf(&mut self, val: super::vals::Frf) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
            }
            #[doc = "SSPCLKOUT polarity"]
            #[must_use]
            #[inline(always)]
            pub const fn spo(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SSPCLKOUT polarity"]
            #[inline(always)]
            pub const fn set_spo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "SSPCLKOUT phase"]
            #[must_use]
            #[inline(always)]
            pub const fn sph(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "SSPCLKOUT phase"]
            #[inline(always)]
            pub const fn set_sph(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Serial clock rate"]
            #[must_use]
            #[inline(always)]
            pub const fn scr(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Serial clock rate"]
            #[inline(always)]
            pub const fn set_scr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Sspcr0 {
            #[inline(always)]
            fn default() -> Sspcr0 {
                Sspcr0(0)
            }
        }
        impl core::fmt::Debug for Sspcr0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspcr0")
                    .field("dss", &self.dss())
                    .field("frf", &self.frf())
                    .field("spo", &self.spo())
                    .field("sph", &self.sph())
                    .field("scr", &self.scr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspcr0 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Sspcr0 {{ dss: {:?}, frf: {:?}, spo: {=bool:?}, sph: {=bool:?}, scr: {=u8:?} }}" , self . dss () , self . frf () , self . spo () , self . sph () , self . scr ())
            }
        }
        #[doc = "Control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspcr1(pub u32);
        impl Sspcr1 {
            #[doc = "Loop back mode"]
            #[must_use]
            #[inline(always)]
            pub const fn lbm(&self) -> super::vals::Lbm {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Lbm::from_bits(val as u8)
            }
            #[doc = "Loop back mode"]
            #[inline(always)]
            pub const fn set_lbm(&mut self, val: super::vals::Lbm) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Synchronous serial port enable"]
            #[must_use]
            #[inline(always)]
            pub const fn sse(&self) -> super::vals::Sse {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Sse::from_bits(val as u8)
            }
            #[doc = "Synchronous serial port enable"]
            #[inline(always)]
            pub const fn set_sse(&mut self, val: super::vals::Sse) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Master or slave mode select"]
            #[must_use]
            #[inline(always)]
            pub const fn ms(&self) -> super::vals::Ms {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Ms::from_bits(val as u8)
            }
            #[doc = "Master or slave mode select"]
            #[inline(always)]
            pub const fn set_ms(&mut self, val: super::vals::Ms) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Slave-mode output disable"]
            #[must_use]
            #[inline(always)]
            pub const fn sod(&self) -> super::vals::Sod {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Sod::from_bits(val as u8)
            }
            #[doc = "Slave-mode output disable"]
            #[inline(always)]
            pub const fn set_sod(&mut self, val: super::vals::Sod) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sspcr1 {
            #[inline(always)]
            fn default() -> Sspcr1 {
                Sspcr1(0)
            }
        }
        impl core::fmt::Debug for Sspcr1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspcr1")
                    .field("lbm", &self.lbm())
                    .field("sse", &self.sse())
                    .field("ms", &self.ms())
                    .field("sod", &self.sod())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspcr1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspcr1 {{ lbm: {:?}, sse: {:?}, ms: {:?}, sod: {:?} }}",
                    self.lbm(),
                    self.sse(),
                    self.ms(),
                    self.sod()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspcs(pub u32);
        impl Sspcs {
            #[must_use]
            #[inline(always)]
            pub const fn ssp_cs(&self) -> super::vals::SspCs {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::SspCs::from_bits(val as u8)
            }
            #[inline(always)]
            pub const fn set_ssp_cs(&mut self, val: super::vals::SspCs) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Sspcs {
            #[inline(always)]
            fn default() -> Sspcs {
                Sspcs(0)
            }
        }
        impl core::fmt::Debug for Sspcs {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspcs")
                    .field("ssp_cs", &self.ssp_cs())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspcs {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sspcs {{ ssp_cs: {:?} }}", self.ssp_cs())
            }
        }
        #[doc = "DMA control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspdmacr(pub u32);
        impl Sspdmacr {
            #[doc = "Receive DMA enable"]
            #[must_use]
            #[inline(always)]
            pub const fn rxdmae(&self) -> super::vals::Rxdmae {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rxdmae::from_bits(val as u8)
            }
            #[doc = "Receive DMA enable"]
            #[inline(always)]
            pub const fn set_rxdmae(&mut self, val: super::vals::Rxdmae) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit DMA enable"]
            #[must_use]
            #[inline(always)]
            pub const fn txdmae(&self) -> super::vals::Txdmae {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Txdmae::from_bits(val as u8)
            }
            #[doc = "Transmit DMA enable"]
            #[inline(always)]
            pub const fn set_txdmae(&mut self, val: super::vals::Txdmae) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Sspdmacr {
            #[inline(always)]
            fn default() -> Sspdmacr {
                Sspdmacr(0)
            }
        }
        impl core::fmt::Debug for Sspdmacr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspdmacr")
                    .field("rxdmae", &self.rxdmae())
                    .field("txdmae", &self.txdmae())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspdmacr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspdmacr {{ rxdmae: {:?}, txdmae: {:?} }}",
                    self.rxdmae(),
                    self.txdmae()
                )
            }
        }
        #[doc = "Data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspdr(pub u32);
        impl Sspdr {
            #[doc = "Transmit/Receive FIFO"]
            #[must_use]
            #[inline(always)]
            pub const fn data(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Transmit/Receive FIFO"]
            #[inline(always)]
            pub const fn set_data(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Sspdr {
            #[inline(always)]
            fn default() -> Sspdr {
                Sspdr(0)
            }
        }
        impl core::fmt::Debug for Sspdr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspdr").field("data", &self.data()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspdr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sspdr {{ data: {=u16:?} }}", self.data())
            }
        }
        #[doc = "Interrupt clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspicr(pub u32);
        impl Sspicr {
            #[doc = "Clear the SSPRORINTR interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn roric(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Clear the SSPRORINTR interrupt"]
            #[inline(always)]
            pub const fn set_roric(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Clear the SSPRTINTR interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn rtic(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Clear the SSPRTINTR interrupt"]
            #[inline(always)]
            pub const fn set_rtic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Sspicr {
            #[inline(always)]
            fn default() -> Sspicr {
                Sspicr(0)
            }
        }
        impl core::fmt::Debug for Sspicr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspicr")
                    .field("roric", &self.roric())
                    .field("rtic", &self.rtic())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspicr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspicr {{ roric: {=bool:?}, rtic: {=bool:?} }}",
                    self.roric(),
                    self.rtic()
                )
            }
        }
        #[doc = "Interrupt mask set or clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspimsc(pub u32);
        impl Sspimsc {
            #[doc = "Receive overrun interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rorim(&self) -> super::vals::Rorim {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rorim::from_bits(val as u8)
            }
            #[doc = "Receive overrun interrupt mask"]
            #[inline(always)]
            pub const fn set_rorim(&mut self, val: super::vals::Rorim) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Receive timeout interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rtim(&self) -> super::vals::Rtim {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rtim::from_bits(val as u8)
            }
            #[doc = "Receive timeout interrupt mask"]
            #[inline(always)]
            pub const fn set_rtim(&mut self, val: super::vals::Rtim) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxim(&self) -> super::vals::Rxim {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rxim::from_bits(val as u8)
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_rxim(&mut self, val: super::vals::Rxim) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn txim(&self) -> super::vals::Txim {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Txim::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_txim(&mut self, val: super::vals::Txim) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sspimsc {
            #[inline(always)]
            fn default() -> Sspimsc {
                Sspimsc(0)
            }
        }
        impl core::fmt::Debug for Sspimsc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspimsc")
                    .field("rorim", &self.rorim())
                    .field("rtim", &self.rtim())
                    .field("rxim", &self.rxim())
                    .field("txim", &self.txim())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspimsc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspimsc {{ rorim: {:?}, rtim: {:?}, rxim: {:?}, txim: {:?} }}",
                    self.rorim(),
                    self.rtim(),
                    self.rxim(),
                    self.txim()
                )
            }
        }
        #[doc = "Masked interrupt status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspmis(pub u32);
        impl Sspmis {
            #[doc = "Receive overrun interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rormis(&self) -> super::vals::Rormis {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rormis::from_bits(val as u8)
            }
            #[doc = "Receive overrun interrupt mask"]
            #[inline(always)]
            pub const fn set_rormis(&mut self, val: super::vals::Rormis) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Receive timeout interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rtmis(&self) -> super::vals::Rtmis {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rtmis::from_bits(val as u8)
            }
            #[doc = "Receive timeout interrupt mask"]
            #[inline(always)]
            pub const fn set_rtmis(&mut self, val: super::vals::Rtmis) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxmis(&self) -> super::vals::Rxmis {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rxmis::from_bits(val as u8)
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_rxmis(&mut self, val: super::vals::Rxmis) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn txmis(&self) -> super::vals::Txmis {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Txmis::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_txmis(&mut self, val: super::vals::Txmis) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sspmis {
            #[inline(always)]
            fn default() -> Sspmis {
                Sspmis(0)
            }
        }
        impl core::fmt::Debug for Sspmis {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspmis")
                    .field("rormis", &self.rormis())
                    .field("rtmis", &self.rtmis())
                    .field("rxmis", &self.rxmis())
                    .field("txmis", &self.txmis())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspmis {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspmis {{ rormis: {:?}, rtmis: {:?}, rxmis: {:?}, txmis: {:?} }}",
                    self.rormis(),
                    self.rtmis(),
                    self.rxmis(),
                    self.txmis()
                )
            }
        }
        #[doc = "Raw interrupt status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspris(pub u32);
        impl Sspris {
            #[doc = "Receive overrun interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rorris(&self) -> super::vals::Rorris {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rorris::from_bits(val as u8)
            }
            #[doc = "Receive overrun interrupt mask"]
            #[inline(always)]
            pub const fn set_rorris(&mut self, val: super::vals::Rorris) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Receive timeout interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rtris(&self) -> super::vals::Rtris {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rtris::from_bits(val as u8)
            }
            #[doc = "Receive timeout interrupt mask"]
            #[inline(always)]
            pub const fn set_rtris(&mut self, val: super::vals::Rtris) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxris(&self) -> super::vals::Rxris {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rxris::from_bits(val as u8)
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_rxris(&mut self, val: super::vals::Rxris) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn txris(&self) -> super::vals::Txris {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Txris::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_txris(&mut self, val: super::vals::Txris) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sspris {
            #[inline(always)]
            fn default() -> Sspris {
                Sspris(0)
            }
        }
        impl core::fmt::Debug for Sspris {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspris")
                    .field("rorris", &self.rorris())
                    .field("rtris", &self.rtris())
                    .field("rxris", &self.rxris())
                    .field("txris", &self.txris())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspris {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspris {{ rorris: {:?}, rtris: {:?}, rxris: {:?}, txris: {:?} }}",
                    self.rorris(),
                    self.rtris(),
                    self.rxris(),
                    self.txris()
                )
            }
        }
        #[doc = "Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspsr(pub u32);
        impl Sspsr {
            #[doc = "Transmit FIFO empty"]
            #[must_use]
            #[inline(always)]
            pub const fn tfe(&self) -> super::vals::Tfe {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Tfe::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO empty"]
            #[inline(always)]
            pub const fn set_tfe(&mut self, val: super::vals::Tfe) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit FIFO not full"]
            #[must_use]
            #[inline(always)]
            pub const fn tnf(&self) -> super::vals::Tnf {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Tnf::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO not full"]
            #[inline(always)]
            pub const fn set_tnf(&mut self, val: super::vals::Tnf) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Rceive FIFIO not empty"]
            #[must_use]
            #[inline(always)]
            pub const fn rne(&self) -> super::vals::Rne {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rne::from_bits(val as u8)
            }
            #[doc = "Rceive FIFIO not empty"]
            #[inline(always)]
            pub const fn set_rne(&mut self, val: super::vals::Rne) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Receive FIFO full"]
            #[must_use]
            #[inline(always)]
            pub const fn rff(&self) -> super::vals::Rff {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Rff::from_bits(val as u8)
            }
            #[doc = "Receive FIFO full"]
            #[inline(always)]
            pub const fn set_rff(&mut self, val: super::vals::Rff) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "SSP busy flag"]
            #[must_use]
            #[inline(always)]
            pub const fn bsy(&self) -> super::vals::Bsy {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Bsy::from_bits(val as u8)
            }
            #[doc = "SSP busy flag"]
            #[inline(always)]
            pub const fn set_bsy(&mut self, val: super::vals::Bsy) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Sspsr {
            #[inline(always)]
            fn default() -> Sspsr {
                Sspsr(0)
            }
        }
        impl core::fmt::Debug for Sspsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspsr")
                    .field("tfe", &self.tfe())
                    .field("tnf", &self.tnf())
                    .field("rne", &self.rne())
                    .field("rff", &self.rff())
                    .field("bsy", &self.bsy())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspsr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspsr {{ tfe: {:?}, tnf: {:?}, rne: {:?}, rff: {:?}, bsy: {:?} }}",
                    self.tfe(),
                    self.tnf(),
                    self.rne(),
                    self.rff(),
                    self.bsy()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Bsy {
            #[doc = "SSP is idle"]
            IDLE = 0x0,
            #[doc = "SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty"]
            BUSY = 0x01,
        }
        impl Bsy {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Bsy {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Bsy {
            #[inline(always)]
            fn from(val: u8) -> Bsy {
                Bsy::from_bits(val)
            }
        }
        impl From<Bsy> for u8 {
            #[inline(always)]
            fn from(val: Bsy) -> u8 {
                Bsy::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum CsMode {
            #[doc = "CS controled by controller"]
            AUTO = 0x0,
            #[doc = "CS controled by CS register"]
            MANUAL = 0x01,
        }
        impl CsMode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> CsMode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for CsMode {
            #[inline(always)]
            fn from(val: u8) -> CsMode {
                CsMode::from_bits(val)
            }
        }
        impl From<CsMode> for u8 {
            #[inline(always)]
            fn from(val: CsMode) -> u8 {
                CsMode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dss {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "4bit data"]
            _4BIT = 0x03,
            #[doc = "5bit data"]
            _5BIT = 0x04,
            #[doc = "6bit data"]
            _6BIT = 0x05,
            #[doc = "7bit data"]
            _7BIT = 0x06,
            #[doc = "8bit data"]
            _8BIT = 0x07,
            #[doc = "9bit data"]
            _9BIT = 0x08,
            #[doc = "10bit data"]
            _10BIT = 0x09,
            #[doc = "11bit data"]
            _11BIT = 0x0a,
            #[doc = "12bit data"]
            _12BIT = 0x0b,
            #[doc = "13bit data"]
            _13BIT = 0x0c,
            #[doc = "14bit data"]
            _14BIT = 0x0d,
            #[doc = "15bit data"]
            _15BIT = 0x0e,
            #[doc = "16bit data"]
            _16BIT = 0x0f,
        }
        impl Dss {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dss {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dss {
            #[inline(always)]
            fn from(val: u8) -> Dss {
                Dss::from_bits(val)
            }
        }
        impl From<Dss> for u8 {
            #[inline(always)]
            fn from(val: Dss) -> u8 {
                Dss::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Frf {
            #[doc = "Motorola SPI frame format"]
            MOTOROLA = 0x0,
            #[doc = "TI synchronous serial frame format"]
            TI = 0x01,
            #[doc = "National Microwire frame format"]
            NM = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Frf {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Frf {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Frf {
            #[inline(always)]
            fn from(val: u8) -> Frf {
                Frf::from_bits(val)
            }
        }
        impl From<Frf> for u8 {
            #[inline(always)]
            fn from(val: Frf) -> u8 {
                Frf::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lbm {
            #[doc = "Normal serial port operation enabled"]
            DISABLED = 0x0,
            #[doc = "Output of transmit serial shifter is connected to input of recieve serial shifter internally"]
            ENABLED = 0x01,
        }
        impl Lbm {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lbm {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lbm {
            #[inline(always)]
            fn from(val: u8) -> Lbm {
                Lbm::from_bits(val)
            }
        }
        impl From<Lbm> for u8 {
            #[inline(always)]
            fn from(val: Lbm) -> u8 {
                Lbm::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ms {
            #[doc = "Master mode"]
            MASTER = 0x0,
            #[doc = "Slave mode"]
            SLAVE = 0x01,
        }
        impl Ms {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ms {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ms {
            #[inline(always)]
            fn from(val: u8) -> Ms {
                Ms::from_bits(val)
            }
        }
        impl From<Ms> for u8 {
            #[inline(always)]
            fn from(val: Ms) -> u8 {
                Ms::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rff {
            #[doc = "Receive FIFO is not full"]
            NOTFULL = 0x0,
            #[doc = "Recieve FIFO is full"]
            FULL = 0x01,
        }
        impl Rff {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rff {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rff {
            #[inline(always)]
            fn from(val: u8) -> Rff {
                Rff::from_bits(val)
            }
        }
        impl From<Rff> for u8 {
            #[inline(always)]
            fn from(val: Rff) -> u8 {
                Rff::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rne {
            #[doc = "Receive FIFO is empty"]
            EMPTY = 0x0,
            #[doc = "Receive FIFO is not empty"]
            NOTEMPTY = 0x01,
        }
        impl Rne {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rne {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rne {
            #[inline(always)]
            fn from(val: u8) -> Rne {
                Rne::from_bits(val)
            }
        }
        impl From<Rne> for u8 {
            #[inline(always)]
            fn from(val: Rne) -> u8 {
                Rne::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rorim {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rorim {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rorim {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rorim {
            #[inline(always)]
            fn from(val: u8) -> Rorim {
                Rorim::from_bits(val)
            }
        }
        impl From<Rorim> for u8 {
            #[inline(always)]
            fn from(val: Rorim) -> u8 {
                Rorim::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rormis {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rormis {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rormis {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rormis {
            #[inline(always)]
            fn from(val: u8) -> Rormis {
                Rormis::from_bits(val)
            }
        }
        impl From<Rormis> for u8 {
            #[inline(always)]
            fn from(val: Rormis) -> u8 {
                Rormis::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rorris {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rorris {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rorris {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rorris {
            #[inline(always)]
            fn from(val: u8) -> Rorris {
                Rorris::from_bits(val)
            }
        }
        impl From<Rorris> for u8 {
            #[inline(always)]
            fn from(val: Rorris) -> u8 {
                Rorris::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtim {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rtim {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtim {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtim {
            #[inline(always)]
            fn from(val: u8) -> Rtim {
                Rtim::from_bits(val)
            }
        }
        impl From<Rtim> for u8 {
            #[inline(always)]
            fn from(val: Rtim) -> u8 {
                Rtim::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtmis {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rtmis {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtmis {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtmis {
            #[inline(always)]
            fn from(val: u8) -> Rtmis {
                Rtmis::from_bits(val)
            }
        }
        impl From<Rtmis> for u8 {
            #[inline(always)]
            fn from(val: Rtmis) -> u8 {
                Rtmis::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtris {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rtris {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtris {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtris {
            #[inline(always)]
            fn from(val: u8) -> Rtris {
                Rtris::from_bits(val)
            }
        }
        impl From<Rtris> for u8 {
            #[inline(always)]
            fn from(val: Rtris) -> u8 {
                Rtris::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxdmae {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Rxdmae {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxdmae {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxdmae {
            #[inline(always)]
            fn from(val: u8) -> Rxdmae {
                Rxdmae::from_bits(val)
            }
        }
        impl From<Rxdmae> for u8 {
            #[inline(always)]
            fn from(val: Rxdmae) -> u8 {
                Rxdmae::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxim {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rxim {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxim {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxim {
            #[inline(always)]
            fn from(val: u8) -> Rxim {
                Rxim::from_bits(val)
            }
        }
        impl From<Rxim> for u8 {
            #[inline(always)]
            fn from(val: Rxim) -> u8 {
                Rxim::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxmis {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rxmis {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxmis {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxmis {
            #[inline(always)]
            fn from(val: u8) -> Rxmis {
                Rxmis::from_bits(val)
            }
        }
        impl From<Rxmis> for u8 {
            #[inline(always)]
            fn from(val: Rxmis) -> u8 {
                Rxmis::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxris {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rxris {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxris {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxris {
            #[inline(always)]
            fn from(val: u8) -> Rxris {
                Rxris::from_bits(val)
            }
        }
        impl From<Rxris> for u8 {
            #[inline(always)]
            fn from(val: Rxris) -> u8 {
                Rxris::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum SlaveType {
            #[doc = "Select CS0"]
            CS0 = 0x0,
            #[doc = "Select CS1"]
            CS1 = 0x01,
            #[doc = "Select CS2"]
            CS2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl SlaveType {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> SlaveType {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for SlaveType {
            #[inline(always)]
            fn from(val: u8) -> SlaveType {
                SlaveType::from_bits(val)
            }
        }
        impl From<SlaveType> for u8 {
            #[inline(always)]
            fn from(val: SlaveType) -> u8 {
                SlaveType::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sod {
            #[doc = "SSP can drive the SSPTXD output in slave mode"]
            ENABLE = 0x0,
            #[doc = "SSP must not drive the SSPTXD output in slave mode"]
            DISABLE = 0x01,
        }
        impl Sod {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sod {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sod {
            #[inline(always)]
            fn from(val: u8) -> Sod {
                Sod::from_bits(val)
            }
        }
        impl From<Sod> for u8 {
            #[inline(always)]
            fn from(val: Sod) -> u8 {
                Sod::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sse {
            #[doc = "SSP operation disabled"]
            DISABLED = 0x0,
            #[doc = "SSP operation enabled"]
            ENABLED = 0x01,
        }
        impl Sse {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sse {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sse {
            #[inline(always)]
            fn from(val: u8) -> Sse {
                Sse::from_bits(val)
            }
        }
        impl From<Sse> for u8 {
            #[inline(always)]
            fn from(val: Sse) -> u8 {
                Sse::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum SspCs {
            #[doc = "Output low to the CS"]
            LOW = 0x0,
            #[doc = "Output high to the CS"]
            HIGH = 0x01,
        }
        impl SspCs {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> SspCs {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for SspCs {
            #[inline(always)]
            fn from(val: u8) -> SspCs {
                SspCs::from_bits(val)
            }
        }
        impl From<SspCs> for u8 {
            #[inline(always)]
            fn from(val: SspCs) -> u8 {
                SspCs::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tfe {
            #[doc = "Transmit FIFO is not empty"]
            NOTEMPTY = 0x0,
            #[doc = "Transmit FIFO is empty"]
            EMPTY = 0x01,
        }
        impl Tfe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tfe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tfe {
            #[inline(always)]
            fn from(val: u8) -> Tfe {
                Tfe::from_bits(val)
            }
        }
        impl From<Tfe> for u8 {
            #[inline(always)]
            fn from(val: Tfe) -> u8 {
                Tfe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tnf {
            #[doc = "Transmit FIFO is full"]
            FULL = 0x0,
            #[doc = "Transmit FIFO is not full"]
            NOTFULL = 0x01,
        }
        impl Tnf {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tnf {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tnf {
            #[inline(always)]
            fn from(val: u8) -> Tnf {
                Tnf::from_bits(val)
            }
        }
        impl From<Tnf> for u8 {
            #[inline(always)]
            fn from(val: Tnf) -> u8 {
                Tnf::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txdmae {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Txdmae {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txdmae {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txdmae {
            #[inline(always)]
            fn from(val: u8) -> Txdmae {
                Txdmae::from_bits(val)
            }
        }
        impl From<Txdmae> for u8 {
            #[inline(always)]
            fn from(val: Txdmae) -> u8 {
                Txdmae::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txim {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Txim {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txim {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txim {
            #[inline(always)]
            fn from(val: u8) -> Txim {
                Txim::from_bits(val)
            }
        }
        impl From<Txim> for u8 {
            #[inline(always)]
            fn from(val: Txim) -> u8 {
                Txim::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txmis {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Txmis {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txmis {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txmis {
            #[inline(always)]
            fn from(val: u8) -> Txmis {
                Txmis::from_bits(val)
            }
        }
        impl From<Txmis> for u8 {
            #[inline(always)]
            fn from(val: Txmis) -> u8 {
                Txmis::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txris {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Txris {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txris {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txris {
            #[inline(always)]
            fn from(val: u8) -> Txris {
                Txris::from_bits(val)
            }
        }
        impl From<Txris> for u8 {
            #[inline(always)]
            fn from(val: Txris) -> u8 {
                Txris::to_bits(val)
            }
        }
    }
}
pub mod spi4 {
    #[doc = "Synchronous Serial Port Controller (IMG SPI)"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spi4 {
        ptr: *mut u8,
    }
    unsafe impl Send for Spi4 {}
    unsafe impl Sync for Spi4 {}
    impl Spi4 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Control register 0"]
        #[inline(always)]
        pub const fn sspcr0(self) -> crate::common::Reg<regs::Sspcr0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Control register 1"]
        #[inline(always)]
        pub const fn sspcr1(self) -> crate::common::Reg<regs::Sspcr1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Data register"]
        #[inline(always)]
        pub const fn sspdr(self) -> crate::common::Reg<regs::Sspdr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Status register"]
        #[inline(always)]
        pub const fn sspsr(self) -> crate::common::Reg<regs::Sspsr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Clock prescale register"]
        #[inline(always)]
        pub const fn sspcpsr(self) -> crate::common::Reg<regs::Sspcpsr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Interrupt mask set or clear register"]
        #[inline(always)]
        pub const fn sspimsc(self) -> crate::common::Reg<regs::Sspimsc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "Raw interrupt status register"]
        #[inline(always)]
        pub const fn sspris(self) -> crate::common::Reg<regs::Sspris, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "Masked interrupt status register"]
        #[inline(always)]
        pub const fn sspmis(self) -> crate::common::Reg<regs::Sspmis, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Interrupt clear register"]
        #[inline(always)]
        pub const fn sspicr(self) -> crate::common::Reg<regs::Sspicr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "DMA control register"]
        #[inline(always)]
        pub const fn sspdmacr(self) -> crate::common::Reg<regs::Sspdmacr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Clock prescale register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspcpsr(pub u32);
        impl Sspcpsr {
            #[doc = "Clock prescale divisor"]
            #[must_use]
            #[inline(always)]
            pub const fn cpsdvsr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Clock prescale divisor"]
            #[inline(always)]
            pub const fn set_cpsdvsr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Sspcpsr {
            #[inline(always)]
            fn default() -> Sspcpsr {
                Sspcpsr(0)
            }
        }
        impl core::fmt::Debug for Sspcpsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspcpsr")
                    .field("cpsdvsr", &self.cpsdvsr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspcpsr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sspcpsr {{ cpsdvsr: {=u8:?} }}", self.cpsdvsr())
            }
        }
        #[doc = "Control register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspcr0(pub u32);
        impl Sspcr0 {
            #[doc = "Data Size Select"]
            #[must_use]
            #[inline(always)]
            pub const fn dss(&self) -> super::vals::Dss {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Dss::from_bits(val as u8)
            }
            #[doc = "Data Size Select"]
            #[inline(always)]
            pub const fn set_dss(&mut self, val: super::vals::Dss) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[doc = "Frame format"]
            #[must_use]
            #[inline(always)]
            pub const fn frf(&self) -> super::vals::Frf {
                let val = (self.0 >> 4usize) & 0x03;
                super::vals::Frf::from_bits(val as u8)
            }
            #[doc = "Frame format"]
            #[inline(always)]
            pub const fn set_frf(&mut self, val: super::vals::Frf) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
            }
            #[doc = "SSPCLKOUT polarity"]
            #[must_use]
            #[inline(always)]
            pub const fn spo(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SSPCLKOUT polarity"]
            #[inline(always)]
            pub const fn set_spo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "SSPCLKOUT phase"]
            #[must_use]
            #[inline(always)]
            pub const fn sph(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "SSPCLKOUT phase"]
            #[inline(always)]
            pub const fn set_sph(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Serial clock rate"]
            #[must_use]
            #[inline(always)]
            pub const fn scr(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Serial clock rate"]
            #[inline(always)]
            pub const fn set_scr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Sspcr0 {
            #[inline(always)]
            fn default() -> Sspcr0 {
                Sspcr0(0)
            }
        }
        impl core::fmt::Debug for Sspcr0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspcr0")
                    .field("dss", &self.dss())
                    .field("frf", &self.frf())
                    .field("spo", &self.spo())
                    .field("sph", &self.sph())
                    .field("scr", &self.scr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspcr0 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Sspcr0 {{ dss: {:?}, frf: {:?}, spo: {=bool:?}, sph: {=bool:?}, scr: {=u8:?} }}" , self . dss () , self . frf () , self . spo () , self . sph () , self . scr ())
            }
        }
        #[doc = "Control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspcr1(pub u32);
        impl Sspcr1 {
            #[doc = "Loop back mode"]
            #[must_use]
            #[inline(always)]
            pub const fn lbm(&self) -> super::vals::Lbm {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Lbm::from_bits(val as u8)
            }
            #[doc = "Loop back mode"]
            #[inline(always)]
            pub const fn set_lbm(&mut self, val: super::vals::Lbm) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Synchronous serial port enable"]
            #[must_use]
            #[inline(always)]
            pub const fn sse(&self) -> super::vals::Sse {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Sse::from_bits(val as u8)
            }
            #[doc = "Synchronous serial port enable"]
            #[inline(always)]
            pub const fn set_sse(&mut self, val: super::vals::Sse) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Master or slave mode select"]
            #[must_use]
            #[inline(always)]
            pub const fn ms(&self) -> super::vals::Ms {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Ms::from_bits(val as u8)
            }
            #[doc = "Master or slave mode select"]
            #[inline(always)]
            pub const fn set_ms(&mut self, val: super::vals::Ms) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Slave-mode output disable"]
            #[must_use]
            #[inline(always)]
            pub const fn sod(&self) -> super::vals::Sod {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Sod::from_bits(val as u8)
            }
            #[doc = "Slave-mode output disable"]
            #[inline(always)]
            pub const fn set_sod(&mut self, val: super::vals::Sod) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sspcr1 {
            #[inline(always)]
            fn default() -> Sspcr1 {
                Sspcr1(0)
            }
        }
        impl core::fmt::Debug for Sspcr1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspcr1")
                    .field("lbm", &self.lbm())
                    .field("sse", &self.sse())
                    .field("ms", &self.ms())
                    .field("sod", &self.sod())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspcr1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspcr1 {{ lbm: {:?}, sse: {:?}, ms: {:?}, sod: {:?} }}",
                    self.lbm(),
                    self.sse(),
                    self.ms(),
                    self.sod()
                )
            }
        }
        #[doc = "DMA control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspdmacr(pub u32);
        impl Sspdmacr {
            #[doc = "Receive DMA enable"]
            #[must_use]
            #[inline(always)]
            pub const fn rxdmae(&self) -> super::vals::Rxdmae {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rxdmae::from_bits(val as u8)
            }
            #[doc = "Receive DMA enable"]
            #[inline(always)]
            pub const fn set_rxdmae(&mut self, val: super::vals::Rxdmae) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit DMA enable"]
            #[must_use]
            #[inline(always)]
            pub const fn txdmae(&self) -> super::vals::Txdmae {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Txdmae::from_bits(val as u8)
            }
            #[doc = "Transmit DMA enable"]
            #[inline(always)]
            pub const fn set_txdmae(&mut self, val: super::vals::Txdmae) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Sspdmacr {
            #[inline(always)]
            fn default() -> Sspdmacr {
                Sspdmacr(0)
            }
        }
        impl core::fmt::Debug for Sspdmacr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspdmacr")
                    .field("rxdmae", &self.rxdmae())
                    .field("txdmae", &self.txdmae())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspdmacr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspdmacr {{ rxdmae: {:?}, txdmae: {:?} }}",
                    self.rxdmae(),
                    self.txdmae()
                )
            }
        }
        #[doc = "Data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspdr(pub u32);
        impl Sspdr {
            #[doc = "Transmit/Receive FIFO"]
            #[must_use]
            #[inline(always)]
            pub const fn data(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Transmit/Receive FIFO"]
            #[inline(always)]
            pub const fn set_data(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Sspdr {
            #[inline(always)]
            fn default() -> Sspdr {
                Sspdr(0)
            }
        }
        impl core::fmt::Debug for Sspdr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspdr").field("data", &self.data()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspdr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sspdr {{ data: {=u16:?} }}", self.data())
            }
        }
        #[doc = "Interrupt clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspicr(pub u32);
        impl Sspicr {
            #[doc = "Clear the SSPRORINTR interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn roric(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Clear the SSPRORINTR interrupt"]
            #[inline(always)]
            pub const fn set_roric(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Clear the SSPRTINTR interrupt"]
            #[must_use]
            #[inline(always)]
            pub const fn rtic(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Clear the SSPRTINTR interrupt"]
            #[inline(always)]
            pub const fn set_rtic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Sspicr {
            #[inline(always)]
            fn default() -> Sspicr {
                Sspicr(0)
            }
        }
        impl core::fmt::Debug for Sspicr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspicr")
                    .field("roric", &self.roric())
                    .field("rtic", &self.rtic())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspicr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspicr {{ roric: {=bool:?}, rtic: {=bool:?} }}",
                    self.roric(),
                    self.rtic()
                )
            }
        }
        #[doc = "Interrupt mask set or clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspimsc(pub u32);
        impl Sspimsc {
            #[doc = "Receive overrun interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rorim(&self) -> super::vals::Rorim {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rorim::from_bits(val as u8)
            }
            #[doc = "Receive overrun interrupt mask"]
            #[inline(always)]
            pub const fn set_rorim(&mut self, val: super::vals::Rorim) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Receive timeout interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rtim(&self) -> super::vals::Rtim {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rtim::from_bits(val as u8)
            }
            #[doc = "Receive timeout interrupt mask"]
            #[inline(always)]
            pub const fn set_rtim(&mut self, val: super::vals::Rtim) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxim(&self) -> super::vals::Rxim {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rxim::from_bits(val as u8)
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_rxim(&mut self, val: super::vals::Rxim) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn txim(&self) -> super::vals::Txim {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Txim::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_txim(&mut self, val: super::vals::Txim) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sspimsc {
            #[inline(always)]
            fn default() -> Sspimsc {
                Sspimsc(0)
            }
        }
        impl core::fmt::Debug for Sspimsc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspimsc")
                    .field("rorim", &self.rorim())
                    .field("rtim", &self.rtim())
                    .field("rxim", &self.rxim())
                    .field("txim", &self.txim())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspimsc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspimsc {{ rorim: {:?}, rtim: {:?}, rxim: {:?}, txim: {:?} }}",
                    self.rorim(),
                    self.rtim(),
                    self.rxim(),
                    self.txim()
                )
            }
        }
        #[doc = "Masked interrupt status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspmis(pub u32);
        impl Sspmis {
            #[doc = "Receive overrun interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rormis(&self) -> super::vals::Rormis {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rormis::from_bits(val as u8)
            }
            #[doc = "Receive overrun interrupt mask"]
            #[inline(always)]
            pub const fn set_rormis(&mut self, val: super::vals::Rormis) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Receive timeout interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rtmis(&self) -> super::vals::Rtmis {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rtmis::from_bits(val as u8)
            }
            #[doc = "Receive timeout interrupt mask"]
            #[inline(always)]
            pub const fn set_rtmis(&mut self, val: super::vals::Rtmis) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxmis(&self) -> super::vals::Rxmis {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rxmis::from_bits(val as u8)
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_rxmis(&mut self, val: super::vals::Rxmis) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn txmis(&self) -> super::vals::Txmis {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Txmis::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_txmis(&mut self, val: super::vals::Txmis) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sspmis {
            #[inline(always)]
            fn default() -> Sspmis {
                Sspmis(0)
            }
        }
        impl core::fmt::Debug for Sspmis {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspmis")
                    .field("rormis", &self.rormis())
                    .field("rtmis", &self.rtmis())
                    .field("rxmis", &self.rxmis())
                    .field("txmis", &self.txmis())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspmis {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspmis {{ rormis: {:?}, rtmis: {:?}, rxmis: {:?}, txmis: {:?} }}",
                    self.rormis(),
                    self.rtmis(),
                    self.rxmis(),
                    self.txmis()
                )
            }
        }
        #[doc = "Raw interrupt status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspris(pub u32);
        impl Sspris {
            #[doc = "Receive overrun interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rorris(&self) -> super::vals::Rorris {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rorris::from_bits(val as u8)
            }
            #[doc = "Receive overrun interrupt mask"]
            #[inline(always)]
            pub const fn set_rorris(&mut self, val: super::vals::Rorris) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Receive timeout interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rtris(&self) -> super::vals::Rtris {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rtris::from_bits(val as u8)
            }
            #[doc = "Receive timeout interrupt mask"]
            #[inline(always)]
            pub const fn set_rtris(&mut self, val: super::vals::Rtris) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxris(&self) -> super::vals::Rxris {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rxris::from_bits(val as u8)
            }
            #[doc = "Receive FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_rxris(&mut self, val: super::vals::Rxris) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn txris(&self) -> super::vals::Txris {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Txris::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO interrupt mask"]
            #[inline(always)]
            pub const fn set_txris(&mut self, val: super::vals::Txris) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sspris {
            #[inline(always)]
            fn default() -> Sspris {
                Sspris(0)
            }
        }
        impl core::fmt::Debug for Sspris {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspris")
                    .field("rorris", &self.rorris())
                    .field("rtris", &self.rtris())
                    .field("rxris", &self.rxris())
                    .field("txris", &self.txris())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspris {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspris {{ rorris: {:?}, rtris: {:?}, rxris: {:?}, txris: {:?} }}",
                    self.rorris(),
                    self.rtris(),
                    self.rxris(),
                    self.txris()
                )
            }
        }
        #[doc = "Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sspsr(pub u32);
        impl Sspsr {
            #[doc = "Transmit FIFO empty"]
            #[must_use]
            #[inline(always)]
            pub const fn tfe(&self) -> super::vals::Tfe {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Tfe::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO empty"]
            #[inline(always)]
            pub const fn set_tfe(&mut self, val: super::vals::Tfe) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit FIFO not full"]
            #[must_use]
            #[inline(always)]
            pub const fn tnf(&self) -> super::vals::Tnf {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Tnf::from_bits(val as u8)
            }
            #[doc = "Transmit FIFO not full"]
            #[inline(always)]
            pub const fn set_tnf(&mut self, val: super::vals::Tnf) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Rceive FIFIO not empty"]
            #[must_use]
            #[inline(always)]
            pub const fn rne(&self) -> super::vals::Rne {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rne::from_bits(val as u8)
            }
            #[doc = "Rceive FIFIO not empty"]
            #[inline(always)]
            pub const fn set_rne(&mut self, val: super::vals::Rne) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Receive FIFO full"]
            #[must_use]
            #[inline(always)]
            pub const fn rff(&self) -> super::vals::Rff {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Rff::from_bits(val as u8)
            }
            #[doc = "Receive FIFO full"]
            #[inline(always)]
            pub const fn set_rff(&mut self, val: super::vals::Rff) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "SSP busy flag"]
            #[must_use]
            #[inline(always)]
            pub const fn bsy(&self) -> super::vals::Bsy {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Bsy::from_bits(val as u8)
            }
            #[doc = "SSP busy flag"]
            #[inline(always)]
            pub const fn set_bsy(&mut self, val: super::vals::Bsy) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Sspsr {
            #[inline(always)]
            fn default() -> Sspsr {
                Sspsr(0)
            }
        }
        impl core::fmt::Debug for Sspsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sspsr")
                    .field("tfe", &self.tfe())
                    .field("tnf", &self.tnf())
                    .field("rne", &self.rne())
                    .field("rff", &self.rff())
                    .field("bsy", &self.bsy())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sspsr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Sspsr {{ tfe: {:?}, tnf: {:?}, rne: {:?}, rff: {:?}, bsy: {:?} }}",
                    self.tfe(),
                    self.tnf(),
                    self.rne(),
                    self.rff(),
                    self.bsy()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Bsy {
            #[doc = "SSP is idle"]
            IDLE = 0x0,
            #[doc = "SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty"]
            BUSY = 0x01,
        }
        impl Bsy {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Bsy {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Bsy {
            #[inline(always)]
            fn from(val: u8) -> Bsy {
                Bsy::from_bits(val)
            }
        }
        impl From<Bsy> for u8 {
            #[inline(always)]
            fn from(val: Bsy) -> u8 {
                Bsy::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dss {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "4bit data"]
            _4BIT = 0x03,
            #[doc = "5bit data"]
            _5BIT = 0x04,
            #[doc = "6bit data"]
            _6BIT = 0x05,
            #[doc = "7bit data"]
            _7BIT = 0x06,
            #[doc = "8bit data"]
            _8BIT = 0x07,
            #[doc = "9bit data"]
            _9BIT = 0x08,
            #[doc = "10bit data"]
            _10BIT = 0x09,
            #[doc = "11bit data"]
            _11BIT = 0x0a,
            #[doc = "12bit data"]
            _12BIT = 0x0b,
            #[doc = "13bit data"]
            _13BIT = 0x0c,
            #[doc = "14bit data"]
            _14BIT = 0x0d,
            #[doc = "15bit data"]
            _15BIT = 0x0e,
            #[doc = "16bit data"]
            _16BIT = 0x0f,
        }
        impl Dss {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dss {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dss {
            #[inline(always)]
            fn from(val: u8) -> Dss {
                Dss::from_bits(val)
            }
        }
        impl From<Dss> for u8 {
            #[inline(always)]
            fn from(val: Dss) -> u8 {
                Dss::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Frf {
            #[doc = "Motorola SPI frame format"]
            MOTOROLA = 0x0,
            #[doc = "TI synchronous serial frame format"]
            TI = 0x01,
            #[doc = "National Microwire frame format"]
            NM = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Frf {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Frf {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Frf {
            #[inline(always)]
            fn from(val: u8) -> Frf {
                Frf::from_bits(val)
            }
        }
        impl From<Frf> for u8 {
            #[inline(always)]
            fn from(val: Frf) -> u8 {
                Frf::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lbm {
            #[doc = "Normal serial port operation enabled"]
            DISABLED = 0x0,
            #[doc = "Output of transmit serial shifter is connected to input of recieve serial shifter internally"]
            ENABLED = 0x01,
        }
        impl Lbm {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lbm {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lbm {
            #[inline(always)]
            fn from(val: u8) -> Lbm {
                Lbm::from_bits(val)
            }
        }
        impl From<Lbm> for u8 {
            #[inline(always)]
            fn from(val: Lbm) -> u8 {
                Lbm::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ms {
            #[doc = "Master mode"]
            MASTER = 0x0,
            #[doc = "Slave mode"]
            SLAVE = 0x01,
        }
        impl Ms {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ms {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ms {
            #[inline(always)]
            fn from(val: u8) -> Ms {
                Ms::from_bits(val)
            }
        }
        impl From<Ms> for u8 {
            #[inline(always)]
            fn from(val: Ms) -> u8 {
                Ms::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rff {
            #[doc = "Receive FIFO is not full"]
            NOTFULL = 0x0,
            #[doc = "Recieve FIFO is full"]
            FULL = 0x01,
        }
        impl Rff {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rff {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rff {
            #[inline(always)]
            fn from(val: u8) -> Rff {
                Rff::from_bits(val)
            }
        }
        impl From<Rff> for u8 {
            #[inline(always)]
            fn from(val: Rff) -> u8 {
                Rff::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rne {
            #[doc = "Receive FIFO is empty"]
            EMPTY = 0x0,
            #[doc = "Receive FIFO is not empty"]
            NOTEMPTY = 0x01,
        }
        impl Rne {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rne {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rne {
            #[inline(always)]
            fn from(val: u8) -> Rne {
                Rne::from_bits(val)
            }
        }
        impl From<Rne> for u8 {
            #[inline(always)]
            fn from(val: Rne) -> u8 {
                Rne::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rorim {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rorim {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rorim {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rorim {
            #[inline(always)]
            fn from(val: u8) -> Rorim {
                Rorim::from_bits(val)
            }
        }
        impl From<Rorim> for u8 {
            #[inline(always)]
            fn from(val: Rorim) -> u8 {
                Rorim::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rormis {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rormis {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rormis {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rormis {
            #[inline(always)]
            fn from(val: u8) -> Rormis {
                Rormis::from_bits(val)
            }
        }
        impl From<Rormis> for u8 {
            #[inline(always)]
            fn from(val: Rormis) -> u8 {
                Rormis::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rorris {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rorris {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rorris {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rorris {
            #[inline(always)]
            fn from(val: u8) -> Rorris {
                Rorris::from_bits(val)
            }
        }
        impl From<Rorris> for u8 {
            #[inline(always)]
            fn from(val: Rorris) -> u8 {
                Rorris::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtim {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rtim {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtim {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtim {
            #[inline(always)]
            fn from(val: u8) -> Rtim {
                Rtim::from_bits(val)
            }
        }
        impl From<Rtim> for u8 {
            #[inline(always)]
            fn from(val: Rtim) -> u8 {
                Rtim::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtmis {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rtmis {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtmis {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtmis {
            #[inline(always)]
            fn from(val: u8) -> Rtmis {
                Rtmis::from_bits(val)
            }
        }
        impl From<Rtmis> for u8 {
            #[inline(always)]
            fn from(val: Rtmis) -> u8 {
                Rtmis::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtris {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rtris {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtris {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtris {
            #[inline(always)]
            fn from(val: u8) -> Rtris {
                Rtris::from_bits(val)
            }
        }
        impl From<Rtris> for u8 {
            #[inline(always)]
            fn from(val: Rtris) -> u8 {
                Rtris::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxdmae {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Rxdmae {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxdmae {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxdmae {
            #[inline(always)]
            fn from(val: u8) -> Rxdmae {
                Rxdmae::from_bits(val)
            }
        }
        impl From<Rxdmae> for u8 {
            #[inline(always)]
            fn from(val: Rxdmae) -> u8 {
                Rxdmae::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxim {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rxim {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxim {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxim {
            #[inline(always)]
            fn from(val: u8) -> Rxim {
                Rxim::from_bits(val)
            }
        }
        impl From<Rxim> for u8 {
            #[inline(always)]
            fn from(val: Rxim) -> u8 {
                Rxim::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxmis {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rxmis {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxmis {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxmis {
            #[inline(always)]
            fn from(val: u8) -> Rxmis {
                Rxmis::from_bits(val)
            }
        }
        impl From<Rxmis> for u8 {
            #[inline(always)]
            fn from(val: Rxmis) -> u8 {
                Rxmis::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxris {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Rxris {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxris {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxris {
            #[inline(always)]
            fn from(val: u8) -> Rxris {
                Rxris::from_bits(val)
            }
        }
        impl From<Rxris> for u8 {
            #[inline(always)]
            fn from(val: Rxris) -> u8 {
                Rxris::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sod {
            #[doc = "SSP can drive the SSPTXD output in slave mode"]
            ENABLE = 0x0,
            #[doc = "SSP must not drive the SSPTXD output in slave mode"]
            DISABLE = 0x01,
        }
        impl Sod {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sod {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sod {
            #[inline(always)]
            fn from(val: u8) -> Sod {
                Sod::from_bits(val)
            }
        }
        impl From<Sod> for u8 {
            #[inline(always)]
            fn from(val: Sod) -> u8 {
                Sod::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sse {
            #[doc = "SSP operation disabled"]
            DISABLED = 0x0,
            #[doc = "SSP operation enabled"]
            ENABLED = 0x01,
        }
        impl Sse {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sse {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sse {
            #[inline(always)]
            fn from(val: u8) -> Sse {
                Sse::from_bits(val)
            }
        }
        impl From<Sse> for u8 {
            #[inline(always)]
            fn from(val: Sse) -> u8 {
                Sse::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tfe {
            #[doc = "Transmit FIFO is not empty"]
            NOTEMPTY = 0x0,
            #[doc = "Transmit FIFO is empty"]
            EMPTY = 0x01,
        }
        impl Tfe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tfe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tfe {
            #[inline(always)]
            fn from(val: u8) -> Tfe {
                Tfe::from_bits(val)
            }
        }
        impl From<Tfe> for u8 {
            #[inline(always)]
            fn from(val: Tfe) -> u8 {
                Tfe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tnf {
            #[doc = "Transmit FIFO is full"]
            FULL = 0x0,
            #[doc = "Transmit FIFO is not full"]
            NOTFULL = 0x01,
        }
        impl Tnf {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tnf {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tnf {
            #[inline(always)]
            fn from(val: u8) -> Tnf {
                Tnf::from_bits(val)
            }
        }
        impl From<Tnf> for u8 {
            #[inline(always)]
            fn from(val: Tnf) -> u8 {
                Tnf::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txdmae {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Txdmae {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txdmae {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txdmae {
            #[inline(always)]
            fn from(val: u8) -> Txdmae {
                Txdmae::from_bits(val)
            }
        }
        impl From<Txdmae> for u8 {
            #[inline(always)]
            fn from(val: Txdmae) -> u8 {
                Txdmae::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txim {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Txim {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txim {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txim {
            #[inline(always)]
            fn from(val: u8) -> Txim {
                Txim::from_bits(val)
            }
        }
        impl From<Txim> for u8 {
            #[inline(always)]
            fn from(val: Txim) -> u8 {
                Txim::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txmis {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Txmis {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txmis {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txmis {
            #[inline(always)]
            fn from(val: u8) -> Txmis {
                Txmis::from_bits(val)
            }
        }
        impl From<Txmis> for u8 {
            #[inline(always)]
            fn from(val: Txmis) -> u8 {
                Txmis::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txris {
            #[doc = "Masked"]
            MASKED = 0x0,
            #[doc = "Not masked"]
            NOTMASKED = 0x01,
        }
        impl Txris {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txris {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txris {
            #[inline(always)]
            fn from(val: u8) -> Txris {
                Txris::from_bits(val)
            }
        }
        impl From<Txris> for u8 {
            #[inline(always)]
            fn from(val: Txris) -> u8 {
                Txris::to_bits(val)
            }
        }
    }
}
pub mod uart1 {
    #[doc = "UART"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uart1 {
        ptr: *mut u8,
    }
    unsafe impl Send for Uart1 {}
    unsafe impl Sync for Uart1 {}
    impl Uart1 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[inline(always)]
        pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Receive Status and Clear Register"]
        #[inline(always)]
        pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Flags Register"]
        #[inline(always)]
        pub const fn fr(self) -> crate::common::Reg<regs::Fr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "IrDA Low-Power Counter Register"]
        #[inline(always)]
        pub const fn ilpr(self) -> crate::common::Reg<regs::Ilpr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "The integer part of the baud rate divisor"]
        #[inline(always)]
        pub const fn ibrd(self) -> crate::common::Reg<regs::Ibrd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "The fractional part of the baud rate divisor"]
        #[inline(always)]
        pub const fn fbrd(self) -> crate::common::Reg<regs::Fbrd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "Line Control Register"]
        #[inline(always)]
        pub const fn lcr_h(self) -> crate::common::Reg<regs::LcrH, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
        }
        #[doc = "Control Register"]
        #[inline(always)]
        pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[doc = "Interrupt FIFO Level Select Register"]
        #[inline(always)]
        pub const fn ifls(self) -> crate::common::Reg<regs::Ifls, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
        }
        #[doc = "Interrupt Mask Set and Clear Register"]
        #[inline(always)]
        pub const fn imsc(self) -> crate::common::Reg<regs::Imsc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
        }
        #[doc = "Raw Interrupt Status Register"]
        #[inline(always)]
        pub const fn ris(self) -> crate::common::Reg<regs::Ris, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
        }
        #[doc = "Masked Interrupt Status Register"]
        #[inline(always)]
        pub const fn mis(self) -> crate::common::Reg<regs::Mis, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
        }
        #[doc = "Interrupt Clear Register"]
        #[inline(always)]
        pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
        }
        #[doc = "DMA Control Regsiter"]
        #[inline(always)]
        pub const fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "UART enable"]
            #[must_use]
            #[inline(always)]
            pub const fn uarten(&self) -> super::vals::Uarten {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Uarten::from_bits(val as u8)
            }
            #[doc = "UART enable"]
            #[inline(always)]
            pub const fn set_uarten(&mut self, val: super::vals::Uarten) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "SIR enable"]
            #[must_use]
            #[inline(always)]
            pub const fn siren(&self) -> super::vals::Siren {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Siren::from_bits(val as u8)
            }
            #[doc = "SIR enable"]
            #[inline(always)]
            pub const fn set_siren(&mut self, val: super::vals::Siren) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "SIR low-power IrDA"]
            #[must_use]
            #[inline(always)]
            pub const fn sirlp(&self) -> super::vals::Sirlp {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Sirlp::from_bits(val as u8)
            }
            #[doc = "SIR low-power IrDA"]
            #[inline(always)]
            pub const fn set_sirlp(&mut self, val: super::vals::Sirlp) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Loopback enable"]
            #[must_use]
            #[inline(always)]
            pub const fn lbe(&self) -> super::vals::Lbe {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Lbe::from_bits(val as u8)
            }
            #[doc = "Loopback enable"]
            #[inline(always)]
            pub const fn set_lbe(&mut self, val: super::vals::Lbe) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
            }
            #[doc = "Transmit enable"]
            #[must_use]
            #[inline(always)]
            pub const fn txe(&self) -> super::vals::Txe {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Txe::from_bits(val as u8)
            }
            #[doc = "Transmit enable"]
            #[inline(always)]
            pub const fn set_txe(&mut self, val: super::vals::Txe) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
            #[doc = "Receive enable"]
            #[must_use]
            #[inline(always)]
            pub const fn rxe(&self) -> super::vals::Rxe {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Rxe::from_bits(val as u8)
            }
            #[doc = "Receive enable"]
            #[inline(always)]
            pub const fn set_rxe(&mut self, val: super::vals::Rxe) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
            #[doc = "Data transmit ready"]
            #[must_use]
            #[inline(always)]
            pub const fn dtr(&self) -> super::vals::Dtr {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Dtr::from_bits(val as u8)
            }
            #[doc = "Data transmit ready"]
            #[inline(always)]
            pub const fn set_dtr(&mut self, val: super::vals::Dtr) {
                self.0 =
                    (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "Request to send"]
            #[must_use]
            #[inline(always)]
            pub const fn rts(&self) -> super::vals::Rts {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Rts::from_bits(val as u8)
            }
            #[doc = "Request to send"]
            #[inline(always)]
            pub const fn set_rts(&mut self, val: super::vals::Rts) {
                self.0 =
                    (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
            #[doc = "nUARTOut1 modem status"]
            #[must_use]
            #[inline(always)]
            pub const fn out1(&self) -> super::vals::Out1 {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Out1::from_bits(val as u8)
            }
            #[doc = "nUARTOut1 modem status"]
            #[inline(always)]
            pub const fn set_out1(&mut self, val: super::vals::Out1) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
            #[doc = "nUARTOut2 modem status"]
            #[must_use]
            #[inline(always)]
            pub const fn out2(&self) -> super::vals::Out2 {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Out2::from_bits(val as u8)
            }
            #[doc = "nUARTOut2 modem status"]
            #[inline(always)]
            pub const fn set_out2(&mut self, val: super::vals::Out2) {
                self.0 =
                    (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
            }
            #[doc = "RTS hardware flow control enable"]
            #[must_use]
            #[inline(always)]
            pub const fn rtsen(&self) -> super::vals::Rtsen {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Rtsen::from_bits(val as u8)
            }
            #[doc = "RTS hardware flow control enable"]
            #[inline(always)]
            pub const fn set_rtsen(&mut self, val: super::vals::Rtsen) {
                self.0 =
                    (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
            }
            #[doc = "CTS hardware flow control enable"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsen(&self) -> super::vals::Ctsen {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Ctsen::from_bits(val as u8)
            }
            #[doc = "CTS hardware flow control enable"]
            #[inline(always)]
            pub const fn set_ctsen(&mut self, val: super::vals::Ctsen) {
                self.0 =
                    (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Cr {
            #[inline(always)]
            fn default() -> Cr {
                Cr(0)
            }
        }
        impl core::fmt::Debug for Cr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cr")
                    .field("uarten", &self.uarten())
                    .field("siren", &self.siren())
                    .field("sirlp", &self.sirlp())
                    .field("lbe", &self.lbe())
                    .field("txe", &self.txe())
                    .field("rxe", &self.rxe())
                    .field("dtr", &self.dtr())
                    .field("rts", &self.rts())
                    .field("out1", &self.out1())
                    .field("out2", &self.out2())
                    .field("rtsen", &self.rtsen())
                    .field("ctsen", &self.ctsen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cr {{ uarten: {:?}, siren: {:?}, sirlp: {:?}, lbe: {:?}, txe: {:?}, rxe: {:?}, dtr: {:?}, rts: {:?}, out1: {:?}, out2: {:?}, rtsen: {:?}, ctsen: {:?} }}" , self . uarten () , self . siren () , self . sirlp () , self . lbe () , self . txe () , self . rxe () , self . dtr () , self . rts () , self . out1 () , self . out2 () , self . rtsen () , self . ctsen ())
            }
        }
        #[doc = "DMA Control Regsiter"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacr(pub u32);
        impl Dmacr {
            #[doc = "Receive DMA enable"]
            #[must_use]
            #[inline(always)]
            pub const fn rxdmae(&self) -> super::vals::Rxdmae {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rxdmae::from_bits(val as u8)
            }
            #[doc = "Receive DMA enable"]
            #[inline(always)]
            pub const fn set_rxdmae(&mut self, val: super::vals::Rxdmae) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit DMA enable"]
            #[must_use]
            #[inline(always)]
            pub const fn txdmae(&self) -> super::vals::Txdmae {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Txdmae::from_bits(val as u8)
            }
            #[doc = "Transmit DMA enable"]
            #[inline(always)]
            pub const fn set_txdmae(&mut self, val: super::vals::Txdmae) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "DMA on error"]
            #[must_use]
            #[inline(always)]
            pub const fn dmaonerr(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "DMA on error"]
            #[inline(always)]
            pub const fn set_dmaonerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Dmacr {
            #[inline(always)]
            fn default() -> Dmacr {
                Dmacr(0)
            }
        }
        impl core::fmt::Debug for Dmacr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacr")
                    .field("rxdmae", &self.rxdmae())
                    .field("txdmae", &self.txdmae())
                    .field("dmaonerr", &self.dmaonerr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacr {{ rxdmae: {:?}, txdmae: {:?}, dmaonerr: {=bool:?} }}",
                    self.rxdmae(),
                    self.txdmae(),
                    self.dmaonerr()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "Data"]
            #[must_use]
            #[inline(always)]
            pub const fn data(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Data"]
            #[inline(always)]
            pub const fn set_data(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Framing Error"]
            #[must_use]
            #[inline(always)]
            pub const fn fe(&self) -> super::vals::Fe {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Fe::from_bits(val as u8)
            }
            #[doc = "Framing Error"]
            #[inline(always)]
            pub const fn set_fe(&mut self, val: super::vals::Fe) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
            #[doc = "Parity Error"]
            #[must_use]
            #[inline(always)]
            pub const fn pe(&self) -> super::vals::Pe {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Pe::from_bits(val as u8)
            }
            #[doc = "Parity Error"]
            #[inline(always)]
            pub const fn set_pe(&mut self, val: super::vals::Pe) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
            #[doc = "Break Error"]
            #[must_use]
            #[inline(always)]
            pub const fn be(&self) -> super::vals::Be {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Be::from_bits(val as u8)
            }
            #[doc = "Break Error"]
            #[inline(always)]
            pub const fn set_be(&mut self, val: super::vals::Be) {
                self.0 =
                    (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "Overrun Error"]
            #[must_use]
            #[inline(always)]
            pub const fn oe(&self) -> super::vals::Oe {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Oe::from_bits(val as u8)
            }
            #[doc = "Overrun Error"]
            #[inline(always)]
            pub const fn set_oe(&mut self, val: super::vals::Oe) {
                self.0 =
                    (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
        }
        impl Default for Dr {
            #[inline(always)]
            fn default() -> Dr {
                Dr(0)
            }
        }
        impl core::fmt::Debug for Dr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dr")
                    .field("data", &self.data())
                    .field("fe", &self.fe())
                    .field("pe", &self.pe())
                    .field("be", &self.be())
                    .field("oe", &self.oe())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dr {{ data: {=u8:?}, fe: {:?}, pe: {:?}, be: {:?}, oe: {:?} }}",
                    self.data(),
                    self.fe(),
                    self.pe(),
                    self.be(),
                    self.oe()
                )
            }
        }
        #[doc = "The fractional part of the baud rate divisor"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fbrd(pub u32);
        impl Fbrd {
            #[doc = "The fractional baud rate divisor"]
            #[must_use]
            #[inline(always)]
            pub const fn baud_divfrac(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x3f;
                val as u8
            }
            #[doc = "The fractional baud rate divisor"]
            #[inline(always)]
            pub const fn set_baud_divfrac(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
            }
        }
        impl Default for Fbrd {
            #[inline(always)]
            fn default() -> Fbrd {
                Fbrd(0)
            }
        }
        impl core::fmt::Debug for Fbrd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Fbrd")
                    .field("baud_divfrac", &self.baud_divfrac())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Fbrd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Fbrd {{ baud_divfrac: {=u8:?} }}", self.baud_divfrac())
            }
        }
        #[doc = "Flags Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fr(pub u32);
        impl Fr {
            #[doc = "Clear to send"]
            #[must_use]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Clear to send"]
            #[inline(always)]
            pub const fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Data set ready"]
            #[must_use]
            #[inline(always)]
            pub const fn dsr(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Data set ready"]
            #[inline(always)]
            pub const fn set_dsr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Data carrier detect"]
            #[must_use]
            #[inline(always)]
            pub const fn dcd(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Data carrier detect"]
            #[inline(always)]
            pub const fn set_dcd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "UART busy"]
            #[must_use]
            #[inline(always)]
            pub const fn busy(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "UART busy"]
            #[inline(always)]
            pub const fn set_busy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive FIFO empty"]
            #[must_use]
            #[inline(always)]
            pub const fn rxfe(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive FIFO empty"]
            #[inline(always)]
            pub const fn set_rxfe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit FIFO full"]
            #[must_use]
            #[inline(always)]
            pub const fn txff(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit FIFO full"]
            #[inline(always)]
            pub const fn set_txff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive FIFO full"]
            #[must_use]
            #[inline(always)]
            pub const fn rxff(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive FIFO full"]
            #[inline(always)]
            pub const fn set_rxff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Transmit FIFO empty"]
            #[must_use]
            #[inline(always)]
            pub const fn txfe(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit FIFO empty"]
            #[inline(always)]
            pub const fn set_txfe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Ring Indicator"]
            #[must_use]
            #[inline(always)]
            pub const fn ri(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Ring Indicator"]
            #[inline(always)]
            pub const fn set_ri(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Fr {
            #[inline(always)]
            fn default() -> Fr {
                Fr(0)
            }
        }
        impl core::fmt::Debug for Fr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Fr")
                    .field("cts", &self.cts())
                    .field("dsr", &self.dsr())
                    .field("dcd", &self.dcd())
                    .field("busy", &self.busy())
                    .field("rxfe", &self.rxfe())
                    .field("txff", &self.txff())
                    .field("rxff", &self.rxff())
                    .field("txfe", &self.txfe())
                    .field("ri", &self.ri())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Fr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Fr {{ cts: {=bool:?}, dsr: {=bool:?}, dcd: {=bool:?}, busy: {=bool:?}, rxfe: {=bool:?}, txff: {=bool:?}, rxff: {=bool:?}, txfe: {=bool:?}, ri: {=bool:?} }}" , self . cts () , self . dsr () , self . dcd () , self . busy () , self . rxfe () , self . txff () , self . rxff () , self . txfe () , self . ri ())
            }
        }
        #[doc = "The integer part of the baud rate divisor"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ibrd(pub u32);
        impl Ibrd {
            #[doc = "The integer baud rate divisor"]
            #[must_use]
            #[inline(always)]
            pub const fn baud_divint(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "The integer baud rate divisor"]
            #[inline(always)]
            pub const fn set_baud_divint(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Ibrd {
            #[inline(always)]
            fn default() -> Ibrd {
                Ibrd(0)
            }
        }
        impl core::fmt::Debug for Ibrd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ibrd")
                    .field("baud_divint", &self.baud_divint())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ibrd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ibrd {{ baud_divint: {=u16:?} }}", self.baud_divint())
            }
        }
        #[doc = "Interrupt Clear Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Icr(pub u32);
        impl Icr {
            #[doc = "nUARTRI modem interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn rimic(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTRI modem interrupt clear"]
            #[inline(always)]
            pub const fn set_rimic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "nUARTCTS modem interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsmic(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTCTS modem interrupt clear"]
            #[inline(always)]
            pub const fn set_ctsmic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "nUARTDCD modem interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn dcdmic(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDCD modem interrupt clear"]
            #[inline(always)]
            pub const fn set_dcdmic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "nUARTDSR modem interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn dsrmic(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDSR modem interrupt clear"]
            #[inline(always)]
            pub const fn set_dsrmic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn rxic(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive interrupt clear"]
            #[inline(always)]
            pub const fn set_rxic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn txic(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit interrupt clear"]
            #[inline(always)]
            pub const fn set_txic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive timeout interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn rtic(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive timeout interrupt clear"]
            #[inline(always)]
            pub const fn set_rtic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Framing error interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn feic(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error interrupt clear"]
            #[inline(always)]
            pub const fn set_feic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Parity error interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn peic(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error interrupt clear"]
            #[inline(always)]
            pub const fn set_peic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Break error interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn beic(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Break error interrupt clear"]
            #[inline(always)]
            pub const fn set_beic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun error interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn oeic(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error interrupt clear"]
            #[inline(always)]
            pub const fn set_oeic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Icr {
            #[inline(always)]
            fn default() -> Icr {
                Icr(0)
            }
        }
        impl core::fmt::Debug for Icr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Icr")
                    .field("rimic", &self.rimic())
                    .field("ctsmic", &self.ctsmic())
                    .field("dcdmic", &self.dcdmic())
                    .field("dsrmic", &self.dsrmic())
                    .field("rxic", &self.rxic())
                    .field("txic", &self.txic())
                    .field("rtic", &self.rtic())
                    .field("feic", &self.feic())
                    .field("peic", &self.peic())
                    .field("beic", &self.beic())
                    .field("oeic", &self.oeic())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Icr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Icr {{ rimic: {=bool:?}, ctsmic: {=bool:?}, dcdmic: {=bool:?}, dsrmic: {=bool:?}, rxic: {=bool:?}, txic: {=bool:?}, rtic: {=bool:?}, feic: {=bool:?}, peic: {=bool:?}, beic: {=bool:?}, oeic: {=bool:?} }}" , self . rimic () , self . ctsmic () , self . dcdmic () , self . dsrmic () , self . rxic () , self . txic () , self . rtic () , self . feic () , self . peic () , self . beic () , self . oeic ())
            }
        }
        #[doc = "Interrupt FIFO Level Select Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ifls(pub u32);
        impl Ifls {
            #[doc = "Transmit interrupt FIFO level select"]
            #[must_use]
            #[inline(always)]
            pub const fn txiflsel(&self) -> super::vals::Txiflsel {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Txiflsel::from_bits(val as u8)
            }
            #[doc = "Transmit interrupt FIFO level select"]
            #[inline(always)]
            pub const fn set_txiflsel(&mut self, val: super::vals::Txiflsel) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
            #[doc = "Receive interrupt FIFO level select"]
            #[must_use]
            #[inline(always)]
            pub const fn rxiflsel(&self) -> super::vals::Rxiflsel {
                let val = (self.0 >> 3usize) & 0x07;
                super::vals::Rxiflsel::from_bits(val as u8)
            }
            #[doc = "Receive interrupt FIFO level select"]
            #[inline(always)]
            pub const fn set_rxiflsel(&mut self, val: super::vals::Rxiflsel) {
                self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
            }
        }
        impl Default for Ifls {
            #[inline(always)]
            fn default() -> Ifls {
                Ifls(0)
            }
        }
        impl core::fmt::Debug for Ifls {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ifls")
                    .field("txiflsel", &self.txiflsel())
                    .field("rxiflsel", &self.rxiflsel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ifls {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ifls {{ txiflsel: {:?}, rxiflsel: {:?} }}",
                    self.txiflsel(),
                    self.rxiflsel()
                )
            }
        }
        #[doc = "IrDA Low-Power Counter Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ilpr(pub u32);
        impl Ilpr {
            #[doc = "8-bit low-power divisor value"]
            #[must_use]
            #[inline(always)]
            pub const fn ilpdvsr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "8-bit low-power divisor value"]
            #[inline(always)]
            pub const fn set_ilpdvsr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Ilpr {
            #[inline(always)]
            fn default() -> Ilpr {
                Ilpr(0)
            }
        }
        impl core::fmt::Debug for Ilpr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ilpr")
                    .field("ilpdvsr", &self.ilpdvsr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ilpr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ilpr {{ ilpdvsr: {=u8:?} }}", self.ilpdvsr())
            }
        }
        #[doc = "Interrupt Mask Set and Clear Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Imsc(pub u32);
        impl Imsc {
            #[doc = "nUARTRI modem interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rimim(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTRI modem interrupt mask"]
            #[inline(always)]
            pub const fn set_rimim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "nUARTCTS modem interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsmim(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTCTS modem interrupt mask"]
            #[inline(always)]
            pub const fn set_ctsmim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "nUARTDCD modem interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn dcdmim(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDCD modem interrupt mask"]
            #[inline(always)]
            pub const fn set_dcdmim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "nUARTDSR modem interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn dsrmim(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDSR modem interrupt mask"]
            #[inline(always)]
            pub const fn set_dsrmim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxim(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive interrupt mask"]
            #[inline(always)]
            pub const fn set_rxim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn txim(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit interrupt mask"]
            #[inline(always)]
            pub const fn set_txim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive timeout interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rtim(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive timeout interrupt mask"]
            #[inline(always)]
            pub const fn set_rtim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Framing error interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn feim(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error interrupt mask"]
            #[inline(always)]
            pub const fn set_feim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Parity error interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn peim(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error interrupt mask"]
            #[inline(always)]
            pub const fn set_peim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Break error interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn beim(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Break error interrupt mask"]
            #[inline(always)]
            pub const fn set_beim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun error interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn oeim(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error interrupt mask"]
            #[inline(always)]
            pub const fn set_oeim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Imsc {
            #[inline(always)]
            fn default() -> Imsc {
                Imsc(0)
            }
        }
        impl core::fmt::Debug for Imsc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Imsc")
                    .field("rimim", &self.rimim())
                    .field("ctsmim", &self.ctsmim())
                    .field("dcdmim", &self.dcdmim())
                    .field("dsrmim", &self.dsrmim())
                    .field("rxim", &self.rxim())
                    .field("txim", &self.txim())
                    .field("rtim", &self.rtim())
                    .field("feim", &self.feim())
                    .field("peim", &self.peim())
                    .field("beim", &self.beim())
                    .field("oeim", &self.oeim())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Imsc {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Imsc {{ rimim: {=bool:?}, ctsmim: {=bool:?}, dcdmim: {=bool:?}, dsrmim: {=bool:?}, rxim: {=bool:?}, txim: {=bool:?}, rtim: {=bool:?}, feim: {=bool:?}, peim: {=bool:?}, beim: {=bool:?}, oeim: {=bool:?} }}" , self . rimim () , self . ctsmim () , self . dcdmim () , self . dsrmim () , self . rxim () , self . txim () , self . rtim () , self . feim () , self . peim () , self . beim () , self . oeim ())
            }
        }
        #[doc = "Line Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct LcrH(pub u32);
        impl LcrH {
            #[doc = "Send break"]
            #[must_use]
            #[inline(always)]
            pub const fn brk(&self) -> super::vals::Brk {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Brk::from_bits(val as u8)
            }
            #[doc = "Send break"]
            #[inline(always)]
            pub const fn set_brk(&mut self, val: super::vals::Brk) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity enable"]
            #[must_use]
            #[inline(always)]
            pub const fn pen(&self) -> super::vals::Pen {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Pen::from_bits(val as u8)
            }
            #[doc = "Parity enable"]
            #[inline(always)]
            pub const fn set_pen(&mut self, val: super::vals::Pen) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Even parity select"]
            #[must_use]
            #[inline(always)]
            pub const fn eps(&self) -> super::vals::Eps {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Eps::from_bits(val as u8)
            }
            #[doc = "Even parity select"]
            #[inline(always)]
            pub const fn set_eps(&mut self, val: super::vals::Eps) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Two stop bits select"]
            #[must_use]
            #[inline(always)]
            pub const fn stp2(&self) -> super::vals::Stp2 {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Stp2::from_bits(val as u8)
            }
            #[doc = "Two stop bits select"]
            #[inline(always)]
            pub const fn set_stp2(&mut self, val: super::vals::Stp2) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable FIFOs"]
            #[must_use]
            #[inline(always)]
            pub const fn fen(&self) -> super::vals::Fen {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Fen::from_bits(val as u8)
            }
            #[doc = "Enable FIFOs"]
            #[inline(always)]
            pub const fn set_fen(&mut self, val: super::vals::Fen) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
            #[doc = "Word Length"]
            #[must_use]
            #[inline(always)]
            pub const fn wlen(&self) -> super::vals::Wlen {
                let val = (self.0 >> 5usize) & 0x03;
                super::vals::Wlen::from_bits(val as u8)
            }
            #[doc = "Word Length"]
            #[inline(always)]
            pub const fn set_wlen(&mut self, val: super::vals::Wlen) {
                self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
            }
            #[doc = "Stick parity select"]
            #[must_use]
            #[inline(always)]
            pub const fn sps(&self) -> super::vals::Sps {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Sps::from_bits(val as u8)
            }
            #[doc = "Stick parity select"]
            #[inline(always)]
            pub const fn set_sps(&mut self, val: super::vals::Sps) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
            }
        }
        impl Default for LcrH {
            #[inline(always)]
            fn default() -> LcrH {
                LcrH(0)
            }
        }
        impl core::fmt::Debug for LcrH {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("LcrH")
                    .field("brk", &self.brk())
                    .field("pen", &self.pen())
                    .field("eps", &self.eps())
                    .field("stp2", &self.stp2())
                    .field("fen", &self.fen())
                    .field("wlen", &self.wlen())
                    .field("sps", &self.sps())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for LcrH {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "LcrH {{ brk: {:?}, pen: {:?}, eps: {:?}, stp2: {:?}, fen: {:?}, wlen: {:?}, sps: {:?} }}" , self . brk () , self . pen () , self . eps () , self . stp2 () , self . fen () , self . wlen () , self . sps ())
            }
        }
        #[doc = "Masked Interrupt Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mis(pub u32);
        impl Mis {
            #[doc = "nUARTRI modem masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rimmis(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTRI modem masked interrupt status"]
            #[inline(always)]
            pub const fn set_rimmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "nUARTCTS modem masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsmmis(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTCTS modem masked interrupt status"]
            #[inline(always)]
            pub const fn set_ctsmmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "nUARTDCD modem masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn dcdmmis(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDCD modem masked interrupt status"]
            #[inline(always)]
            pub const fn set_dcdmmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "nUARTDSR modem masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn dsrmmis(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDSR modem masked interrupt status"]
            #[inline(always)]
            pub const fn set_dsrmmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rxmis(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive masked interrupt status"]
            #[inline(always)]
            pub const fn set_rxmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn txmis(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit masked interrupt status"]
            #[inline(always)]
            pub const fn set_txmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive timeout masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rtmis(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive timeout masked interrupt status"]
            #[inline(always)]
            pub const fn set_rtmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Framing error masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn femis(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error masked interrupt status"]
            #[inline(always)]
            pub const fn set_femis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Parity error masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn pemis(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error masked interrupt status"]
            #[inline(always)]
            pub const fn set_pemis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Break error masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn bemis(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Break error masked interrupt status"]
            #[inline(always)]
            pub const fn set_bemis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun error masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn oemis(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error masked interrupt status"]
            #[inline(always)]
            pub const fn set_oemis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Mis {
            #[inline(always)]
            fn default() -> Mis {
                Mis(0)
            }
        }
        impl core::fmt::Debug for Mis {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mis")
                    .field("rimmis", &self.rimmis())
                    .field("ctsmmis", &self.ctsmmis())
                    .field("dcdmmis", &self.dcdmmis())
                    .field("dsrmmis", &self.dsrmmis())
                    .field("rxmis", &self.rxmis())
                    .field("txmis", &self.txmis())
                    .field("rtmis", &self.rtmis())
                    .field("femis", &self.femis())
                    .field("pemis", &self.pemis())
                    .field("bemis", &self.bemis())
                    .field("oemis", &self.oemis())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mis {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Mis {{ rimmis: {=bool:?}, ctsmmis: {=bool:?}, dcdmmis: {=bool:?}, dsrmmis: {=bool:?}, rxmis: {=bool:?}, txmis: {=bool:?}, rtmis: {=bool:?}, femis: {=bool:?}, pemis: {=bool:?}, bemis: {=bool:?}, oemis: {=bool:?} }}" , self . rimmis () , self . ctsmmis () , self . dcdmmis () , self . dsrmmis () , self . rxmis () , self . txmis () , self . rtmis () , self . femis () , self . pemis () , self . bemis () , self . oemis ())
            }
        }
        #[doc = "Raw Interrupt Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ris(pub u32);
        impl Ris {
            #[doc = "nUARTRI modem interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rirris(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTRI modem interrupt status"]
            #[inline(always)]
            pub const fn set_rirris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "nUARTCTS modem interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsmris(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTCTS modem interrupt status"]
            #[inline(always)]
            pub const fn set_ctsmris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "nUARTDCD modem interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn dcdmris(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDCD modem interrupt status"]
            #[inline(always)]
            pub const fn set_dcdmris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "nUARTDSR modem interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn dsrmris(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDSR modem interrupt status"]
            #[inline(always)]
            pub const fn set_dsrmris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rxris(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive interrupt status"]
            #[inline(always)]
            pub const fn set_rxris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn txris(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit interrupt status"]
            #[inline(always)]
            pub const fn set_txris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive timeout interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rtris(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive timeout interrupt status"]
            #[inline(always)]
            pub const fn set_rtris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Framing error interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn feris(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error interrupt status"]
            #[inline(always)]
            pub const fn set_feris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Parity error interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn peris(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error interrupt status"]
            #[inline(always)]
            pub const fn set_peris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Break error interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn beris(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Break error interrupt status"]
            #[inline(always)]
            pub const fn set_beris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun error interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn oeris(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error interrupt status"]
            #[inline(always)]
            pub const fn set_oeris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Ris {
            #[inline(always)]
            fn default() -> Ris {
                Ris(0)
            }
        }
        impl core::fmt::Debug for Ris {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ris")
                    .field("rirris", &self.rirris())
                    .field("ctsmris", &self.ctsmris())
                    .field("dcdmris", &self.dcdmris())
                    .field("dsrmris", &self.dsrmris())
                    .field("rxris", &self.rxris())
                    .field("txris", &self.txris())
                    .field("rtris", &self.rtris())
                    .field("feris", &self.feris())
                    .field("peris", &self.peris())
                    .field("beris", &self.beris())
                    .field("oeris", &self.oeris())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ris {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ris {{ rirris: {=bool:?}, ctsmris: {=bool:?}, dcdmris: {=bool:?}, dsrmris: {=bool:?}, rxris: {=bool:?}, txris: {=bool:?}, rtris: {=bool:?}, feris: {=bool:?}, peris: {=bool:?}, beris: {=bool:?}, oeris: {=bool:?} }}" , self . rirris () , self . ctsmris () , self . dcdmris () , self . dsrmris () , self . rxris () , self . txris () , self . rtris () , self . feris () , self . peris () , self . beris () , self . oeris ())
            }
        }
        #[doc = "Receive Status and Clear Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rsr(pub u32);
        impl Rsr {
            #[doc = "Framing Error"]
            #[must_use]
            #[inline(always)]
            pub const fn rfe(&self) -> super::vals::Rfe {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rfe::from_bits(val as u8)
            }
            #[doc = "Framing Error"]
            #[inline(always)]
            pub const fn set_rfe(&mut self, val: super::vals::Rfe) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity Error"]
            #[must_use]
            #[inline(always)]
            pub const fn rpe(&self) -> super::vals::Rpe {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rpe::from_bits(val as u8)
            }
            #[doc = "Parity Error"]
            #[inline(always)]
            pub const fn set_rpe(&mut self, val: super::vals::Rpe) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Break Error"]
            #[must_use]
            #[inline(always)]
            pub const fn rbe(&self) -> super::vals::Rbe {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rbe::from_bits(val as u8)
            }
            #[doc = "Break Error"]
            #[inline(always)]
            pub const fn set_rbe(&mut self, val: super::vals::Rbe) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Overrun Error"]
            #[must_use]
            #[inline(always)]
            pub const fn roe(&self) -> super::vals::Roe {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Roe::from_bits(val as u8)
            }
            #[doc = "Overrun Error"]
            #[inline(always)]
            pub const fn set_roe(&mut self, val: super::vals::Roe) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Rsr {
            #[inline(always)]
            fn default() -> Rsr {
                Rsr(0)
            }
        }
        impl core::fmt::Debug for Rsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rsr")
                    .field("rfe", &self.rfe())
                    .field("rpe", &self.rpe())
                    .field("rbe", &self.rbe())
                    .field("roe", &self.roe())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rsr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Rsr {{ rfe: {:?}, rpe: {:?}, rbe: {:?}, roe: {:?} }}",
                    self.rfe(),
                    self.rpe(),
                    self.rbe(),
                    self.roe()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Be {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Be {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Be {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Be {
            #[inline(always)]
            fn from(val: u8) -> Be {
                Be::from_bits(val)
            }
        }
        impl From<Be> for u8 {
            #[inline(always)]
            fn from(val: Be) -> u8 {
                Be::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Brk {
            #[doc = "Normal operation"]
            NORMAL_OPS = 0x0,
            #[doc = "Send break"]
            SEND_BREAK = 0x01,
        }
        impl Brk {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Brk {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Brk {
            #[inline(always)]
            fn from(val: u8) -> Brk {
                Brk::from_bits(val)
            }
        }
        impl From<Brk> for u8 {
            #[inline(always)]
            fn from(val: Brk) -> u8 {
                Brk::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ctsen {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Ctsen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ctsen {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ctsen {
            #[inline(always)]
            fn from(val: u8) -> Ctsen {
                Ctsen::from_bits(val)
            }
        }
        impl From<Ctsen> for u8 {
            #[inline(always)]
            fn from(val: Ctsen) -> u8 {
                Ctsen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dtr {
            #[doc = "Low"]
            LOW = 0x0,
            #[doc = "High"]
            HIGH = 0x01,
        }
        impl Dtr {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dtr {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dtr {
            #[inline(always)]
            fn from(val: u8) -> Dtr {
                Dtr::from_bits(val)
            }
        }
        impl From<Dtr> for u8 {
            #[inline(always)]
            fn from(val: Dtr) -> u8 {
                Dtr::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Eps {
            #[doc = "Odd parity"]
            ODD_PARITY = 0x0,
            #[doc = "Even parity"]
            EVEN_PARITY = 0x01,
        }
        impl Eps {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Eps {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Eps {
            #[inline(always)]
            fn from(val: u8) -> Eps {
                Eps::from_bits(val)
            }
        }
        impl From<Eps> for u8 {
            #[inline(always)]
            fn from(val: Eps) -> u8 {
                Eps::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Fe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Fe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Fe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Fe {
            #[inline(always)]
            fn from(val: u8) -> Fe {
                Fe::from_bits(val)
            }
        }
        impl From<Fe> for u8 {
            #[inline(always)]
            fn from(val: Fe) -> u8 {
                Fe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Fen {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Fen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Fen {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Fen {
            #[inline(always)]
            fn from(val: u8) -> Fen {
                Fen::from_bits(val)
            }
        }
        impl From<Fen> for u8 {
            #[inline(always)]
            fn from(val: Fen) -> u8 {
                Fen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lbe {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Lbe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lbe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lbe {
            #[inline(always)]
            fn from(val: u8) -> Lbe {
                Lbe::from_bits(val)
            }
        }
        impl From<Lbe> for u8 {
            #[inline(always)]
            fn from(val: Lbe) -> u8 {
                Lbe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Oe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Oe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Oe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Oe {
            #[inline(always)]
            fn from(val: u8) -> Oe {
                Oe::from_bits(val)
            }
        }
        impl From<Oe> for u8 {
            #[inline(always)]
            fn from(val: Oe) -> u8 {
                Oe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Out1 {
            #[doc = "Output zero"]
            ZERO = 0x0,
            #[doc = "Output one"]
            ONE = 0x01,
        }
        impl Out1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Out1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Out1 {
            #[inline(always)]
            fn from(val: u8) -> Out1 {
                Out1::from_bits(val)
            }
        }
        impl From<Out1> for u8 {
            #[inline(always)]
            fn from(val: Out1) -> u8 {
                Out1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Out2 {
            #[doc = "Output zero"]
            ZERO = 0x0,
            #[doc = "Output one"]
            ONE = 0x01,
        }
        impl Out2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Out2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Out2 {
            #[inline(always)]
            fn from(val: u8) -> Out2 {
                Out2::from_bits(val)
            }
        }
        impl From<Out2> for u8 {
            #[inline(always)]
            fn from(val: Out2) -> u8 {
                Out2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Pe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pe {
            #[inline(always)]
            fn from(val: u8) -> Pe {
                Pe::from_bits(val)
            }
        }
        impl From<Pe> for u8 {
            #[inline(always)]
            fn from(val: Pe) -> u8 {
                Pe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pen {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Pen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pen {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pen {
            #[inline(always)]
            fn from(val: u8) -> Pen {
                Pen::from_bits(val)
            }
        }
        impl From<Pen> for u8 {
            #[inline(always)]
            fn from(val: Pen) -> u8 {
                Pen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rbe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Rbe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rbe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rbe {
            #[inline(always)]
            fn from(val: u8) -> Rbe {
                Rbe::from_bits(val)
            }
        }
        impl From<Rbe> for u8 {
            #[inline(always)]
            fn from(val: Rbe) -> u8 {
                Rbe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rfe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Rfe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rfe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rfe {
            #[inline(always)]
            fn from(val: u8) -> Rfe {
                Rfe::from_bits(val)
            }
        }
        impl From<Rfe> for u8 {
            #[inline(always)]
            fn from(val: Rfe) -> u8 {
                Rfe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Roe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Roe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Roe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Roe {
            #[inline(always)]
            fn from(val: u8) -> Roe {
                Roe::from_bits(val)
            }
        }
        impl From<Roe> for u8 {
            #[inline(always)]
            fn from(val: Roe) -> u8 {
                Roe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rpe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Rpe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rpe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rpe {
            #[inline(always)]
            fn from(val: u8) -> Rpe {
                Rpe::from_bits(val)
            }
        }
        impl From<Rpe> for u8 {
            #[inline(always)]
            fn from(val: Rpe) -> u8 {
                Rpe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rts {
            #[doc = "Low"]
            LOW = 0x0,
            #[doc = "High"]
            HIGH = 0x01,
        }
        impl Rts {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rts {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rts {
            #[inline(always)]
            fn from(val: u8) -> Rts {
                Rts::from_bits(val)
            }
        }
        impl From<Rts> for u8 {
            #[inline(always)]
            fn from(val: Rts) -> u8 {
                Rts::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtsen {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Rtsen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtsen {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtsen {
            #[inline(always)]
            fn from(val: u8) -> Rtsen {
                Rtsen::from_bits(val)
            }
        }
        impl From<Rtsen> for u8 {
            #[inline(always)]
            fn from(val: Rtsen) -> u8 {
                Rtsen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxdmae {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Rxdmae {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxdmae {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxdmae {
            #[inline(always)]
            fn from(val: u8) -> Rxdmae {
                Rxdmae::from_bits(val)
            }
        }
        impl From<Rxdmae> for u8 {
            #[inline(always)]
            fn from(val: Rxdmae) -> u8 {
                Rxdmae::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxe {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Rxe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxe {
            #[inline(always)]
            fn from(val: u8) -> Rxe {
                Rxe::from_bits(val)
            }
        }
        impl From<Rxe> for u8 {
            #[inline(always)]
            fn from(val: Rxe) -> u8 {
                Rxe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxiflsel {
            #[doc = "Receive FIFO becomes >= 1/8 full"]
            FIFO_1_8_FULL = 0x0,
            #[doc = "Receive FIFO becomes >= 1/4 full"]
            FIFO_1_4_FULL = 0x01,
            #[doc = "Receive FIFO becomes >= 1/2 full"]
            FIFO_1_2_FULL = 0x02,
            #[doc = "Receive FIFO becomes >= 3/4 full"]
            FIFO_3_4_FULL = 0x03,
            #[doc = "Receive FIFO becomes >= 7/8 full"]
            FIFO_7_8_FULL = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Rxiflsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxiflsel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxiflsel {
            #[inline(always)]
            fn from(val: u8) -> Rxiflsel {
                Rxiflsel::from_bits(val)
            }
        }
        impl From<Rxiflsel> for u8 {
            #[inline(always)]
            fn from(val: Rxiflsel) -> u8 {
                Rxiflsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Siren {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Siren {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Siren {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Siren {
            #[inline(always)]
            fn from(val: u8) -> Siren {
                Siren::from_bits(val)
            }
        }
        impl From<Siren> for u8 {
            #[inline(always)]
            fn from(val: Siren) -> u8 {
                Siren::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sirlp {
            #[doc = "Normal"]
            NORMAL = 0x0,
            #[doc = "Low-power"]
            LOW_POWER = 0x01,
        }
        impl Sirlp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sirlp {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sirlp {
            #[inline(always)]
            fn from(val: u8) -> Sirlp {
                Sirlp::from_bits(val)
            }
        }
        impl From<Sirlp> for u8 {
            #[inline(always)]
            fn from(val: Sirlp) -> u8 {
                Sirlp::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sps {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Sps {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sps {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sps {
            #[inline(always)]
            fn from(val: u8) -> Sps {
                Sps::from_bits(val)
            }
        }
        impl From<Sps> for u8 {
            #[inline(always)]
            fn from(val: Sps) -> u8 {
                Sps::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Stp2 {
            #[doc = "Not selected"]
            NOT_SELECTED = 0x0,
            #[doc = "Selected"]
            SELECTED = 0x01,
        }
        impl Stp2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Stp2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Stp2 {
            #[inline(always)]
            fn from(val: u8) -> Stp2 {
                Stp2::from_bits(val)
            }
        }
        impl From<Stp2> for u8 {
            #[inline(always)]
            fn from(val: Stp2) -> u8 {
                Stp2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txdmae {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Txdmae {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txdmae {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txdmae {
            #[inline(always)]
            fn from(val: u8) -> Txdmae {
                Txdmae::from_bits(val)
            }
        }
        impl From<Txdmae> for u8 {
            #[inline(always)]
            fn from(val: Txdmae) -> u8 {
                Txdmae::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txe {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Txe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txe {
            #[inline(always)]
            fn from(val: u8) -> Txe {
                Txe::from_bits(val)
            }
        }
        impl From<Txe> for u8 {
            #[inline(always)]
            fn from(val: Txe) -> u8 {
                Txe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txiflsel {
            #[doc = "Receive FIFO becomes >= 1/8 full"]
            FIFO_1_8_FULL = 0x0,
            #[doc = "Receive FIFO becomes >= 1/4 full"]
            FIFO_1_4_FULL = 0x01,
            #[doc = "Receive FIFO becomes >= 1/2 full"]
            FIFO_1_2_FULL = 0x02,
            #[doc = "Receive FIFO becomes >= 3/4 full"]
            FIFO_3_4_FULL = 0x03,
            #[doc = "Receive FIFO becomes >= 7/8 full"]
            FIFO_7_8_FULL = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Txiflsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txiflsel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txiflsel {
            #[inline(always)]
            fn from(val: u8) -> Txiflsel {
                Txiflsel::from_bits(val)
            }
        }
        impl From<Txiflsel> for u8 {
            #[inline(always)]
            fn from(val: Txiflsel) -> u8 {
                Txiflsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Uarten {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Uarten {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Uarten {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Uarten {
            #[inline(always)]
            fn from(val: u8) -> Uarten {
                Uarten::from_bits(val)
            }
        }
        impl From<Uarten> for u8 {
            #[inline(always)]
            fn from(val: Uarten) -> u8 {
                Uarten::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Wlen {
            #[doc = "5 bits"]
            _5BITS = 0x0,
            #[doc = "6 bits"]
            _6BITS = 0x01,
            #[doc = "7 bits"]
            _7BITS = 0x02,
            #[doc = "8 bits"]
            _8BITS = 0x03,
        }
        impl Wlen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Wlen {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Wlen {
            #[inline(always)]
            fn from(val: u8) -> Wlen {
                Wlen::from_bits(val)
            }
        }
        impl From<Wlen> for u8 {
            #[inline(always)]
            fn from(val: Wlen) -> u8 {
                Wlen::to_bits(val)
            }
        }
    }
}
pub mod uart2 {
    #[doc = "UART"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uart2 {
        ptr: *mut u8,
    }
    unsafe impl Send for Uart2 {}
    unsafe impl Sync for Uart2 {}
    impl Uart2 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[inline(always)]
        pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Receive Status and Clear Register"]
        #[inline(always)]
        pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Flags Register"]
        #[inline(always)]
        pub const fn fr(self) -> crate::common::Reg<regs::Fr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "Receive Timeout Configuration Register"]
        #[inline(always)]
        pub const fn rto(self) -> crate::common::Reg<regs::Rto, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "IrDA Low-Power Counter Register"]
        #[inline(always)]
        pub const fn ilpr(self) -> crate::common::Reg<regs::Ilpr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "The integer part of the baud rate divisor"]
        #[inline(always)]
        pub const fn ibrd(self) -> crate::common::Reg<regs::Ibrd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "The fractional part of the baud rate divisor"]
        #[inline(always)]
        pub const fn fbrd(self) -> crate::common::Reg<regs::Fbrd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "Line Control Register"]
        #[inline(always)]
        pub const fn lcr_h(self) -> crate::common::Reg<regs::LcrH, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
        }
        #[doc = "Control Register"]
        #[inline(always)]
        pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[doc = "Interrupt FIFO Level Select Register"]
        #[inline(always)]
        pub const fn ifls(self) -> crate::common::Reg<regs::Ifls, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
        }
        #[doc = "Interrupt Mask Set and Clear Register"]
        #[inline(always)]
        pub const fn imsc(self) -> crate::common::Reg<regs::Imsc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
        }
        #[doc = "Raw Interrupt Status Register"]
        #[inline(always)]
        pub const fn ris(self) -> crate::common::Reg<regs::Ris, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
        }
        #[doc = "Masked Interrupt Status Register"]
        #[inline(always)]
        pub const fn mis(self) -> crate::common::Reg<regs::Mis, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
        }
        #[doc = "Interrupt Clear Register"]
        #[inline(always)]
        pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
        }
        #[doc = "DMA Control Regsiter"]
        #[inline(always)]
        pub const fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "UART enable"]
            #[must_use]
            #[inline(always)]
            pub const fn uarten(&self) -> super::vals::Uarten {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Uarten::from_bits(val as u8)
            }
            #[doc = "UART enable"]
            #[inline(always)]
            pub const fn set_uarten(&mut self, val: super::vals::Uarten) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "SIR enable"]
            #[must_use]
            #[inline(always)]
            pub const fn siren(&self) -> super::vals::Siren {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Siren::from_bits(val as u8)
            }
            #[doc = "SIR enable"]
            #[inline(always)]
            pub const fn set_siren(&mut self, val: super::vals::Siren) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "SIR low-power IrDA"]
            #[must_use]
            #[inline(always)]
            pub const fn sirlp(&self) -> super::vals::Sirlp {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Sirlp::from_bits(val as u8)
            }
            #[doc = "SIR low-power IrDA"]
            #[inline(always)]
            pub const fn set_sirlp(&mut self, val: super::vals::Sirlp) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Invert SIR input"]
            #[must_use]
            #[inline(always)]
            pub const fn siriinv(&self) -> super::vals::Siriinv {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Siriinv::from_bits(val as u8)
            }
            #[doc = "Invert SIR input"]
            #[inline(always)]
            pub const fn set_siriinv(&mut self, val: super::vals::Siriinv) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "Invert SIR output"]
            #[must_use]
            #[inline(always)]
            pub const fn siroinv(&self) -> super::vals::Siroinv {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Siroinv::from_bits(val as u8)
            }
            #[doc = "Invert SIR output"]
            #[inline(always)]
            pub const fn set_siroinv(&mut self, val: super::vals::Siroinv) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
            #[doc = "RxD mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxdmsk(&self) -> super::vals::Rxdmsk {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Rxdmsk::from_bits(val as u8)
            }
            #[doc = "RxD mask"]
            #[inline(always)]
            pub const fn set_rxdmsk(&mut self, val: super::vals::Rxdmsk) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
            }
            #[doc = "Invert DTR signal"]
            #[must_use]
            #[inline(always)]
            pub const fn dtrinv(&self) -> super::vals::Dtrinv {
                let val = (self.0 >> 6usize) & 0x01;
                super::vals::Dtrinv::from_bits(val as u8)
            }
            #[doc = "Invert DTR signal"]
            #[inline(always)]
            pub const fn set_dtrinv(&mut self, val: super::vals::Dtrinv) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
            }
            #[doc = "Loopback enable"]
            #[must_use]
            #[inline(always)]
            pub const fn lbe(&self) -> super::vals::Lbe {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Lbe::from_bits(val as u8)
            }
            #[doc = "Loopback enable"]
            #[inline(always)]
            pub const fn set_lbe(&mut self, val: super::vals::Lbe) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
            }
            #[doc = "Transmit enable"]
            #[must_use]
            #[inline(always)]
            pub const fn txe(&self) -> super::vals::Txe {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Txe::from_bits(val as u8)
            }
            #[doc = "Transmit enable"]
            #[inline(always)]
            pub const fn set_txe(&mut self, val: super::vals::Txe) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
            #[doc = "Receive enable"]
            #[must_use]
            #[inline(always)]
            pub const fn rxe(&self) -> super::vals::Rxe {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Rxe::from_bits(val as u8)
            }
            #[doc = "Receive enable"]
            #[inline(always)]
            pub const fn set_rxe(&mut self, val: super::vals::Rxe) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
            #[doc = "Data transmit ready"]
            #[must_use]
            #[inline(always)]
            pub const fn dtr(&self) -> super::vals::Dtr {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Dtr::from_bits(val as u8)
            }
            #[doc = "Data transmit ready"]
            #[inline(always)]
            pub const fn set_dtr(&mut self, val: super::vals::Dtr) {
                self.0 =
                    (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "Request to send"]
            #[must_use]
            #[inline(always)]
            pub const fn rts(&self) -> super::vals::Rts {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Rts::from_bits(val as u8)
            }
            #[doc = "Request to send"]
            #[inline(always)]
            pub const fn set_rts(&mut self, val: super::vals::Rts) {
                self.0 =
                    (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
            #[doc = "nUARTOut1 modem status"]
            #[must_use]
            #[inline(always)]
            pub const fn out1(&self) -> super::vals::Out1 {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Out1::from_bits(val as u8)
            }
            #[doc = "nUARTOut1 modem status"]
            #[inline(always)]
            pub const fn set_out1(&mut self, val: super::vals::Out1) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
            #[doc = "nUARTOut2 modem status"]
            #[must_use]
            #[inline(always)]
            pub const fn out2(&self) -> super::vals::Out2 {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Out2::from_bits(val as u8)
            }
            #[doc = "nUARTOut2 modem status"]
            #[inline(always)]
            pub const fn set_out2(&mut self, val: super::vals::Out2) {
                self.0 =
                    (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
            }
            #[doc = "RTS hardware flow control enable"]
            #[must_use]
            #[inline(always)]
            pub const fn rtsen(&self) -> super::vals::Rtsen {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Rtsen::from_bits(val as u8)
            }
            #[doc = "RTS hardware flow control enable"]
            #[inline(always)]
            pub const fn set_rtsen(&mut self, val: super::vals::Rtsen) {
                self.0 =
                    (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
            }
            #[doc = "CTS hardware flow control enable"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsen(&self) -> super::vals::Ctsen {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Ctsen::from_bits(val as u8)
            }
            #[doc = "CTS hardware flow control enable"]
            #[inline(always)]
            pub const fn set_ctsen(&mut self, val: super::vals::Ctsen) {
                self.0 =
                    (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Cr {
            #[inline(always)]
            fn default() -> Cr {
                Cr(0)
            }
        }
        impl core::fmt::Debug for Cr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cr")
                    .field("uarten", &self.uarten())
                    .field("siren", &self.siren())
                    .field("sirlp", &self.sirlp())
                    .field("siriinv", &self.siriinv())
                    .field("siroinv", &self.siroinv())
                    .field("rxdmsk", &self.rxdmsk())
                    .field("dtrinv", &self.dtrinv())
                    .field("lbe", &self.lbe())
                    .field("txe", &self.txe())
                    .field("rxe", &self.rxe())
                    .field("dtr", &self.dtr())
                    .field("rts", &self.rts())
                    .field("out1", &self.out1())
                    .field("out2", &self.out2())
                    .field("rtsen", &self.rtsen())
                    .field("ctsen", &self.ctsen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cr {{ uarten: {:?}, siren: {:?}, sirlp: {:?}, siriinv: {:?}, siroinv: {:?}, rxdmsk: {:?}, dtrinv: {:?}, lbe: {:?}, txe: {:?}, rxe: {:?}, dtr: {:?}, rts: {:?}, out1: {:?}, out2: {:?}, rtsen: {:?}, ctsen: {:?} }}" , self . uarten () , self . siren () , self . sirlp () , self . siriinv () , self . siroinv () , self . rxdmsk () , self . dtrinv () , self . lbe () , self . txe () , self . rxe () , self . dtr () , self . rts () , self . out1 () , self . out2 () , self . rtsen () , self . ctsen ())
            }
        }
        #[doc = "DMA Control Regsiter"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dmacr(pub u32);
        impl Dmacr {
            #[doc = "Receive DMA enable"]
            #[must_use]
            #[inline(always)]
            pub const fn rxdmae(&self) -> super::vals::Rxdmae {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rxdmae::from_bits(val as u8)
            }
            #[doc = "Receive DMA enable"]
            #[inline(always)]
            pub const fn set_rxdmae(&mut self, val: super::vals::Rxdmae) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit DMA enable"]
            #[must_use]
            #[inline(always)]
            pub const fn txdmae(&self) -> super::vals::Txdmae {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Txdmae::from_bits(val as u8)
            }
            #[doc = "Transmit DMA enable"]
            #[inline(always)]
            pub const fn set_txdmae(&mut self, val: super::vals::Txdmae) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "DMA on error"]
            #[must_use]
            #[inline(always)]
            pub const fn dmaonerr(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "DMA on error"]
            #[inline(always)]
            pub const fn set_dmaonerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Dmacr {
            #[inline(always)]
            fn default() -> Dmacr {
                Dmacr(0)
            }
        }
        impl core::fmt::Debug for Dmacr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dmacr")
                    .field("rxdmae", &self.rxdmae())
                    .field("txdmae", &self.txdmae())
                    .field("dmaonerr", &self.dmaonerr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dmacr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dmacr {{ rxdmae: {:?}, txdmae: {:?}, dmaonerr: {=bool:?} }}",
                    self.rxdmae(),
                    self.txdmae(),
                    self.dmaonerr()
                )
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "Data"]
            #[must_use]
            #[inline(always)]
            pub const fn data(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Data"]
            #[inline(always)]
            pub const fn set_data(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Framing Error"]
            #[must_use]
            #[inline(always)]
            pub const fn fe(&self) -> super::vals::Fe {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Fe::from_bits(val as u8)
            }
            #[doc = "Framing Error"]
            #[inline(always)]
            pub const fn set_fe(&mut self, val: super::vals::Fe) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
            #[doc = "Parity Error"]
            #[must_use]
            #[inline(always)]
            pub const fn pe(&self) -> super::vals::Pe {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Pe::from_bits(val as u8)
            }
            #[doc = "Parity Error"]
            #[inline(always)]
            pub const fn set_pe(&mut self, val: super::vals::Pe) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
            #[doc = "Break Error"]
            #[must_use]
            #[inline(always)]
            pub const fn be(&self) -> super::vals::Be {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Be::from_bits(val as u8)
            }
            #[doc = "Break Error"]
            #[inline(always)]
            pub const fn set_be(&mut self, val: super::vals::Be) {
                self.0 =
                    (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "Overrun Error"]
            #[must_use]
            #[inline(always)]
            pub const fn oe(&self) -> super::vals::Oe {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Oe::from_bits(val as u8)
            }
            #[doc = "Overrun Error"]
            #[inline(always)]
            pub const fn set_oe(&mut self, val: super::vals::Oe) {
                self.0 =
                    (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
        }
        impl Default for Dr {
            #[inline(always)]
            fn default() -> Dr {
                Dr(0)
            }
        }
        impl core::fmt::Debug for Dr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dr")
                    .field("data", &self.data())
                    .field("fe", &self.fe())
                    .field("pe", &self.pe())
                    .field("be", &self.be())
                    .field("oe", &self.oe())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dr {{ data: {=u8:?}, fe: {:?}, pe: {:?}, be: {:?}, oe: {:?} }}",
                    self.data(),
                    self.fe(),
                    self.pe(),
                    self.be(),
                    self.oe()
                )
            }
        }
        #[doc = "The fractional part of the baud rate divisor"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fbrd(pub u32);
        impl Fbrd {
            #[doc = "The fractional baud rate divisor"]
            #[must_use]
            #[inline(always)]
            pub const fn baud_divfrac(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x3f;
                val as u8
            }
            #[doc = "The fractional baud rate divisor"]
            #[inline(always)]
            pub const fn set_baud_divfrac(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
            }
        }
        impl Default for Fbrd {
            #[inline(always)]
            fn default() -> Fbrd {
                Fbrd(0)
            }
        }
        impl core::fmt::Debug for Fbrd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Fbrd")
                    .field("baud_divfrac", &self.baud_divfrac())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Fbrd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Fbrd {{ baud_divfrac: {=u8:?} }}", self.baud_divfrac())
            }
        }
        #[doc = "Flags Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fr(pub u32);
        impl Fr {
            #[doc = "Clear to send"]
            #[must_use]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Clear to send"]
            #[inline(always)]
            pub const fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Data set ready"]
            #[must_use]
            #[inline(always)]
            pub const fn dsr(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Data set ready"]
            #[inline(always)]
            pub const fn set_dsr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Data carrier detect"]
            #[must_use]
            #[inline(always)]
            pub const fn dcd(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Data carrier detect"]
            #[inline(always)]
            pub const fn set_dcd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "UART busy"]
            #[must_use]
            #[inline(always)]
            pub const fn busy(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "UART busy"]
            #[inline(always)]
            pub const fn set_busy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive FIFO empty"]
            #[must_use]
            #[inline(always)]
            pub const fn rxfe(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive FIFO empty"]
            #[inline(always)]
            pub const fn set_rxfe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit FIFO full"]
            #[must_use]
            #[inline(always)]
            pub const fn txff(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit FIFO full"]
            #[inline(always)]
            pub const fn set_txff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive FIFO full"]
            #[must_use]
            #[inline(always)]
            pub const fn rxff(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive FIFO full"]
            #[inline(always)]
            pub const fn set_rxff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Transmit FIFO empty"]
            #[must_use]
            #[inline(always)]
            pub const fn txfe(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit FIFO empty"]
            #[inline(always)]
            pub const fn set_txfe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Ring Indicator"]
            #[must_use]
            #[inline(always)]
            pub const fn ri(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Ring Indicator"]
            #[inline(always)]
            pub const fn set_ri(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Fr {
            #[inline(always)]
            fn default() -> Fr {
                Fr(0)
            }
        }
        impl core::fmt::Debug for Fr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Fr")
                    .field("cts", &self.cts())
                    .field("dsr", &self.dsr())
                    .field("dcd", &self.dcd())
                    .field("busy", &self.busy())
                    .field("rxfe", &self.rxfe())
                    .field("txff", &self.txff())
                    .field("rxff", &self.rxff())
                    .field("txfe", &self.txfe())
                    .field("ri", &self.ri())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Fr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Fr {{ cts: {=bool:?}, dsr: {=bool:?}, dcd: {=bool:?}, busy: {=bool:?}, rxfe: {=bool:?}, txff: {=bool:?}, rxff: {=bool:?}, txfe: {=bool:?}, ri: {=bool:?} }}" , self . cts () , self . dsr () , self . dcd () , self . busy () , self . rxfe () , self . txff () , self . rxff () , self . txfe () , self . ri ())
            }
        }
        #[doc = "The integer part of the baud rate divisor"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ibrd(pub u32);
        impl Ibrd {
            #[doc = "The integer baud rate divisor"]
            #[must_use]
            #[inline(always)]
            pub const fn baud_divint(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "The integer baud rate divisor"]
            #[inline(always)]
            pub const fn set_baud_divint(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Ibrd {
            #[inline(always)]
            fn default() -> Ibrd {
                Ibrd(0)
            }
        }
        impl core::fmt::Debug for Ibrd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ibrd")
                    .field("baud_divint", &self.baud_divint())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ibrd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ibrd {{ baud_divint: {=u16:?} }}", self.baud_divint())
            }
        }
        #[doc = "Interrupt Clear Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Icr(pub u32);
        impl Icr {
            #[doc = "nUARTRI modem interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn rimic(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTRI modem interrupt clear"]
            #[inline(always)]
            pub const fn set_rimic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "nUARTCTS modem interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsmic(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTCTS modem interrupt clear"]
            #[inline(always)]
            pub const fn set_ctsmic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "nUARTDCD modem interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn dcdmic(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDCD modem interrupt clear"]
            #[inline(always)]
            pub const fn set_dcdmic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "nUARTDSR modem interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn dsrmic(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDSR modem interrupt clear"]
            #[inline(always)]
            pub const fn set_dsrmic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn rxic(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive interrupt clear"]
            #[inline(always)]
            pub const fn set_rxic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn txic(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit interrupt clear"]
            #[inline(always)]
            pub const fn set_txic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive timeout interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn rtic(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive timeout interrupt clear"]
            #[inline(always)]
            pub const fn set_rtic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Framing error interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn feic(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error interrupt clear"]
            #[inline(always)]
            pub const fn set_feic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Parity error interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn peic(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error interrupt clear"]
            #[inline(always)]
            pub const fn set_peic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Break error interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn beic(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Break error interrupt clear"]
            #[inline(always)]
            pub const fn set_beic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun error interrupt clear"]
            #[must_use]
            #[inline(always)]
            pub const fn oeic(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error interrupt clear"]
            #[inline(always)]
            pub const fn set_oeic(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Icr {
            #[inline(always)]
            fn default() -> Icr {
                Icr(0)
            }
        }
        impl core::fmt::Debug for Icr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Icr")
                    .field("rimic", &self.rimic())
                    .field("ctsmic", &self.ctsmic())
                    .field("dcdmic", &self.dcdmic())
                    .field("dsrmic", &self.dsrmic())
                    .field("rxic", &self.rxic())
                    .field("txic", &self.txic())
                    .field("rtic", &self.rtic())
                    .field("feic", &self.feic())
                    .field("peic", &self.peic())
                    .field("beic", &self.beic())
                    .field("oeic", &self.oeic())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Icr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Icr {{ rimic: {=bool:?}, ctsmic: {=bool:?}, dcdmic: {=bool:?}, dsrmic: {=bool:?}, rxic: {=bool:?}, txic: {=bool:?}, rtic: {=bool:?}, feic: {=bool:?}, peic: {=bool:?}, beic: {=bool:?}, oeic: {=bool:?} }}" , self . rimic () , self . ctsmic () , self . dcdmic () , self . dsrmic () , self . rxic () , self . txic () , self . rtic () , self . feic () , self . peic () , self . beic () , self . oeic ())
            }
        }
        #[doc = "Interrupt FIFO Level Select Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ifls(pub u32);
        impl Ifls {
            #[doc = "Transmit interrupt FIFO level select"]
            #[must_use]
            #[inline(always)]
            pub const fn txiflsel(&self) -> super::vals::Txiflsel {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Txiflsel::from_bits(val as u8)
            }
            #[doc = "Transmit interrupt FIFO level select"]
            #[inline(always)]
            pub const fn set_txiflsel(&mut self, val: super::vals::Txiflsel) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
            #[doc = "Receive interrupt FIFO level select"]
            #[must_use]
            #[inline(always)]
            pub const fn rxiflsel(&self) -> super::vals::Rxiflsel {
                let val = (self.0 >> 3usize) & 0x07;
                super::vals::Rxiflsel::from_bits(val as u8)
            }
            #[doc = "Receive interrupt FIFO level select"]
            #[inline(always)]
            pub const fn set_rxiflsel(&mut self, val: super::vals::Rxiflsel) {
                self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
            }
        }
        impl Default for Ifls {
            #[inline(always)]
            fn default() -> Ifls {
                Ifls(0)
            }
        }
        impl core::fmt::Debug for Ifls {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ifls")
                    .field("txiflsel", &self.txiflsel())
                    .field("rxiflsel", &self.rxiflsel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ifls {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ifls {{ txiflsel: {:?}, rxiflsel: {:?} }}",
                    self.txiflsel(),
                    self.rxiflsel()
                )
            }
        }
        #[doc = "IrDA Low-Power Counter Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ilpr(pub u32);
        impl Ilpr {
            #[doc = "8-bit low-power divisor value"]
            #[must_use]
            #[inline(always)]
            pub const fn ilpdvsr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "8-bit low-power divisor value"]
            #[inline(always)]
            pub const fn set_ilpdvsr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Ilpr {
            #[inline(always)]
            fn default() -> Ilpr {
                Ilpr(0)
            }
        }
        impl core::fmt::Debug for Ilpr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ilpr")
                    .field("ilpdvsr", &self.ilpdvsr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ilpr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ilpr {{ ilpdvsr: {=u8:?} }}", self.ilpdvsr())
            }
        }
        #[doc = "Interrupt Mask Set and Clear Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Imsc(pub u32);
        impl Imsc {
            #[doc = "nUARTRI modem interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rimim(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTRI modem interrupt mask"]
            #[inline(always)]
            pub const fn set_rimim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "nUARTCTS modem interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsmim(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTCTS modem interrupt mask"]
            #[inline(always)]
            pub const fn set_ctsmim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "nUARTDCD modem interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn dcdmim(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDCD modem interrupt mask"]
            #[inline(always)]
            pub const fn set_dcdmim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "nUARTDSR modem interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn dsrmim(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDSR modem interrupt mask"]
            #[inline(always)]
            pub const fn set_dsrmim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rxim(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive interrupt mask"]
            #[inline(always)]
            pub const fn set_rxim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn txim(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit interrupt mask"]
            #[inline(always)]
            pub const fn set_txim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive timeout interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn rtim(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive timeout interrupt mask"]
            #[inline(always)]
            pub const fn set_rtim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Framing error interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn feim(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error interrupt mask"]
            #[inline(always)]
            pub const fn set_feim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Parity error interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn peim(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error interrupt mask"]
            #[inline(always)]
            pub const fn set_peim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Break error interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn beim(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Break error interrupt mask"]
            #[inline(always)]
            pub const fn set_beim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun error interrupt mask"]
            #[must_use]
            #[inline(always)]
            pub const fn oeim(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error interrupt mask"]
            #[inline(always)]
            pub const fn set_oeim(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Imsc {
            #[inline(always)]
            fn default() -> Imsc {
                Imsc(0)
            }
        }
        impl core::fmt::Debug for Imsc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Imsc")
                    .field("rimim", &self.rimim())
                    .field("ctsmim", &self.ctsmim())
                    .field("dcdmim", &self.dcdmim())
                    .field("dsrmim", &self.dsrmim())
                    .field("rxim", &self.rxim())
                    .field("txim", &self.txim())
                    .field("rtim", &self.rtim())
                    .field("feim", &self.feim())
                    .field("peim", &self.peim())
                    .field("beim", &self.beim())
                    .field("oeim", &self.oeim())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Imsc {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Imsc {{ rimim: {=bool:?}, ctsmim: {=bool:?}, dcdmim: {=bool:?}, dsrmim: {=bool:?}, rxim: {=bool:?}, txim: {=bool:?}, rtim: {=bool:?}, feim: {=bool:?}, peim: {=bool:?}, beim: {=bool:?}, oeim: {=bool:?} }}" , self . rimim () , self . ctsmim () , self . dcdmim () , self . dsrmim () , self . rxim () , self . txim () , self . rtim () , self . feim () , self . peim () , self . beim () , self . oeim ())
            }
        }
        #[doc = "Line Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct LcrH(pub u32);
        impl LcrH {
            #[doc = "Send break"]
            #[must_use]
            #[inline(always)]
            pub const fn brk(&self) -> super::vals::Brk {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Brk::from_bits(val as u8)
            }
            #[doc = "Send break"]
            #[inline(always)]
            pub const fn set_brk(&mut self, val: super::vals::Brk) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity enable"]
            #[must_use]
            #[inline(always)]
            pub const fn pen(&self) -> super::vals::Pen {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Pen::from_bits(val as u8)
            }
            #[doc = "Parity enable"]
            #[inline(always)]
            pub const fn set_pen(&mut self, val: super::vals::Pen) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Even parity select"]
            #[must_use]
            #[inline(always)]
            pub const fn eps(&self) -> super::vals::Eps {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Eps::from_bits(val as u8)
            }
            #[doc = "Even parity select"]
            #[inline(always)]
            pub const fn set_eps(&mut self, val: super::vals::Eps) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Two stop bits select"]
            #[must_use]
            #[inline(always)]
            pub const fn stp2(&self) -> super::vals::Stp2 {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Stp2::from_bits(val as u8)
            }
            #[doc = "Two stop bits select"]
            #[inline(always)]
            pub const fn set_stp2(&mut self, val: super::vals::Stp2) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable FIFOs"]
            #[must_use]
            #[inline(always)]
            pub const fn fen(&self) -> super::vals::Fen {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Fen::from_bits(val as u8)
            }
            #[doc = "Enable FIFOs"]
            #[inline(always)]
            pub const fn set_fen(&mut self, val: super::vals::Fen) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
            #[doc = "Word Length"]
            #[must_use]
            #[inline(always)]
            pub const fn wlen(&self) -> super::vals::Wlen {
                let val = (self.0 >> 5usize) & 0x03;
                super::vals::Wlen::from_bits(val as u8)
            }
            #[doc = "Word Length"]
            #[inline(always)]
            pub const fn set_wlen(&mut self, val: super::vals::Wlen) {
                self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
            }
            #[doc = "Stick parity select"]
            #[must_use]
            #[inline(always)]
            pub const fn sps(&self) -> super::vals::Sps {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Sps::from_bits(val as u8)
            }
            #[doc = "Stick parity select"]
            #[inline(always)]
            pub const fn set_sps(&mut self, val: super::vals::Sps) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
            }
        }
        impl Default for LcrH {
            #[inline(always)]
            fn default() -> LcrH {
                LcrH(0)
            }
        }
        impl core::fmt::Debug for LcrH {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("LcrH")
                    .field("brk", &self.brk())
                    .field("pen", &self.pen())
                    .field("eps", &self.eps())
                    .field("stp2", &self.stp2())
                    .field("fen", &self.fen())
                    .field("wlen", &self.wlen())
                    .field("sps", &self.sps())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for LcrH {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "LcrH {{ brk: {:?}, pen: {:?}, eps: {:?}, stp2: {:?}, fen: {:?}, wlen: {:?}, sps: {:?} }}" , self . brk () , self . pen () , self . eps () , self . stp2 () , self . fen () , self . wlen () , self . sps ())
            }
        }
        #[doc = "Masked Interrupt Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mis(pub u32);
        impl Mis {
            #[doc = "nUARTRI modem masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rimmis(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTRI modem masked interrupt status"]
            #[inline(always)]
            pub const fn set_rimmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "nUARTCTS modem masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsmmis(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTCTS modem masked interrupt status"]
            #[inline(always)]
            pub const fn set_ctsmmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "nUARTDCD modem masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn dcdmmis(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDCD modem masked interrupt status"]
            #[inline(always)]
            pub const fn set_dcdmmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "nUARTDSR modem masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn dsrmmis(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDSR modem masked interrupt status"]
            #[inline(always)]
            pub const fn set_dsrmmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rxmis(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive masked interrupt status"]
            #[inline(always)]
            pub const fn set_rxmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn txmis(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit masked interrupt status"]
            #[inline(always)]
            pub const fn set_txmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive timeout masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rtmis(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive timeout masked interrupt status"]
            #[inline(always)]
            pub const fn set_rtmis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Framing error masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn femis(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error masked interrupt status"]
            #[inline(always)]
            pub const fn set_femis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Parity error masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn pemis(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error masked interrupt status"]
            #[inline(always)]
            pub const fn set_pemis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Break error masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn bemis(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Break error masked interrupt status"]
            #[inline(always)]
            pub const fn set_bemis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun error masked interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn oemis(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error masked interrupt status"]
            #[inline(always)]
            pub const fn set_oemis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Mis {
            #[inline(always)]
            fn default() -> Mis {
                Mis(0)
            }
        }
        impl core::fmt::Debug for Mis {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mis")
                    .field("rimmis", &self.rimmis())
                    .field("ctsmmis", &self.ctsmmis())
                    .field("dcdmmis", &self.dcdmmis())
                    .field("dsrmmis", &self.dsrmmis())
                    .field("rxmis", &self.rxmis())
                    .field("txmis", &self.txmis())
                    .field("rtmis", &self.rtmis())
                    .field("femis", &self.femis())
                    .field("pemis", &self.pemis())
                    .field("bemis", &self.bemis())
                    .field("oemis", &self.oemis())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mis {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Mis {{ rimmis: {=bool:?}, ctsmmis: {=bool:?}, dcdmmis: {=bool:?}, dsrmmis: {=bool:?}, rxmis: {=bool:?}, txmis: {=bool:?}, rtmis: {=bool:?}, femis: {=bool:?}, pemis: {=bool:?}, bemis: {=bool:?}, oemis: {=bool:?} }}" , self . rimmis () , self . ctsmmis () , self . dcdmmis () , self . dsrmmis () , self . rxmis () , self . txmis () , self . rtmis () , self . femis () , self . pemis () , self . bemis () , self . oemis ())
            }
        }
        #[doc = "Raw Interrupt Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ris(pub u32);
        impl Ris {
            #[doc = "nUARTRI modem interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rirris(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTRI modem interrupt status"]
            #[inline(always)]
            pub const fn set_rirris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "nUARTCTS modem interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn ctsmris(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTCTS modem interrupt status"]
            #[inline(always)]
            pub const fn set_ctsmris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "nUARTDCD modem interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn dcdmris(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDCD modem interrupt status"]
            #[inline(always)]
            pub const fn set_dcdmris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "nUARTDSR modem interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn dsrmris(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "nUARTDSR modem interrupt status"]
            #[inline(always)]
            pub const fn set_dsrmris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Receive interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rxris(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Receive interrupt status"]
            #[inline(always)]
            pub const fn set_rxris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Transmit interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn txris(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit interrupt status"]
            #[inline(always)]
            pub const fn set_txris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Receive timeout interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn rtris(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Receive timeout interrupt status"]
            #[inline(always)]
            pub const fn set_rtris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Framing error interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn feris(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error interrupt status"]
            #[inline(always)]
            pub const fn set_feris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Parity error interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn peris(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error interrupt status"]
            #[inline(always)]
            pub const fn set_peris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Break error interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn beris(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Break error interrupt status"]
            #[inline(always)]
            pub const fn set_beris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Overrun error interrupt status"]
            #[must_use]
            #[inline(always)]
            pub const fn oeris(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error interrupt status"]
            #[inline(always)]
            pub const fn set_oeris(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Ris {
            #[inline(always)]
            fn default() -> Ris {
                Ris(0)
            }
        }
        impl core::fmt::Debug for Ris {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ris")
                    .field("rirris", &self.rirris())
                    .field("ctsmris", &self.ctsmris())
                    .field("dcdmris", &self.dcdmris())
                    .field("dsrmris", &self.dsrmris())
                    .field("rxris", &self.rxris())
                    .field("txris", &self.txris())
                    .field("rtris", &self.rtris())
                    .field("feris", &self.feris())
                    .field("peris", &self.peris())
                    .field("beris", &self.beris())
                    .field("oeris", &self.oeris())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ris {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ris {{ rirris: {=bool:?}, ctsmris: {=bool:?}, dcdmris: {=bool:?}, dsrmris: {=bool:?}, rxris: {=bool:?}, txris: {=bool:?}, rtris: {=bool:?}, feris: {=bool:?}, peris: {=bool:?}, beris: {=bool:?}, oeris: {=bool:?} }}" , self . rirris () , self . ctsmris () , self . dcdmris () , self . dsrmris () , self . rxris () , self . txris () , self . rtris () , self . feris () , self . peris () , self . beris () , self . oeris ())
            }
        }
        #[doc = "Receive Status and Clear Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rsr(pub u32);
        impl Rsr {
            #[doc = "Framing Error"]
            #[must_use]
            #[inline(always)]
            pub const fn rfe(&self) -> super::vals::Rfe {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Rfe::from_bits(val as u8)
            }
            #[doc = "Framing Error"]
            #[inline(always)]
            pub const fn set_rfe(&mut self, val: super::vals::Rfe) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity Error"]
            #[must_use]
            #[inline(always)]
            pub const fn rpe(&self) -> super::vals::Rpe {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rpe::from_bits(val as u8)
            }
            #[doc = "Parity Error"]
            #[inline(always)]
            pub const fn set_rpe(&mut self, val: super::vals::Rpe) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Break Error"]
            #[must_use]
            #[inline(always)]
            pub const fn rbe(&self) -> super::vals::Rbe {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Rbe::from_bits(val as u8)
            }
            #[doc = "Break Error"]
            #[inline(always)]
            pub const fn set_rbe(&mut self, val: super::vals::Rbe) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Overrun Error"]
            #[must_use]
            #[inline(always)]
            pub const fn roe(&self) -> super::vals::Roe {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Roe::from_bits(val as u8)
            }
            #[doc = "Overrun Error"]
            #[inline(always)]
            pub const fn set_roe(&mut self, val: super::vals::Roe) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Rsr {
            #[inline(always)]
            fn default() -> Rsr {
                Rsr(0)
            }
        }
        impl core::fmt::Debug for Rsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rsr")
                    .field("rfe", &self.rfe())
                    .field("rpe", &self.rpe())
                    .field("rbe", &self.rbe())
                    .field("roe", &self.roe())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rsr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Rsr {{ rfe: {:?}, rpe: {:?}, rbe: {:?}, roe: {:?} }}",
                    self.rfe(),
                    self.rpe(),
                    self.rbe(),
                    self.roe()
                )
            }
        }
        #[doc = "Receive Timeout Configuration Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rto(pub u32);
        impl Rto {
            #[doc = "Recieve timeout"]
            #[must_use]
            #[inline(always)]
            pub const fn timeout(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Recieve timeout"]
            #[inline(always)]
            pub const fn set_timeout(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Rto {
            #[inline(always)]
            fn default() -> Rto {
                Rto(0)
            }
        }
        impl core::fmt::Debug for Rto {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rto")
                    .field("timeout", &self.timeout())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rto {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rto {{ timeout: {=u16:?} }}", self.timeout())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Be {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Be {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Be {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Be {
            #[inline(always)]
            fn from(val: u8) -> Be {
                Be::from_bits(val)
            }
        }
        impl From<Be> for u8 {
            #[inline(always)]
            fn from(val: Be) -> u8 {
                Be::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Brk {
            #[doc = "Normal operation"]
            NORMAL_OPS = 0x0,
            #[doc = "Send break"]
            SEND_BREAK = 0x01,
        }
        impl Brk {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Brk {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Brk {
            #[inline(always)]
            fn from(val: u8) -> Brk {
                Brk::from_bits(val)
            }
        }
        impl From<Brk> for u8 {
            #[inline(always)]
            fn from(val: Brk) -> u8 {
                Brk::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ctsen {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Ctsen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ctsen {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ctsen {
            #[inline(always)]
            fn from(val: u8) -> Ctsen {
                Ctsen::from_bits(val)
            }
        }
        impl From<Ctsen> for u8 {
            #[inline(always)]
            fn from(val: Ctsen) -> u8 {
                Ctsen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dtr {
            #[doc = "Low"]
            LOW = 0x0,
            #[doc = "High"]
            HIGH = 0x01,
        }
        impl Dtr {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dtr {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dtr {
            #[inline(always)]
            fn from(val: u8) -> Dtr {
                Dtr::from_bits(val)
            }
        }
        impl From<Dtr> for u8 {
            #[inline(always)]
            fn from(val: Dtr) -> u8 {
                Dtr::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dtrinv {
            #[doc = "Normal"]
            NORMAL = 0x0,
            #[doc = "Inverted"]
            INVERTED = 0x01,
        }
        impl Dtrinv {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dtrinv {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dtrinv {
            #[inline(always)]
            fn from(val: u8) -> Dtrinv {
                Dtrinv::from_bits(val)
            }
        }
        impl From<Dtrinv> for u8 {
            #[inline(always)]
            fn from(val: Dtrinv) -> u8 {
                Dtrinv::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Eps {
            #[doc = "Odd parity"]
            ODD_PARITY = 0x0,
            #[doc = "Even parity"]
            EVEN_PARITY = 0x01,
        }
        impl Eps {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Eps {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Eps {
            #[inline(always)]
            fn from(val: u8) -> Eps {
                Eps::from_bits(val)
            }
        }
        impl From<Eps> for u8 {
            #[inline(always)]
            fn from(val: Eps) -> u8 {
                Eps::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Fe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Fe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Fe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Fe {
            #[inline(always)]
            fn from(val: u8) -> Fe {
                Fe::from_bits(val)
            }
        }
        impl From<Fe> for u8 {
            #[inline(always)]
            fn from(val: Fe) -> u8 {
                Fe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Fen {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Fen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Fen {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Fen {
            #[inline(always)]
            fn from(val: u8) -> Fen {
                Fen::from_bits(val)
            }
        }
        impl From<Fen> for u8 {
            #[inline(always)]
            fn from(val: Fen) -> u8 {
                Fen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lbe {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Lbe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lbe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lbe {
            #[inline(always)]
            fn from(val: u8) -> Lbe {
                Lbe::from_bits(val)
            }
        }
        impl From<Lbe> for u8 {
            #[inline(always)]
            fn from(val: Lbe) -> u8 {
                Lbe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Oe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Oe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Oe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Oe {
            #[inline(always)]
            fn from(val: u8) -> Oe {
                Oe::from_bits(val)
            }
        }
        impl From<Oe> for u8 {
            #[inline(always)]
            fn from(val: Oe) -> u8 {
                Oe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Out1 {
            #[doc = "Output zero"]
            ZERO = 0x0,
            #[doc = "Output one"]
            ONE = 0x01,
        }
        impl Out1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Out1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Out1 {
            #[inline(always)]
            fn from(val: u8) -> Out1 {
                Out1::from_bits(val)
            }
        }
        impl From<Out1> for u8 {
            #[inline(always)]
            fn from(val: Out1) -> u8 {
                Out1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Out2 {
            #[doc = "Output zero"]
            ZERO = 0x0,
            #[doc = "Output one"]
            ONE = 0x01,
        }
        impl Out2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Out2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Out2 {
            #[inline(always)]
            fn from(val: u8) -> Out2 {
                Out2::from_bits(val)
            }
        }
        impl From<Out2> for u8 {
            #[inline(always)]
            fn from(val: Out2) -> u8 {
                Out2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Pe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pe {
            #[inline(always)]
            fn from(val: u8) -> Pe {
                Pe::from_bits(val)
            }
        }
        impl From<Pe> for u8 {
            #[inline(always)]
            fn from(val: Pe) -> u8 {
                Pe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pen {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Pen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pen {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pen {
            #[inline(always)]
            fn from(val: u8) -> Pen {
                Pen::from_bits(val)
            }
        }
        impl From<Pen> for u8 {
            #[inline(always)]
            fn from(val: Pen) -> u8 {
                Pen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rbe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Rbe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rbe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rbe {
            #[inline(always)]
            fn from(val: u8) -> Rbe {
                Rbe::from_bits(val)
            }
        }
        impl From<Rbe> for u8 {
            #[inline(always)]
            fn from(val: Rbe) -> u8 {
                Rbe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rfe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Rfe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rfe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rfe {
            #[inline(always)]
            fn from(val: u8) -> Rfe {
                Rfe::from_bits(val)
            }
        }
        impl From<Rfe> for u8 {
            #[inline(always)]
            fn from(val: Rfe) -> u8 {
                Rfe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Roe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Roe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Roe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Roe {
            #[inline(always)]
            fn from(val: u8) -> Roe {
                Roe::from_bits(val)
            }
        }
        impl From<Roe> for u8 {
            #[inline(always)]
            fn from(val: Roe) -> u8 {
                Roe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rpe {
            #[doc = "no error"]
            NONE = 0x0,
            #[doc = "error"]
            ERROR = 0x01,
        }
        impl Rpe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rpe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rpe {
            #[inline(always)]
            fn from(val: u8) -> Rpe {
                Rpe::from_bits(val)
            }
        }
        impl From<Rpe> for u8 {
            #[inline(always)]
            fn from(val: Rpe) -> u8 {
                Rpe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rts {
            #[doc = "Low"]
            LOW = 0x0,
            #[doc = "High"]
            HIGH = 0x01,
        }
        impl Rts {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rts {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rts {
            #[inline(always)]
            fn from(val: u8) -> Rts {
                Rts::from_bits(val)
            }
        }
        impl From<Rts> for u8 {
            #[inline(always)]
            fn from(val: Rts) -> u8 {
                Rts::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtsen {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Rtsen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtsen {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtsen {
            #[inline(always)]
            fn from(val: u8) -> Rtsen {
                Rtsen::from_bits(val)
            }
        }
        impl From<Rtsen> for u8 {
            #[inline(always)]
            fn from(val: Rtsen) -> u8 {
                Rtsen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxdmae {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Rxdmae {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxdmae {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxdmae {
            #[inline(always)]
            fn from(val: u8) -> Rxdmae {
                Rxdmae::from_bits(val)
            }
        }
        impl From<Rxdmae> for u8 {
            #[inline(always)]
            fn from(val: Rxdmae) -> u8 {
                Rxdmae::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxdmsk {
            #[doc = "Normal"]
            NORMAL = 0x0,
            #[doc = "Force 1"]
            FORCE_ONE = 0x01,
        }
        impl Rxdmsk {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxdmsk {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxdmsk {
            #[inline(always)]
            fn from(val: u8) -> Rxdmsk {
                Rxdmsk::from_bits(val)
            }
        }
        impl From<Rxdmsk> for u8 {
            #[inline(always)]
            fn from(val: Rxdmsk) -> u8 {
                Rxdmsk::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxe {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Rxe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxe {
            #[inline(always)]
            fn from(val: u8) -> Rxe {
                Rxe::from_bits(val)
            }
        }
        impl From<Rxe> for u8 {
            #[inline(always)]
            fn from(val: Rxe) -> u8 {
                Rxe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rxiflsel {
            #[doc = "Receive FIFO becomes >= 1/8 full"]
            FIFO_1_8_FULL = 0x0,
            #[doc = "Receive FIFO becomes >= 1/4 full"]
            FIFO_1_4_FULL = 0x01,
            #[doc = "Receive FIFO becomes >= 1/2 full"]
            FIFO_1_2_FULL = 0x02,
            #[doc = "Receive FIFO becomes >= 3/4 full"]
            FIFO_3_4_FULL = 0x03,
            #[doc = "Receive FIFO becomes >= 7/8 full"]
            FIFO_7_8_FULL = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Rxiflsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rxiflsel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rxiflsel {
            #[inline(always)]
            fn from(val: u8) -> Rxiflsel {
                Rxiflsel::from_bits(val)
            }
        }
        impl From<Rxiflsel> for u8 {
            #[inline(always)]
            fn from(val: Rxiflsel) -> u8 {
                Rxiflsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Siren {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Siren {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Siren {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Siren {
            #[inline(always)]
            fn from(val: u8) -> Siren {
                Siren::from_bits(val)
            }
        }
        impl From<Siren> for u8 {
            #[inline(always)]
            fn from(val: Siren) -> u8 {
                Siren::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Siriinv {
            #[doc = "Normal"]
            NORMAL = 0x0,
            #[doc = "Inverted"]
            INVERTED = 0x01,
        }
        impl Siriinv {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Siriinv {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Siriinv {
            #[inline(always)]
            fn from(val: u8) -> Siriinv {
                Siriinv::from_bits(val)
            }
        }
        impl From<Siriinv> for u8 {
            #[inline(always)]
            fn from(val: Siriinv) -> u8 {
                Siriinv::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sirlp {
            #[doc = "Normal"]
            NORMAL = 0x0,
            #[doc = "Low-power"]
            LOW_POWER = 0x01,
        }
        impl Sirlp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sirlp {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sirlp {
            #[inline(always)]
            fn from(val: u8) -> Sirlp {
                Sirlp::from_bits(val)
            }
        }
        impl From<Sirlp> for u8 {
            #[inline(always)]
            fn from(val: Sirlp) -> u8 {
                Sirlp::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Siroinv {
            #[doc = "Normal"]
            NORMAL = 0x0,
            #[doc = "Inverted"]
            INVERTED = 0x01,
        }
        impl Siroinv {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Siroinv {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Siroinv {
            #[inline(always)]
            fn from(val: u8) -> Siroinv {
                Siroinv::from_bits(val)
            }
        }
        impl From<Siroinv> for u8 {
            #[inline(always)]
            fn from(val: Siroinv) -> u8 {
                Siroinv::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sps {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Sps {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sps {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sps {
            #[inline(always)]
            fn from(val: u8) -> Sps {
                Sps::from_bits(val)
            }
        }
        impl From<Sps> for u8 {
            #[inline(always)]
            fn from(val: Sps) -> u8 {
                Sps::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Stp2 {
            #[doc = "Not selected"]
            NOT_SELECTED = 0x0,
            #[doc = "Selected"]
            SELECTED = 0x01,
        }
        impl Stp2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Stp2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Stp2 {
            #[inline(always)]
            fn from(val: u8) -> Stp2 {
                Stp2::from_bits(val)
            }
        }
        impl From<Stp2> for u8 {
            #[inline(always)]
            fn from(val: Stp2) -> u8 {
                Stp2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txdmae {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Txdmae {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txdmae {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txdmae {
            #[inline(always)]
            fn from(val: u8) -> Txdmae {
                Txdmae::from_bits(val)
            }
        }
        impl From<Txdmae> for u8 {
            #[inline(always)]
            fn from(val: Txdmae) -> u8 {
                Txdmae::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txe {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Txe {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txe {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txe {
            #[inline(always)]
            fn from(val: u8) -> Txe {
                Txe::from_bits(val)
            }
        }
        impl From<Txe> for u8 {
            #[inline(always)]
            fn from(val: Txe) -> u8 {
                Txe::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txiflsel {
            #[doc = "Receive FIFO becomes >= 1/8 full"]
            FIFO_1_8_FULL = 0x0,
            #[doc = "Receive FIFO becomes >= 1/4 full"]
            FIFO_1_4_FULL = 0x01,
            #[doc = "Receive FIFO becomes >= 1/2 full"]
            FIFO_1_2_FULL = 0x02,
            #[doc = "Receive FIFO becomes >= 3/4 full"]
            FIFO_3_4_FULL = 0x03,
            #[doc = "Receive FIFO becomes >= 7/8 full"]
            FIFO_7_8_FULL = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Txiflsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txiflsel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txiflsel {
            #[inline(always)]
            fn from(val: u8) -> Txiflsel {
                Txiflsel::from_bits(val)
            }
        }
        impl From<Txiflsel> for u8 {
            #[inline(always)]
            fn from(val: Txiflsel) -> u8 {
                Txiflsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Uarten {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            #[doc = "Enabled"]
            ENABLED = 0x01,
        }
        impl Uarten {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Uarten {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Uarten {
            #[inline(always)]
            fn from(val: u8) -> Uarten {
                Uarten::from_bits(val)
            }
        }
        impl From<Uarten> for u8 {
            #[inline(always)]
            fn from(val: Uarten) -> u8 {
                Uarten::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Wlen {
            #[doc = "5 bits"]
            _5BITS = 0x0,
            #[doc = "6 bits"]
            _6BITS = 0x01,
            #[doc = "7 bits"]
            _7BITS = 0x02,
            #[doc = "8 bits"]
            _8BITS = 0x03,
        }
        impl Wlen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Wlen {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Wlen {
            #[inline(always)]
            fn from(val: u8) -> Wlen {
                Wlen::from_bits(val)
            }
        }
        impl From<Wlen> for u8 {
            #[inline(always)]
            fn from(val: Wlen) -> u8 {
                Wlen::to_bits(val)
            }
        }
    }
}
