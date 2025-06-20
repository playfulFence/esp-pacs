#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TIMER0_STOP_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 0 stops."]
pub type TIMER0_STOP_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER1_STOP_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 1 stops."]
pub type TIMER1_STOP_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER2_STOP_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 2 stops."]
pub type TIMER2_STOP_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER0_TEZ_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 0 TEZ event."]
pub type TIMER0_TEZ_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER1_TEZ_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 1 TEZ event."]
pub type TIMER1_TEZ_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER2_TEZ_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 2 TEZ event."]
pub type TIMER2_TEZ_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER0_TEP_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 0 TEP event."]
pub type TIMER0_TEP_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER1_TEP_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 1 TEP event."]
pub type TIMER1_TEP_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER2_TEP_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 2 TEP event."]
pub type TIMER2_TEP_INT_ST_R = crate::BitReader;
#[doc = "Field `FAULT0_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f0 starts."]
pub type FAULT0_INT_ST_R = crate::BitReader;
#[doc = "Field `FAULT1_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f1 starts."]
pub type FAULT1_INT_ST_R = crate::BitReader;
#[doc = "Field `FAULT2_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f2 starts."]
pub type FAULT2_INT_ST_R = crate::BitReader;
#[doc = "Field `FAULT0_CLR_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f0 clears."]
pub type FAULT0_CLR_INT_ST_R = crate::BitReader;
#[doc = "Field `FAULT1_CLR_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f1 clears."]
pub type FAULT1_CLR_INT_ST_R = crate::BitReader;
#[doc = "Field `FAULT2_CLR_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f2 clears."]
pub type FAULT2_CLR_INT_ST_R = crate::BitReader;
#[doc = "Field `CMPR0_TEA_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 0 TEA event"]
pub type CMPR0_TEA_INT_ST_R = crate::BitReader;
#[doc = "Field `CMPR1_TEA_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 1 TEA event"]
pub type CMPR1_TEA_INT_ST_R = crate::BitReader;
#[doc = "Field `CMPR2_TEA_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 2 TEA event"]
pub type CMPR2_TEA_INT_ST_R = crate::BitReader;
#[doc = "Field `CMPR0_TEB_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 0 TEB event"]
pub type CMPR0_TEB_INT_ST_R = crate::BitReader;
#[doc = "Field `CMPR1_TEB_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 1 TEB event"]
pub type CMPR1_TEB_INT_ST_R = crate::BitReader;
#[doc = "Field `CMPR2_TEB_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 2 TEB event"]
pub type CMPR2_TEB_INT_ST_R = crate::BitReader;
#[doc = "Field `TZ0_CBC_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub type TZ0_CBC_INT_ST_R = crate::BitReader;
#[doc = "Field `TZ1_CBC_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub type TZ1_CBC_INT_ST_R = crate::BitReader;
#[doc = "Field `TZ2_CBC_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub type TZ2_CBC_INT_ST_R = crate::BitReader;
#[doc = "Field `TZ0_OST_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM0."]
pub type TZ0_OST_INT_ST_R = crate::BitReader;
#[doc = "Field `TZ1_OST_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM1."]
pub type TZ1_OST_INT_ST_R = crate::BitReader;
#[doc = "Field `TZ2_OST_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM2."]
pub type TZ2_OST_INT_ST_R = crate::BitReader;
#[doc = "Field `CAP0_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP0."]
pub type CAP0_INT_ST_R = crate::BitReader;
#[doc = "Field `CAP1_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP1."]
pub type CAP1_INT_ST_R = crate::BitReader;
#[doc = "Field `CAP2_INT_ST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP2."]
pub type CAP2_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    pub fn timer0_stop_int_st(&self) -> TIMER0_STOP_INT_ST_R {
        TIMER0_STOP_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    pub fn timer1_stop_int_st(&self) -> TIMER1_STOP_INT_ST_R {
        TIMER1_STOP_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    pub fn timer2_stop_int_st(&self) -> TIMER2_STOP_INT_ST_R {
        TIMER2_STOP_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    pub fn timer0_tez_int_st(&self) -> TIMER0_TEZ_INT_ST_R {
        TIMER0_TEZ_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    pub fn timer1_tez_int_st(&self) -> TIMER1_TEZ_INT_ST_R {
        TIMER1_TEZ_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    pub fn timer2_tez_int_st(&self) -> TIMER2_TEZ_INT_ST_R {
        TIMER2_TEZ_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    pub fn timer0_tep_int_st(&self) -> TIMER0_TEP_INT_ST_R {
        TIMER0_TEP_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    pub fn timer1_tep_int_st(&self) -> TIMER1_TEP_INT_ST_R {
        TIMER1_TEP_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    pub fn timer2_tep_int_st(&self) -> TIMER2_TEP_INT_ST_R {
        TIMER2_TEP_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    pub fn fault0_int_st(&self) -> FAULT0_INT_ST_R {
        FAULT0_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    pub fn fault1_int_st(&self) -> FAULT1_INT_ST_R {
        FAULT1_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    pub fn fault2_int_st(&self) -> FAULT2_INT_ST_R {
        FAULT2_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f0 clears."]
    #[inline(always)]
    pub fn fault0_clr_int_st(&self) -> FAULT0_CLR_INT_ST_R {
        FAULT0_CLR_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f1 clears."]
    #[inline(always)]
    pub fn fault1_clr_int_st(&self) -> FAULT1_CLR_INT_ST_R {
        FAULT1_CLR_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f2 clears."]
    #[inline(always)]
    pub fn fault2_clr_int_st(&self) -> FAULT2_CLR_INT_ST_R {
        FAULT2_CLR_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn cmpr0_tea_int_st(&self) -> CMPR0_TEA_INT_ST_R {
        CMPR0_TEA_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn cmpr1_tea_int_st(&self) -> CMPR1_TEA_INT_ST_R {
        CMPR1_TEA_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn cmpr2_tea_int_st(&self) -> CMPR2_TEA_INT_ST_R {
        CMPR2_TEA_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn cmpr0_teb_int_st(&self) -> CMPR0_TEB_INT_ST_R {
        CMPR0_TEB_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn cmpr1_teb_int_st(&self) -> CMPR1_TEB_INT_ST_R {
        CMPR1_TEB_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn cmpr2_teb_int_st(&self) -> CMPR2_TEB_INT_ST_R {
        CMPR2_TEB_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_cbc_int_st(&self) -> TZ0_CBC_INT_ST_R {
        TZ0_CBC_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_cbc_int_st(&self) -> TZ1_CBC_INT_ST_R {
        TZ1_CBC_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_cbc_int_st(&self) -> TZ2_CBC_INT_ST_R {
        TZ2_CBC_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_ost_int_st(&self) -> TZ0_OST_INT_ST_R {
        TZ0_OST_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_ost_int_st(&self) -> TZ1_OST_INT_ST_R {
        TZ1_OST_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_ost_int_st(&self) -> TZ2_OST_INT_ST_R {
        TZ2_OST_INT_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP0."]
    #[inline(always)]
    pub fn cap0_int_st(&self) -> CAP0_INT_ST_R {
        CAP0_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP1."]
    #[inline(always)]
    pub fn cap1_int_st(&self) -> CAP1_INT_ST_R {
        CAP1_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP2."]
    #[inline(always)]
    pub fn cap2_int_st(&self) -> CAP2_INT_ST_R {
        CAP2_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("timer0_stop_int_st", &self.timer0_stop_int_st())
            .field("timer1_stop_int_st", &self.timer1_stop_int_st())
            .field("timer2_stop_int_st", &self.timer2_stop_int_st())
            .field("timer0_tez_int_st", &self.timer0_tez_int_st())
            .field("timer1_tez_int_st", &self.timer1_tez_int_st())
            .field("timer2_tez_int_st", &self.timer2_tez_int_st())
            .field("timer0_tep_int_st", &self.timer0_tep_int_st())
            .field("timer1_tep_int_st", &self.timer1_tep_int_st())
            .field("timer2_tep_int_st", &self.timer2_tep_int_st())
            .field("fault0_int_st", &self.fault0_int_st())
            .field("fault1_int_st", &self.fault1_int_st())
            .field("fault2_int_st", &self.fault2_int_st())
            .field("fault0_clr_int_st", &self.fault0_clr_int_st())
            .field("fault1_clr_int_st", &self.fault1_clr_int_st())
            .field("fault2_clr_int_st", &self.fault2_clr_int_st())
            .field("cmpr0_tea_int_st", &self.cmpr0_tea_int_st())
            .field("cmpr1_tea_int_st", &self.cmpr1_tea_int_st())
            .field("cmpr2_tea_int_st", &self.cmpr2_tea_int_st())
            .field("cmpr0_teb_int_st", &self.cmpr0_teb_int_st())
            .field("cmpr1_teb_int_st", &self.cmpr1_teb_int_st())
            .field("cmpr2_teb_int_st", &self.cmpr2_teb_int_st())
            .field("tz0_cbc_int_st", &self.tz0_cbc_int_st())
            .field("tz1_cbc_int_st", &self.tz1_cbc_int_st())
            .field("tz2_cbc_int_st", &self.tz2_cbc_int_st())
            .field("tz0_ost_int_st", &self.tz0_ost_int_st())
            .field("tz1_ost_int_st", &self.tz1_ost_int_st())
            .field("tz2_ost_int_st", &self.tz2_ost_int_st())
            .field("cap0_int_st", &self.cap0_int_st())
            .field("cap1_int_st", &self.cap1_int_st())
            .field("cap2_int_st", &self.cap2_int_st())
            .finish()
    }
}
#[doc = "Interrupt masked status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
