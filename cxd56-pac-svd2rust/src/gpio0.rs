#[repr(C)]
///Register block
pub struct RegisterBlock {
    gp_i2c4_bck: GpI2c4Bck,
    _reserved1: [u8; 0x0164],
    pin97: Pin97,
}
impl RegisterBlock {
    ///0x00 - GPIO SYS pin 1 — I2C4 clock / Arduino D14
    #[inline(always)]
    pub const fn gp_i2c4_bck(&self) -> &GpI2c4Bck {
        &self.gp_i2c4_bck
    }
    ///0x168 - GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board
    #[inline(always)]
    pub const fn pin97(&self) -> &Pin97 {
        &self.pin97
    }
}
/**GP_I2C4_BCK (rw) register accessor: GPIO SYS pin 1 — I2C4 clock / Arduino D14

You can [`read`](crate::Reg::read) this register and get [`gp_i2c4_bck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2c4_bck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gp_i2c4_bck`] module*/
#[doc(alias = "GP_I2C4_BCK")]
pub type GpI2c4Bck = crate::Reg<gp_i2c4_bck::GpI2c4BckSpec>;
///GPIO SYS pin 1 — I2C4 clock / Arduino D14
pub mod gp_i2c4_bck;
/**PIN97 (rw) register accessor: GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`pin97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin97`] module*/
#[doc(alias = "PIN97")]
pub type Pin97 = crate::Reg<pin97::Pin97Spec>;
///GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board
pub mod pin97;
