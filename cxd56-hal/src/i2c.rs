use crate::clocks::{ClockError, Clocks, PeripheralId};
use crate::pac;
use embedded_hal::i2c::{ErrorKind, ErrorType, I2c, Operation};
use fugit::Hertz;

#[derive(Debug)]
pub enum I2cError {
    /// Address not acknowledged (no device at that address).
    NoAck,
    /// Bus arbitration lost.
    Arbitration,
    /// Operation timed out waiting for FIFO or controller.
    Timeout,
    /// Clock enable failed.
    Clock(ClockError),
}

impl From<ClockError> for I2cError {
    fn from(e: ClockError) -> Self {
        Self::Clock(e)
    }
}

impl embedded_hal::i2c::Error for I2cError {
    fn kind(&self) -> ErrorKind {
        match self {
            I2cError::NoAck => ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Unknown),
            I2cError::Arbitration => ErrorKind::ArbitrationLoss,
            I2cError::Timeout => ErrorKind::Other,
            I2cError::Clock(_) => ErrorKind::Other,
        }
    }
}

pub struct I2cConfig {
    pub freq: Hertz<u32>,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            freq: Hertz::<u32>::kHz(400),
        }
    }
}

pub struct I2c0 {
    i2c: pac::I2c0,
    scu_hz: u32,
    config: I2cConfig,
}

// I2C0_BCK = SCL, I2C0_BDT = SDA — TOPREG offsets 0x8b0/0x8b4.
// Both pads: 2 mA drive (LOWEMI=1), pull-up (PUN=1), input enabled (ENZI=1)
// for open-drain operation. IOCSYS_IOMD1[19:18] = 1 (Func1 → I2C0).
fn i2c0_pinmux() {
    let t = unsafe { &*pac::Topreg::PTR };
    t.io_i2c0_bck()
        .write(|w| w.lowemi().set_bit().pun().set_bit().enzi().set_bit());
    t.io_i2c0_bdt()
        .write(|w| w.lowemi().set_bit().pun().set_bit().enzi().set_bit());
    t.iocsys_iomd1()
        .modify(|_, w| unsafe { w.i2c0().bits(1) });
}

// Disable the controller and wait for IC_ENABLE_STATUS.IC_EN to clear.
// The DW_apb_i2c manual requires this before changing IC_CON or IC_TAR.
fn ic_disable(i2c: &pac::I2c0) -> Result<(), I2cError> {
    i2c.ic_enable().write(|w| unsafe { w.bits(0) });
    let mut retry = 25_000u32;
    while i2c.ic_enable_status().read().ic_en().bit_is_set() {
        retry = retry.checked_sub(1).ok_or(I2cError::Timeout)?;
    }
    Ok(())
}

// Compute SCL high/low counts from the SCU base clock and a target frequency.
// Mirrors cxd56_i2c_setfrequency (cxd56_i2c.c). Controller must be disabled.
fn set_frequency(i2c: &pac::I2c0, base_hz: u32, target: Hertz<u32>) {
    let target_hz = target.to_Hz();
    let (t_high, t_low, spklen, use_fs_regs): (u64, u64, u64, bool) = if target_hz <= 200_000 {
        (40, 47, 4, false) // standard-speed
    } else {
        (6, 13, 1, true) // fast-speed
    };

    let base_khz = base_hz as u64 / 1_000;
    let hcnt = ((base_khz * t_high).div_ceil(1_000_000)).saturating_sub(spklen + 7).max(1) as u32;
    let lcnt = ((base_khz * t_low).div_ceil(1_000_000)).max(1) as u32;

    if use_fs_regs {
        i2c.ic_fs_scl_hcnt()
            .write(|w| unsafe { w.bits(hcnt) });
        i2c.ic_fs_scl_lcnt()
            .write(|w| unsafe { w.bits(lcnt) });
    } else {
        i2c.ic_ss_scl_hcnt()
            .write(|w| unsafe { w.bits(hcnt) });
        i2c.ic_ss_scl_lcnt()
            .write(|w| unsafe { w.bits(lcnt) });
    }
}

