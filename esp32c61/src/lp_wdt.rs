#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    config0: CONFIG0,
    config1: CONFIG1,
    config2: CONFIG2,
    config3: CONFIG3,
    config4: CONFIG4,
    wdtfeed: WDTFEED,
    wprotect: WPROTECT,
    swd_config: SWD_CONFIG,
    swd_wprotect: SWD_WPROTECT,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved13: [u8; 0x03c8],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Configure the RWDT operation."]
    #[inline(always)]
    pub const fn config0(&self) -> &CONFIG0 {
        &self.config0
    }
    #[doc = "0x04 - Configure the RWDT timeout of stage0"]
    #[inline(always)]
    pub const fn config1(&self) -> &CONFIG1 {
        &self.config1
    }
    #[doc = "0x08 - Configure the RWDT timeout of stage1"]
    #[inline(always)]
    pub const fn config2(&self) -> &CONFIG2 {
        &self.config2
    }
    #[doc = "0x0c - Configure the RWDT timeout of stage2"]
    #[inline(always)]
    pub const fn config3(&self) -> &CONFIG3 {
        &self.config3
    }
    #[doc = "0x10 - Configure the RWDT timeout of stage3"]
    #[inline(always)]
    pub const fn config4(&self) -> &CONFIG4 {
        &self.config4
    }
    #[doc = "0x14 - Configure the feed function of RWDT"]
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    #[doc = "0x18 - Configure the lock function of SWD"]
    #[inline(always)]
    pub const fn wprotect(&self) -> &WPROTECT {
        &self.wprotect
    }
    #[doc = "0x1c - Configure the SWD operation"]
    #[inline(always)]
    pub const fn swd_config(&self) -> &SWD_CONFIG {
        &self.swd_config
    }
    #[doc = "0x20 - Configure the lock function of SWD"]
    #[inline(always)]
    pub const fn swd_wprotect(&self) -> &SWD_WPROTECT {
        &self.swd_wprotect
    }
    #[doc = "0x24 - Configure whether to generate timeout interrupt"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x28 - The interrupt status register of WDT"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x2c - The interrupt enable register of WDT"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x30 - The interrupt clear register of WDT"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONFIG0 (rw) register accessor: Configure the RWDT operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0`] module"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = "Configure the RWDT operation."]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: Configure the RWDT timeout of stage0\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1`] module"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = "Configure the RWDT timeout of stage0"]
pub mod config1;
#[doc = "CONFIG2 (rw) register accessor: Configure the RWDT timeout of stage1\n\nYou can [`read`](crate::Reg::read) this register and get [`config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config2`] module"]
pub type CONFIG2 = crate::Reg<config2::CONFIG2_SPEC>;
#[doc = "Configure the RWDT timeout of stage1"]
pub mod config2;
#[doc = "CONFIG3 (rw) register accessor: Configure the RWDT timeout of stage2\n\nYou can [`read`](crate::Reg::read) this register and get [`config3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config3`] module"]
pub type CONFIG3 = crate::Reg<config3::CONFIG3_SPEC>;
#[doc = "Configure the RWDT timeout of stage2"]
pub mod config3;
#[doc = "CONFIG4 (rw) register accessor: Configure the RWDT timeout of stage3\n\nYou can [`read`](crate::Reg::read) this register and get [`config4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config4`] module"]
pub type CONFIG4 = crate::Reg<config4::CONFIG4_SPEC>;
#[doc = "Configure the RWDT timeout of stage3"]
pub mod config4;
#[doc = "WDTFEED (w) register accessor: Configure the feed function of RWDT\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "Configure the feed function of RWDT"]
pub mod wdtfeed;
#[doc = "WPROTECT (rw) register accessor: Configure the lock function of SWD\n\nYou can [`read`](crate::Reg::read) this register and get [`wprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wprotect`] module"]
pub type WPROTECT = crate::Reg<wprotect::WPROTECT_SPEC>;
#[doc = "Configure the lock function of SWD"]
pub mod wprotect;
#[doc = "SWD_CONFIG (rw) register accessor: Configure the SWD operation\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swd_config`] module"]
pub type SWD_CONFIG = crate::Reg<swd_config::SWD_CONFIG_SPEC>;
#[doc = "Configure the SWD operation"]
pub mod swd_config;
#[doc = "SWD_WPROTECT (rw) register accessor: Configure the lock function of SWD\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_wprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_wprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swd_wprotect`] module"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "Configure the lock function of SWD"]
pub mod swd_wprotect;
#[doc = "INT_RAW (rw) register accessor: Configure whether to generate timeout interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Configure whether to generate timeout interrupt"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: The interrupt status register of WDT\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "The interrupt status register of WDT"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: The interrupt enable register of WDT\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "The interrupt enable register of WDT"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: The interrupt clear register of WDT\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "The interrupt clear register of WDT"]
pub mod int_clr;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
