#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    linesize: LINESIZE,
    destination: DESTINATION,
    physical_address: PHYSICAL_ADDRESS,
    trigger: TRIGGER,
    release: RELEASE,
    destroy: DESTROY,
    state: STATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x40 - XTS-AES line-size register /"]
    #[inline(always)]
    pub const fn linesize(&self) -> &LINESIZE {
        &self.linesize
    }
    #[doc = "0x44 - XTS-AES destination register /"]
    #[inline(always)]
    pub const fn destination(&self) -> &DESTINATION {
        &self.destination
    }
    #[doc = "0x48 - XTS-AES physical address register /"]
    #[inline(always)]
    pub const fn physical_address(&self) -> &PHYSICAL_ADDRESS {
        &self.physical_address
    }
    #[doc = "0x4c - XTS-AES trigger register /"]
    #[inline(always)]
    pub const fn trigger(&self) -> &TRIGGER {
        &self.trigger
    }
    #[doc = "0x50 - XTS-AES release register /"]
    #[inline(always)]
    pub const fn release(&self) -> &RELEASE {
        &self.release
    }
    #[doc = "0x54 - XTS-AES destroy register /"]
    #[inline(always)]
    pub const fn destroy(&self) -> &DESTROY {
        &self.destroy
    }
    #[doc = "0x58 - XTS-AES status register /"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x5c - XTS-AES version control register /"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "LINESIZE (rw) register accessor: XTS-AES line-size register /\n\nYou can [`read`](crate::Reg::read) this register and get [`linesize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linesize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linesize`] module"]
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
#[doc = "XTS-AES line-size register /"]
pub mod linesize;
#[doc = "DESTINATION (rw) register accessor: XTS-AES destination register /\n\nYou can [`read`](crate::Reg::read) this register and get [`destination::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`destination::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination`] module"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "XTS-AES destination register /"]
pub mod destination;
#[doc = "PHYSICAL_ADDRESS (rw) register accessor: XTS-AES physical address register /\n\nYou can [`read`](crate::Reg::read) this register and get [`physical_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physical_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_address`] module"]
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
#[doc = "XTS-AES physical address register /"]
pub mod physical_address;
#[doc = "TRIGGER (w) register accessor: XTS-AES trigger register /\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "XTS-AES trigger register /"]
pub mod trigger;
#[doc = "RELEASE (w) register accessor: XTS-AES release register /\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@release`] module"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "XTS-AES release register /"]
pub mod release;
#[doc = "DESTROY (w) register accessor: XTS-AES destroy register /\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destroy`] module"]
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
#[doc = "XTS-AES destroy register /"]
pub mod destroy;
#[doc = "STATE (r) register accessor: XTS-AES status register /\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "XTS-AES status register /"]
pub mod state;
pub use crate::aes::{date, DATE};
