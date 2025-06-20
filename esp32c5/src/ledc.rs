#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch_conf0: (),
    _reserved1: [u8; 0x04],
    ch_hpoint: (),
    _reserved2: [u8; 0x04],
    ch_duty: (),
    _reserved3: [u8; 0x04],
    ch_conf1: (),
    _reserved4: [u8; 0x04],
    ch_duty_r: (),
    _reserved5: [u8; 0x90],
    timer_conf: (),
    _reserved6: [u8; 0x04],
    timer_value: (),
    _reserved7: [u8; 0x1c],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved11: [u8; 0x30],
    ch_gamma_conf: [CH_GAMMA_CONF; 6],
    _reserved12: [u8; 0x08],
    evt_task_en0: EVT_TASK_EN0,
    evt_task_en1: EVT_TASK_EN1,
    evt_task_en2: EVT_TASK_EN2,
    _reserved15: [u8; 0x14],
    timer_cmp: [TIMER_CMP; 4],
    timer_cnt_cap: [TIMER_CNT_CAP; 4],
    _reserved17: [u8; 0x10],
    conf: CONF,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x18 - Configuration register 0 for channel %s"]
    #[inline(always)]
    pub const fn ch_conf0(&self, n: usize) -> &CH_CONF0 {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x18 - Configuration register 0 for channel %s"]
    #[inline(always)]
    pub fn ch_conf0_iter(&self) -> impl Iterator<Item = &CH_CONF0> {
        (0..6).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20 * n).cast() })
    }
    #[doc = "0x00 - Configuration register 0 for channel 0"]
    #[inline(always)]
    pub const fn ch0_conf0(&self) -> &CH_CONF0 {
        self.ch_conf0(0)
    }
    #[doc = "0x14 - Configuration register 0 for channel 1"]
    #[inline(always)]
    pub const fn ch1_conf0(&self) -> &CH_CONF0 {
        self.ch_conf0(1)
    }
    #[doc = "0x28 - Configuration register 0 for channel 2"]
    #[inline(always)]
    pub const fn ch2_conf0(&self) -> &CH_CONF0 {
        self.ch_conf0(2)
    }
    #[doc = "0x3c - Configuration register 0 for channel 3"]
    #[inline(always)]
    pub const fn ch3_conf0(&self) -> &CH_CONF0 {
        self.ch_conf0(3)
    }
    #[doc = "0x50 - Configuration register 0 for channel 4"]
    #[inline(always)]
    pub const fn ch4_conf0(&self) -> &CH_CONF0 {
        self.ch_conf0(4)
    }
    #[doc = "0x64 - Configuration register 0 for channel 5"]
    #[inline(always)]
    pub const fn ch5_conf0(&self) -> &CH_CONF0 {
        self.ch_conf0(5)
    }
    #[doc = "0x04..0x1c - High point register for channel %s"]
    #[inline(always)]
    pub const fn ch_hpoint(&self, n: usize) -> &CH_HPOINT {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x1c - High point register for channel %s"]
    #[inline(always)]
    pub fn ch_hpoint_iter(&self) -> impl Iterator<Item = &CH_HPOINT> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x04 - High point register for channel 0"]
    #[inline(always)]
    pub const fn ch0_hpoint(&self) -> &CH_HPOINT {
        self.ch_hpoint(0)
    }
    #[doc = "0x18 - High point register for channel 1"]
    #[inline(always)]
    pub const fn ch1_hpoint(&self) -> &CH_HPOINT {
        self.ch_hpoint(1)
    }
    #[doc = "0x2c - High point register for channel 2"]
    #[inline(always)]
    pub const fn ch2_hpoint(&self) -> &CH_HPOINT {
        self.ch_hpoint(2)
    }
    #[doc = "0x40 - High point register for channel 3"]
    #[inline(always)]
    pub const fn ch3_hpoint(&self) -> &CH_HPOINT {
        self.ch_hpoint(3)
    }
    #[doc = "0x54 - High point register for channel 4"]
    #[inline(always)]
    pub const fn ch4_hpoint(&self) -> &CH_HPOINT {
        self.ch_hpoint(4)
    }
    #[doc = "0x68 - High point register for channel 5"]
    #[inline(always)]
    pub const fn ch5_hpoint(&self) -> &CH_HPOINT {
        self.ch_hpoint(5)
    }
    #[doc = "0x08..0x20 - Initial duty cycle register for channel %s"]
    #[inline(always)]
    pub const fn ch_duty(&self, n: usize) -> &CH_DUTY {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x20 - Initial duty cycle register for channel %s"]
    #[inline(always)]
    pub fn ch_duty_iter(&self) -> impl Iterator<Item = &CH_DUTY> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x08 - Initial duty cycle register for channel 0"]
    #[inline(always)]
    pub const fn ch0_duty(&self) -> &CH_DUTY {
        self.ch_duty(0)
    }
    #[doc = "0x1c - Initial duty cycle register for channel 1"]
    #[inline(always)]
    pub const fn ch1_duty(&self) -> &CH_DUTY {
        self.ch_duty(1)
    }
    #[doc = "0x30 - Initial duty cycle register for channel 2"]
    #[inline(always)]
    pub const fn ch2_duty(&self) -> &CH_DUTY {
        self.ch_duty(2)
    }
    #[doc = "0x44 - Initial duty cycle register for channel 3"]
    #[inline(always)]
    pub const fn ch3_duty(&self) -> &CH_DUTY {
        self.ch_duty(3)
    }
    #[doc = "0x58 - Initial duty cycle register for channel 4"]
    #[inline(always)]
    pub const fn ch4_duty(&self) -> &CH_DUTY {
        self.ch_duty(4)
    }
    #[doc = "0x6c - Initial duty cycle register for channel 5"]
    #[inline(always)]
    pub const fn ch5_duty(&self) -> &CH_DUTY {
        self.ch_duty(5)
    }
    #[doc = "0x0c..0x24 - Configuration register 1 for channel %s"]
    #[inline(always)]
    pub const fn ch_conf1(&self, n: usize) -> &CH_CONF1 {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x24 - Configuration register 1 for channel %s"]
    #[inline(always)]
    pub fn ch_conf1_iter(&self) -> impl Iterator<Item = &CH_CONF1> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x0c - Configuration register 1 for channel 0"]
    #[inline(always)]
    pub const fn ch0_conf1(&self) -> &CH_CONF1 {
        self.ch_conf1(0)
    }
    #[doc = "0x20 - Configuration register 1 for channel 1"]
    #[inline(always)]
    pub const fn ch1_conf1(&self) -> &CH_CONF1 {
        self.ch_conf1(1)
    }
    #[doc = "0x34 - Configuration register 1 for channel 2"]
    #[inline(always)]
    pub const fn ch2_conf1(&self) -> &CH_CONF1 {
        self.ch_conf1(2)
    }
    #[doc = "0x48 - Configuration register 1 for channel 3"]
    #[inline(always)]
    pub const fn ch3_conf1(&self) -> &CH_CONF1 {
        self.ch_conf1(3)
    }
    #[doc = "0x5c - Configuration register 1 for channel 4"]
    #[inline(always)]
    pub const fn ch4_conf1(&self) -> &CH_CONF1 {
        self.ch_conf1(4)
    }
    #[doc = "0x70 - Configuration register 1 for channel 5"]
    #[inline(always)]
    pub const fn ch5_conf1(&self) -> &CH_CONF1 {
        self.ch_conf1(5)
    }
    #[doc = "0x10..0x28 - Current duty cycle register for channel %s"]
    #[inline(always)]
    pub const fn ch_duty_r(&self, n: usize) -> &CH_DUTY_R {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x28 - Current duty cycle register for channel %s"]
    #[inline(always)]
    pub fn ch_duty_r_iter(&self) -> impl Iterator<Item = &CH_DUTY_R> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x10 - Current duty cycle register for channel 0"]
    #[inline(always)]
    pub const fn ch0_duty_r(&self) -> &CH_DUTY_R {
        self.ch_duty_r(0)
    }
    #[doc = "0x24 - Current duty cycle register for channel 1"]
    #[inline(always)]
    pub const fn ch1_duty_r(&self) -> &CH_DUTY_R {
        self.ch_duty_r(1)
    }
    #[doc = "0x38 - Current duty cycle register for channel 2"]
    #[inline(always)]
    pub const fn ch2_duty_r(&self) -> &CH_DUTY_R {
        self.ch_duty_r(2)
    }
    #[doc = "0x4c - Current duty cycle register for channel 3"]
    #[inline(always)]
    pub const fn ch3_duty_r(&self) -> &CH_DUTY_R {
        self.ch_duty_r(3)
    }
    #[doc = "0x60 - Current duty cycle register for channel 4"]
    #[inline(always)]
    pub const fn ch4_duty_r(&self) -> &CH_DUTY_R {
        self.ch_duty_r(4)
    }
    #[doc = "0x74 - Current duty cycle register for channel 5"]
    #[inline(always)]
    pub const fn ch5_duty_r(&self) -> &CH_DUTY_R {
        self.ch_duty_r(5)
    }
    #[doc = "0xa0..0xb0 - Timer %s configuration register"]
    #[inline(always)]
    pub const fn timer_conf(&self, n: usize) -> &TIMER_CONF {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(160)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xb0 - Timer %s configuration register"]
    #[inline(always)]
    pub fn timer_conf_iter(&self) -> impl Iterator<Item = &TIMER_CONF> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(160)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0xa0 - Timer 0 configuration register"]
    #[inline(always)]
    pub const fn timer0_conf(&self) -> &TIMER_CONF {
        self.timer_conf(0)
    }
    #[doc = "0xa8 - Timer 1 configuration register"]
    #[inline(always)]
    pub const fn timer1_conf(&self) -> &TIMER_CONF {
        self.timer_conf(1)
    }
    #[doc = "0xb0 - Timer 2 configuration register"]
    #[inline(always)]
    pub const fn timer2_conf(&self) -> &TIMER_CONF {
        self.timer_conf(2)
    }
    #[doc = "0xb8 - Timer 3 configuration register"]
    #[inline(always)]
    pub const fn timer3_conf(&self) -> &TIMER_CONF {
        self.timer_conf(3)
    }
    #[doc = "0xa4..0xb4 - Timer %s current counter value register"]
    #[inline(always)]
    pub const fn timer_value(&self, n: usize) -> &TIMER_VALUE {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(164)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa4..0xb4 - Timer %s current counter value register"]
    #[inline(always)]
    pub fn timer_value_iter(&self) -> impl Iterator<Item = &TIMER_VALUE> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(164)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0xa4 - Timer 0 current counter value register"]
    #[inline(always)]
    pub const fn timer0_value(&self) -> &TIMER_VALUE {
        self.timer_value(0)
    }
    #[doc = "0xac - Timer 1 current counter value register"]
    #[inline(always)]
    pub const fn timer1_value(&self) -> &TIMER_VALUE {
        self.timer_value(1)
    }
    #[doc = "0xb4 - Timer 2 current counter value register"]
    #[inline(always)]
    pub const fn timer2_value(&self) -> &TIMER_VALUE {
        self.timer_value(2)
    }
    #[doc = "0xbc - Timer 3 current counter value register"]
    #[inline(always)]
    pub const fn timer3_value(&self) -> &TIMER_VALUE {
        self.timer_value(3)
    }
    #[doc = "0xc0 - Interrupt raw status register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xc4 - Interrupt masked status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xc8 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xcc - Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x100..0x118 - Ledc ch%s gamma config register."]
    #[inline(always)]
    pub const fn ch_gamma_conf(&self, n: usize) -> &CH_GAMMA_CONF {
        &self.ch_gamma_conf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x118 - Ledc ch%s gamma config register."]
    #[inline(always)]
    pub fn ch_gamma_conf_iter(&self) -> impl Iterator<Item = &CH_GAMMA_CONF> {
        self.ch_gamma_conf.iter()
    }
    #[doc = "0x100 - Ledc ch0 gamma config register."]
    #[inline(always)]
    pub const fn ch0_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(0)
    }
    #[doc = "0x104 - Ledc ch1 gamma config register."]
    #[inline(always)]
    pub const fn ch1_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(1)
    }
    #[doc = "0x108 - Ledc ch2 gamma config register."]
    #[inline(always)]
    pub const fn ch2_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(2)
    }
    #[doc = "0x10c - Ledc ch3 gamma config register."]
    #[inline(always)]
    pub const fn ch3_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(3)
    }
    #[doc = "0x110 - Ledc ch4 gamma config register."]
    #[inline(always)]
    pub const fn ch4_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(4)
    }
    #[doc = "0x114 - Ledc ch5 gamma config register."]
    #[inline(always)]
    pub const fn ch5_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(5)
    }
    #[doc = "0x120 - Ledc event task enable bit register0."]
    #[inline(always)]
    pub const fn evt_task_en0(&self) -> &EVT_TASK_EN0 {
        &self.evt_task_en0
    }
    #[doc = "0x124 - Ledc event task enable bit register1."]
    #[inline(always)]
    pub const fn evt_task_en1(&self) -> &EVT_TASK_EN1 {
        &self.evt_task_en1
    }
    #[doc = "0x128 - Ledc event task enable bit register2."]
    #[inline(always)]
    pub const fn evt_task_en2(&self) -> &EVT_TASK_EN2 {
        &self.evt_task_en2
    }
    #[doc = "0x140..0x150 - Ledc timer%s compare value register."]
    #[inline(always)]
    pub const fn timer_cmp(&self, n: usize) -> &TIMER_CMP {
        &self.timer_cmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - Ledc timer%s compare value register."]
    #[inline(always)]
    pub fn timer_cmp_iter(&self) -> impl Iterator<Item = &TIMER_CMP> {
        self.timer_cmp.iter()
    }
    #[doc = "0x140 - Ledc timer0 compare value register."]
    #[inline(always)]
    pub const fn timer0_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(0)
    }
    #[doc = "0x144 - Ledc timer1 compare value register."]
    #[inline(always)]
    pub const fn timer1_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(1)
    }
    #[doc = "0x148 - Ledc timer2 compare value register."]
    #[inline(always)]
    pub const fn timer2_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(2)
    }
    #[doc = "0x14c - Ledc timer3 compare value register."]
    #[inline(always)]
    pub const fn timer3_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(3)
    }
    #[doc = "0x150..0x160 - Ledc timer%s captured count value register."]
    #[inline(always)]
    pub const fn timer_cnt_cap(&self, n: usize) -> &TIMER_CNT_CAP {
        &self.timer_cnt_cap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - Ledc timer%s captured count value register."]
    #[inline(always)]
    pub fn timer_cnt_cap_iter(&self) -> impl Iterator<Item = &TIMER_CNT_CAP> {
        self.timer_cnt_cap.iter()
    }
    #[doc = "0x150 - Ledc timer0 captured count value register."]
    #[inline(always)]
    pub const fn timer0_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(0)
    }
    #[doc = "0x154 - Ledc timer1 captured count value register."]
    #[inline(always)]
    pub const fn timer1_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(1)
    }
    #[doc = "0x158 - Ledc timer2 captured count value register."]
    #[inline(always)]
    pub const fn timer2_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(2)
    }
    #[doc = "0x15c - Ledc timer3 captured count value register."]
    #[inline(always)]
    pub const fn timer3_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(3)
    }
    #[doc = "0x170 - LEDC global configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x174 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CH_CONF0 (rw) register accessor: Configuration register 0 for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_conf0`] module"]
