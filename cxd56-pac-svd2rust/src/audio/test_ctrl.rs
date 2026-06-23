///Register `TEST_CTRL` reader
pub type R = crate::R<TestCtrlSpec>;
///Register `TEST_CTRL` writer
pub type W = crate::W<TestCtrlSpec>;
///Field `TEST_OUT_SEL0` reader - SRC1 (I2S0 path) filter bypass (1=bypass)
pub type TestOutSel0R = crate::BitReader;
///Field `TEST_OUT_SEL0` writer - SRC1 (I2S0 path) filter bypass (1=bypass)
pub type TestOutSel0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEST_OUT_SEL1` reader - SRC2 (I2S1 path) filter bypass (1=bypass)
pub type TestOutSel1R = crate::BitReader;
///Field `TEST_OUT_SEL1` writer - SRC2 (I2S1 path) filter bypass (1=bypass)
pub type TestOutSel1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S_RESET` reader - Audio codec soft reset
pub type SResetR = crate::BitReader;
///Field `S_RESET` writer - Audio codec soft reset
pub type SResetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HALT_INHIBIT` reader - Inhibit SRC clock halt (0=allow SRC clock halt)
pub type HaltInhibitR = crate::BitReader;
///Field `HALT_INHIBIT` writer - Inhibit SRC clock halt (0=allow SRC clock halt)
pub type HaltInhibitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARWPHSET` reader - SRC auto mute (0=disable)
pub type ArwphsetR = crate::BitReader;
///Field `ARWPHSET` writer - SRC auto mute (0=disable)
pub type ArwphsetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSPRAM4_CLR` reader - Clear DSP RAM bank 4 (codec initdsp). NuttX RI_DSPRAM4_CLR
pub type Dspram4ClrR = crate::BitReader;
///Field `DSPRAM4_CLR` writer - Clear DSP RAM bank 4 (codec initdsp). NuttX RI_DSPRAM4_CLR
pub type Dspram4ClrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSPRAM2_CLR` reader - Clear DSP RAM bank 2 (codec initdsp). NuttX RI_DSPRAM2_CLR
pub type Dspram2ClrR = crate::BitReader;
///Field `DSPRAM2_CLR` writer - Clear DSP RAM bank 2 (codec initdsp). NuttX RI_DSPRAM2_CLR
pub type Dspram2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSPRAM1_CLR` reader - Clear DSP RAM bank 1 (codec initdsp). NuttX RI_DSPRAM1_CLR
pub type Dspram1ClrR = crate::BitReader;
///Field `DSPRAM1_CLR` writer - Clear DSP RAM bank 1 (codec initdsp). NuttX RI_DSPRAM1_CLR
pub type Dspram1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - SRC1 (I2S0 path) filter bypass (1=bypass)
    #[inline(always)]
    pub fn test_out_sel0(&self) -> TestOutSel0R {
        TestOutSel0R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SRC2 (I2S1 path) filter bypass (1=bypass)
    #[inline(always)]
    pub fn test_out_sel1(&self) -> TestOutSel1R {
        TestOutSel1R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Audio codec soft reset
    #[inline(always)]
    pub fn s_reset(&self) -> SResetR {
        SResetR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - Inhibit SRC clock halt (0=allow SRC clock halt)
    #[inline(always)]
    pub fn halt_inhibit(&self) -> HaltInhibitR {
        HaltInhibitR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - SRC auto mute (0=disable)
    #[inline(always)]
    pub fn arwphset(&self) -> ArwphsetR {
        ArwphsetR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 28 - Clear DSP RAM bank 4 (codec initdsp). NuttX RI_DSPRAM4_CLR
    #[inline(always)]
    pub fn dspram4_clr(&self) -> Dspram4ClrR {
        Dspram4ClrR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Clear DSP RAM bank 2 (codec initdsp). NuttX RI_DSPRAM2_CLR
    #[inline(always)]
    pub fn dspram2_clr(&self) -> Dspram2ClrR {
        Dspram2ClrR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Clear DSP RAM bank 1 (codec initdsp). NuttX RI_DSPRAM1_CLR
    #[inline(always)]
    pub fn dspram1_clr(&self) -> Dspram1ClrR {
        Dspram1ClrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - SRC1 (I2S0 path) filter bypass (1=bypass)
    #[inline(always)]
    pub fn test_out_sel0(&mut self) -> TestOutSel0W<'_, TestCtrlSpec> {
        TestOutSel0W::new(self, 6)
    }
    ///Bit 7 - SRC2 (I2S1 path) filter bypass (1=bypass)
    #[inline(always)]
    pub fn test_out_sel1(&mut self) -> TestOutSel1W<'_, TestCtrlSpec> {
        TestOutSel1W::new(self, 7)
    }
    ///Bit 16 - Audio codec soft reset
    #[inline(always)]
    pub fn s_reset(&mut self) -> SResetW<'_, TestCtrlSpec> {
        SResetW::new(self, 16)
    }
    ///Bit 19 - Inhibit SRC clock halt (0=allow SRC clock halt)
    #[inline(always)]
    pub fn halt_inhibit(&mut self) -> HaltInhibitW<'_, TestCtrlSpec> {
        HaltInhibitW::new(self, 19)
    }
    ///Bit 23 - SRC auto mute (0=disable)
    #[inline(always)]
    pub fn arwphset(&mut self) -> ArwphsetW<'_, TestCtrlSpec> {
        ArwphsetW::new(self, 23)
    }
    ///Bit 28 - Clear DSP RAM bank 4 (codec initdsp). NuttX RI_DSPRAM4_CLR
    #[inline(always)]
    pub fn dspram4_clr(&mut self) -> Dspram4ClrW<'_, TestCtrlSpec> {
        Dspram4ClrW::new(self, 28)
    }
    ///Bit 30 - Clear DSP RAM bank 2 (codec initdsp). NuttX RI_DSPRAM2_CLR
    #[inline(always)]
    pub fn dspram2_clr(&mut self) -> Dspram2ClrW<'_, TestCtrlSpec> {
        Dspram2ClrW::new(self, 30)
    }
    ///Bit 31 - Clear DSP RAM bank 1 (codec initdsp). NuttX RI_DSPRAM1_CLR
    #[inline(always)]
    pub fn dspram1_clr(&mut self) -> Dspram1ClrW<'_, TestCtrlSpec> {
        Dspram1ClrW::new(self, 31)
    }
}
/**Audio test/control — SRC filter bypass, soft reset, SRC clock-halt inhibit

You can [`read`](crate::Reg::read) this register and get [`test_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TestCtrlSpec;
impl crate::RegisterSpec for TestCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`test_ctrl::R`](R) reader structure
impl crate::Readable for TestCtrlSpec {}
///`write(|w| ..)` method takes [`test_ctrl::W`](W) writer structure
impl crate::Writable for TestCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEST_CTRL to value 0x0008_0000
impl crate::Resettable for TestCtrlSpec {
    const RESET_VALUE: u32 = 0x0008_0000;
}
