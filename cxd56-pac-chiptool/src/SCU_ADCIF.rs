///SCU ADC Interface — LPADC/HPADC control and per-channel FIFO read ports. CPU APB read of LPADC_FIFO(n) dequeues one sample from the hardware FIFO of LPADC channel n (no iSoP needed). UM §3.21.12.1; Mirror base 0x0418DC00.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCU_ADCIF {
    ptr: *mut u8,
}
unsafe impl Send for SCU_ADCIF {}
unsafe impl Sync for SCU_ADCIF {}
impl SCU_ADCIF {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///LPADC ch%s FIFO Read Port — dequeue one 10-bit sample (MSB-aligned in 16-bit word; DATA = bits\[15:6\] as 0..=1023).
    #[inline(always)]
    pub const fn LPADC_FIFO(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LPADC_FIFO, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    ///HPADC ch%s FIFO Read Port (SEN_AIN0/1, Arduino A4/A5).
    #[inline(always)]
    pub const fn HPADC_FIFO(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::HPADC_FIFO, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    ///LPADC Enable Control — LV_ADC_EN (UM Table ADC-106).
    #[inline(always)]
    pub const fn LPADC_A0(self) -> crate::common::Reg<regs::LPADC_A0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    ///LPADC Channel Switching Control (UM Table ADC-107).
    #[inline(always)]
    pub const fn LPADC_A1(self) -> crate::common::Reg<regs::LPADC_A1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    ///LPADC Software Reset, common to all channels (UM Table ADC-108).
    #[inline(always)]
    pub const fn LPADC_D0(self) -> crate::common::Reg<regs::LPADC_D0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    ///LPADC ch0 sample-rate divider and FIFO watermark (UM Table ADC-109).
    #[inline(always)]
    pub const fn LPADC_D1(self) -> crate::common::Reg<regs::LPADC_D1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    ///LPADC per-channel FIFO enable (UM Table ADC-110). Bit n=1 enables ADCIF hardware FIFO for channel n.
    #[inline(always)]
    pub const fn LPADC_D2(self) -> crate::common::Reg<regs::LPADC_D2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    ///LPADC ch1 sample-rate divider and FIFO watermark (UM Table ADC-111).
    #[inline(always)]
    pub const fn LPADC_D4(self) -> crate::common::Reg<regs::LPADC_D4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    ///LPADC ch2 (SEN_AIN4 / Arduino A2) sample-rate and watermark.
    #[inline(always)]
    pub const fn LPADC_D5(self) -> crate::common::Reg<regs::LPADC_D5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    ///LPADC ch3 (SEN_AIN5 / Arduino A3) sample-rate and watermark.
    #[inline(always)]
    pub const fn LPADC_D6(self) -> crate::common::Reg<regs::LPADC_D6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    ///HPADC common clock control (placeholder).
    #[inline(always)]
    pub const fn HPADC_AC0(self) -> crate::common::Reg<regs::HPADC_AC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    ///HPADC common enable control (placeholder).
    #[inline(always)]
    pub const fn HPADC_AC1(self) -> crate::common::Reg<regs::HPADC_AC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    ///HPADC two-element vector mode control (placeholder).
    #[inline(always)]
    pub const fn HPADC_DC(self) -> crate::common::Reg<regs::HPADC_DC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    ///HPADC0 clock selection (placeholder).
    #[inline(always)]
    pub const fn HPADC0_A0(self) -> crate::common::Reg<regs::HPADC0_A0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    ///HPADC0 enable control (placeholder).
    #[inline(always)]
    pub const fn HPADC0_A1(self) -> crate::common::Reg<regs::HPADC0_A1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    ///HPADC0 clock enable (placeholder).
    #[inline(always)]
    pub const fn HPADC0_A2(self) -> crate::common::Reg<regs::HPADC0_A2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    ///HPADC0 LPF control (placeholder; reset=0x100).
    #[inline(always)]
    pub const fn HPADC0_A3(self) -> crate::common::Reg<regs::HPADC0_A3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    ///HPADC0 software reset (placeholder).
    #[inline(always)]
    pub const fn HPADC0_D0(self) -> crate::common::Reg<regs::HPADC0_D0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    ///HPADC0 basic setting (placeholder; reset=0x10).
    #[inline(always)]
    pub const fn HPADC0_D1(self) -> crate::common::Reg<regs::HPADC0_D1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    ///HPADC0 ADC data acceptance (placeholder).
    #[inline(always)]
    pub const fn HPADC0_D2(self) -> crate::common::Reg<regs::HPADC0_D2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    ///HPADC1 clock selection (placeholder).
    #[inline(always)]
    pub const fn HPADC1_A0(self) -> crate::common::Reg<regs::HPADC1_A0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    ///HPADC1 enable control (placeholder).
    #[inline(always)]
    pub const fn HPADC1_A1(self) -> crate::common::Reg<regs::HPADC1_A1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    ///HPADC1 clock enable (placeholder).
    #[inline(always)]
    pub const fn HPADC1_A2(self) -> crate::common::Reg<regs::HPADC1_A2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    ///HPADC1 LPF control (placeholder; reset=0x100).
    #[inline(always)]
    pub const fn HPADC1_A3(self) -> crate::common::Reg<regs::HPADC1_A3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    ///HPADC1 software reset (placeholder).
    #[inline(always)]
    pub const fn HPADC1_D0(self) -> crate::common::Reg<regs::HPADC1_D0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    ///HPADC1 basic setting (placeholder; reset=0x10).
    #[inline(always)]
    pub const fn HPADC1_D1(self) -> crate::common::Reg<regs::HPADC1_D1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    ///HPADC1 ADC data acceptance (placeholder).
    #[inline(always)]
    pub const fn HPADC1_D2(self) -> crate::common::Reg<regs::HPADC1_D2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    ///LPADC analog timing register 0 (bit0 must be cleared, cxd56_adc.c:469).
    #[inline(always)]
    pub const fn LPADC_AT0(self) -> crate::common::Reg<regs::LPADC_AT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    ///LPADC analog warm-up timing (SDK sets bits\[7:0\]=0x04, cxd56_adc.c:462).
    #[inline(always)]
    pub const fn LPADC_AT1(self) -> crate::common::Reg<regs::LPADC_AT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    ///ADCIF register-map revision code (RO; reads 0xADC1F000).
    #[inline(always)]
    pub const fn ADCIF_DCT(self) -> crate::common::Reg<regs::ADCIF_DCT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    ///ADCIF clock and power control (reset=0x1 = clocked; write 0 to gate).
    #[inline(always)]
    pub const fn SCU_ADCIF_CKPOWER(
        self,
    ) -> crate::common::Reg<regs::SCU_ADCIF_CKPOWER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
}
pub mod regs;