pub type CH_CONF0 = crate::Reg<ch_conf0::CH_CONF0_SPEC>;
#[doc = "Configuration register 0 for channel %s"]
pub mod ch_conf0;
#[doc = "CH_HPOINT (rw) register accessor: High point register for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_hpoint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_hpoint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_hpoint`] module"]
pub type CH_HPOINT = crate::Reg<ch_hpoint::CH_HPOINT_SPEC>;
#[doc = "High point register for channel %s"]
pub mod ch_hpoint;
#[doc = "CH_DUTY (rw) register accessor: Initial duty cycle register for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_duty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_duty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_duty`] module"]
pub type CH_DUTY = crate::Reg<ch_duty::CH_DUTY_SPEC>;
#[doc = "Initial duty cycle register for channel %s"]
pub mod ch_duty;
#[doc = "CH_CONF1 (rw) register accessor: Configuration register 1 for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_conf1`] module"]
pub type CH_CONF1 = crate::Reg<ch_conf1::CH_CONF1_SPEC>;
#[doc = "Configuration register 1 for channel %s"]
pub mod ch_conf1;
#[doc = "CH_DUTY_R (r) register accessor: Current duty cycle register for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_duty_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_duty_r`] module"]
pub type CH_DUTY_R = crate::Reg<ch_duty_r::CH_DUTY_R_SPEC>;
#[doc = "Current duty cycle register for channel %s"]
pub mod ch_duty_r;
#[doc = "TIMER_CONF (rw) register accessor: Timer %s configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_conf`] module"]
pub type TIMER_CONF = crate::Reg<timer_conf::TIMER_CONF_SPEC>;
#[doc = "Timer %s configuration register"]
pub mod timer_conf;
#[doc = "TIMER_VALUE (r) register accessor: Timer %s current counter value register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_value`] module"]
pub type TIMER_VALUE = crate::Reg<timer_value::TIMER_VALUE_SPEC>;
#[doc = "Timer %s current counter value register"]
pub mod timer_value;
#[doc = "INT_RAW (rw) register accessor: Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Interrupt masked status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt masked status register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod int_clr;
#[doc = "CH_GAMMA_CONF (rw) register accessor: Ledc ch%s gamma config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_gamma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_gamma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_gamma_conf`] module"]
pub type CH_GAMMA_CONF = crate::Reg<ch_gamma_conf::CH_GAMMA_CONF_SPEC>;
#[doc = "Ledc ch%s gamma config register."]
pub mod ch_gamma_conf;
#[doc = "EVT_TASK_EN0 (rw) register accessor: Ledc event task enable bit register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en0`] module"]
pub type EVT_TASK_EN0 = crate::Reg<evt_task_en0::EVT_TASK_EN0_SPEC>;
#[doc = "Ledc event task enable bit register0."]
pub mod evt_task_en0;
#[doc = "EVT_TASK_EN1 (rw) register accessor: Ledc event task enable bit register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en1`] module"]
pub type EVT_TASK_EN1 = crate::Reg<evt_task_en1::EVT_TASK_EN1_SPEC>;
#[doc = "Ledc event task enable bit register1."]
pub mod evt_task_en1;
#[doc = "EVT_TASK_EN2 (rw) register accessor: Ledc event task enable bit register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en2`] module"]
pub type EVT_TASK_EN2 = crate::Reg<evt_task_en2::EVT_TASK_EN2_SPEC>;
#[doc = "Ledc event task enable bit register2."]
pub mod evt_task_en2;
#[doc = "TIMER_CMP (rw) register accessor: Ledc timer%s compare value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cmp`] module"]
pub type TIMER_CMP = crate::Reg<timer_cmp::TIMER_CMP_SPEC>;
#[doc = "Ledc timer%s compare value register."]
pub mod timer_cmp;
#[doc = "TIMER_CNT_CAP (r) register accessor: Ledc timer%s captured count value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cnt_cap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cnt_cap`] module"]
pub type TIMER_CNT_CAP = crate::Reg<timer_cnt_cap::TIMER_CNT_CAP_SPEC>;
#[doc = "Ledc timer%s captured count value register."]
pub mod timer_cnt_cap;
#[doc = "CONF (rw) register accessor: LEDC global configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "LEDC global configuration register"]
pub mod conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
