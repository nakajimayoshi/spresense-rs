///DesignWare DW_apb_i2c master controller (SCU_I2C0 / sensor I2C bus).
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C0 {
    ptr: *mut u8,
}
unsafe impl Send for I2C0 {}
unsafe impl Sync for I2C0 {}
impl I2C0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///I2C control register.
    #[inline(always)]
    pub const fn IC_CON(self) -> crate::common::Reg<regs::IC_CON, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    ///I2C target address register.
    #[inline(always)]
    pub const fn IC_TAR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    ///I2C slave address register.
    #[inline(always)]
    pub const fn IC_SAR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    ///I2C high speed master mode code address register.
    #[inline(always)]
    pub const fn IC_HS_MADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    ///I2C Rx/Tx data buffer and command register.
    #[inline(always)]
    pub const fn IC_DATA_CMD(self) -> crate::common::Reg<regs::IC_DATA_CMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    ///Standard speed SCL high period count.
    #[inline(always)]
    pub const fn IC_SS_SCL_HCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    ///Standard speed SCL low period count.
    #[inline(always)]
    pub const fn IC_SS_SCL_LCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    ///Fast speed SCL high period count.
    #[inline(always)]
    pub const fn IC_FS_SCL_HCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    ///Fast speed SCL low period count.
    #[inline(always)]
    pub const fn IC_FS_SCL_LCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    ///High speed SCL high period count.
    #[inline(always)]
    pub const fn IC_HS_SCL_HCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    ///High speed SCL low period count.
    #[inline(always)]
    pub const fn IC_HS_SCL_LCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    ///I2C interrupt status register (read-only, masked).
    #[inline(always)]
    pub const fn IC_INTR_STAT(self) -> crate::common::Reg<regs::IC_INTR_STAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    ///I2C interrupt mask register (1 = unmasked).
    #[inline(always)]
    pub const fn IC_INTR_MASK(self) -> crate::common::Reg<regs::IC_INTR_MASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    ///I2C raw interrupt status register (read-only, unmasked).
    #[inline(always)]
    pub const fn IC_RAW_INTR_STAT(
        self,
    ) -> crate::common::Reg<regs::IC_RAW_INTR_STAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    ///I2C receive FIFO threshold register.
    #[inline(always)]
    pub const fn IC_RX_TL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    ///I2C transmit FIFO threshold register.
    #[inline(always)]
    pub const fn IC_TX_TL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    ///Clear combined interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_INTR(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    ///Clear RX_UNDER interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_RX_UNDER(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    ///Clear RX_OVER interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_RX_OVER(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    ///Clear TX_OVER interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_TX_OVER(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    ///Clear RD_REQ interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_RD_REQ(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    ///Clear TX_ABRT interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_TX_ABRT(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    ///Clear RX_DONE interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_RX_DONE(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    ///Clear ACTIVITY interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_ACTIVITY(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    ///Clear STOP_DET interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_STOP_DET(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    ///Clear START_DET interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_START_DET(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    ///Clear GEN_CALL interrupt (read-to-clear).
    #[inline(always)]
    pub const fn IC_CLR_GEN_CALL(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    ///I2C enable register.
    #[inline(always)]
    pub const fn IC_ENABLE(self) -> crate::common::Reg<regs::IC_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    ///I2C status register (read-only).
    #[inline(always)]
    pub const fn IC_STATUS(self) -> crate::common::Reg<regs::IC_STATUS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    ///I2C transmit FIFO level (read-only).
    #[inline(always)]
    pub const fn IC_TXFLR(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    ///I2C receive FIFO level (read-only).
    #[inline(always)]
    pub const fn IC_RXFLR(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    ///I2C SDA hold time length register.
    #[inline(always)]
    pub const fn IC_SDA_HOLD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    ///I2C transmit abort source register (read-only).
    #[inline(always)]
    pub const fn IC_TX_ABRT_SOURCE(
        self,
    ) -> crate::common::Reg<regs::IC_TX_ABRT_SOURCE, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    ///Generate NACK for data bytes in slave mode.
    #[inline(always)]
    pub const fn IC_SLV_DATA_NACK_ONLY(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    ///I2C DMA control register.
    #[inline(always)]
    pub const fn IC_DMA_CR(self) -> crate::common::Reg<regs::IC_DMA_CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    ///I2C DMA transmit data level register.
    #[inline(always)]
    pub const fn IC_DMA_TDLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    ///I2C DMA receive data level register.
    #[inline(always)]
    pub const fn IC_DMA_RDLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    ///I2C SDA setup time register.
    #[inline(always)]
    pub const fn IC_SDA_SETUP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    ///I2C ACK general call register.
    #[inline(always)]
    pub const fn IC_ACK_GENERAL_CALL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    ///I2C enable status register (read-only).
    #[inline(always)]
    pub const fn IC_ENABLE_STATUS(
        self,
    ) -> crate::common::Reg<regs::IC_ENABLE_STATUS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    ///I2C SS/FS spike suppression limit register.
    #[inline(always)]
    pub const fn IC_FS_SPKLEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    ///I2C HS spike suppression limit register.
    #[inline(always)]
    pub const fn IC_HS_SPKLEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    ///Component parameter 1 (read-only, configuration encoded at synthesis).
    #[inline(always)]
    pub const fn IC_COMP_PARAM_1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    ///Component version register (read-only).
    #[inline(always)]
    pub const fn IC_COMP_VERSION(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    ///Component type register (read-only, 0x44570140 = DW_apb_i2c).
    #[inline(always)]
    pub const fn IC_COMP_TYPE(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
pub mod regs;
