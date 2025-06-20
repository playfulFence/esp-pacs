#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `TIMER0_OVF_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value."]
pub type TIMER0_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER0_OVF_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value."]
pub type TIMER0_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_OVF_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value."]
pub type TIMER1_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER1_OVF_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value."]
pub type TIMER1_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_OVF_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value."]
pub type TIMER2_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER2_OVF_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value."]
pub type TIMER2_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_OVF_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value."]
pub type TIMER3_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER3_OVF_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value."]
pub type TIMER3_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH1_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH2_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH2_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH5_INT_RAW_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished."]
pub type DUTY_CHNG_END_CH5_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH0_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
pub type OVF_CNT_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH0_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
pub type OVF_CNT_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH1_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
pub type OVF_CNT_CH1_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH1_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
pub type OVF_CNT_CH1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH2_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
pub type OVF_CNT_CH2_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH2_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
pub type OVF_CNT_CH2_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH3_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
pub type OVF_CNT_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH3_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
pub type OVF_CNT_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH4_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
pub type OVF_CNT_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH4_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
pub type OVF_CNT_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH5_INT_RAW` reader - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
pub type OVF_CNT_CH5_INT_RAW_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH5_INT_RAW` writer - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
pub type OVF_CNT_CH5_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer0_ovf_int_raw(&self) -> TIMER0_OVF_INT_RAW_R {
        TIMER0_OVF_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer1_ovf_int_raw(&self) -> TIMER1_OVF_INT_RAW_R {
        TIMER1_OVF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer2_ovf_int_raw(&self) -> TIMER2_OVF_INT_RAW_R {
        TIMER2_OVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer3_ovf_int_raw(&self) -> TIMER3_OVF_INT_RAW_R {
        TIMER3_OVF_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_raw(&self) -> DUTY_CHNG_END_CH0_INT_RAW_R {
        DUTY_CHNG_END_CH0_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_raw(&self) -> DUTY_CHNG_END_CH1_INT_RAW_R {
        DUTY_CHNG_END_CH1_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_raw(&self) -> DUTY_CHNG_END_CH2_INT_RAW_R {
        DUTY_CHNG_END_CH2_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_raw(&self) -> DUTY_CHNG_END_CH3_INT_RAW_R {
        DUTY_CHNG_END_CH3_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_raw(&self) -> DUTY_CHNG_END_CH4_INT_RAW_R {
        DUTY_CHNG_END_CH4_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_raw(&self) -> DUTY_CHNG_END_CH5_INT_RAW_R {
        DUTY_CHNG_END_CH5_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_raw(&self) -> OVF_CNT_CH0_INT_RAW_R {
        OVF_CNT_CH0_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_raw(&self) -> OVF_CNT_CH1_INT_RAW_R {
        OVF_CNT_CH1_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_raw(&self) -> OVF_CNT_CH2_INT_RAW_R {
        OVF_CNT_CH2_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_raw(&self) -> OVF_CNT_CH3_INT_RAW_R {
        OVF_CNT_CH3_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_raw(&self) -> OVF_CNT_CH4_INT_RAW_R {
        OVF_CNT_CH4_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_raw(&self) -> OVF_CNT_CH5_INT_RAW_R {
        OVF_CNT_CH5_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("timer0_ovf_int_raw", &self.timer0_ovf_int_raw())
            .field("timer1_ovf_int_raw", &self.timer1_ovf_int_raw())
            .field("timer2_ovf_int_raw", &self.timer2_ovf_int_raw())
            .field("timer3_ovf_int_raw", &self.timer3_ovf_int_raw())
            .field(
                "duty_chng_end_ch0_int_raw",
                &self.duty_chng_end_ch0_int_raw(),
            )
            .field(
                "duty_chng_end_ch1_int_raw",
                &self.duty_chng_end_ch1_int_raw(),
            )
            .field(
                "duty_chng_end_ch2_int_raw",
                &self.duty_chng_end_ch2_int_raw(),
            )
            .field(
                "duty_chng_end_ch3_int_raw",
                &self.duty_chng_end_ch3_int_raw(),
            )
            .field(
                "duty_chng_end_ch4_int_raw",
                &self.duty_chng_end_ch4_int_raw(),
            )
            .field(
                "duty_chng_end_ch5_int_raw",
                &self.duty_chng_end_ch5_int_raw(),
            )
            .field("ovf_cnt_ch0_int_raw", &self.ovf_cnt_ch0_int_raw())
            .field("ovf_cnt_ch1_int_raw", &self.ovf_cnt_ch1_int_raw())
            .field("ovf_cnt_ch2_int_raw", &self.ovf_cnt_ch2_int_raw())
            .field("ovf_cnt_ch3_int_raw", &self.ovf_cnt_ch3_int_raw())
            .field("ovf_cnt_ch4_int_raw", &self.ovf_cnt_ch4_int_raw())
            .field("ovf_cnt_ch5_int_raw", &self.ovf_cnt_ch5_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Raw status bit: The raw interrupt status of LEDC_TIMER0_OVF_INT. Triggered when the timer0 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer0_ovf_int_raw(&mut self) -> TIMER0_OVF_INT_RAW_W<INT_RAW_SPEC> {
        TIMER0_OVF_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - Raw status bit: The raw interrupt status of LEDC_TIMER1_OVF_INT. Triggered when the timer1 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer1_ovf_int_raw(&mut self) -> TIMER1_OVF_INT_RAW_W<INT_RAW_SPEC> {
        TIMER1_OVF_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - Raw status bit: The raw interrupt status of LEDC_TIMER2_OVF_INT. Triggered when the timer2 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer2_ovf_int_raw(&mut self) -> TIMER2_OVF_INT_RAW_W<INT_RAW_SPEC> {
        TIMER2_OVF_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - Raw status bit: The raw interrupt status of LEDC_TIMER3_OVF_INT. Triggered when the timer3 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer3_ovf_int_raw(&mut self) -> TIMER3_OVF_INT_RAW_W<INT_RAW_SPEC> {
        TIMER3_OVF_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH0_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_raw(&mut self) -> DUTY_CHNG_END_CH0_INT_RAW_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_CH0_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH1_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_raw(&mut self) -> DUTY_CHNG_END_CH1_INT_RAW_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_CH1_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH2_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_raw(&mut self) -> DUTY_CHNG_END_CH2_INT_RAW_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_CH2_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH3_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_raw(&mut self) -> DUTY_CHNG_END_CH3_INT_RAW_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_CH3_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH4_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_raw(&mut self) -> DUTY_CHNG_END_CH4_INT_RAW_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_CH4_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw status bit: The raw interrupt status of LEDC_DUTY_CHNG_END_CH5_INT. Triggered when the fading of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_raw(&mut self) -> DUTY_CHNG_END_CH5_INT_RAW_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_CH5_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 12 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH0_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH0."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_raw(&mut self) -> OVF_CNT_CH0_INT_RAW_W<INT_RAW_SPEC> {
        OVF_CNT_CH0_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 13 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH1_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH1."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_raw(&mut self) -> OVF_CNT_CH1_INT_RAW_W<INT_RAW_SPEC> {
        OVF_CNT_CH1_INT_RAW_W::new(self, 13)
    }
    #[doc = "Bit 14 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH2_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH2."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_raw(&mut self) -> OVF_CNT_CH2_INT_RAW_W<INT_RAW_SPEC> {
        OVF_CNT_CH2_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 15 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH3_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH3."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_raw(&mut self) -> OVF_CNT_CH3_INT_RAW_W<INT_RAW_SPEC> {
        OVF_CNT_CH3_INT_RAW_W::new(self, 15)
    }
    #[doc = "Bit 16 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH4_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH4."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_raw(&mut self) -> OVF_CNT_CH4_INT_RAW_W<INT_RAW_SPEC> {
        OVF_CNT_CH4_INT_RAW_W::new(self, 16)
    }
    #[doc = "Bit 17 - Raw status bit: The raw interrupt status of LEDC_OVF_CNT_CH5_INT. Triggered when the ovf_cnt has reached the value specified by LEDC_OVF_NUM_CH5."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_raw(&mut self) -> OVF_CNT_CH5_INT_RAW_W<INT_RAW_SPEC> {
        OVF_CNT_CH5_INT_RAW_W::new(self, 17)
    }
}
#[doc = "Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
