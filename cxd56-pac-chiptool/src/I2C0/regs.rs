///I2C control register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_CON(pub u32);
impl IC_CON {
    ///Master mode enable.
    #[must_use]
    #[inline(always)]
    pub const fn MASTER_MODE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Master mode enable.
    #[inline(always)]
    pub const fn set_MASTER_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///I2C speed mode (1=SS 100k, 2=FS 400k, 3=HS 3.4M).
    #[must_use]
    #[inline(always)]
    pub const fn SPEED(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    ///I2C speed mode (1=SS 100k, 2=FS 400k, 3=HS 3.4M).
    #[inline(always)]
    pub const fn set_SPEED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    ///10-bit addressing for slave.
    #[must_use]
    #[inline(always)]
    pub const fn IC_10BITADDR_SLAVE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///10-bit addressing for slave.
    #[inline(always)]
    pub const fn set_IC_10BITADDR_SLAVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///10-bit addressing for master transfers.
    #[must_use]
    #[inline(always)]
    pub const fn IC_10BITADDR_MASTER(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///10-bit addressing for master transfers.
    #[inline(always)]
    pub const fn set_IC_10BITADDR_MASTER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Master restart enable.
    #[must_use]
    #[inline(always)]
    pub const fn RESTART_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Master restart enable.
    #[inline(always)]
    pub const fn set_RESTART_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Slave mode disable (1 = master only).
    #[must_use]
    #[inline(always)]
    pub const fn SLAVE_DISABLE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Slave mode disable (1 = master only).
    #[inline(always)]
    pub const fn set_SLAVE_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Issue STOP_DET only when addressed as slave.
    #[must_use]
    #[inline(always)]
    pub const fn STOP_DET_IFADDRESSED(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Issue STOP_DET only when addressed as slave.
    #[inline(always)]
    pub const fn set_STOP_DET_IFADDRESSED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Controlled TX_EMPTY interrupt generation.
    #[must_use]
    #[inline(always)]
    pub const fn TX_EMPTY_CTRL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Controlled TX_EMPTY interrupt generation.
    #[inline(always)]
    pub const fn set_TX_EMPTY_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Hold bus when RX FIFO is full.
    #[must_use]
    #[inline(always)]
    pub const fn RX_FIFO_FULL_HLD_CTRL(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Hold bus when RX FIFO is full.
    #[inline(always)]
    pub const fn set_RX_FIFO_FULL_HLD_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for IC_CON {
    #[inline(always)]
    fn default() -> IC_CON {
        IC_CON(0)
    }
}
impl core::fmt::Debug for IC_CON {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_CON")
            .field("MASTER_MODE", &self.MASTER_MODE())
            .field("SPEED", &self.SPEED())
            .field("IC_10BITADDR_SLAVE", &self.IC_10BITADDR_SLAVE())
            .field("IC_10BITADDR_MASTER", &self.IC_10BITADDR_MASTER())
            .field("RESTART_EN", &self.RESTART_EN())
            .field("SLAVE_DISABLE", &self.SLAVE_DISABLE())
            .field("STOP_DET_IFADDRESSED", &self.STOP_DET_IFADDRESSED())
            .field("TX_EMPTY_CTRL", &self.TX_EMPTY_CTRL())
            .field("RX_FIFO_FULL_HLD_CTRL", &self.RX_FIFO_FULL_HLD_CTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_CON {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_CON {{ MASTER_MODE: {=bool:?}, SPEED: {=u8:?}, IC_10BITADDR_SLAVE: {=bool:?}, IC_10BITADDR_MASTER: {=bool:?}, RESTART_EN: {=bool:?}, SLAVE_DISABLE: {=bool:?}, STOP_DET_IFADDRESSED: {=bool:?}, TX_EMPTY_CTRL: {=bool:?}, RX_FIFO_FULL_HLD_CTRL: {=bool:?} }}",
            self.MASTER_MODE(),
            self.SPEED(),
            self.IC_10BITADDR_SLAVE(),
            self.IC_10BITADDR_MASTER(),
            self.RESTART_EN(),
            self.SLAVE_DISABLE(),
            self.STOP_DET_IFADDRESSED(),
            self.TX_EMPTY_CTRL(),
            self.RX_FIFO_FULL_HLD_CTRL()
        )
    }
}
///I2C Rx/Tx data buffer and command register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_DATA_CMD(pub u32);
impl IC_DATA_CMD {
    ///Data byte for TX, or received byte in RX.
    #[must_use]
    #[inline(always)]
    pub const fn DAT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    ///Data byte for TX, or received byte in RX.
    #[inline(always)]
    pub const fn set_DAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    ///Transfer direction (0=write, 1=read).
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Transfer direction (0=write, 1=read).
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Issue STOP after this byte.
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Issue STOP after this byte.
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///Issue RESTART before this byte.
    #[must_use]
    #[inline(always)]
    pub const fn RESTART(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///Issue RESTART before this byte.
    #[inline(always)]
    pub const fn set_RESTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///First data byte after address phase (read-only).
    #[must_use]
    #[inline(always)]
    pub const fn FIRST_DATA_BYTE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///First data byte after address phase (read-only).
    #[inline(always)]
    pub const fn set_FIRST_DATA_BYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IC_DATA_CMD {
    #[inline(always)]
    fn default() -> IC_DATA_CMD {
        IC_DATA_CMD(0)
    }
}
impl core::fmt::Debug for IC_DATA_CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_DATA_CMD")
            .field("DAT", &self.DAT())
            .field("CMD", &self.CMD())
            .field("STOP", &self.STOP())
            .field("RESTART", &self.RESTART())
            .field("FIRST_DATA_BYTE", &self.FIRST_DATA_BYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_DATA_CMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_DATA_CMD {{ DAT: {=u8:?}, CMD: {=bool:?}, STOP: {=bool:?}, RESTART: {=bool:?}, FIRST_DATA_BYTE: {=bool:?} }}",
            self.DAT(),
            self.CMD(),
            self.STOP(),
            self.RESTART(),
            self.FIRST_DATA_BYTE()
        )
    }
}
///I2C DMA control register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_DMA_CR(pub u32);
impl IC_DMA_CR {
    ///RX DMA enable.
    #[must_use]
    #[inline(always)]
    pub const fn RDMAE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///RX DMA enable.
    #[inline(always)]
    pub const fn set_RDMAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///TX DMA enable.
    #[must_use]
    #[inline(always)]
    pub const fn TDMAE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///TX DMA enable.
    #[inline(always)]
    pub const fn set_TDMAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IC_DMA_CR {
    #[inline(always)]
    fn default() -> IC_DMA_CR {
        IC_DMA_CR(0)
    }
}
impl core::fmt::Debug for IC_DMA_CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_DMA_CR")
            .field("RDMAE", &self.RDMAE())
            .field("TDMAE", &self.TDMAE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_DMA_CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_DMA_CR {{ RDMAE: {=bool:?}, TDMAE: {=bool:?} }}",
            self.RDMAE(),
            self.TDMAE()
        )
    }
}
///I2C enable register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_ENABLE(pub u32);
impl IC_ENABLE {
    ///I2C controller enable.
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///I2C controller enable.
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IC_ENABLE {
    #[inline(always)]
    fn default() -> IC_ENABLE {
        IC_ENABLE(0)
    }
}
impl core::fmt::Debug for IC_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_ENABLE")
            .field("ENABLE", &self.ENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IC_ENABLE {{ ENABLE: {=bool:?} }}", self.ENABLE())
    }
}
///I2C enable status register (read-only).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_ENABLE_STATUS(pub u32);
impl IC_ENABLE_STATUS {
    ///I2C enabled (clock-synced reflection of IC_ENABLE.ENABLE).
    #[must_use]
    #[inline(always)]
    pub const fn IC_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///I2C enabled (clock-synced reflection of IC_ENABLE.ENABLE).
    #[inline(always)]
    pub const fn set_IC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Slave RX aborted during disable.
    #[must_use]
    #[inline(always)]
    pub const fn SLV_RX_ABORTED(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Slave RX aborted during disable.
    #[inline(always)]
    pub const fn set_SLV_RX_ABORTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Slave FIFO flushed after controller was disabled.
    #[must_use]
    #[inline(always)]
    pub const fn SLV_FIFO_FLUSHED(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Slave FIFO flushed after controller was disabled.
    #[inline(always)]
    pub const fn set_SLV_FIFO_FLUSHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for IC_ENABLE_STATUS {
    #[inline(always)]
    fn default() -> IC_ENABLE_STATUS {
        IC_ENABLE_STATUS(0)
    }
}
impl core::fmt::Debug for IC_ENABLE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_ENABLE_STATUS")
            .field("IC_EN", &self.IC_EN())
            .field("SLV_RX_ABORTED", &self.SLV_RX_ABORTED())
            .field("SLV_FIFO_FLUSHED", &self.SLV_FIFO_FLUSHED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_ENABLE_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_ENABLE_STATUS {{ IC_EN: {=bool:?}, SLV_RX_ABORTED: {=bool:?}, SLV_FIFO_FLUSHED: {=bool:?} }}",
            self.IC_EN(),
            self.SLV_RX_ABORTED(),
            self.SLV_FIFO_FLUSHED()
        )
    }
}
///I2C interrupt mask register (1 = unmasked).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_INTR_MASK(pub u32);
impl IC_INTR_MASK {
    ///Mask RX_UNDER interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RX_UNDER(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Mask RX_UNDER interrupt.
    #[inline(always)]
    pub const fn set_RX_UNDER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Mask RX_OVER interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RX_OVER(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Mask RX_OVER interrupt.
    #[inline(always)]
    pub const fn set_RX_OVER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Mask RX_FULL interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RX_FULL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Mask RX_FULL interrupt.
    #[inline(always)]
    pub const fn set_RX_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///Mask TX_OVER interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn TX_OVER(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///Mask TX_OVER interrupt.
    #[inline(always)]
    pub const fn set_TX_OVER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Mask TX_EMPTY interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn TX_EMPTY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Mask TX_EMPTY interrupt.
    #[inline(always)]
    pub const fn set_TX_EMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Mask RD_REQ interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RD_REQ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Mask RD_REQ interrupt.
    #[inline(always)]
    pub const fn set_RD_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Mask TX_ABRT interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn TX_ABRT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Mask TX_ABRT interrupt.
    #[inline(always)]
    pub const fn set_TX_ABRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Mask RX_DONE interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RX_DONE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Mask RX_DONE interrupt.
    #[inline(always)]
    pub const fn set_RX_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Mask ACTIVITY interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVITY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Mask ACTIVITY interrupt.
    #[inline(always)]
    pub const fn set_ACTIVITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Mask STOP_DET interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn STOP_DET(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Mask STOP_DET interrupt.
    #[inline(always)]
    pub const fn set_STOP_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///Mask START_DET interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn START_DET(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///Mask START_DET interrupt.
    #[inline(always)]
    pub const fn set_START_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///Mask GEN_CALL interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn GEN_CALL(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///Mask GEN_CALL interrupt.
    #[inline(always)]
    pub const fn set_GEN_CALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    ///Mask RESTART_DET interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn RESTART_DET(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///Mask RESTART_DET interrupt.
    #[inline(always)]
    pub const fn set_RESTART_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///Mask MST_ON_HOLD interrupt.
    #[must_use]
    #[inline(always)]
    pub const fn MST_ON_HOLD(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///Mask MST_ON_HOLD interrupt.
    #[inline(always)]
    pub const fn set_MST_ON_HOLD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for IC_INTR_MASK {
    #[inline(always)]
    fn default() -> IC_INTR_MASK {
        IC_INTR_MASK(0)
    }
}
impl core::fmt::Debug for IC_INTR_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_INTR_MASK")
            .field("RX_UNDER", &self.RX_UNDER())
            .field("RX_OVER", &self.RX_OVER())
            .field("RX_FULL", &self.RX_FULL())
            .field("TX_OVER", &self.TX_OVER())
            .field("TX_EMPTY", &self.TX_EMPTY())
            .field("RD_REQ", &self.RD_REQ())
            .field("TX_ABRT", &self.TX_ABRT())
            .field("RX_DONE", &self.RX_DONE())
            .field("ACTIVITY", &self.ACTIVITY())
            .field("STOP_DET", &self.STOP_DET())
            .field("START_DET", &self.START_DET())
            .field("GEN_CALL", &self.GEN_CALL())
            .field("RESTART_DET", &self.RESTART_DET())
            .field("MST_ON_HOLD", &self.MST_ON_HOLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_INTR_MASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_INTR_MASK {{ RX_UNDER: {=bool:?}, RX_OVER: {=bool:?}, RX_FULL: {=bool:?}, TX_OVER: {=bool:?}, TX_EMPTY: {=bool:?}, RD_REQ: {=bool:?}, TX_ABRT: {=bool:?}, RX_DONE: {=bool:?}, ACTIVITY: {=bool:?}, STOP_DET: {=bool:?}, START_DET: {=bool:?}, GEN_CALL: {=bool:?}, RESTART_DET: {=bool:?}, MST_ON_HOLD: {=bool:?} }}",
            self.RX_UNDER(),
            self.RX_OVER(),
            self.RX_FULL(),
            self.TX_OVER(),
            self.TX_EMPTY(),
            self.RD_REQ(),
            self.TX_ABRT(),
            self.RX_DONE(),
            self.ACTIVITY(),
            self.STOP_DET(),
            self.START_DET(),
            self.GEN_CALL(),
            self.RESTART_DET(),
            self.MST_ON_HOLD()
        )
    }
}
///I2C interrupt status register (read-only, masked).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_INTR_STAT(pub u32);
impl IC_INTR_STAT {
    ///RX FIFO underflow.
    #[must_use]
    #[inline(always)]
    pub const fn RX_UNDER(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///RX FIFO underflow.
    #[inline(always)]
    pub const fn set_RX_UNDER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///RX FIFO overflow.
    #[must_use]
    #[inline(always)]
    pub const fn RX_OVER(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///RX FIFO overflow.
    #[inline(always)]
    pub const fn set_RX_OVER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///RX FIFO at or above threshold.
    #[must_use]
    #[inline(always)]
    pub const fn RX_FULL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///RX FIFO at or above threshold.
    #[inline(always)]
    pub const fn set_RX_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///TX FIFO overflow.
    #[must_use]
    #[inline(always)]
    pub const fn TX_OVER(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///TX FIFO overflow.
    #[inline(always)]
    pub const fn set_TX_OVER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///TX FIFO at or below threshold.
    #[must_use]
    #[inline(always)]
    pub const fn TX_EMPTY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///TX FIFO at or below threshold.
    #[inline(always)]
    pub const fn set_TX_EMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Read request (slave mode).
    #[must_use]
    #[inline(always)]
    pub const fn RD_REQ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Read request (slave mode).
    #[inline(always)]
    pub const fn set_RD_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Transmit abort.
    #[must_use]
    #[inline(always)]
    pub const fn TX_ABRT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Transmit abort.
    #[inline(always)]
    pub const fn set_TX_ABRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///RX done (slave transmitter finished).
    #[must_use]
    #[inline(always)]
    pub const fn RX_DONE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///RX done (slave transmitter finished).
    #[inline(always)]
    pub const fn set_RX_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///I2C activity.
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVITY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///I2C activity.
    #[inline(always)]
    pub const fn set_ACTIVITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///STOP condition detected.
    #[must_use]
    #[inline(always)]
    pub const fn STOP_DET(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///STOP condition detected.
    #[inline(always)]
    pub const fn set_STOP_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///START or RESTART condition detected.
    #[must_use]
    #[inline(always)]
    pub const fn START_DET(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///START or RESTART condition detected.
    #[inline(always)]
    pub const fn set_START_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///General call received.
    #[must_use]
    #[inline(always)]
    pub const fn GEN_CALL(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///General call received.
    #[inline(always)]
    pub const fn set_GEN_CALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    ///RESTART condition detected.
    #[must_use]
    #[inline(always)]
    pub const fn RESTART_DET(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///RESTART condition detected.
    #[inline(always)]
    pub const fn set_RESTART_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///Master on hold.
    #[must_use]
    #[inline(always)]
    pub const fn MST_ON_HOLD(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///Master on hold.
    #[inline(always)]
    pub const fn set_MST_ON_HOLD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for IC_INTR_STAT {
    #[inline(always)]
    fn default() -> IC_INTR_STAT {
        IC_INTR_STAT(0)
    }
}
impl core::fmt::Debug for IC_INTR_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_INTR_STAT")
            .field("RX_UNDER", &self.RX_UNDER())
            .field("RX_OVER", &self.RX_OVER())
            .field("RX_FULL", &self.RX_FULL())
            .field("TX_OVER", &self.TX_OVER())
            .field("TX_EMPTY", &self.TX_EMPTY())
            .field("RD_REQ", &self.RD_REQ())
            .field("TX_ABRT", &self.TX_ABRT())
            .field("RX_DONE", &self.RX_DONE())
            .field("ACTIVITY", &self.ACTIVITY())
            .field("STOP_DET", &self.STOP_DET())
            .field("START_DET", &self.START_DET())
            .field("GEN_CALL", &self.GEN_CALL())
            .field("RESTART_DET", &self.RESTART_DET())
            .field("MST_ON_HOLD", &self.MST_ON_HOLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_INTR_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_INTR_STAT {{ RX_UNDER: {=bool:?}, RX_OVER: {=bool:?}, RX_FULL: {=bool:?}, TX_OVER: {=bool:?}, TX_EMPTY: {=bool:?}, RD_REQ: {=bool:?}, TX_ABRT: {=bool:?}, RX_DONE: {=bool:?}, ACTIVITY: {=bool:?}, STOP_DET: {=bool:?}, START_DET: {=bool:?}, GEN_CALL: {=bool:?}, RESTART_DET: {=bool:?}, MST_ON_HOLD: {=bool:?} }}",
            self.RX_UNDER(),
            self.RX_OVER(),
            self.RX_FULL(),
            self.TX_OVER(),
            self.TX_EMPTY(),
            self.RD_REQ(),
            self.TX_ABRT(),
            self.RX_DONE(),
            self.ACTIVITY(),
            self.STOP_DET(),
            self.START_DET(),
            self.GEN_CALL(),
            self.RESTART_DET(),
            self.MST_ON_HOLD()
        )
    }
}
///I2C raw interrupt status register (read-only, unmasked).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_RAW_INTR_STAT(pub u32);
impl IC_RAW_INTR_STAT {
    ///RX FIFO underflow (raw).
    #[must_use]
    #[inline(always)]
    pub const fn RX_UNDER(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///RX FIFO underflow (raw).
    #[inline(always)]
    pub const fn set_RX_UNDER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///RX FIFO overflow (raw).
    #[must_use]
    #[inline(always)]
    pub const fn RX_OVER(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///RX FIFO overflow (raw).
    #[inline(always)]
    pub const fn set_RX_OVER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///RX FIFO at threshold (raw).
    #[must_use]
    #[inline(always)]
    pub const fn RX_FULL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///RX FIFO at threshold (raw).
    #[inline(always)]
    pub const fn set_RX_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///TX FIFO overflow (raw).
    #[must_use]
    #[inline(always)]
    pub const fn TX_OVER(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///TX FIFO overflow (raw).
    #[inline(always)]
    pub const fn set_TX_OVER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///TX FIFO empty (raw).
    #[must_use]
    #[inline(always)]
    pub const fn TX_EMPTY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///TX FIFO empty (raw).
    #[inline(always)]
    pub const fn set_TX_EMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Read request (raw).
    #[must_use]
    #[inline(always)]
    pub const fn RD_REQ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Read request (raw).
    #[inline(always)]
    pub const fn set_RD_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Transmit abort (raw).
    #[must_use]
    #[inline(always)]
    pub const fn TX_ABRT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Transmit abort (raw).
    #[inline(always)]
    pub const fn set_TX_ABRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///RX done (raw).
    #[must_use]
    #[inline(always)]
    pub const fn RX_DONE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///RX done (raw).
    #[inline(always)]
    pub const fn set_RX_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///I2C activity (raw).
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVITY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///I2C activity (raw).
    #[inline(always)]
    pub const fn set_ACTIVITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///STOP detected (raw).
    #[must_use]
    #[inline(always)]
    pub const fn STOP_DET(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///STOP detected (raw).
    #[inline(always)]
    pub const fn set_STOP_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///START detected (raw).
    #[must_use]
    #[inline(always)]
    pub const fn START_DET(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///START detected (raw).
    #[inline(always)]
    pub const fn set_START_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///General call received (raw).
    #[must_use]
    #[inline(always)]
    pub const fn GEN_CALL(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///General call received (raw).
    #[inline(always)]
    pub const fn set_GEN_CALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    ///RESTART detected (raw).
    #[must_use]
    #[inline(always)]
    pub const fn RESTART_DET(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///RESTART detected (raw).
    #[inline(always)]
    pub const fn set_RESTART_DET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///Master on hold (raw).
    #[must_use]
    #[inline(always)]
    pub const fn MST_ON_HOLD(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///Master on hold (raw).
    #[inline(always)]
    pub const fn set_MST_ON_HOLD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for IC_RAW_INTR_STAT {
    #[inline(always)]
    fn default() -> IC_RAW_INTR_STAT {
        IC_RAW_INTR_STAT(0)
    }
}
impl core::fmt::Debug for IC_RAW_INTR_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_RAW_INTR_STAT")
            .field("RX_UNDER", &self.RX_UNDER())
            .field("RX_OVER", &self.RX_OVER())
            .field("RX_FULL", &self.RX_FULL())
            .field("TX_OVER", &self.TX_OVER())
            .field("TX_EMPTY", &self.TX_EMPTY())
            .field("RD_REQ", &self.RD_REQ())
            .field("TX_ABRT", &self.TX_ABRT())
            .field("RX_DONE", &self.RX_DONE())
            .field("ACTIVITY", &self.ACTIVITY())
            .field("STOP_DET", &self.STOP_DET())
            .field("START_DET", &self.START_DET())
            .field("GEN_CALL", &self.GEN_CALL())
            .field("RESTART_DET", &self.RESTART_DET())
            .field("MST_ON_HOLD", &self.MST_ON_HOLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_RAW_INTR_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_RAW_INTR_STAT {{ RX_UNDER: {=bool:?}, RX_OVER: {=bool:?}, RX_FULL: {=bool:?}, TX_OVER: {=bool:?}, TX_EMPTY: {=bool:?}, RD_REQ: {=bool:?}, TX_ABRT: {=bool:?}, RX_DONE: {=bool:?}, ACTIVITY: {=bool:?}, STOP_DET: {=bool:?}, START_DET: {=bool:?}, GEN_CALL: {=bool:?}, RESTART_DET: {=bool:?}, MST_ON_HOLD: {=bool:?} }}",
            self.RX_UNDER(),
            self.RX_OVER(),
            self.RX_FULL(),
            self.TX_OVER(),
            self.TX_EMPTY(),
            self.RD_REQ(),
            self.TX_ABRT(),
            self.RX_DONE(),
            self.ACTIVITY(),
            self.STOP_DET(),
            self.START_DET(),
            self.GEN_CALL(),
            self.RESTART_DET(),
            self.MST_ON_HOLD()
        )
    }
}
///I2C status register (read-only).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_STATUS(pub u32);
impl IC_STATUS {
    ///I2C activity.
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVITY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///I2C activity.
    #[inline(always)]
    pub const fn set_ACTIVITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///TX FIFO not full.
    #[must_use]
    #[inline(always)]
    pub const fn TFNF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///TX FIFO not full.
    #[inline(always)]
    pub const fn set_TFNF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///TX FIFO empty.
    #[must_use]
    #[inline(always)]
    pub const fn TFE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///TX FIFO empty.
    #[inline(always)]
    pub const fn set_TFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///RX FIFO not empty.
    #[must_use]
    #[inline(always)]
    pub const fn RFNE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///RX FIFO not empty.
    #[inline(always)]
    pub const fn set_RFNE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///RX FIFO full.
    #[must_use]
    #[inline(always)]
    pub const fn RFF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///RX FIFO full.
    #[inline(always)]
    pub const fn set_RFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Master FSM activity.
    #[must_use]
    #[inline(always)]
    pub const fn MST_ACTIVITY(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///Master FSM activity.
    #[inline(always)]
    pub const fn set_MST_ACTIVITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///Slave FSM activity.
    #[must_use]
    #[inline(always)]
    pub const fn SLV_ACTIVITY(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///Slave FSM activity.
    #[inline(always)]
    pub const fn set_SLV_ACTIVITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for IC_STATUS {
    #[inline(always)]
    fn default() -> IC_STATUS {
        IC_STATUS(0)
    }
}
impl core::fmt::Debug for IC_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_STATUS")
            .field("ACTIVITY", &self.ACTIVITY())
            .field("TFNF", &self.TFNF())
            .field("TFE", &self.TFE())
            .field("RFNE", &self.RFNE())
            .field("RFF", &self.RFF())
            .field("MST_ACTIVITY", &self.MST_ACTIVITY())
            .field("SLV_ACTIVITY", &self.SLV_ACTIVITY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_STATUS {{ ACTIVITY: {=bool:?}, TFNF: {=bool:?}, TFE: {=bool:?}, RFNE: {=bool:?}, RFF: {=bool:?}, MST_ACTIVITY: {=bool:?}, SLV_ACTIVITY: {=bool:?} }}",
            self.ACTIVITY(),
            self.TFNF(),
            self.TFE(),
            self.RFNE(),
            self.RFF(),
            self.MST_ACTIVITY(),
            self.SLV_ACTIVITY()
        )
    }
}
///I2C transmit abort source register (read-only).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IC_TX_ABRT_SOURCE(pub u32);
impl IC_TX_ABRT_SOURCE {
    ///7-bit address not ACK'd.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_7B_ADDR_NOACK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///7-bit address not ACK'd.
    #[inline(always)]
    pub const fn set_ABRT_7B_ADDR_NOACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///10-bit address byte 1 not ACK'd.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_10ADDR1_NOACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///10-bit address byte 1 not ACK'd.
    #[inline(always)]
    pub const fn set_ABRT_10ADDR1_NOACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///10-bit address byte 2 not ACK'd.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_10ADDR2_NOACK(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///10-bit address byte 2 not ACK'd.
    #[inline(always)]
    pub const fn set_ABRT_10ADDR2_NOACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///TX data byte not ACK'd.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_TXDATA_NOACK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///TX data byte not ACK'd.
    #[inline(always)]
    pub const fn set_ABRT_TXDATA_NOACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///General call not ACK'd by any slave.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_GCALL_NOACK(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///General call not ACK'd by any slave.
    #[inline(always)]
    pub const fn set_ABRT_GCALL_NOACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///General call with read bit set.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_GCALL_READ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///General call with read bit set.
    #[inline(always)]
    pub const fn set_ABRT_GCALL_READ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///HS master code ACK detected.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_HS_ACKDET(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///HS master code ACK detected.
    #[inline(always)]
    pub const fn set_ABRT_HS_ACKDET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///START byte ACK detected.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_SBYTE_ACKDET(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///START byte ACK detected.
    #[inline(always)]
    pub const fn set_ABRT_SBYTE_ACKDET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///HS mode without RESTART.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_HS_NORSTRT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///HS mode without RESTART.
    #[inline(always)]
    pub const fn set_ABRT_HS_NORSTRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///START byte without RESTART.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_SBYTE_NORSTRT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///START byte without RESTART.
    #[inline(always)]
    pub const fn set_ABRT_SBYTE_NORSTRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///10-bit read without RESTART.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_10B_RD_NORSTRT(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///10-bit read without RESTART.
    #[inline(always)]
    pub const fn set_ABRT_10B_RD_NORSTRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///Master disabled during transfer.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_MASTER_DIS(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///Master disabled during transfer.
    #[inline(always)]
    pub const fn set_ABRT_MASTER_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    ///Arbitration lost.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_ARB_LOST(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///Arbitration lost.
    #[inline(always)]
    pub const fn set_ABRT_ARB_LOST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///Slave flush TX FIFO on read command.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_SLVFLUSH_TXFIFO(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///Slave flush TX FIFO on read command.
    #[inline(always)]
    pub const fn set_ABRT_SLVFLUSH_TXFIFO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    ///Slave arbitration lost.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_SLV_ARBLOST(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    ///Slave arbitration lost.
    #[inline(always)]
    pub const fn set_ABRT_SLV_ARBLOST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    ///Slave read request in TX mode.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_SLVRD_INTX(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    ///Slave read request in TX mode.
    #[inline(always)]
    pub const fn set_ABRT_SLVRD_INTX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    ///User abort.
    #[must_use]
    #[inline(always)]
    pub const fn ABRT_USER_ABRT(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///User abort.
    #[inline(always)]
    pub const fn set_ABRT_USER_ABRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for IC_TX_ABRT_SOURCE {
    #[inline(always)]
    fn default() -> IC_TX_ABRT_SOURCE {
        IC_TX_ABRT_SOURCE(0)
    }
}
impl core::fmt::Debug for IC_TX_ABRT_SOURCE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_TX_ABRT_SOURCE")
            .field("ABRT_7B_ADDR_NOACK", &self.ABRT_7B_ADDR_NOACK())
            .field("ABRT_10ADDR1_NOACK", &self.ABRT_10ADDR1_NOACK())
            .field("ABRT_10ADDR2_NOACK", &self.ABRT_10ADDR2_NOACK())
            .field("ABRT_TXDATA_NOACK", &self.ABRT_TXDATA_NOACK())
            .field("ABRT_GCALL_NOACK", &self.ABRT_GCALL_NOACK())
            .field("ABRT_GCALL_READ", &self.ABRT_GCALL_READ())
            .field("ABRT_HS_ACKDET", &self.ABRT_HS_ACKDET())
            .field("ABRT_SBYTE_ACKDET", &self.ABRT_SBYTE_ACKDET())
            .field("ABRT_HS_NORSTRT", &self.ABRT_HS_NORSTRT())
            .field("ABRT_SBYTE_NORSTRT", &self.ABRT_SBYTE_NORSTRT())
            .field("ABRT_10B_RD_NORSTRT", &self.ABRT_10B_RD_NORSTRT())
            .field("ABRT_MASTER_DIS", &self.ABRT_MASTER_DIS())
            .field("ABRT_ARB_LOST", &self.ABRT_ARB_LOST())
            .field("ABRT_SLVFLUSH_TXFIFO", &self.ABRT_SLVFLUSH_TXFIFO())
            .field("ABRT_SLV_ARBLOST", &self.ABRT_SLV_ARBLOST())
            .field("ABRT_SLVRD_INTX", &self.ABRT_SLVRD_INTX())
            .field("ABRT_USER_ABRT", &self.ABRT_USER_ABRT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IC_TX_ABRT_SOURCE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IC_TX_ABRT_SOURCE {{ ABRT_7B_ADDR_NOACK: {=bool:?}, ABRT_10ADDR1_NOACK: {=bool:?}, ABRT_10ADDR2_NOACK: {=bool:?}, ABRT_TXDATA_NOACK: {=bool:?}, ABRT_GCALL_NOACK: {=bool:?}, ABRT_GCALL_READ: {=bool:?}, ABRT_HS_ACKDET: {=bool:?}, ABRT_SBYTE_ACKDET: {=bool:?}, ABRT_HS_NORSTRT: {=bool:?}, ABRT_SBYTE_NORSTRT: {=bool:?}, ABRT_10B_RD_NORSTRT: {=bool:?}, ABRT_MASTER_DIS: {=bool:?}, ABRT_ARB_LOST: {=bool:?}, ABRT_SLVFLUSH_TXFIFO: {=bool:?}, ABRT_SLV_ARBLOST: {=bool:?}, ABRT_SLVRD_INTX: {=bool:?}, ABRT_USER_ABRT: {=bool:?} }}",
            self.ABRT_7B_ADDR_NOACK(),
            self.ABRT_10ADDR1_NOACK(),
            self.ABRT_10ADDR2_NOACK(),
            self.ABRT_TXDATA_NOACK(),
            self.ABRT_GCALL_NOACK(),
            self.ABRT_GCALL_READ(),
            self.ABRT_HS_ACKDET(),
            self.ABRT_SBYTE_ACKDET(),
            self.ABRT_HS_NORSTRT(),
            self.ABRT_SBYTE_NORSTRT(),
            self.ABRT_10B_RD_NORSTRT(),
            self.ABRT_MASTER_DIS(),
            self.ABRT_ARB_LOST(),
            self.ABRT_SLVFLUSH_TXFIFO(),
            self.ABRT_SLV_ARBLOST(),
            self.ABRT_SLVRD_INTX(),
            self.ABRT_USER_ABRT()
        )
    }
}
