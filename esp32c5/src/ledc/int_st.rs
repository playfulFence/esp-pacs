#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TIMER0_OVF_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_TIMER0_OVF_INT. Valid only when LEDC_TIMER0_OVF_INT_ENA is set to 1."]
pub type TIMER0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER1_OVF_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_TIMER1_OVF_INT. Valid only when LEDC_TIMER1_OVF_INT_ENA is set to 1."]
pub type TIMER1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER2_OVF_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_TIMER2_OVF_INT. Valid only when LEDC_TIMER2_OVF_INT_ENA is set to 1."]
pub type TIMER2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMER3_OVF_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_TIMER3_OVF_INT. Valid only when LEDC_TIMER3_OVF_INT_ENA is set to 1."]
pub type TIMER3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Valid only when LEDC_DUTY_CHNG_END_CH0_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Valid only when LEDC_DUTY_CHNG_END_CH1_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Valid only when LEDC_DUTY_CHNG_END_CH2_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH2_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Valid only when LEDC_DUTY_CHNG_END_CH3_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Valid only when LEDC_DUTY_CHNG_END_CH4_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH4_INT_ST_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Valid only when LEDC_DUTY_CHNG_END_CH5_INT_ENA is set to 1."]
pub type DUTY_CHNG_END_CH5_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH0_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH0_INT. Valid only when LEDC_OVF_CNT_CH0_INT_ENA is set to 1."]
pub type OVF_CNT_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH1_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH1_INT. Valid only when LEDC_OVF_CNT_CH1_INT_ENA is set to 1."]
pub type OVF_CNT_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH2_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH2_INT. Valid only when LEDC_OVF_CNT_CH2_INT_ENA is set to 1."]
pub type OVF_CNT_CH2_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH3_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH3_INT. Valid only when LEDC_OVF_CNT_CH3_INT_ENA is set to 1."]
pub type OVF_CNT_CH3_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH4_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH4_INT. Valid only when LEDC_OVF_CNT_CH4_INT_ENA is set to 1."]
pub type OVF_CNT_CH4_INT_ST_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH5_INT_ST` reader - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH5_INT. Valid only when LEDC_OVF_CNT_CH5_INT_ENA is set to 1."]
pub type OVF_CNT_CH5_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked status bit: The masked interrupt status of LEDC_TIMER0_OVF_INT. Valid only when LEDC_TIMER0_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer0_ovf_int_st(&self) -> TIMER0_OVF_INT_ST_R {
        TIMER0_OVF_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked status bit: The masked interrupt status of LEDC_TIMER1_OVF_INT. Valid only when LEDC_TIMER1_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer1_ovf_int_st(&self) -> TIMER1_OVF_INT_ST_R {
        TIMER1_OVF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked status bit: The masked interrupt status of LEDC_TIMER2_OVF_INT. Valid only when LEDC_TIMER2_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer2_ovf_int_st(&self) -> TIMER2_OVF_INT_ST_R {
        TIMER2_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked status bit: The masked interrupt status of LEDC_TIMER3_OVF_INT. Valid only when LEDC_TIMER3_OVF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn timer3_ovf_int_st(&self) -> TIMER3_OVF_INT_ST_R {
        TIMER3_OVF_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Valid only when LEDC_DUTY_CHNG_END_CH0_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_st(&self) -> DUTY_CHNG_END_CH0_INT_ST_R {
        DUTY_CHNG_END_CH0_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Valid only when LEDC_DUTY_CHNG_END_CH1_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_st(&self) -> DUTY_CHNG_END_CH1_INT_ST_R {
        DUTY_CHNG_END_CH1_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Valid only when LEDC_DUTY_CHNG_END_CH2_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_st(&self) -> DUTY_CHNG_END_CH2_INT_ST_R {
        DUTY_CHNG_END_CH2_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Valid only when LEDC_DUTY_CHNG_END_CH3_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_st(&self) -> DUTY_CHNG_END_CH3_INT_ST_R {
        DUTY_CHNG_END_CH3_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Valid only when LEDC_DUTY_CHNG_END_CH4_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_st(&self) -> DUTY_CHNG_END_CH4_INT_ST_R {
        DUTY_CHNG_END_CH4_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Masked status bit: The masked interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Valid only when LEDC_DUTY_CHNG_END_CH5_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_st(&self) -> DUTY_CHNG_END_CH5_INT_ST_R {
        DUTY_CHNG_END_CH5_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH0_INT. Valid only when LEDC_OVF_CNT_CH0_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_st(&self) -> OVF_CNT_CH0_INT_ST_R {
        OVF_CNT_CH0_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH1_INT. Valid only when LEDC_OVF_CNT_CH1_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_st(&self) -> OVF_CNT_CH1_INT_ST_R {
        OVF_CNT_CH1_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH2_INT. Valid only when LEDC_OVF_CNT_CH2_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_st(&self) -> OVF_CNT_CH2_INT_ST_R {
        OVF_CNT_CH2_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH3_INT. Valid only when LEDC_OVF_CNT_CH3_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_st(&self) -> OVF_CNT_CH3_INT_ST_R {
        OVF_CNT_CH3_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH4_INT. Valid only when LEDC_OVF_CNT_CH4_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_st(&self) -> OVF_CNT_CH4_INT_ST_R {
        OVF_CNT_CH4_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Masked status bit: The masked interrupt status of LEDC_OVF_CNT_CH5_INT. Valid only when LEDC_OVF_CNT_CH5_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_st(&self) -> OVF_CNT_CH5_INT_ST_R {
        OVF_CNT_CH5_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("timer0_ovf_int_st", &self.timer0_ovf_int_st())
            .field("timer1_ovf_int_st", &self.timer1_ovf_int_st())
            .field("timer2_ovf_int_st", &self.timer2_ovf_int_st())
            .field("timer3_ovf_int_st", &self.timer3_ovf_int_st())
            .field("duty_chng_end_ch0_int_st", &self.duty_chng_end_ch0_int_st())
            .field("duty_chng_end_ch1_int_st", &self.duty_chng_end_ch1_int_st())
            .field("duty_chng_end_ch2_int_st", &self.duty_chng_end_ch2_int_st())
            .field("duty_chng_end_ch3_int_st", &self.duty_chng_end_ch3_int_st())
            .field("duty_chng_end_ch4_int_st", &self.duty_chng_end_ch4_int_st())
            .field("duty_chng_end_ch5_int_st", &self.duty_chng_end_ch5_int_st())
            .field("ovf_cnt_ch0_int_st", &self.ovf_cnt_ch0_int_st())
            .field("ovf_cnt_ch1_int_st", &self.ovf_cnt_ch1_int_st())
            .field("ovf_cnt_ch2_int_st", &self.ovf_cnt_ch2_int_st())
            .field("ovf_cnt_ch3_int_st", &self.ovf_cnt_ch3_int_st())
            .field("ovf_cnt_ch4_int_st", &self.ovf_cnt_ch4_int_st())
            .field("ovf_cnt_ch5_int_st", &self.ovf_cnt_ch5_int_st())
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
