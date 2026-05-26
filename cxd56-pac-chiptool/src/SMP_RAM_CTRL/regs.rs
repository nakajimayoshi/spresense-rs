///SRAM tile (128KB) clock gating control (0 = gated).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct APP_TILE_CLK_GATING_ENB(pub u32);
impl APP_TILE_CLK_GATING_ENB {
    ///Enable clock gating.
    #[must_use]
    #[inline(always)]
    pub const fn TILE_CLK_GATING_ENB(&self, n: usize) -> super::vals::TILE_CLK_GATING_ENB {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::TILE_CLK_GATING_ENB::from_bits(val as u8)
    }
    ///Enable clock gating.
    #[inline(always)]
    pub const fn set_TILE_CLK_GATING_ENB(
        &mut self,
        n: usize,
        val: super::vals::TILE_CLK_GATING_ENB,
    ) {
        assert!(n < 12usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for APP_TILE_CLK_GATING_ENB {
    #[inline(always)]
    fn default() -> APP_TILE_CLK_GATING_ENB {
        APP_TILE_CLK_GATING_ENB(0)
    }
}
impl core::fmt::Debug for APP_TILE_CLK_GATING_ENB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_TILE_CLK_GATING_ENB")
            .field("TILE_CLK_GATING_ENB[0]", &self.TILE_CLK_GATING_ENB(0usize))
            .field("TILE_CLK_GATING_ENB[1]", &self.TILE_CLK_GATING_ENB(1usize))
            .field("TILE_CLK_GATING_ENB[2]", &self.TILE_CLK_GATING_ENB(2usize))
            .field("TILE_CLK_GATING_ENB[3]", &self.TILE_CLK_GATING_ENB(3usize))
            .field("TILE_CLK_GATING_ENB[4]", &self.TILE_CLK_GATING_ENB(4usize))
            .field("TILE_CLK_GATING_ENB[5]", &self.TILE_CLK_GATING_ENB(5usize))
            .field("TILE_CLK_GATING_ENB[6]", &self.TILE_CLK_GATING_ENB(6usize))
            .field("TILE_CLK_GATING_ENB[7]", &self.TILE_CLK_GATING_ENB(7usize))
            .field("TILE_CLK_GATING_ENB[8]", &self.TILE_CLK_GATING_ENB(8usize))
            .field("TILE_CLK_GATING_ENB[9]", &self.TILE_CLK_GATING_ENB(9usize))
            .field(
                "TILE_CLK_GATING_ENB[10]",
                &self.TILE_CLK_GATING_ENB(10usize),
            )
            .field(
                "TILE_CLK_GATING_ENB[11]",
                &self.TILE_CLK_GATING_ENB(11usize),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for APP_TILE_CLK_GATING_ENB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "APP_TILE_CLK_GATING_ENB {{ TILE_CLK_GATING_ENB[0]: {:?}, TILE_CLK_GATING_ENB[1]: {:?}, TILE_CLK_GATING_ENB[2]: {:?}, TILE_CLK_GATING_ENB[3]: {:?}, TILE_CLK_GATING_ENB[4]: {:?}, TILE_CLK_GATING_ENB[5]: {:?}, TILE_CLK_GATING_ENB[6]: {:?}, TILE_CLK_GATING_ENB[7]: {:?}, TILE_CLK_GATING_ENB[8]: {:?}, TILE_CLK_GATING_ENB[9]: {:?}, TILE_CLK_GATING_ENB[10]: {:?}, TILE_CLK_GATING_ENB[11]: {:?} }}",
            self.TILE_CLK_GATING_ENB(0usize),
            self.TILE_CLK_GATING_ENB(1usize),
            self.TILE_CLK_GATING_ENB(2usize),
            self.TILE_CLK_GATING_ENB(3usize),
            self.TILE_CLK_GATING_ENB(4usize),
            self.TILE_CLK_GATING_ENB(5usize),
            self.TILE_CLK_GATING_ENB(6usize),
            self.TILE_CLK_GATING_ENB(7usize),
            self.TILE_CLK_GATING_ENB(8usize),
            self.TILE_CLK_GATING_ENB(9usize),
            self.TILE_CLK_GATING_ENB(10usize),
            self.TILE_CLK_GATING_ENB(11usize)
        )
    }
}
