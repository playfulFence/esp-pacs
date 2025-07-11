#[doc = "Register `CARRIER_CFG` reader"]
pub type R = crate::R<CARRIER_CFG_SPEC>;
#[doc = "Register `CARRIER_CFG` writer"]
pub type W = crate::W<CARRIER_CFG_SPEC>;
#[doc = "Field `EN` reader - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALE` reader - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
pub type PRESCALE_R = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
pub type PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DUTY` reader - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
pub type DUTY_R = crate::FieldReader;
#[doc = "Field `DUTY` writer - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
pub type DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OSHTWTH` reader - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
pub type OSHTWTH_R = crate::FieldReader;
#[doc = "Field `OSHTWTH` writer - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
pub type OSHTWTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OUT_INVERT` reader - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type OUT_INVERT_R = crate::BitReader;
#[doc = "Field `OUT_INVERT` writer - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type OUT_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_INVERT` reader - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type IN_INVERT_R = crate::BitReader;
#[doc = "Field `IN_INVERT` writer - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
    #[inline(always)]
    pub fn oshtwth(&self) -> OSHTWTH_R {
        OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn out_invert(&self) -> OUT_INVERT_R {
        OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn in_invert(&self) -> IN_INVERT_R {
        IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARRIER_CFG")
            .field("en", &self.en())
            .field("prescale", &self.prescale())
            .field("duty", &self.duty())
            .field("oshtwth", &self.oshtwth())
            .field("out_invert", &self.out_invert())
            .field("in_invert", &self.in_invert())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CARRIER_CFG_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W<CARRIER_CFG_SPEC> {
        PRESCALE_W::new(self, 1)
    }
    #[doc = "Bits 5:7 - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W<CARRIER_CFG_SPEC> {
        DUTY_W::new(self, 5)
    }
    #[doc = "Bits 8:11 - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
    #[inline(always)]
    pub fn oshtwth(&mut self) -> OSHTWTH_W<CARRIER_CFG_SPEC> {
        OSHTWTH_W::new(self, 8)
    }
    #[doc = "Bit 12 - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn out_invert(&mut self) -> OUT_INVERT_W<CARRIER_CFG_SPEC> {
        OUT_INVERT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn in_invert(&mut self) -> IN_INVERT_W<CARRIER_CFG_SPEC> {
        IN_INVERT_W::new(self, 13)
    }
}
#[doc = "Carrier0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`carrier_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`carrier_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARRIER_CFG_SPEC;
impl crate::RegisterSpec for CARRIER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier_cfg::R`](R) reader structure"]
impl crate::Readable for CARRIER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`carrier_cfg::W`](W) writer structure"]
impl crate::Writable for CARRIER_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CARRIER_CFG to value 0"]
impl crate::Resettable for CARRIER_CFG_SPEC {}
