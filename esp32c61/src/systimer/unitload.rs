#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO"]
pub struct UNITLOAD {
    hi: HI,
    lo: LO,
}
impl UNITLOAD {
    #[doc = "0x00 - High 20 bits to be loaded to UNIT0"]
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    #[doc = "0x04 - Low 32 bits to be loaded to UNIT0"]
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
}
#[doc = "HI (rw) register accessor: High 20 bits to be loaded to UNIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
pub type HI = crate::Reg<hi::HI_SPEC>;
#[doc = "High 20 bits to be loaded to UNIT0"]
pub mod hi;
#[doc = "LO (rw) register accessor: Low 32 bits to be loaded to UNIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
pub type LO = crate::Reg<lo::LO_SPEC>;
#[doc = "Low 32 bits to be loaded to UNIT0"]
pub mod lo;
