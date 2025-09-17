#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intr_status: IntrStatus,
    intr_enable: IntrEnable,
    intr_disable: IntrDisable,
    intr_clear: IntrClear,
    command: Command,
    set_direction: SetDirection,
    set_src_hsize: SetSrcHsize,
    set_src_vsize: SetSrcVsize,
    set_src_address: SetSrcAddress,
    set_src_pitch: SetSrcPitch,
    set_dst_address: SetDstAddress,
    set_dst_pitch: SetDstPitch,
    status: Status,
    conv_ctrl: ConvCtrl,
    rgb_alignment: RgbAlignment,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Status"]
    #[inline(always)]
    pub const fn intr_status(&self) -> &IntrStatus {
        &self.intr_status
    }
    #[doc = "0x04 - Interrupt Enable"]
    #[inline(always)]
    pub const fn intr_enable(&self) -> &IntrEnable {
        &self.intr_enable
    }
    #[doc = "0x08 - Interrupt Disable"]
    #[inline(always)]
    pub const fn intr_disable(&self) -> &IntrDisable {
        &self.intr_disable
    }
    #[doc = "0x0c - Interrupt Clear"]
    #[inline(always)]
    pub const fn intr_clear(&self) -> &IntrClear {
        &self.intr_clear
    }
    #[doc = "0x10 - Start rotation processing"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x14 - Set Rotation Angle"]
    #[inline(always)]
    pub const fn set_direction(&self) -> &SetDirection {
        &self.set_direction
    }
    #[doc = "0x18 - Source Image Horizontal Size (Actual size - 1)"]
    #[inline(always)]
    pub const fn set_src_hsize(&self) -> &SetSrcHsize {
        &self.set_src_hsize
    }
    #[doc = "0x1c - Source Image Vertical Size (Actual size - 1)"]
    #[inline(always)]
    pub const fn set_src_vsize(&self) -> &SetSrcVsize {
        &self.set_src_vsize
    }
    #[doc = "0x20 - Source Image Address"]
    #[inline(always)]
    pub const fn set_src_address(&self) -> &SetSrcAddress {
        &self.set_src_address
    }
    #[doc = "0x24 - Source Image Pitch (Actual size - 1)"]
    #[inline(always)]
    pub const fn set_src_pitch(&self) -> &SetSrcPitch {
        &self.set_src_pitch
    }
    #[doc = "0x28 - Destination Address"]
    #[inline(always)]
    pub const fn set_dst_address(&self) -> &SetDstAddress {
        &self.set_dst_address
    }
    #[doc = "0x2c - Destination Image Pitch (Actual size - 1)"]
    #[inline(always)]
    pub const fn set_dst_pitch(&self) -> &SetDstPitch {
        &self.set_dst_pitch
    }
    #[doc = "0x30 - Running Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x34 - Color Convertion Control"]
    #[inline(always)]
    pub const fn conv_ctrl(&self) -> &ConvCtrl {
        &self.conv_ctrl
    }
    #[doc = "0x38 - RGB format selector"]
    #[inline(always)]
    pub const fn rgb_alignment(&self) -> &RgbAlignment {
        &self.rgb_alignment
    }
}
#[doc = "INTR_STATUS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status`] module"]
#[doc(alias = "INTR_STATUS")]
pub type IntrStatus = crate::Reg<intr_status::IntrStatusSpec>;
#[doc = "Interrupt Status"]
pub mod intr_status;
#[doc = "INTR_ENABLE (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enable`] module"]
#[doc(alias = "INTR_ENABLE")]
pub type IntrEnable = crate::Reg<intr_enable::IntrEnableSpec>;
#[doc = "Interrupt Enable"]
pub mod intr_enable;
#[doc = "INTR_DISABLE (w) register accessor: Interrupt Disable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_disable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_disable`] module"]
#[doc(alias = "INTR_DISABLE")]
pub type IntrDisable = crate::Reg<intr_disable::IntrDisableSpec>;
#[doc = "Interrupt Disable"]
pub mod intr_disable;
#[doc = "INTR_CLEAR (w) register accessor: Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_clear`] module"]
#[doc(alias = "INTR_CLEAR")]
pub type IntrClear = crate::Reg<intr_clear::IntrClearSpec>;
#[doc = "Interrupt Clear"]
pub mod intr_clear;
#[doc = "COMMAND (w) register accessor: Start rotation processing\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Start rotation processing"]
pub mod command;
#[doc = "SET_DIRECTION (rw) register accessor: Set Rotation Angle\n\nYou can [`read`](crate::Reg::read) this register and get [`set_direction::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_direction::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_direction`] module"]
#[doc(alias = "SET_DIRECTION")]
pub type SetDirection = crate::Reg<set_direction::SetDirectionSpec>;
#[doc = "Set Rotation Angle"]
pub mod set_direction;
#[doc = "SET_SRC_HSIZE (rw) register accessor: Source Image Horizontal Size (Actual size - 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`set_src_hsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_src_hsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_src_hsize`] module"]
#[doc(alias = "SET_SRC_HSIZE")]
pub type SetSrcHsize = crate::Reg<set_src_hsize::SetSrcHsizeSpec>;
#[doc = "Source Image Horizontal Size (Actual size - 1)"]
pub mod set_src_hsize;
#[doc = "SET_SRC_VSIZE (rw) register accessor: Source Image Vertical Size (Actual size - 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`set_src_vsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_src_vsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_src_vsize`] module"]
#[doc(alias = "SET_SRC_VSIZE")]
pub type SetSrcVsize = crate::Reg<set_src_vsize::SetSrcVsizeSpec>;
#[doc = "Source Image Vertical Size (Actual size - 1)"]
pub mod set_src_vsize;
#[doc = "SET_SRC_ADDRESS (rw) register accessor: Source Image Address\n\nYou can [`read`](crate::Reg::read) this register and get [`set_src_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_src_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_src_address`] module"]
#[doc(alias = "SET_SRC_ADDRESS")]
pub type SetSrcAddress = crate::Reg<set_src_address::SetSrcAddressSpec>;
#[doc = "Source Image Address"]
pub mod set_src_address;
#[doc = "SET_SRC_PITCH (rw) register accessor: Source Image Pitch (Actual size - 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`set_src_pitch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_src_pitch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_src_pitch`] module"]
#[doc(alias = "SET_SRC_PITCH")]
pub type SetSrcPitch = crate::Reg<set_src_pitch::SetSrcPitchSpec>;
#[doc = "Source Image Pitch (Actual size - 1)"]
pub mod set_src_pitch;
#[doc = "SET_DST_ADDRESS (rw) register accessor: Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`set_dst_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_dst_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_dst_address`] module"]
#[doc(alias = "SET_DST_ADDRESS")]
pub type SetDstAddress = crate::Reg<set_dst_address::SetDstAddressSpec>;
#[doc = "Destination Address"]
pub mod set_dst_address;
#[doc = "SET_DST_PITCH (rw) register accessor: Destination Image Pitch (Actual size - 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`set_dst_pitch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_dst_pitch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_dst_pitch`] module"]
#[doc(alias = "SET_DST_PITCH")]
pub type SetDstPitch = crate::Reg<set_dst_pitch::SetDstPitchSpec>;
#[doc = "Destination Image Pitch (Actual size - 1)"]
pub mod set_dst_pitch;
#[doc = "STATUS (r) register accessor: Running Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Running Status"]
pub mod status;
#[doc = "CONV_CTRL (rw) register accessor: Color Convertion Control\n\nYou can [`read`](crate::Reg::read) this register and get [`conv_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conv_ctrl`] module"]
#[doc(alias = "CONV_CTRL")]
pub type ConvCtrl = crate::Reg<conv_ctrl::ConvCtrlSpec>;
#[doc = "Color Convertion Control"]
pub mod conv_ctrl;
#[doc = "RGB_ALIGNMENT (rw) register accessor: RGB format selector\n\nYou can [`read`](crate::Reg::read) this register and get [`rgb_alignment::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgb_alignment::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgb_alignment`] module"]
#[doc(alias = "RGB_ALIGNMENT")]
pub type RgbAlignment = crate::Reg<rgb_alignment::RgbAlignmentSpec>;
#[doc = "RGB format selector"]
pub mod rgb_alignment;
