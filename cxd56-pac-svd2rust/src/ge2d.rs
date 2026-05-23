#[repr(C)]
///Register block
pub struct RegisterBlock {
    intr_enable: IntrEnable,
    intr_stat: IntrStat,
    address_descriptor_start: AddressDescriptorStart,
    _reserved_3_status: [u8; 0x04],
    stat_normal_descriptor_address: StatNormalDescriptorAddress,
    stat_current_descriptor_address: StatCurrentDescriptorAddress,
}
impl RegisterBlock {
    ///0x00 - 2D Graphics Engine Interrupt Control
    #[inline(always)]
    pub const fn intr_enable(&self) -> &IntrEnable {
        &self.intr_enable
    }
    ///0x04 - 2D Graphics Engine Interrupt Status and Clear
    #[inline(always)]
    pub const fn intr_stat(&self) -> &IntrStat {
        &self.intr_stat
    }
    ///0x08 - Descriptor Address Set Register
    #[inline(always)]
    pub const fn address_descriptor_start(&self) -> &AddressDescriptorStart {
        &self.address_descriptor_start
    }
    ///0x0c - Command Register
    #[inline(always)]
    pub const fn cmd_descriptor(&self) -> &CmdDescriptor {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x0c - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x10 - Normal Descriptor Address Register
    #[inline(always)]
    pub const fn stat_normal_descriptor_address(&self) -> &StatNormalDescriptorAddress {
        &self.stat_normal_descriptor_address
    }
    ///0x14 - Current Descriptor Address Register
    #[inline(always)]
    pub const fn stat_current_descriptor_address(&self) -> &StatCurrentDescriptorAddress {
        &self.stat_current_descriptor_address
    }
}
/**INTR_ENABLE (rw) register accessor: 2D Graphics Engine Interrupt Control

You can [`read`](crate::Reg::read) this register and get [`intr_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_enable`] module*/
#[doc(alias = "INTR_ENABLE")]
pub type IntrEnable = crate::Reg<intr_enable::IntrEnableSpec>;
///2D Graphics Engine Interrupt Control
pub mod intr_enable;
pub use IntrEnable as IntrStat;
pub use intr_enable as intr_stat;
/**ADDRESS_DESCRIPTOR_START (rw) register accessor: Descriptor Address Set Register

You can [`read`](crate::Reg::read) this register and get [`address_descriptor_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address_descriptor_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@address_descriptor_start`] module*/
#[doc(alias = "ADDRESS_DESCRIPTOR_START")]
pub type AddressDescriptorStart = crate::Reg<address_descriptor_start::AddressDescriptorStartSpec>;
///Descriptor Address Set Register
pub mod address_descriptor_start;
/**STATUS (r) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
///Status Register
pub mod status;
/**CMD_DESCRIPTOR (w) register accessor: Command Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_descriptor::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd_descriptor`] module*/
#[doc(alias = "CMD_DESCRIPTOR")]
pub type CmdDescriptor = crate::Reg<cmd_descriptor::CmdDescriptorSpec>;
///Command Register
pub mod cmd_descriptor;
/**STAT_NORMAL_DESCRIPTOR_ADDRESS (r) register accessor: Normal Descriptor Address Register

You can [`read`](crate::Reg::read) this register and get [`stat_normal_descriptor_address::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stat_normal_descriptor_address`] module*/
#[doc(alias = "STAT_NORMAL_DESCRIPTOR_ADDRESS")]
pub type StatNormalDescriptorAddress =
    crate::Reg<stat_normal_descriptor_address::StatNormalDescriptorAddressSpec>;
///Normal Descriptor Address Register
pub mod stat_normal_descriptor_address;
/**STAT_CURRENT_DESCRIPTOR_ADDRESS (r) register accessor: Current Descriptor Address Register

You can [`read`](crate::Reg::read) this register and get [`stat_current_descriptor_address::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stat_current_descriptor_address`] module*/
#[doc(alias = "STAT_CURRENT_DESCRIPTOR_ADDRESS")]
pub type StatCurrentDescriptorAddress =
    crate::Reg<stat_current_descriptor_address::StatCurrentDescriptorAddressSpec>;
///Current Descriptor Address Register
pub mod stat_current_descriptor_address;
