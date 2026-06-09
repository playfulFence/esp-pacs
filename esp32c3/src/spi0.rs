#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: CMD,
    addr: ADDR,
    ctrl: CTRL,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    mosi_dlen: MOSI_DLEN,
    miso_dlen: MISO_DLEN,
    rd_status: RD_STATUS,
    _reserved12: [u8; 0x04],
    misc: MISC,
    tx_crc: TX_CRC,
    cache_fctrl: CACHE_FCTRL,
    _reserved15: [u8; 0x14],
    fsm: FSM,
    w0: W0,
    w1: W1,
    w2: W2,
    w3: W3,
    w4: W4,
    w5: W5,
    w6: W6,
    w7: W7,
    w8: W8,
    w9: W9,
    w10: W10,
    w11: W11,
    w12: W12,
    w13: W13,
    w14: W14,
    w15: W15,
    flash_waiti_ctrl: FLASH_WAITI_CTRL,
    flash_sus_ctrl: FLASH_SUS_CTRL,
    flash_sus_cmd: FLASH_SUS_CMD,
    sus_status: SUS_STATUS,
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    _reserved40: [u8; 0x08],
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    _reserved44: [u8; 0x0c],
    clock_gate: CLOCK_GATE,
    core_clk_sel: CORE_CLK_SEL,
    _reserved46: [u8; 0x0318],
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
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MOSI_DLEN {
        &self.mosi_dlen
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MISO_DLEN {
        &self.miso_dlen
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn tx_crc(&self) -> &TX_CRC {
        &self.tx_crc
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn w0(&self) -> &W0 {
        &self.w0
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn w1(&self) -> &W1 {
        &self.w1
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn w2(&self) -> &W2 {
        &self.w2
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn w3(&self) -> &W3 {
        &self.w3
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn w4(&self) -> &W4 {
        &self.w4
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn w5(&self) -> &W5 {
        &self.w5
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn w6(&self) -> &W6 {
        &self.w6
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn w7(&self) -> &W7 {
        &self.w7
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn w8(&self) -> &W8 {
        &self.w8
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn w9(&self) -> &W9 {
        &self.w9
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn w10(&self) -> &W10 {
        &self.w10
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn w11(&self) -> &W11 {
        &self.w11
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn w12(&self) -> &W12 {
        &self.w12
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn w13(&self) -> &W13 {
        &self.w13
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn w14(&self) -> &W14 {
        &self.w14
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn w15(&self) -> &W15 {
        &self.w15
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn flash_waiti_ctrl(&self) -> &FLASH_WAITI_CTRL {
        &self.flash_waiti_ctrl
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn flash_sus_ctrl(&self) -> &FLASH_SUS_CTRL {
        &self.flash_sus_ctrl
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn flash_sus_cmd(&self) -> &FLASH_SUS_CMD {
        &self.flash_sus_cmd
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn sus_status(&self) -> &SUS_STATUS {
        &self.sus_status
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn core_clk_sel(&self) -> &CORE_CLK_SEL {
        &self.core_clk_sel
    }
    #[doc = "0x3fc - AES version control register"]
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
#[doc = "CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = ""]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = ""]
pub mod ctrl2;
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
#[doc = "MOSI_DLEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mosi_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi_dlen`] module"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = ""]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`miso_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso_dlen`] module"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = ""]
pub mod miso_dlen;
#[doc = "RD_STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = ""]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = ""]
pub mod misc;
#[doc = "TX_CRC (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc`] module"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = ""]
pub mod tx_crc;
#[doc = "CACHE_FCTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = ""]
pub mod cache_fctrl;
#[doc = "FSM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`] module"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = ""]
pub mod fsm;
#[doc = "W0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0`] module"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = ""]
pub mod w0;
#[doc = "W1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w1`] module"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = ""]
pub mod w1;
#[doc = "W2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w2`] module"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = ""]
pub mod w2;
#[doc = "W3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w3`] module"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = ""]
pub mod w3;
#[doc = "W4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w4`] module"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = ""]
pub mod w4;
#[doc = "W5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w5`] module"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = ""]
pub mod w5;
#[doc = "W6 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w6`] module"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = ""]
pub mod w6;
#[doc = "W7 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w7`] module"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = ""]
pub mod w7;
#[doc = "W8 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8`] module"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = ""]
pub mod w8;
#[doc = "W9 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w9`] module"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = ""]
pub mod w9;
#[doc = "W10 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w10`] module"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = ""]
pub mod w10;
#[doc = "W11 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w11`] module"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = ""]
pub mod w11;
#[doc = "W12 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w12`] module"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = ""]
pub mod w12;
#[doc = "W13 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w13`] module"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = ""]
pub mod w13;
#[doc = "W14 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w14`] module"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = ""]
pub mod w14;
#[doc = "W15 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w15`] module"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = ""]
pub mod w15;
#[doc = "FLASH_WAITI_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_waiti_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_waiti_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_waiti_ctrl`] module"]
pub type FLASH_WAITI_CTRL = crate::Reg<flash_waiti_ctrl::FLASH_WAITI_CTRL_SPEC>;
#[doc = ""]
pub mod flash_waiti_ctrl;
#[doc = "FLASH_SUS_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_ctrl`] module"]
pub type FLASH_SUS_CTRL = crate::Reg<flash_sus_ctrl::FLASH_SUS_CTRL_SPEC>;
#[doc = ""]
pub mod flash_sus_ctrl;
#[doc = "FLASH_SUS_CMD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_cmd`] module"]
pub type FLASH_SUS_CMD = crate::Reg<flash_sus_cmd::FLASH_SUS_CMD_SPEC>;
#[doc = ""]
pub mod flash_sus_cmd;
#[doc = "SUS_STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sus_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sus_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sus_status`] module"]
pub type SUS_STATUS = crate::Reg<sus_status::SUS_STATUS_SPEC>;
#[doc = ""]
pub mod sus_status;
#[doc = "TIMING_CALI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cali`] module"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = ""]
pub mod timing_cali;
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
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "CLOCK_GATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = ""]
pub mod clock_gate;
#[doc = "CORE_CLK_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_clk_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_clk_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_clk_sel`] module"]
pub type CORE_CLK_SEL = crate::Reg<core_clk_sel::CORE_CLK_SEL_SPEC>;
#[doc = ""]
pub mod core_clk_sel;
pub use crate::aes::{date, DATE};
