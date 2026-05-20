#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x84],
    app_tile_clk_gating_enb: AppTileClkGatingEnb,
}
impl RegisterBlock {
    #[doc = "0x84 - SRAM tile (128KB) clock gating control (0 = gated)"]
    #[inline(always)]
    pub const fn app_tile_clk_gating_enb(&self) -> &AppTileClkGatingEnb {
        &self.app_tile_clk_gating_enb
    }
}
#[doc = "APP_TILE_CLK_GATING_ENB (rw) register accessor: SRAM tile (128KB) clock gating control (0 = gated)\n\nYou can [`read`](crate::Reg::read) this register and get [`app_tile_clk_gating_enb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_tile_clk_gating_enb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_tile_clk_gating_enb`] module"]
#[doc(alias = "APP_TILE_CLK_GATING_ENB")]
pub type AppTileClkGatingEnb = crate::Reg<app_tile_clk_gating_enb::AppTileClkGatingEnbSpec>;
#[doc = "SRAM tile (128KB) clock gating control (0 = gated)"]
pub mod app_tile_clk_gating_enb;
