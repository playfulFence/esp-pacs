#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: CMD,
    addr: ADDR,
    ctrl: CTRL,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    ms_dlen: MS_DLEN,
    misc: MISC,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    dma_conf: DMA_CONF,
    dma_int_ena: DMA_INT_ENA,
    dma_int_clr: DMA_INT_CLR,
    dma_int_raw: DMA_INT_RAW,
    dma_int_st: DMA_INT_ST,
    _reserved17: [u8; 0x54],
    w: [W; 16],
    _reserved18: [u8; 0x08],
    slave: SLAVE,
    slave1: SLAVE1,
    clk_gate: CLK_GATE,
    _reserved21: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn ms_dlen(&self) -> &MS_DLEN {
        &self.ms_dlen
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        &self.dma_conf
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn dma_int_ena(&self) -> &DMA_INT_ENA {
        &self.dma_int_ena
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn dma_int_clr(&self) -> &DMA_INT_CLR {
        &self.dma_int_clr
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn dma_int_raw(&self) -> &DMA_INT_RAW {
        &self.dma_int_raw
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn dma_int_st(&self) -> &DMA_INT_ST {
        &self.dma_int_st
    }
    #[doc = "0x98..0xd8 - "]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98..0xd8 - "]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn slave(&self) -> &SLAVE {
        &self.slave
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn slave1(&self) -> &SLAVE1 {
        &self.slave1
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn clk_gate(&self) -> &CLK_GATE {
        &self.clk_gate
    }
    #[doc = "0xf0 - AES version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = ""]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = ""]
pub mod addr;
#[doc = "CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "CLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = ""]
pub mod clock;
#[doc = "USER (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = ""]
pub mod user;
#[doc = "USER1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = ""]
pub mod user1;
#[doc = "USER2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = ""]
pub mod user2;
#[doc = "MS_DLEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ms_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_dlen`] module"]
pub type MS_DLEN = crate::Reg<ms_dlen::MS_DLEN_SPEC>;
#[doc = ""]
pub mod ms_dlen;
#[doc = "MISC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = ""]
pub mod misc;
#[doc = "DIN_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_mode`] module"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = ""]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_num`] module"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = ""]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_mode`] module"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = ""]
pub mod dout_mode;
#[doc = "DMA_CONF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = ""]
pub mod dma_conf;
#[doc = "DMA_INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_ena`] module"]
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
#[doc = ""]
pub mod dma_int_ena;
#[doc = "DMA_INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_clr`] module"]
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
#[doc = ""]
pub mod dma_int_clr;
#[doc = "DMA_INT_RAW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_raw`] module"]
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
#[doc = ""]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_st`] module"]
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
#[doc = ""]
pub mod dma_int_st;
#[doc = "W (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w`] module"]
pub type W = crate::Reg<w::W_SPEC>;
#[doc = ""]
pub mod w;
#[doc = "SLAVE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`slave::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave`] module"]
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
#[doc = ""]
pub mod slave;
#[doc = "SLAVE1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`slave1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave1`] module"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = ""]
pub mod slave1;
#[doc = "CLK_GATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gate`] module"]
pub type CLK_GATE = crate::Reg<clk_gate::CLK_GATE_SPEC>;
#[doc = ""]
pub mod clk_gate;
pub use crate::aes::{date, DATE};
