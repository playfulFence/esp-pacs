#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `TIMER_SEL_CH` reader - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
pub type TIMER_SEL_CH_R = crate::FieldReader;
#[doc = "Field `TIMER_SEL_CH` writer - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
pub type TIMER_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIG_OUT_EN_CH` reader - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
pub type SIG_OUT_EN_CH_R = crate::BitReader;
#[doc = "Field `SIG_OUT_EN_CH` writer - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
pub type SIG_OUT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_LV_CH` reader - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
pub type IDLE_LV_CH_R = crate::BitReader;
#[doc = "Field `IDLE_LV_CH` writer - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
pub type IDLE_LV_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARA_UP_CH` writer - Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_OVF_CNT_EN_CH%s fields and duty cycle range configuration for channel %s, and will be automatically cleared by hardware.\\\\0: Invalid. No effect\\\\1: Update"]
pub type PARA_UP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_NUM_CH` reader - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_CH_R = crate::FieldReader<u16>;
#[doc = "Field `OVF_NUM_CH` writer - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
pub type OVF_NUM_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OVF_CNT_EN_CH` reader - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
pub type OVF_CNT_EN_CH_R = crate::BitReader;
#[doc = "Field `OVF_CNT_EN_CH` writer - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
pub type OVF_CNT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_RESET_CH` writer - Configures whether or not to reset the ovf_cnt of channel %s.\\\\0: Invalid. No effect\\\\1: Reset the ovf_cnt"]
pub type OVF_CNT_RESET_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
    #[inline(always)]
    pub fn timer_sel_ch(&self) -> TIMER_SEL_CH_R {
        TIMER_SEL_CH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
    #[inline(always)]
    pub fn sig_out_en_ch(&self) -> SIG_OUT_EN_CH_R {
        SIG_OUT_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
    #[inline(always)]
    pub fn idle_lv_ch(&self) -> IDLE_LV_CH_R {
        IDLE_LV_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    pub fn ovf_num_ch(&self) -> OVF_NUM_CH_R {
        OVF_NUM_CH_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&self) -> OVF_CNT_EN_CH_R {
        OVF_CNT_EN_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("timer_sel_ch", &self.timer_sel_ch())
            .field("sig_out_en_ch", &self.sig_out_en_ch())
            .field("idle_lv_ch", &self.idle_lv_ch())
            .field("ovf_num_ch", &self.ovf_num_ch())
            .field("ovf_cnt_en_ch", &self.ovf_cnt_en_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures which timer is channel %s selected.\\\\0: Select timer0\\\\1: Select timer1\\\\2: Select timer2\\\\3: Select timer3"]
    #[inline(always)]
    pub fn timer_sel_ch(&mut self) -> TIMER_SEL_CH_W<CONF0_SPEC> {
        TIMER_SEL_CH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable signal output on channel %s.\\\\0: Signal output disable\\\\1: Signal output enable"]
    #[inline(always)]
    pub fn sig_out_en_ch(&mut self) -> SIG_OUT_EN_CH_W<CONF0_SPEC> {
        SIG_OUT_EN_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the output value when channel %s is inactive. Valid only when LEDC_SIG_OUT_EN_CH%s is 0.\\\\0: Output level is low\\\\1: Output level is high"]
    #[inline(always)]
    pub fn idle_lv_ch(&mut self) -> IDLE_LV_CH_W<CONF0_SPEC> {
        IDLE_LV_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to update LEDC_HPOINT_CH%s, LEDC_DUTY_START_CH%s, LEDC_SIG_OUT_EN_CH%s, LEDC_TIMER_SEL_CH%s, LEDC_OVF_CNT_EN_CH%s fields and duty cycle range configuration for channel %s, and will be automatically cleared by hardware.\\\\0: Invalid. No effect\\\\1: Update"]
    #[inline(always)]
    pub fn para_up_ch(&mut self) -> PARA_UP_CH_W<CONF0_SPEC> {
        PARA_UP_CH_W::new(self, 4)
    }
    #[doc = "Bits 5:14 - Configures the maximum times of overflow minus 1.The LEDC_OVF_CNT_CH%s_INT interrupt will be triggered when channel %s overflows for (LEDC_OVF_NUM_CH%s + 1) times."]
    #[inline(always)]
    pub fn ovf_num_ch(&mut self) -> OVF_NUM_CH_W<CONF0_SPEC> {
        OVF_NUM_CH_W::new(self, 5)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the ovf_cnt of channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ovf_cnt_en_ch(&mut self) -> OVF_CNT_EN_CH_W<CONF0_SPEC> {
        OVF_CNT_EN_CH_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to reset the ovf_cnt of channel %s.\\\\0: Invalid. No effect\\\\1: Reset the ovf_cnt"]
    #[inline(always)]
    pub fn ovf_cnt_reset_ch(&mut self) -> OVF_CNT_RESET_CH_W<CONF0_SPEC> {
        OVF_CNT_RESET_CH_W::new(self, 16)
    }
}
#[doc = "Configuration register 0 for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF0 to value 0"]
impl crate::Resettable for CONF0_SPEC {}
