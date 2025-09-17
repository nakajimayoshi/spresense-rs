#[doc = "Register `APP_TILE_CLK_GATING_ENB` reader"]
pub type R = crate::R<AppTileClkGatingEnbSpec>;
#[doc = "Register `APP_TILE_CLK_GATING_ENB` writer"]
pub type W = crate::W<AppTileClkGatingEnbSpec>;
#[doc = "Enable clock gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileClkGatingEnb {
    #[doc = "0: `0`"]
    Gated = 0,
    #[doc = "1: `1`"]
    Ungated = 1,
}
impl From<TileClkGatingEnb> for bool {
    #[inline(always)]
    fn from(variant: TileClkGatingEnb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TILE_CLK_GATING_ENB(0-11)` reader - Enable clock gating"]
pub type TileClkGatingEnbR = crate::BitReader<TileClkGatingEnb>;
impl TileClkGatingEnbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TileClkGatingEnb {
        match self.bits {
            false => TileClkGatingEnb::Gated,
            true => TileClkGatingEnb::Ungated,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == TileClkGatingEnb::Gated
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ungated(&self) -> bool {
        *self == TileClkGatingEnb::Ungated
    }
}
#[doc = "Field `TILE_CLK_GATING_ENB(0-11)` writer - Enable clock gating"]
pub type TileClkGatingEnbW<'a, REG> = crate::BitWriter<'a, REG, TileClkGatingEnb>;
impl<'a, REG> TileClkGatingEnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(TileClkGatingEnb::Gated)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ungated(self) -> &'a mut crate::W<REG> {
        self.variant(TileClkGatingEnb::Ungated)
    }
}
impl R {
    #[doc = "Enable clock gating"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TILE0_CLK_GATING_ENB` field.</div>"]
    #[inline(always)]
    pub fn tile_clk_gating_enb(&self, n: u8) -> TileClkGatingEnbR {
        #[allow(clippy::no_effect)]
        [(); 12][n as usize];
        TileClkGatingEnbR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn tile_clk_gating_enb_iter(&self) -> impl Iterator<Item = TileClkGatingEnbR> + '_ {
        (0..12).map(move |n| TileClkGatingEnbR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Enable clock gating"]
    #[inline(always)]
    pub fn tile0_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable clock gating"]
    #[inline(always)]
    pub fn tile1_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable clock gating"]
    #[inline(always)]
    pub fn tile2_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable clock gating"]
    #[inline(always)]
    pub fn tile3_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable clock gating"]
    #[inline(always)]
    pub fn tile4_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable clock gating"]
    #[inline(always)]
    pub fn tile5_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable clock gating"]
    #[inline(always)]
    pub fn tile6_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable clock gating"]
    #[inline(always)]
    pub fn tile7_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable clock gating"]
    #[inline(always)]
    pub fn tile8_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable clock gating"]
    #[inline(always)]
    pub fn tile9_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable clock gating"]
    #[inline(always)]
    pub fn tile10_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable clock gating"]
    #[inline(always)]
    pub fn tile11_clk_gating_enb(&self) -> TileClkGatingEnbR {
        TileClkGatingEnbR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Enable clock gating"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TILE0_CLK_GATING_ENB` field.</div>"]
    #[inline(always)]
    pub fn tile_clk_gating_enb(&mut self, n: u8) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        #[allow(clippy::no_effect)]
        [(); 12][n as usize];
        TileClkGatingEnbW::new(self, n)
    }
    #[doc = "Bit 0 - Enable clock gating"]
    #[inline(always)]
    pub fn tile0_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable clock gating"]
    #[inline(always)]
    pub fn tile1_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable clock gating"]
    #[inline(always)]
    pub fn tile2_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable clock gating"]
    #[inline(always)]
    pub fn tile3_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable clock gating"]
    #[inline(always)]
    pub fn tile4_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable clock gating"]
    #[inline(always)]
    pub fn tile5_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable clock gating"]
    #[inline(always)]
    pub fn tile6_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable clock gating"]
    #[inline(always)]
    pub fn tile7_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable clock gating"]
    #[inline(always)]
    pub fn tile8_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable clock gating"]
    #[inline(always)]
    pub fn tile9_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable clock gating"]
    #[inline(always)]
    pub fn tile10_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable clock gating"]
    #[inline(always)]
    pub fn tile11_clk_gating_enb(&mut self) -> TileClkGatingEnbW<'_, AppTileClkGatingEnbSpec> {
        TileClkGatingEnbW::new(self, 11)
    }
}
#[doc = "SRAM tile (128KB) clock gating control (0 = gated)\n\nYou can [`read`](crate::Reg::read) this register and get [`app_tile_clk_gating_enb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_tile_clk_gating_enb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppTileClkGatingEnbSpec;
impl crate::RegisterSpec for AppTileClkGatingEnbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_tile_clk_gating_enb::R`](R) reader structure"]
impl crate::Readable for AppTileClkGatingEnbSpec {}
#[doc = "`write(|w| ..)` method takes [`app_tile_clk_gating_enb::W`](W) writer structure"]
impl crate::Writable for AppTileClkGatingEnbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APP_TILE_CLK_GATING_ENB to value 0"]
impl crate::Resettable for AppTileClkGatingEnbSpec {}
