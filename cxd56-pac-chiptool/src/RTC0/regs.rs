///Alarm flag clear register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALMCLR(pub u32);
impl ALMCLR {
    ///Clear alarm 0 flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Clear alarm 0 flag.
    #[inline(always)]
    pub const fn set_ALM0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Clear alarm 1 flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Clear alarm 1 flag.
    #[inline(always)]
    pub const fn set_ALM1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Clear alarm 2 flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Clear alarm 2 flag.
    #[inline(always)]
    pub const fn set_ALM2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///Clear alarm 0 error flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM0_ERR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Clear alarm 0 error flag.
    #[inline(always)]
    pub const fn set_ALM0_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Clear alarm 1 error flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM1_ERR(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Clear alarm 1 error flag.
    #[inline(always)]
    pub const fn set_ALM1_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for ALMCLR {
    #[inline(always)]
    fn default() -> ALMCLR {
        ALMCLR(0)
    }
}
impl core::fmt::Debug for ALMCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALMCLR")
            .field("ALM0", &self.ALM0())
            .field("ALM1", &self.ALM1())
            .field("ALM2", &self.ALM2())
            .field("ALM0_ERR", &self.ALM0_ERR())
            .field("ALM1_ERR", &self.ALM1_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALMCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALMCLR {{ ALM0: {=bool:?}, ALM1: {=bool:?}, ALM2: {=bool:?}, ALM0_ERR: {=bool:?}, ALM1_ERR: {=bool:?} }}",
            self.ALM0(),
            self.ALM1(),
            self.ALM2(),
            self.ALM0_ERR(),
            self.ALM1_ERR()
        )
    }
}
///Alarm flag status (read-only).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALMFLG(pub u32);
impl ALMFLG {
    ///Alarm 0 flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Alarm 0 flag.
    #[inline(always)]
    pub const fn set_ALM0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Alarm 1 flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Alarm 1 flag.
    #[inline(always)]
    pub const fn set_ALM1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Alarm 2 flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Alarm 2 flag.
    #[inline(always)]
    pub const fn set_ALM2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///Alarm 0 error flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM0_ERR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Alarm 0 error flag.
    #[inline(always)]
    pub const fn set_ALM0_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Alarm 1 error flag.
    #[must_use]
    #[inline(always)]
    pub const fn ALM1_ERR(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Alarm 1 error flag.
    #[inline(always)]
    pub const fn set_ALM1_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for ALMFLG {
    #[inline(always)]
    fn default() -> ALMFLG {
        ALMFLG(0)
    }
}
impl core::fmt::Debug for ALMFLG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALMFLG")
            .field("ALM0", &self.ALM0())
            .field("ALM1", &self.ALM1())
            .field("ALM2", &self.ALM2())
            .field("ALM0_ERR", &self.ALM0_ERR())
            .field("ALM1_ERR", &self.ALM1_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALMFLG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALMFLG {{ ALM0: {=bool:?}, ALM1: {=bool:?}, ALM2: {=bool:?}, ALM0_ERR: {=bool:?}, ALM1_ERR: {=bool:?} }}",
            self.ALM0(),
            self.ALM1(),
            self.ALM2(),
            self.ALM0_ERR(),
            self.ALM1_ERR()
        )
    }
}
///Alarm 0 output enable and busy status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALMOUTEN0(pub u32);
impl ALMOUTEN0 {
    ///Alarm 0 enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Alarm 0 enable.
    #[inline(always)]
    pub const fn set_ALM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Alarm 0 write busy.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_BUSY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Alarm 0 write busy.
    #[inline(always)]
    pub const fn set_ALM_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Alarm 0 debug enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_DBG(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    ///Alarm 0 debug enable.
    #[inline(always)]
    pub const fn set_ALM_DBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    ///Alarm 0 error output enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_ERREN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Alarm 0 error output enable.
    #[inline(always)]
    pub const fn set_ALM_ERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Alarm 0 error debug enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_ERRDBG(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    ///Alarm 0 error debug enable.
    #[inline(always)]
    pub const fn set_ALM_ERRDBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ALMOUTEN0 {
    #[inline(always)]
    fn default() -> ALMOUTEN0 {
        ALMOUTEN0(0)
    }
}
impl core::fmt::Debug for ALMOUTEN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALMOUTEN0")
            .field("ALM_EN", &self.ALM_EN())
            .field("ALM_BUSY", &self.ALM_BUSY())
            .field("ALM_DBG", &self.ALM_DBG())
            .field("ALM_ERREN", &self.ALM_ERREN())
            .field("ALM_ERRDBG", &self.ALM_ERRDBG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALMOUTEN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALMOUTEN0 {{ ALM_EN: {=bool:?}, ALM_BUSY: {=bool:?}, ALM_DBG: {=bool:?}, ALM_ERREN: {=bool:?}, ALM_ERRDBG: {=bool:?} }}",
            self.ALM_EN(),
            self.ALM_BUSY(),
            self.ALM_DBG(),
            self.ALM_ERREN(),
            self.ALM_ERRDBG()
        )
    }
}
///Alarm 1 output enable and busy status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALMOUTEN1(pub u32);
impl ALMOUTEN1 {
    ///Alarm 1 enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Alarm 1 enable.
    #[inline(always)]
    pub const fn set_ALM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Alarm 1 write busy.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_BUSY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Alarm 1 write busy.
    #[inline(always)]
    pub const fn set_ALM_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Alarm 1 debug enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_DBG(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    ///Alarm 1 debug enable.
    #[inline(always)]
    pub const fn set_ALM_DBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    ///Alarm 1 error output enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_ERREN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Alarm 1 error output enable.
    #[inline(always)]
    pub const fn set_ALM_ERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Alarm 1 error debug enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_ERRDBG(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    ///Alarm 1 error debug enable.
    #[inline(always)]
    pub const fn set_ALM_ERRDBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ALMOUTEN1 {
    #[inline(always)]
    fn default() -> ALMOUTEN1 {
        ALMOUTEN1(0)
    }
}
impl core::fmt::Debug for ALMOUTEN1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALMOUTEN1")
            .field("ALM_EN", &self.ALM_EN())
            .field("ALM_BUSY", &self.ALM_BUSY())
            .field("ALM_DBG", &self.ALM_DBG())
            .field("ALM_ERREN", &self.ALM_ERREN())
            .field("ALM_ERRDBG", &self.ALM_ERRDBG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALMOUTEN1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALMOUTEN1 {{ ALM_EN: {=bool:?}, ALM_BUSY: {=bool:?}, ALM_DBG: {=bool:?}, ALM_ERREN: {=bool:?}, ALM_ERRDBG: {=bool:?} }}",
            self.ALM_EN(),
            self.ALM_BUSY(),
            self.ALM_DBG(),
            self.ALM_ERREN(),
            self.ALM_ERRDBG()
        )
    }
}
///Alarm 2 output enable and busy status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALMOUTEN2(pub u32);
impl ALMOUTEN2 {
    ///Alarm 2 enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Alarm 2 enable.
    #[inline(always)]
    pub const fn set_ALM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Alarm 2 write busy.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_BUSY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Alarm 2 write busy.
    #[inline(always)]
    pub const fn set_ALM_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Alarm 2 debug enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_DBG(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    ///Alarm 2 debug enable.
    #[inline(always)]
    pub const fn set_ALM_DBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    ///Alarm 2 error output enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_ERREN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Alarm 2 error output enable.
    #[inline(always)]
    pub const fn set_ALM_ERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Alarm 2 error debug enable.
    #[must_use]
    #[inline(always)]
    pub const fn ALM_ERRDBG(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    ///Alarm 2 error debug enable.
    #[inline(always)]
    pub const fn set_ALM_ERRDBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ALMOUTEN2 {
    #[inline(always)]
    fn default() -> ALMOUTEN2 {
        ALMOUTEN2(0)
    }
}
impl core::fmt::Debug for ALMOUTEN2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALMOUTEN2")
            .field("ALM_EN", &self.ALM_EN())
            .field("ALM_BUSY", &self.ALM_BUSY())
            .field("ALM_DBG", &self.ALM_DBG())
            .field("ALM_ERREN", &self.ALM_ERREN())
            .field("ALM_ERRDBG", &self.ALM_ERRDBG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALMOUTEN2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALMOUTEN2 {{ ALM_EN: {=bool:?}, ALM_BUSY: {=bool:?}, ALM_DBG: {=bool:?}, ALM_ERREN: {=bool:?}, ALM_ERRDBG: {=bool:?} }}",
            self.ALM_EN(),
            self.ALM_BUSY(),
            self.ALM_DBG(),
            self.ALM_ERREN(),
            self.ALM_ERRDBG()
        )
    }
}
///Frequency offset correction request and busy.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OFFSETREQ(pub u32);
impl OFFSETREQ {
    ///Offset correction write busy.
    #[must_use]
    #[inline(always)]
    pub const fn ASET_BUSY(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Offset correction write busy.
    #[inline(always)]
    pub const fn set_ASET_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for OFFSETREQ {
    #[inline(always)]
    fn default() -> OFFSETREQ {
        OFFSETREQ(0)
    }
}
impl core::fmt::Debug for OFFSETREQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFFSETREQ")
            .field("ASET_BUSY", &self.ASET_BUSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OFFSETREQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OFFSETREQ {{ ASET_BUSY: {=bool:?} }}", self.ASET_BUSY())
    }
}
///Counter write request and busy status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WRREGREQ(pub u32);
impl WRREGREQ {
    ///Counter write A-side busy.
    #[must_use]
    #[inline(always)]
    pub const fn BUSYA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Counter write A-side busy.
    #[inline(always)]
    pub const fn set_BUSYA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Counter write B-side busy.
    #[must_use]
    #[inline(always)]
    pub const fn BUSYB(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Counter write B-side busy.
    #[inline(always)]
    pub const fn set_BUSYB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for WRREGREQ {
    #[inline(always)]
    fn default() -> WRREGREQ {
        WRREGREQ(0)
    }
}
impl core::fmt::Debug for WRREGREQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRREGREQ")
            .field("BUSYA", &self.BUSYA())
            .field("BUSYB", &self.BUSYB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WRREGREQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WRREGREQ {{ BUSYA: {=bool:?}, BUSYB: {=bool:?} }}",
            self.BUSYA(),
            self.BUSYB()
        )
    }
}