impl I2c0 {
    /// Initialise I2C0 and configure it as an I2C master.
    ///
    /// Enables the SCU clock domain, muxes the I2C0_BCK/BDT pads, and
    /// programs the DW_apb_i2c control registers. Must be called after
    /// `Clocks::freeze`. Uses the SCU clock as the I2C bit-rate source.
    pub fn new(i2c: pac::I2c0, clocks: &Clocks, config: I2cConfig) -> Result<Self, I2cError> {
        PeripheralId::I2c0.enable()?;
        i2c0_pinmux();

        ic_disable(&i2c)?;

        // Mask all interrupts (we poll status registers, not IRQs).
        i2c.ic_intr_mask().write(|w| unsafe { w.bits(0) });
        // Read CLR_INTR to clear any pending interrupts.
        let _ = i2c.ic_clr_intr().read();

        // RX threshold: interrupt when FIFO is full (0xff = 255, effectively
        // "drain manually"). TX threshold: 0 = interrupt when FIFO empty.
        i2c.ic_rx_tl().write(|w| unsafe { w.bits(0xff) });
        i2c.ic_tx_tl().write(|w| unsafe { w.bits(0) });

        // SDA hold = 1 cycle (minimum).
        i2c.ic_sda_hold().write(|w| unsafe { w.bits(1) });

        // IC_CON: master mode, SPEED=FS (2), restart enable, slave disable,
        // TX_EMPTY_CTRL, RX_FIFO_FULL_HLD_CTRL. SPEED field value 2 is used
        // for both SS and FS — the actual timing comes from the hcnt/lcnt
        // registers, not this field. Mirrors NuttX cxd56_i2c.c init.
        i2c.ic_con().write(|w| unsafe {
            w.bits(1 | (2 << 1) | (1 << 5) | (1 << 6) | (1 << 8) | (1 << 9))
        });

        let scu_hz = clocks.scu.to_Hz();
        set_frequency(&i2c, scu_hz, config.freq);

        Ok(Self { i2c, scu_hz, config })
    }

    // Prepare the controller for a transaction: disable, re-apply frequency
    // and target address, then re-enable.
    fn begin_transfer(&mut self, addr: u8) -> Result<(), I2cError> {
        ic_disable(&self.i2c)?;
        set_frequency(&self.i2c, self.scu_hz, self.config.freq);
        self.i2c
            .ic_tar()
            .write(|w| unsafe { w.bits(addr as u32 & 0x7f) });
        self.i2c.ic_enable().write(|w| unsafe { w.bits(1) });
        Ok(())
    }

    // Check IC_RAW_INTR_STAT for TX_ABRT and translate to an I2cError.
    // Clears the abort latch via CLR_TX_ABRT and CLR_INTR.
    fn check_abort(&self) -> Result<(), I2cError> {
        let raw = self.i2c.ic_raw_intr_stat().read();
        if raw.tx_abrt().bit_is_set() {
            let src = self.i2c.ic_tx_abrt_source().read();
            let _ = self.i2c.ic_clr_tx_abrt().read();
            let _ = self.i2c.ic_clr_intr().read();
            if src.abrt_arb_lost().bit_is_set() {
                return Err(I2cError::Arbitration);
            }
            // NOACK on address or data → NoAck
            return Err(I2cError::NoAck);
        }
        Ok(())
    }

    fn write_bytes(&mut self, buf: &[u8]) -> Result<(), I2cError> {
        let len = buf.len();
        for (i, &byte) in buf.iter().enumerate() {
            // Poll TX FIFO not-full with timeout.
            let mut retry = 25_000u32;
            while !self.i2c.ic_status().read().tfnf().bit_is_set() {
                self.check_abort()?;
                retry = retry.checked_sub(1).ok_or(I2cError::Timeout)?;
            }
            let stop = if i == len - 1 { 1u32 << 9 } else { 0 };
            self.i2c
                .ic_data_cmd()
                .write(|w| unsafe { w.bits(byte as u32 | stop) });
        }

        // Wait for TX_EMPTY (all bytes shifted out).
        let mut retry = 100_000u32;
        loop {
            self.check_abort()?;
            if self.i2c.ic_raw_intr_stat().read().tx_empty().bit_is_set() {
                break;
            }
            retry = retry.checked_sub(1).ok_or(I2cError::Timeout)?;
        }

        let _ = self.i2c.ic_clr_intr().read();
        Ok(())
    }

    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<(), I2cError> {
        let len = buf.len();
        for (i, slot) in buf.iter_mut().enumerate() {
            let stop = if i == len - 1 { 1u32 << 9 } else { 0 };
            // CMD_READ (bit 8) queues a read command.
            self.i2c
                .ic_data_cmd()
                .write(|w| unsafe { w.bits((1 << 8) | stop) });

            // Poll RX FIFO not-empty with timeout.
            let mut retry = 25_000u32;
            while !self.i2c.ic_status().read().rfne().bit_is_set() {
                self.check_abort()?;
                retry = retry.checked_sub(1).ok_or(I2cError::Timeout)?;
            }
            *slot = (self.i2c.ic_data_cmd().read().bits() & 0xff) as u8;
        }

        self.check_abort()?;
        let _ = self.i2c.ic_clr_intr().read();
        Ok(())
    }
}

impl ErrorType for I2c0 {
    type Error = I2cError;
}

impl I2c for I2c0 {
    fn transaction(
        &mut self,
        addr: u8,
        ops: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.begin_transfer(addr)?;
        for op in ops {
            match op {
                Operation::Write(buf) => self.write_bytes(buf)?,
                Operation::Read(buf) => self.read_bytes(buf)?,
            }
        }
        Ok(())
    }
}
