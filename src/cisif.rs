#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intr_stat: IntrStat,
    intr_enable: IntrEnable,
    intr_disable: IntrDisable,
    intr_clear: IntrClear,
    _reserved4: [u8; 0x10],
    din_enable: DinEnable,
    cis_size: CisSize,
    act_pos: ActPos,
    act_size: ActSize,
    mode: Mode,
    ilcode: Ilcode,
    format: Format,
    pol: Pol,
    ycc_start_addr: YccStartAddr,
    ycc_darea_size: YccDareaSize,
    ycc_nstrg_size: YccNstrgSize,
    ycc_dstrg_cont: YccDstrgCont,
    _reserved16: [u8; 0x02],
    ycc_dread_cont: YccDreadCont,
    _reserved17: [u8; 0x0c],
    jpg_start_addr: JpgStartAddr,
    jpg_darea_size: JpgDareaSize,
    jpg_nstrg_size: JpgNstrgSize,
    jpg_dstrg_cont: JpgDstrgCont,
    _reserved21: [u8; 0x02],
    jpg_dread_cont: JpgDreadCont,
    _reserved22: [u8; 0x0c],
    exe_cmd: ExeCmd,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status register"]
    #[inline(always)]
    pub const fn intr_stat(&self) -> &IntrStat {
        &self.intr_stat
    }
    #[doc = "0x04 - Interrupt enable register"]
    #[inline(always)]
    pub const fn intr_enable(&self) -> &IntrEnable {
        &self.intr_enable
    }
    #[doc = "0x08 - Interrupt disable register"]
    #[inline(always)]
    pub const fn intr_disable(&self) -> &IntrDisable {
        &self.intr_disable
    }
    #[doc = "0x0c - Interrupt clear register"]
    #[inline(always)]
    pub const fn intr_clear(&self) -> &IntrClear {
        &self.intr_clear
    }
    #[doc = "0x20 - Input data enable register"]
    #[inline(always)]
    pub const fn din_enable(&self) -> &DinEnable {
        &self.din_enable
    }
    #[doc = "0x24 - CIS input activa area size setting register"]
    #[inline(always)]
    pub const fn cis_size(&self) -> &CisSize {
        &self.cis_size
    }
    #[doc = "0x28 - Active area position setting register"]
    #[inline(always)]
    pub const fn act_pos(&self) -> &ActPos {
        &self.act_pos
    }
    #[doc = "0x2c - Active area size setting register"]
    #[inline(always)]
    pub const fn act_size(&self) -> &ActSize {
        &self.act_size
    }
    #[doc = "0x30 - CIS input mode register"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x34 - CIS input in line code setting register"]
    #[inline(always)]
    pub const fn ilcode(&self) -> &Ilcode {
        &self.ilcode
    }
    #[doc = "0x38 - CIS input data format setting register"]
    #[inline(always)]
    pub const fn format(&self) -> &Format {
        &self.format
    }
    #[doc = "0x3c - CIS input Vsync/Hsync polarity setting register"]
    #[inline(always)]
    pub const fn pol(&self) -> &Pol {
        &self.pol
    }
    #[doc = "0x40 - YCC data frame memory start address"]
    #[inline(always)]
    pub const fn ycc_start_addr(&self) -> &YccStartAddr {
        &self.ycc_start_addr
    }
    #[doc = "0x44 - YCC data frame memory area size"]
    #[inline(always)]
    pub const fn ycc_darea_size(&self) -> &YccDareaSize {
        &self.ycc_darea_size
    }
    #[doc = "0x48 - YCC data frame memory notice of storage size"]
    #[inline(always)]
    pub const fn ycc_nstrg_size(&self) -> &YccNstrgSize {
        &self.ycc_nstrg_size
    }
    #[doc = "0x4c - YCC data frame memory storage size counter"]
    #[inline(always)]
    pub const fn ycc_dstrg_cont(&self) -> &YccDstrgCont {
        &self.ycc_dstrg_cont
    }
    #[doc = "0x50 - YCC data frame memory read counter"]
    #[inline(always)]
    pub const fn ycc_dread_cont(&self) -> &YccDreadCont {
        &self.ycc_dread_cont
    }
    #[doc = "0x60 - JPEG data frame memory start address"]
    #[inline(always)]
    pub const fn jpg_start_addr(&self) -> &JpgStartAddr {
        &self.jpg_start_addr
    }
    #[doc = "0x64 - JPEG data frame memory area size"]
    #[inline(always)]
    pub const fn jpg_darea_size(&self) -> &JpgDareaSize {
        &self.jpg_darea_size
    }
    #[doc = "0x68 - JPEG data frame memory notice of storage size"]
    #[inline(always)]
    pub const fn jpg_nstrg_size(&self) -> &JpgNstrgSize {
        &self.jpg_nstrg_size
    }
    #[doc = "0x6c - JPEG data frame memory storage size counter"]
    #[inline(always)]
    pub const fn jpg_dstrg_cont(&self) -> &JpgDstrgCont {
        &self.jpg_dstrg_cont
    }
    #[doc = "0x70 - JPEG data frame memory read counter"]
    #[inline(always)]
    pub const fn jpg_dread_cont(&self) -> &JpgDreadCont {
        &self.jpg_dread_cont
    }
    #[doc = "0x80 - Execution command register"]
    #[inline(always)]
    pub const fn exe_cmd(&self) -> &ExeCmd {
        &self.exe_cmd
    }
}
#[doc = "INTR_STAT (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_stat`] module"]
#[doc(alias = "INTR_STAT")]
pub type IntrStat = crate::Reg<intr_stat::IntrStatSpec>;
#[doc = "Interrupt status register"]
pub mod intr_stat;
pub use IntrStat as IntrEnable;
pub use IntrStat as IntrDisable;
pub use IntrStat as IntrClear;
pub use intr_stat as intr_enable;
pub use intr_stat as intr_disable;
pub use intr_stat as intr_clear;
#[doc = "DIN_ENABLE (rw) register accessor: Input data enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`din_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_enable`] module"]
#[doc(alias = "DIN_ENABLE")]
pub type DinEnable = crate::Reg<din_enable::DinEnableSpec>;
#[doc = "Input data enable register"]
pub mod din_enable;
#[doc = "CIS_SIZE (rw) register accessor: CIS input activa area size setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_size`] module"]
#[doc(alias = "CIS_SIZE")]
pub type CisSize = crate::Reg<cis_size::CisSizeSpec>;
#[doc = "CIS input activa area size setting register"]
pub mod cis_size;
#[doc = "ACT_POS (rw) register accessor: Active area position setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`act_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`act_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_pos`] module"]
#[doc(alias = "ACT_POS")]
pub type ActPos = crate::Reg<act_pos::ActPosSpec>;
#[doc = "Active area position setting register"]
pub mod act_pos;
#[doc = "ACT_SIZE (rw) register accessor: Active area size setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`act_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`act_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_size`] module"]
#[doc(alias = "ACT_SIZE")]
pub type ActSize = crate::Reg<act_size::ActSizeSpec>;
#[doc = "Active area size setting register"]
pub mod act_size;
#[doc = "MODE (rw) register accessor: CIS input mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "CIS input mode register"]
pub mod mode;
#[doc = "ILCODE (rw) register accessor: CIS input in line code setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`ilcode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilcode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ilcode`] module"]
#[doc(alias = "ILCODE")]
pub type Ilcode = crate::Reg<ilcode::IlcodeSpec>;
#[doc = "CIS input in line code setting register"]
pub mod ilcode;
#[doc = "FORMAT (rw) register accessor: CIS input data format setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@format`] module"]
#[doc(alias = "FORMAT")]
pub type Format = crate::Reg<format::FormatSpec>;
#[doc = "CIS input data format setting register"]
pub mod format;
#[doc = "POL (rw) register accessor: CIS input Vsync/Hsync polarity setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pol`] module"]
#[doc(alias = "POL")]
pub type Pol = crate::Reg<pol::PolSpec>;
#[doc = "CIS input Vsync/Hsync polarity setting register"]
pub mod pol;
#[doc = "YCC_START_ADDR (rw) register accessor: YCC data frame memory start address\n\nYou can [`read`](crate::Reg::read) this register and get [`ycc_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ycc_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ycc_start_addr`] module"]
#[doc(alias = "YCC_START_ADDR")]
pub type YccStartAddr = crate::Reg<ycc_start_addr::YccStartAddrSpec>;
#[doc = "YCC data frame memory start address"]
pub mod ycc_start_addr;
#[doc = "YCC_DAREA_SIZE (rw) register accessor: YCC data frame memory area size\n\nYou can [`read`](crate::Reg::read) this register and get [`ycc_darea_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ycc_darea_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ycc_darea_size`] module"]
#[doc(alias = "YCC_DAREA_SIZE")]
pub type YccDareaSize = crate::Reg<ycc_darea_size::YccDareaSizeSpec>;
#[doc = "YCC data frame memory area size"]
pub mod ycc_darea_size;
#[doc = "YCC_NSTRG_SIZE (rw) register accessor: YCC data frame memory notice of storage size\n\nYou can [`read`](crate::Reg::read) this register and get [`ycc_nstrg_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ycc_nstrg_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ycc_nstrg_size`] module"]
#[doc(alias = "YCC_NSTRG_SIZE")]
pub type YccNstrgSize = crate::Reg<ycc_nstrg_size::YccNstrgSizeSpec>;
#[doc = "YCC data frame memory notice of storage size"]
pub mod ycc_nstrg_size;
#[doc = "YCC_DSTRG_CONT (r) register accessor: YCC data frame memory storage size counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ycc_dstrg_cont::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ycc_dstrg_cont`] module"]
#[doc(alias = "YCC_DSTRG_CONT")]
pub type YccDstrgCont = crate::Reg<ycc_dstrg_cont::YccDstrgContSpec>;
#[doc = "YCC data frame memory storage size counter"]
pub mod ycc_dstrg_cont;
#[doc = "YCC_DREAD_CONT (rw) register accessor: YCC data frame memory read counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ycc_dread_cont::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ycc_dread_cont::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ycc_dread_cont`] module"]
#[doc(alias = "YCC_DREAD_CONT")]
pub type YccDreadCont = crate::Reg<ycc_dread_cont::YccDreadContSpec>;
#[doc = "YCC data frame memory read counter"]
pub mod ycc_dread_cont;
#[doc = "JPG_START_ADDR (rw) register accessor: JPEG data frame memory start address\n\nYou can [`read`](crate::Reg::read) this register and get [`jpg_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpg_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpg_start_addr`] module"]
#[doc(alias = "JPG_START_ADDR")]
pub type JpgStartAddr = crate::Reg<jpg_start_addr::JpgStartAddrSpec>;
#[doc = "JPEG data frame memory start address"]
pub mod jpg_start_addr;
#[doc = "JPG_DAREA_SIZE (rw) register accessor: JPEG data frame memory area size\n\nYou can [`read`](crate::Reg::read) this register and get [`jpg_darea_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpg_darea_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpg_darea_size`] module"]
#[doc(alias = "JPG_DAREA_SIZE")]
pub type JpgDareaSize = crate::Reg<jpg_darea_size::JpgDareaSizeSpec>;
#[doc = "JPEG data frame memory area size"]
pub mod jpg_darea_size;
#[doc = "JPG_NSTRG_SIZE (rw) register accessor: JPEG data frame memory notice of storage size\n\nYou can [`read`](crate::Reg::read) this register and get [`jpg_nstrg_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpg_nstrg_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpg_nstrg_size`] module"]
#[doc(alias = "JPG_NSTRG_SIZE")]
pub type JpgNstrgSize = crate::Reg<jpg_nstrg_size::JpgNstrgSizeSpec>;
#[doc = "JPEG data frame memory notice of storage size"]
pub mod jpg_nstrg_size;
#[doc = "JPG_DSTRG_CONT (r) register accessor: JPEG data frame memory storage size counter\n\nYou can [`read`](crate::Reg::read) this register and get [`jpg_dstrg_cont::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpg_dstrg_cont`] module"]
#[doc(alias = "JPG_DSTRG_CONT")]
pub type JpgDstrgCont = crate::Reg<jpg_dstrg_cont::JpgDstrgContSpec>;
#[doc = "JPEG data frame memory storage size counter"]
pub mod jpg_dstrg_cont;
#[doc = "JPG_DREAD_CONT (rw) register accessor: JPEG data frame memory read counter\n\nYou can [`read`](crate::Reg::read) this register and get [`jpg_dread_cont::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpg_dread_cont::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpg_dread_cont`] module"]
#[doc(alias = "JPG_DREAD_CONT")]
pub type JpgDreadCont = crate::Reg<jpg_dread_cont::JpgDreadContSpec>;
#[doc = "JPEG data frame memory read counter"]
pub mod jpg_dread_cont;
#[doc = "EXE_CMD (w) register accessor: Execution command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exe_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exe_cmd`] module"]
#[doc(alias = "EXE_CMD")]
pub type ExeCmd = crate::Reg<exe_cmd::ExeCmdSpec>;
#[doc = "Execution command register"]
pub mod exe_cmd;
