#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp_clk_conf: LP_CLK_CONF,
    lp_clk_po_en: LP_CLK_PO_EN,
    lp_clk_en: LP_CLK_EN,
    lp_rst_en: LP_RST_EN,
    reset_cause: RESET_CAUSE,
    cpu_reset: CPU_RESET,
    fosc_cntl: FOSC_CNTL,
    rc32k_cntl: RC32K_CNTL,
    clk_to_hp: CLK_TO_HP,
    lpmem_force: LPMEM_FORCE,
    lpperi: LPPERI,
    xtal32k: XTAL32K,
    _reserved12: [u8; 0x03cc],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Configures the root clk of LP system"]
    #[inline(always)]
    pub const fn lp_clk_conf(&self) -> &LP_CLK_CONF {
        &self.lp_clk_conf
    }
    #[doc = "0x04 - Configures the clk gate to pad"]
    #[inline(always)]
    pub const fn lp_clk_po_en(&self) -> &LP_CLK_PO_EN {
        &self.lp_clk_po_en
    }
    #[doc = "0x08 - Configure LP root clk source gate"]
    #[inline(always)]
    pub const fn lp_clk_en(&self) -> &LP_CLK_EN {
        &self.lp_clk_en
    }
    #[doc = "0x0c - Configures the peri of LP system software reset"]
    #[inline(always)]
    pub const fn lp_rst_en(&self) -> &LP_RST_EN {
        &self.lp_rst_en
    }
    #[doc = "0x10 - Represents the reset casue"]
    #[inline(always)]
    pub const fn reset_cause(&self) -> &RESET_CAUSE {
        &self.reset_cause
    }
    #[doc = "0x14 - Configures CPU reset"]
    #[inline(always)]
    pub const fn cpu_reset(&self) -> &CPU_RESET {
        &self.cpu_reset
    }
    #[doc = "0x18 - Configures the RC_FAST_CLK frequency"]
    #[inline(always)]
    pub const fn fosc_cntl(&self) -> &FOSC_CNTL {
        &self.fosc_cntl
    }
    #[doc = "0x1c - Configures the RC32K_CLK frequency"]
    #[inline(always)]
    pub const fn rc32k_cntl(&self) -> &RC32K_CNTL {
        &self.rc32k_cntl
    }
    #[doc = "0x20 - Configures the clk gate of LP clk to HP system"]
    #[inline(always)]
    pub const fn clk_to_hp(&self) -> &CLK_TO_HP {
        &self.clk_to_hp
    }
    #[doc = "0x24 - Configures the LP_MEM clk gate force parameter"]
    #[inline(always)]
    pub const fn lpmem_force(&self) -> &LPMEM_FORCE {
        &self.lpmem_force
    }
    #[doc = "0x28 - Configures the LP peri clk"]
    #[inline(always)]
    pub const fn lpperi(&self) -> &LPPERI {
        &self.lpperi
    }
    #[doc = "0x2c - Configures the XTAL32K parameter"]
    #[inline(always)]
    pub const fn xtal32k(&self) -> &XTAL32K {
        &self.xtal32k
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "LP_CLK_CONF (rw) register accessor: Configures the root clk of LP system\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_conf`] module"]
pub type LP_CLK_CONF = crate::Reg<lp_clk_conf::LP_CLK_CONF_SPEC>;
#[doc = "Configures the root clk of LP system"]
pub mod lp_clk_conf;
#[doc = "LP_CLK_PO_EN (rw) register accessor: Configures the clk gate to pad\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_po_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_po_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_po_en`] module"]
pub type LP_CLK_PO_EN = crate::Reg<lp_clk_po_en::LP_CLK_PO_EN_SPEC>;
#[doc = "Configures the clk gate to pad"]
pub mod lp_clk_po_en;
#[doc = "LP_CLK_EN (rw) register accessor: Configure LP root clk source gate\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_en`] module"]
pub type LP_CLK_EN = crate::Reg<lp_clk_en::LP_CLK_EN_SPEC>;
#[doc = "Configure LP root clk source gate"]
pub mod lp_clk_en;
#[doc = "LP_RST_EN (rw) register accessor: Configures the peri of LP system software reset\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rst_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rst_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rst_en`] module"]
pub type LP_RST_EN = crate::Reg<lp_rst_en::LP_RST_EN_SPEC>;
#[doc = "Configures the peri of LP system software reset"]
pub mod lp_rst_en;
#[doc = "RESET_CAUSE (rw) register accessor: Represents the reset casue\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_cause`] module"]
pub type RESET_CAUSE = crate::Reg<reset_cause::RESET_CAUSE_SPEC>;
#[doc = "Represents the reset casue"]
pub mod reset_cause;
#[doc = "CPU_RESET (rw) register accessor: Configures CPU reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_reset`] module"]
pub type CPU_RESET = crate::Reg<cpu_reset::CPU_RESET_SPEC>;
#[doc = "Configures CPU reset"]
pub mod cpu_reset;
#[doc = "FOSC_CNTL (rw) register accessor: Configures the RC_FAST_CLK frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`fosc_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fosc_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fosc_cntl`] module"]
pub type FOSC_CNTL = crate::Reg<fosc_cntl::FOSC_CNTL_SPEC>;
#[doc = "Configures the RC_FAST_CLK frequency"]
pub mod fosc_cntl;
#[doc = "RC32K_CNTL (rw) register accessor: Configures the RC32K_CLK frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32k_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32k_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32k_cntl`] module"]
pub type RC32K_CNTL = crate::Reg<rc32k_cntl::RC32K_CNTL_SPEC>;
#[doc = "Configures the RC32K_CLK frequency"]
pub mod rc32k_cntl;
#[doc = "CLK_TO_HP (rw) register accessor: Configures the clk gate of LP clk to HP system\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_to_hp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_to_hp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_to_hp`] module"]
pub type CLK_TO_HP = crate::Reg<clk_to_hp::CLK_TO_HP_SPEC>;
#[doc = "Configures the clk gate of LP clk to HP system"]
pub mod clk_to_hp;
#[doc = "LPMEM_FORCE (rw) register accessor: Configures the LP_MEM clk gate force parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmem_force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmem_force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmem_force`] module"]
pub type LPMEM_FORCE = crate::Reg<lpmem_force::LPMEM_FORCE_SPEC>;
#[doc = "Configures the LP_MEM clk gate force parameter"]
pub mod lpmem_force;
#[doc = "LPPERI (rw) register accessor: Configures the LP peri clk\n\nYou can [`read`](crate::Reg::read) this register and get [`lpperi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpperi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpperi`] module"]
pub type LPPERI = crate::Reg<lpperi::LPPERI_SPEC>;
#[doc = "Configures the LP peri clk"]
pub mod lpperi;
#[doc = "XTAL32K (rw) register accessor: Configures the XTAL32K parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32k`] module"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "Configures the XTAL32K parameter"]
pub mod xtal32k;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
