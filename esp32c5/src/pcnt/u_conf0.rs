#[doc = "Register `U%s_CONF0` reader"]
pub type R = crate::R<U_CONF0_SPEC>;
#[doc = "Register `U%s_CONF0` writer"]
pub type W = crate::W<U_CONF0_SPEC>;
#[doc = "Field `FILTER_THRES_U` reader - Configures the maximum threshold for the filter. Any pulses with width less than this will be ignored when the filter is enabled. \\\\ Measurement unit: APB_CLK cycles.\\\\"]
pub type FILTER_THRES_U_R = crate::FieldReader<u16>;
#[doc = "Field `FILTER_THRES_U` writer - Configures the maximum threshold for the filter. Any pulses with width less than this will be ignored when the filter is enabled. \\\\ Measurement unit: APB_CLK cycles.\\\\"]
pub type FILTER_THRES_U_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FILTER_EN_U` reader - This is the enable bit for unit %s's input filter."]
pub type FILTER_EN_U_R = crate::BitReader;
#[doc = "Field `FILTER_EN_U` writer - This is the enable bit for unit %s's input filter."]
pub type FILTER_EN_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_ZERO_EN_U` reader - This is the enable bit for unit %s's zero comparator."]
pub type THR_ZERO_EN_U_R = crate::BitReader;
#[doc = "Field `THR_ZERO_EN_U` writer - This is the enable bit for unit %s's zero comparator."]
pub type THR_ZERO_EN_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_H_LIM_EN_U` reader - This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt."]
pub type THR_H_LIM_EN_U_R = crate::BitReader;
#[doc = "Field `THR_H_LIM_EN_U` writer - This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt."]
pub type THR_H_LIM_EN_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_L_LIM_EN_U` reader - This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt."]
pub type THR_L_LIM_EN_U_R = crate::BitReader;
#[doc = "Field `THR_L_LIM_EN_U` writer - This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt."]
pub type THR_L_LIM_EN_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_THRES0_EN_U` reader - This is the enable bit for unit %s's thres0 comparator."]
pub type THR_THRES0_EN_U_R = crate::BitReader;
#[doc = "Field `THR_THRES0_EN_U` writer - This is the enable bit for unit %s's thres0 comparator."]
pub type THR_THRES0_EN_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_THRES1_EN_U` reader - This is the enable bit for unit %s's thres1 comparator."]
pub type THR_THRES1_EN_U_R = crate::BitReader;
#[doc = "Field `THR_THRES1_EN_U` writer - This is the enable bit for unit %s's thres1 comparator."]
pub type THR_THRES1_EN_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_NEG_MODE_U` reader - Configures the behavior when the signal input of channel 0 detects a negative edge.\\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
pub type CH0_NEG_MODE_U_R = crate::FieldReader;
#[doc = "Field `CH0_NEG_MODE_U` writer - Configures the behavior when the signal input of channel 0 detects a negative edge.\\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
pub type CH0_NEG_MODE_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0_POS_MODE_U` reader - Configures the behavior when the signal input of channel 0 detects a positive edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
pub type CH0_POS_MODE_U_R = crate::FieldReader;
#[doc = "Field `CH0_POS_MODE_U` writer - Configures the behavior when the signal input of channel 0 detects a positive edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
pub type CH0_POS_MODE_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0_HCTRL_MODE_U` reader - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
pub type CH0_HCTRL_MODE_U_R = crate::FieldReader;
#[doc = "Field `CH0_HCTRL_MODE_U` writer - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
pub type CH0_HCTRL_MODE_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0_LCTRL_MODE_U` reader - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification\\\\"]
pub type CH0_LCTRL_MODE_U_R = crate::FieldReader;
#[doc = "Field `CH0_LCTRL_MODE_U` writer - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification\\\\"]
pub type CH0_LCTRL_MODE_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_NEG_MODE_U` reader - Configures the behavior when the signal input of channel 1 detects a negative edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
pub type CH1_NEG_MODE_U_R = crate::FieldReader;
#[doc = "Field `CH1_NEG_MODE_U` writer - Configures the behavior when the signal input of channel 1 detects a negative edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
pub type CH1_NEG_MODE_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_POS_MODE_U` reader - Configures the behavior when the signal input of channel 1 detects a positive edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
pub type CH1_POS_MODE_U_R = crate::FieldReader;
#[doc = "Field `CH1_POS_MODE_U` writer - Configures the behavior when the signal input of channel 1 detects a positive edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
pub type CH1_POS_MODE_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_HCTRL_MODE_U` reader - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
pub type CH1_HCTRL_MODE_U_R = crate::FieldReader;
#[doc = "Field `CH1_HCTRL_MODE_U` writer - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
pub type CH1_HCTRL_MODE_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_LCTRL_MODE_U` reader - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
pub type CH1_LCTRL_MODE_U_R = crate::FieldReader;
#[doc = "Field `CH1_LCTRL_MODE_U` writer - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
pub type CH1_LCTRL_MODE_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - Configures the maximum threshold for the filter. Any pulses with width less than this will be ignored when the filter is enabled. \\\\ Measurement unit: APB_CLK cycles.\\\\"]
    #[inline(always)]
    pub fn filter_thres_u(&self) -> FILTER_THRES_U_R {
        FILTER_THRES_U_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This is the enable bit for unit %s's input filter."]
    #[inline(always)]
    pub fn filter_en_u(&self) -> FILTER_EN_U_R {
        FILTER_EN_U_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for unit %s's zero comparator."]
    #[inline(always)]
    pub fn thr_zero_en_u(&self) -> THR_ZERO_EN_U_R {
        THR_ZERO_EN_U_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt."]
    #[inline(always)]
    pub fn thr_h_lim_en_u(&self) -> THR_H_LIM_EN_U_R {
        THR_H_LIM_EN_U_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt."]
    #[inline(always)]
    pub fn thr_l_lim_en_u(&self) -> THR_L_LIM_EN_U_R {
        THR_L_LIM_EN_U_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for unit %s's thres0 comparator."]
    #[inline(always)]
    pub fn thr_thres0_en_u(&self) -> THR_THRES0_EN_U_R {
        THR_THRES0_EN_U_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for unit %s's thres1 comparator."]
    #[inline(always)]
    pub fn thr_thres1_en_u(&self) -> THR_THRES1_EN_U_R {
        THR_THRES1_EN_U_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Configures the behavior when the signal input of channel 0 detects a negative edge.\\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
    #[inline(always)]
    pub fn ch0_neg_mode_u(&self) -> CH0_NEG_MODE_U_R {
        CH0_NEG_MODE_U_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Configures the behavior when the signal input of channel 0 detects a positive edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
    #[inline(always)]
    pub fn ch0_pos_mode_u(&self) -> CH0_POS_MODE_U_R {
        CH0_POS_MODE_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u(&self) -> CH0_HCTRL_MODE_U_R {
        CH0_HCTRL_MODE_U_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification\\\\"]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u(&self) -> CH0_LCTRL_MODE_U_R {
        CH0_LCTRL_MODE_U_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Configures the behavior when the signal input of channel 1 detects a negative edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
    #[inline(always)]
    pub fn ch1_neg_mode_u(&self) -> CH1_NEG_MODE_U_R {
        CH1_NEG_MODE_U_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Configures the behavior when the signal input of channel 1 detects a positive edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
    #[inline(always)]
    pub fn ch1_pos_mode_u(&self) -> CH1_POS_MODE_U_R {
        CH1_POS_MODE_U_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u(&self) -> CH1_HCTRL_MODE_U_R {
        CH1_HCTRL_MODE_U_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u(&self) -> CH1_LCTRL_MODE_U_R {
        CH1_LCTRL_MODE_U_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_CONF0")
            .field("filter_thres_u", &self.filter_thres_u())
            .field("filter_en_u", &self.filter_en_u())
            .field("thr_zero_en_u", &self.thr_zero_en_u())
            .field("thr_h_lim_en_u", &self.thr_h_lim_en_u())
            .field("thr_l_lim_en_u", &self.thr_l_lim_en_u())
            .field("thr_thres0_en_u", &self.thr_thres0_en_u())
            .field("thr_thres1_en_u", &self.thr_thres1_en_u())
            .field("ch0_neg_mode_u", &self.ch0_neg_mode_u())
            .field("ch0_pos_mode_u", &self.ch0_pos_mode_u())
            .field("ch0_hctrl_mode_u", &self.ch0_hctrl_mode_u())
            .field("ch0_lctrl_mode_u", &self.ch0_lctrl_mode_u())
            .field("ch1_neg_mode_u", &self.ch1_neg_mode_u())
            .field("ch1_pos_mode_u", &self.ch1_pos_mode_u())
            .field("ch1_hctrl_mode_u", &self.ch1_hctrl_mode_u())
            .field("ch1_lctrl_mode_u", &self.ch1_lctrl_mode_u())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Configures the maximum threshold for the filter. Any pulses with width less than this will be ignored when the filter is enabled. \\\\ Measurement unit: APB_CLK cycles.\\\\"]
    #[inline(always)]
    pub fn filter_thres_u(&mut self) -> FILTER_THRES_U_W<U_CONF0_SPEC> {
        FILTER_THRES_U_W::new(self, 0)
    }
    #[doc = "Bit 10 - This is the enable bit for unit %s's input filter."]
    #[inline(always)]
    pub fn filter_en_u(&mut self) -> FILTER_EN_U_W<U_CONF0_SPEC> {
        FILTER_EN_U_W::new(self, 10)
    }
    #[doc = "Bit 11 - This is the enable bit for unit %s's zero comparator."]
    #[inline(always)]
    pub fn thr_zero_en_u(&mut self) -> THR_ZERO_EN_U_W<U_CONF0_SPEC> {
        THR_ZERO_EN_U_W::new(self, 11)
    }
    #[doc = "Bit 12 - This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt."]
    #[inline(always)]
    pub fn thr_h_lim_en_u(&mut self) -> THR_H_LIM_EN_U_W<U_CONF0_SPEC> {
        THR_H_LIM_EN_U_W::new(self, 12)
    }
    #[doc = "Bit 13 - This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt."]
    #[inline(always)]
    pub fn thr_l_lim_en_u(&mut self) -> THR_L_LIM_EN_U_W<U_CONF0_SPEC> {
        THR_L_LIM_EN_U_W::new(self, 13)
    }
    #[doc = "Bit 14 - This is the enable bit for unit %s's thres0 comparator."]
    #[inline(always)]
    pub fn thr_thres0_en_u(&mut self) -> THR_THRES0_EN_U_W<U_CONF0_SPEC> {
        THR_THRES0_EN_U_W::new(self, 14)
    }
    #[doc = "Bit 15 - This is the enable bit for unit %s's thres1 comparator."]
    #[inline(always)]
    pub fn thr_thres1_en_u(&mut self) -> THR_THRES1_EN_U_W<U_CONF0_SPEC> {
        THR_THRES1_EN_U_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Configures the behavior when the signal input of channel 0 detects a negative edge.\\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
    #[inline(always)]
    pub fn ch0_neg_mode_u(&mut self) -> CH0_NEG_MODE_U_W<U_CONF0_SPEC> {
        CH0_NEG_MODE_U_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Configures the behavior when the signal input of channel 0 detects a positive edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
    #[inline(always)]
    pub fn ch0_pos_mode_u(&mut self) -> CH0_POS_MODE_U_W<U_CONF0_SPEC> {
        CH0_POS_MODE_U_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u(&mut self) -> CH0_HCTRL_MODE_U_W<U_CONF0_SPEC> {
        CH0_HCTRL_MODE_U_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification\\\\"]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u(&mut self) -> CH0_LCTRL_MODE_U_W<U_CONF0_SPEC> {
        CH0_LCTRL_MODE_U_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Configures the behavior when the signal input of channel 1 detects a negative edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
    #[inline(always)]
    pub fn ch1_neg_mode_u(&mut self) -> CH1_NEG_MODE_U_W<U_CONF0_SPEC> {
        CH1_NEG_MODE_U_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Configures the behavior when the signal input of channel 1 detects a positive edge. \\\\ 1: Increment the counter\\\\ 2: Decrement the counter\\\\ 0, 3: No effect \\\\"]
    #[inline(always)]
    pub fn ch1_pos_mode_u(&mut self) -> CH1_POS_MODE_U_W<U_CONF0_SPEC> {
        CH1_POS_MODE_U_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u(&mut self) -> CH1_HCTRL_MODE_U_W<U_CONF0_SPEC> {
        CH1_HCTRL_MODE_U_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. \\\\ 0: No modification\\\\ 1: Invert behavior (increase -> decrease, decrease -> increase)\\\\ 2, 3: Inhibit counter modification \\\\"]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u(&mut self) -> CH1_LCTRL_MODE_U_W<U_CONF0_SPEC> {
        CH1_LCTRL_MODE_U_W::new(self, 30)
    }
}
#[doc = "Configuration register 0 for unit %s\n\nYou can [`read`](crate::Reg::read) this register and get [`u_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U_CONF0_SPEC;
impl crate::RegisterSpec for U_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_conf0::R`](R) reader structure"]
impl crate::Readable for U_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u_conf0::W`](W) writer structure"]
impl crate::Writable for U_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets U%s_CONF0 to value 0x3c10"]
impl crate::Resettable for U_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x3c10;
}
