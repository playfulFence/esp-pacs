#[doc = "Register `CH%s_DUTY` reader"]
pub type R = crate::R<CH_DUTY_SPEC>;
#[doc = "Register `CH%s_DUTY` writer"]
pub type W = crate::W<CH_DUTY_SPEC>;
#[doc = "Field `DUTY_CH` reader - Configures the duty of signal output on channel %s."]
pub type DUTY_CH_R = crate::FieldReader<u32>;
#[doc = "Field `DUTY_CH` writer - Configures the duty of signal output on channel %s."]
pub type DUTY_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Configures the duty of signal output on channel %s."]
    #[inline(always)]
    pub fn duty_ch(&self) -> DUTY_CH_R {
        DUTY_CH_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_DUTY")
            .field("duty_ch", &self.duty_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:24 - Configures the duty of signal output on channel %s."]
    #[inline(always)]
    pub fn duty_ch(&mut self) -> DUTY_CH_W<CH_DUTY_SPEC> {
        DUTY_CH_W::new(self, 0)
    }
}
#[doc = "Initial duty cycle register for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_duty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_duty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_DUTY_SPEC;
impl crate::RegisterSpec for CH_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_duty::R`](R) reader structure"]
impl crate::Readable for CH_DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_duty::W`](W) writer structure"]
impl crate::Writable for CH_DUTY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_DUTY to value 0"]
impl crate::Resettable for CH_DUTY_SPEC {
    const RESET_VALUE: u32 = 0;
}
