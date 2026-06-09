#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sigmadelta0: SIGMADELTA0,
    sigmadelta1: SIGMADELTA1,
    sigmadelta2: SIGMADELTA2,
    sigmadelta3: SIGMADELTA3,
    _reserved4: [u8; 0x10],
    clock_gate: CLOCK_GATE,
    sigmadelta_misc: SIGMADELTA_MISC,
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn sigmadelta0(&self) -> &SIGMADELTA0 {
        &self.sigmadelta0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn sigmadelta1(&self) -> &SIGMADELTA1 {
        &self.sigmadelta1
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn sigmadelta2(&self) -> &SIGMADELTA2 {
        &self.sigmadelta2
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn sigmadelta3(&self) -> &SIGMADELTA3 {
        &self.sigmadelta3
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn sigmadelta_misc(&self) -> &SIGMADELTA_MISC {
        &self.sigmadelta_misc
    }
    #[doc = "0x28 - AES version control register"]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "SIGMADELTA0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta0`] module"]
pub type SIGMADELTA0 = crate::Reg<sigmadelta0::SIGMADELTA0_SPEC>;
#[doc = ""]
pub mod sigmadelta0;
#[doc = "SIGMADELTA1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta1`] module"]
pub type SIGMADELTA1 = crate::Reg<sigmadelta1::SIGMADELTA1_SPEC>;
#[doc = ""]
pub mod sigmadelta1;
#[doc = "SIGMADELTA2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta2`] module"]
pub type SIGMADELTA2 = crate::Reg<sigmadelta2::SIGMADELTA2_SPEC>;
#[doc = ""]
pub mod sigmadelta2;
#[doc = "SIGMADELTA3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta3`] module"]
pub type SIGMADELTA3 = crate::Reg<sigmadelta3::SIGMADELTA3_SPEC>;
#[doc = ""]
pub mod sigmadelta3;
#[doc = "CLOCK_GATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = ""]
pub mod clock_gate;
#[doc = "SIGMADELTA_MISC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta_misc`] module"]
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
#[doc = ""]
pub mod sigmadelta_misc;
pub use crate::aes::{date as version, DATE as VERSION};
