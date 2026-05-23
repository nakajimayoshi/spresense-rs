#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    acnv_p0_dst0: AcnvP0Dst0,
    acnv_p0_dst1: AcnvP0Dst1,
    acnv_p0_dst2: AcnvP0Dst2,
    acnv_p0_dst3: AcnvP0Dst3,
    acnv_p0_dst4: AcnvP0Dst4,
    acnv_p0_dst5: AcnvP0Dst5,
    acnv_p0_dst6: AcnvP0Dst6,
    acnv_p0_dst7: AcnvP0Dst7,
    acnv_p1_dst0: AcnvP1Dst0,
    acnv_p1_dst1: AcnvP1Dst1,
    acnv_p1_dst2: AcnvP1Dst2,
    acnv_p1_dst3: AcnvP1Dst3,
    acnv_p1_dst4: AcnvP1Dst4,
    acnv_p1_dst5: AcnvP1Dst5,
    acnv_p1_dst6: AcnvP1Dst6,
    acnv_p1_dst7: AcnvP1Dst7,
    acnv_p2_dst0: AcnvP2Dst0,
    acnv_p2_dst1: AcnvP2Dst1,
    acnv_p2_dst2: AcnvP2Dst2,
    acnv_p2_dst3: AcnvP2Dst3,
    acnv_p2_dst4: AcnvP2Dst4,
    acnv_p2_dst5: AcnvP2Dst5,
    acnv_p2_dst6: AcnvP2Dst6,
    acnv_p2_dst7: AcnvP2Dst7,
    acnv_p3_dst0: AcnvP3Dst0,
    acnv_p3_dst1: AcnvP3Dst1,
    acnv_p3_dst2: AcnvP3Dst2,
    acnv_p3_dst3: AcnvP3Dst3,
    acnv_p3_dst4: AcnvP3Dst4,
    acnv_p3_dst5: AcnvP3Dst5,
    acnv_p3_dst6: AcnvP3Dst6,
    acnv_p3_dst7: AcnvP3Dst7,
    acnv_p4_dst0: AcnvP4Dst0,
    acnv_p4_dst1: AcnvP4Dst1,
    acnv_p4_dst2: AcnvP4Dst2,
    acnv_p4_dst3: AcnvP4Dst3,
    acnv_p4_dst4: AcnvP4Dst4,
    acnv_p4_dst5: AcnvP4Dst5,
    acnv_p4_dst6: AcnvP4Dst6,
    acnv_p4_dst7: AcnvP4Dst7,
    acnv_p5_dst0: AcnvP5Dst0,
    acnv_p5_dst1: AcnvP5Dst1,
    acnv_p5_dst2: AcnvP5Dst2,
    acnv_p5_dst3: AcnvP5Dst3,
    acnv_p5_dst4: AcnvP5Dst4,
    acnv_p5_dst5: AcnvP5Dst5,
    acnv_p5_dst6: AcnvP5Dst6,
    acnv_p5_dst7: AcnvP5Dst7,
}
impl RegisterBlock {
    ///0x04 - CPU 0 address conversion area 0/1
    #[inline(always)]
    pub const fn acnv_p0_dst0(&self) -> &AcnvP0Dst0 {
        &self.acnv_p0_dst0
    }
    ///0x08 - CPU 0 address conversion area 2/3
    #[inline(always)]
    pub const fn acnv_p0_dst1(&self) -> &AcnvP0Dst1 {
        &self.acnv_p0_dst1
    }
    ///0x0c - CPU 0 address conversion area 4/5
    #[inline(always)]
    pub const fn acnv_p0_dst2(&self) -> &AcnvP0Dst2 {
        &self.acnv_p0_dst2
    }
    ///0x10 - CPU 0 address conversion area 6/7
    #[inline(always)]
    pub const fn acnv_p0_dst3(&self) -> &AcnvP0Dst3 {
        &self.acnv_p0_dst3
    }
    ///0x14 - CPU 0 address conversion area 8/9
    #[inline(always)]
    pub const fn acnv_p0_dst4(&self) -> &AcnvP0Dst4 {
        &self.acnv_p0_dst4
    }
    ///0x18 - CPU 0 address conversion area A/B
    #[inline(always)]
    pub const fn acnv_p0_dst5(&self) -> &AcnvP0Dst5 {
        &self.acnv_p0_dst5
    }
    ///0x1c - CPU 0 address conversion area C/D
    #[inline(always)]
    pub const fn acnv_p0_dst6(&self) -> &AcnvP0Dst6 {
        &self.acnv_p0_dst6
    }
    ///0x20 - CPU 0 address conversion area E/F
    #[inline(always)]
    pub const fn acnv_p0_dst7(&self) -> &AcnvP0Dst7 {
        &self.acnv_p0_dst7
    }
    ///0x24 - CPU 1 address conversion area 0/1
    #[inline(always)]
    pub const fn acnv_p1_dst0(&self) -> &AcnvP1Dst0 {
        &self.acnv_p1_dst0
    }
    ///0x28 - CPU 1 address conversion area 2/3
    #[inline(always)]
    pub const fn acnv_p1_dst1(&self) -> &AcnvP1Dst1 {
        &self.acnv_p1_dst1
    }
    ///0x2c - CPU 1 address conversion area 4/5
    #[inline(always)]
    pub const fn acnv_p1_dst2(&self) -> &AcnvP1Dst2 {
        &self.acnv_p1_dst2
    }
    ///0x30 - CPU 1 address conversion area 6/7
    #[inline(always)]
    pub const fn acnv_p1_dst3(&self) -> &AcnvP1Dst3 {
        &self.acnv_p1_dst3
    }
    ///0x34 - CPU 1 address conversion area 8/9
    #[inline(always)]
    pub const fn acnv_p1_dst4(&self) -> &AcnvP1Dst4 {
        &self.acnv_p1_dst4
    }
    ///0x38 - CPU 1 address conversion area A/B
    #[inline(always)]
    pub const fn acnv_p1_dst5(&self) -> &AcnvP1Dst5 {
        &self.acnv_p1_dst5
    }
    ///0x3c - CPU 1 address conversion area C/D
    #[inline(always)]
    pub const fn acnv_p1_dst6(&self) -> &AcnvP1Dst6 {
        &self.acnv_p1_dst6
    }
    ///0x40 - CPU 1 address conversion area E/F
    #[inline(always)]
    pub const fn acnv_p1_dst7(&self) -> &AcnvP1Dst7 {
        &self.acnv_p1_dst7
    }
    ///0x44 - CPU 2 address conversion area 0/1
    #[inline(always)]
    pub const fn acnv_p2_dst0(&self) -> &AcnvP2Dst0 {
        &self.acnv_p2_dst0
    }
    ///0x48 - CPU 2 address conversion area 2/3
    #[inline(always)]
    pub const fn acnv_p2_dst1(&self) -> &AcnvP2Dst1 {
        &self.acnv_p2_dst1
    }
    ///0x4c - CPU 2 address conversion area 4/5
    #[inline(always)]
    pub const fn acnv_p2_dst2(&self) -> &AcnvP2Dst2 {
        &self.acnv_p2_dst2
    }
    ///0x50 - CPU 2 address conversion area 6/7
    #[inline(always)]
    pub const fn acnv_p2_dst3(&self) -> &AcnvP2Dst3 {
        &self.acnv_p2_dst3
    }
    ///0x54 - CPU 2 address conversion area 8/9
    #[inline(always)]
    pub const fn acnv_p2_dst4(&self) -> &AcnvP2Dst4 {
        &self.acnv_p2_dst4
    }
    ///0x58 - CPU 2 address conversion area A/B
    #[inline(always)]
    pub const fn acnv_p2_dst5(&self) -> &AcnvP2Dst5 {
        &self.acnv_p2_dst5
    }
    ///0x5c - CPU 2 address conversion area C/D
    #[inline(always)]
    pub const fn acnv_p2_dst6(&self) -> &AcnvP2Dst6 {
        &self.acnv_p2_dst6
    }
    ///0x60 - CPU 2 address conversion area E/F
    #[inline(always)]
    pub const fn acnv_p2_dst7(&self) -> &AcnvP2Dst7 {
        &self.acnv_p2_dst7
    }
    ///0x64 - CPU 3 address conversion area 0/1
    #[inline(always)]
    pub const fn acnv_p3_dst0(&self) -> &AcnvP3Dst0 {
        &self.acnv_p3_dst0
    }
    ///0x68 - CPU 3 address conversion area 2/3
    #[inline(always)]
    pub const fn acnv_p3_dst1(&self) -> &AcnvP3Dst1 {
        &self.acnv_p3_dst1
    }
    ///0x6c - CPU 3 address conversion area 4/5
    #[inline(always)]
    pub const fn acnv_p3_dst2(&self) -> &AcnvP3Dst2 {
        &self.acnv_p3_dst2
    }
    ///0x70 - CPU 3 address conversion area 6/7
    #[inline(always)]
    pub const fn acnv_p3_dst3(&self) -> &AcnvP3Dst3 {
        &self.acnv_p3_dst3
    }
    ///0x74 - CPU 3 address conversion area 8/9
    #[inline(always)]
    pub const fn acnv_p3_dst4(&self) -> &AcnvP3Dst4 {
        &self.acnv_p3_dst4
    }
    ///0x78 - CPU 3 address conversion area A/B
    #[inline(always)]
    pub const fn acnv_p3_dst5(&self) -> &AcnvP3Dst5 {
        &self.acnv_p3_dst5
    }
    ///0x7c - CPU 3 address conversion area C/D
    #[inline(always)]
    pub const fn acnv_p3_dst6(&self) -> &AcnvP3Dst6 {
        &self.acnv_p3_dst6
    }
    ///0x80 - CPU 3 address conversion area E/F
    #[inline(always)]
    pub const fn acnv_p3_dst7(&self) -> &AcnvP3Dst7 {
        &self.acnv_p3_dst7
    }
    ///0x84 - CPU 4 address conversion area 0/1
    #[inline(always)]
    pub const fn acnv_p4_dst0(&self) -> &AcnvP4Dst0 {
        &self.acnv_p4_dst0
    }
    ///0x88 - CPU 4 address conversion area 2/3
    #[inline(always)]
    pub const fn acnv_p4_dst1(&self) -> &AcnvP4Dst1 {
        &self.acnv_p4_dst1
    }
    ///0x8c - CPU 4 address conversion area 4/5
    #[inline(always)]
    pub const fn acnv_p4_dst2(&self) -> &AcnvP4Dst2 {
        &self.acnv_p4_dst2
    }
    ///0x90 - CPU 4 address conversion area 6/7
    #[inline(always)]
    pub const fn acnv_p4_dst3(&self) -> &AcnvP4Dst3 {
        &self.acnv_p4_dst3
    }
    ///0x94 - CPU 4 address conversion area 8/9
    #[inline(always)]
    pub const fn acnv_p4_dst4(&self) -> &AcnvP4Dst4 {
        &self.acnv_p4_dst4
    }
    ///0x98 - CPU 4 address conversion area A/B
    #[inline(always)]
    pub const fn acnv_p4_dst5(&self) -> &AcnvP4Dst5 {
        &self.acnv_p4_dst5
    }
    ///0x9c - CPU 4 address conversion area C/D
    #[inline(always)]
    pub const fn acnv_p4_dst6(&self) -> &AcnvP4Dst6 {
        &self.acnv_p4_dst6
    }
    ///0xa0 - CPU 4 address conversion area E/F
    #[inline(always)]
    pub const fn acnv_p4_dst7(&self) -> &AcnvP4Dst7 {
        &self.acnv_p4_dst7
    }
    ///0xa4 - CPU 5 address conversion area 0/1
    #[inline(always)]
    pub const fn acnv_p5_dst0(&self) -> &AcnvP5Dst0 {
        &self.acnv_p5_dst0
    }
    ///0xa8 - CPU 5 address conversion area 2/3
    #[inline(always)]
    pub const fn acnv_p5_dst1(&self) -> &AcnvP5Dst1 {
        &self.acnv_p5_dst1
    }
    ///0xac - CPU 5 address conversion area 4/5
    #[inline(always)]
    pub const fn acnv_p5_dst2(&self) -> &AcnvP5Dst2 {
        &self.acnv_p5_dst2
    }
    ///0xb0 - CPU 5 address conversion area 6/7
    #[inline(always)]
    pub const fn acnv_p5_dst3(&self) -> &AcnvP5Dst3 {
        &self.acnv_p5_dst3
    }
    ///0xb4 - CPU 5 address conversion area 8/9
    #[inline(always)]
    pub const fn acnv_p5_dst4(&self) -> &AcnvP5Dst4 {
        &self.acnv_p5_dst4
    }
    ///0xb8 - CPU 5 address conversion area A/B
    #[inline(always)]
    pub const fn acnv_p5_dst5(&self) -> &AcnvP5Dst5 {
        &self.acnv_p5_dst5
    }
    ///0xbc - CPU 5 address conversion area C/D
    #[inline(always)]
    pub const fn acnv_p5_dst6(&self) -> &AcnvP5Dst6 {
        &self.acnv_p5_dst6
    }
    ///0xc0 - CPU 5 address conversion area E/F
    #[inline(always)]
    pub const fn acnv_p5_dst7(&self) -> &AcnvP5Dst7 {
        &self.acnv_p5_dst7
    }
}
/**ACNV_P0_DST0 (rw) register accessor: CPU 0 address conversion area 0/1

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acnv_p0_dst0`] module*/
#[doc(alias = "ACNV_P0_DST0")]
pub type AcnvP0Dst0 = crate::Reg<acnv_p0_dst0::AcnvP0Dst0Spec>;
///CPU 0 address conversion area 0/1
pub mod acnv_p0_dst0;
/**ACNV_P0_DST1 (rw) register accessor: CPU 0 address conversion area 2/3

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acnv_p0_dst1`] module*/
#[doc(alias = "ACNV_P0_DST1")]
pub type AcnvP0Dst1 = crate::Reg<acnv_p0_dst1::AcnvP0Dst1Spec>;
///CPU 0 address conversion area 2/3
pub mod acnv_p0_dst1;
/**ACNV_P0_DST2 (rw) register accessor: CPU 0 address conversion area 4/5

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acnv_p0_dst2`] module*/
#[doc(alias = "ACNV_P0_DST2")]
pub type AcnvP0Dst2 = crate::Reg<acnv_p0_dst2::AcnvP0Dst2Spec>;
///CPU 0 address conversion area 4/5
pub mod acnv_p0_dst2;
/**ACNV_P0_DST3 (rw) register accessor: CPU 0 address conversion area 6/7

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acnv_p0_dst3`] module*/
#[doc(alias = "ACNV_P0_DST3")]
pub type AcnvP0Dst3 = crate::Reg<acnv_p0_dst3::AcnvP0Dst3Spec>;
///CPU 0 address conversion area 6/7
pub mod acnv_p0_dst3;
/**ACNV_P0_DST4 (rw) register accessor: CPU 0 address conversion area 8/9

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acnv_p0_dst4`] module*/
#[doc(alias = "ACNV_P0_DST4")]
pub type AcnvP0Dst4 = crate::Reg<acnv_p0_dst4::AcnvP0Dst4Spec>;
///CPU 0 address conversion area 8/9
pub mod acnv_p0_dst4;
/**ACNV_P0_DST5 (rw) register accessor: CPU 0 address conversion area A/B

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acnv_p0_dst5`] module*/
#[doc(alias = "ACNV_P0_DST5")]
pub type AcnvP0Dst5 = crate::Reg<acnv_p0_dst5::AcnvP0Dst5Spec>;
///CPU 0 address conversion area A/B
pub mod acnv_p0_dst5;
/**ACNV_P0_DST6 (rw) register accessor: CPU 0 address conversion area C/D

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acnv_p0_dst6`] module*/
#[doc(alias = "ACNV_P0_DST6")]
pub type AcnvP0Dst6 = crate::Reg<acnv_p0_dst6::AcnvP0Dst6Spec>;
///CPU 0 address conversion area C/D
pub mod acnv_p0_dst6;
/**ACNV_P0_DST7 (rw) register accessor: CPU 0 address conversion area E/F

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acnv_p0_dst7`] module*/
#[doc(alias = "ACNV_P0_DST7")]
pub type AcnvP0Dst7 = crate::Reg<acnv_p0_dst7::AcnvP0Dst7Spec>;
///CPU 0 address conversion area E/F
pub mod acnv_p0_dst7;
pub use AcnvP0Dst0 as AcnvP1Dst0;
pub use AcnvP0Dst0 as AcnvP2Dst0;
pub use AcnvP0Dst0 as AcnvP3Dst0;
pub use AcnvP0Dst0 as AcnvP4Dst0;
pub use AcnvP0Dst0 as AcnvP5Dst0;
pub use AcnvP0Dst1 as AcnvP1Dst1;
pub use AcnvP0Dst1 as AcnvP2Dst1;
pub use AcnvP0Dst1 as AcnvP3Dst1;
pub use AcnvP0Dst1 as AcnvP4Dst1;
pub use AcnvP0Dst1 as AcnvP5Dst1;
pub use AcnvP0Dst2 as AcnvP1Dst2;
pub use AcnvP0Dst2 as AcnvP2Dst2;
pub use AcnvP0Dst2 as AcnvP3Dst2;
pub use AcnvP0Dst2 as AcnvP4Dst2;
pub use AcnvP0Dst2 as AcnvP5Dst2;
pub use AcnvP0Dst3 as AcnvP1Dst3;
pub use AcnvP0Dst3 as AcnvP2Dst3;
pub use AcnvP0Dst3 as AcnvP3Dst3;
pub use AcnvP0Dst3 as AcnvP4Dst3;
pub use AcnvP0Dst3 as AcnvP5Dst3;
pub use AcnvP0Dst4 as AcnvP1Dst4;
pub use AcnvP0Dst4 as AcnvP2Dst4;
pub use AcnvP0Dst4 as AcnvP3Dst4;
pub use AcnvP0Dst4 as AcnvP4Dst4;
pub use AcnvP0Dst4 as AcnvP5Dst4;
pub use AcnvP0Dst5 as AcnvP1Dst5;
pub use AcnvP0Dst5 as AcnvP2Dst5;
pub use AcnvP0Dst5 as AcnvP3Dst5;
pub use AcnvP0Dst5 as AcnvP4Dst5;
pub use AcnvP0Dst5 as AcnvP5Dst5;
pub use AcnvP0Dst6 as AcnvP1Dst6;
pub use AcnvP0Dst6 as AcnvP2Dst6;
pub use AcnvP0Dst6 as AcnvP3Dst6;
pub use AcnvP0Dst6 as AcnvP4Dst6;
pub use AcnvP0Dst6 as AcnvP5Dst6;
pub use AcnvP0Dst7 as AcnvP1Dst7;
pub use AcnvP0Dst7 as AcnvP2Dst7;
pub use AcnvP0Dst7 as AcnvP3Dst7;
pub use AcnvP0Dst7 as AcnvP4Dst7;
pub use AcnvP0Dst7 as AcnvP5Dst7;
pub use acnv_p0_dst0 as acnv_p1_dst0;
pub use acnv_p0_dst0 as acnv_p2_dst0;
pub use acnv_p0_dst0 as acnv_p3_dst0;
pub use acnv_p0_dst0 as acnv_p4_dst0;
pub use acnv_p0_dst0 as acnv_p5_dst0;
pub use acnv_p0_dst1 as acnv_p1_dst1;
pub use acnv_p0_dst1 as acnv_p2_dst1;
pub use acnv_p0_dst1 as acnv_p3_dst1;
pub use acnv_p0_dst1 as acnv_p4_dst1;
pub use acnv_p0_dst1 as acnv_p5_dst1;
pub use acnv_p0_dst2 as acnv_p1_dst2;
pub use acnv_p0_dst2 as acnv_p2_dst2;
pub use acnv_p0_dst2 as acnv_p3_dst2;
pub use acnv_p0_dst2 as acnv_p4_dst2;
pub use acnv_p0_dst2 as acnv_p5_dst2;
pub use acnv_p0_dst3 as acnv_p1_dst3;
pub use acnv_p0_dst3 as acnv_p2_dst3;
pub use acnv_p0_dst3 as acnv_p3_dst3;
pub use acnv_p0_dst3 as acnv_p4_dst3;
pub use acnv_p0_dst3 as acnv_p5_dst3;
pub use acnv_p0_dst4 as acnv_p1_dst4;
pub use acnv_p0_dst4 as acnv_p2_dst4;
pub use acnv_p0_dst4 as acnv_p3_dst4;
pub use acnv_p0_dst4 as acnv_p4_dst4;
pub use acnv_p0_dst4 as acnv_p5_dst4;
pub use acnv_p0_dst5 as acnv_p1_dst5;
pub use acnv_p0_dst5 as acnv_p2_dst5;
pub use acnv_p0_dst5 as acnv_p3_dst5;
pub use acnv_p0_dst5 as acnv_p4_dst5;
pub use acnv_p0_dst5 as acnv_p5_dst5;
pub use acnv_p0_dst6 as acnv_p1_dst6;
pub use acnv_p0_dst6 as acnv_p2_dst6;
pub use acnv_p0_dst6 as acnv_p3_dst6;
pub use acnv_p0_dst6 as acnv_p4_dst6;
pub use acnv_p0_dst6 as acnv_p5_dst6;
pub use acnv_p0_dst7 as acnv_p1_dst7;
pub use acnv_p0_dst7 as acnv_p2_dst7;
pub use acnv_p0_dst7 as acnv_p3_dst7;
pub use acnv_p0_dst7 as acnv_p4_dst7;
pub use acnv_p0_dst7 as acnv_p5_dst7;
