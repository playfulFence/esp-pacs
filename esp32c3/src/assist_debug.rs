#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_cpu: [u8; 0x9c],
    _reserved1: [u8; 0x0160],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x9c - Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
    #[inline(always)]
    pub const fn cpu(&self, n: usize) -> &CPU {
        #[allow(clippy::no_effect)]
        [(); 1][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x9c - Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
    #[inline(always)]
    pub fn cpu_iter(&self) -> impl Iterator<Item = &CPU> {
        (0..1).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156 * n).cast() })
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn core_0_intr_rls(&self) -> &CORE_0_INTR_RLS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x68..0x70 - `¯\\_(ツ)_/¯`"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor(
        &self,
        n: usize,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(104)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x68..0x70 - `¯\\_(ツ)_/¯`"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_exception_monitor_iter(
        &self,
    ) -> impl Iterator<Item = &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(104)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn log_setting(&self) -> &LOG_SETTING {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(112).cast() }
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn log_data_0(&self) -> &LOG_DATA_0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(116).cast() }
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn log_data_mask(&self) -> &LOG_DATA_MASK {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(120).cast() }
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn log_min(&self) -> &LOG_MIN {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(124).cast() }
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn log_max(&self) -> &LOG_MAX {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn log_mem_start(&self) -> &LOG_MEM_START {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn log_mem_end(&self) -> &LOG_MEM_END {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn log_mem_writing_addr(&self) -> &LOG_MEM_WRITING_ADDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn log_mem_full_flag(&self) -> &LOG_MEM_FULL_FLAG {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x1fc - AES version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
pub use self::cpu::CPU;
#[doc = r"Cluster"]
#[doc = "Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
pub mod cpu;
#[doc = "CORE_0_INTR_RLS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_rls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_rls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_rls`] module"]
pub type CORE_0_INTR_RLS = crate::Reg<core_0_intr_rls::CORE_0_INTR_RLS_SPEC>;
#[doc = ""]
pub mod core_0_intr_rls;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR (rw) register accessor: `¯\\_(ツ)_/¯`\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR =
    crate::Reg<core_x_iram0_dram0_exception_monitor::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_SPEC>;
#[doc = "`¯\\_(ツ)_/¯`"]
pub mod core_x_iram0_dram0_exception_monitor;
#[doc = "LOG_SETTING (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_setting`] module"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = ""]
pub mod log_setting;
#[doc = "LOG_DATA_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_0`] module"]
pub type LOG_DATA_0 = crate::Reg<log_data_0::LOG_DATA_0_SPEC>;
#[doc = ""]
pub mod log_data_0;
#[doc = "LOG_DATA_MASK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_mask`] module"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = ""]
pub mod log_data_mask;
#[doc = "LOG_MIN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_min`] module"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = ""]
pub mod log_min;
#[doc = "LOG_MAX (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_max`] module"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = ""]
pub mod log_max;
#[doc = "LOG_MEM_START (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_start`] module"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = ""]
pub mod log_mem_start;
#[doc = "LOG_MEM_END (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_end`] module"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = ""]
pub mod log_mem_end;
#[doc = "LOG_MEM_WRITING_ADDR (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_writing_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_writing_addr`] module"]
pub type LOG_MEM_WRITING_ADDR = crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>;
#[doc = ""]
pub mod log_mem_writing_addr;
#[doc = "LOG_MEM_FULL_FLAG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_full_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mem_full_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_full_flag`] module"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = ""]
pub mod log_mem_full_flag;
pub use crate::aes::{date, DATE};
