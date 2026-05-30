#[repr(C)]
///Register block
pub struct RegisterBlock {
    req: (),
    _reserved1: [u8; 0x04],
    sts: (),
}
impl RegisterBlock {
    ///0x00..0x40 - Semaphore request command (write-only)
    #[inline(always)]
    pub const fn req(&self, n: usize) -> &Req {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0x40 - Semaphore request command (write-only)
    #[inline(always)]
    pub fn req_iter(&self) -> impl Iterator<Item = &Req> {
        (0..16).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() })
    }
    ///0x04..0x44 - Semaphore status (read-only)
    #[inline(always)]
    pub const fn sts(&self, n: usize) -> &Sts {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x04..0x44 - Semaphore status (read-only)
    #[inline(always)]
    pub fn sts_iter(&self) -> impl Iterator<Item = &Sts> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        })
    }
}
/**REQ (w) register accessor: Semaphore request command (write-only)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`req::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@req`] module*/
#[doc(alias = "REQ")]
pub type Req = crate::Reg<req::ReqSpec>;
///Semaphore request command (write-only)
pub mod req;
/**STS (r) register accessor: Semaphore status (read-only)

You can [`read`](crate::Reg::read) this register and get [`sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sts`] module*/
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
///Semaphore status (read-only)
pub mod sts;
