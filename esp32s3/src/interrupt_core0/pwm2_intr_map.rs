#[doc = "Register `PWM2_INTR_MAP` reader"]
pub type R = crate::R<PWM2_INTR_MAP_SPEC>;
#[doc = "Register `PWM2_INTR_MAP` writer"]
pub type W = crate::W<PWM2_INTR_MAP_SPEC>;
#[doc = "Field `PWM2_INTR_MAP` reader - this register used to map pwm2 interrupt to one of core0's external interrupt"]
pub type PWM2_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PWM2_INTR_MAP` writer - this register used to map pwm2 interrupt to one of core0's external interrupt"]
pub type PWM2_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map pwm2 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn pwm2_intr_map(&self) -> PWM2_INTR_MAP_R {
        PWM2_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM2_INTR_MAP")
            .field("pwm2_intr_map", &self.pwm2_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map pwm2 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn pwm2_intr_map(&mut self) -> PWM2_INTR_MAP_W<PWM2_INTR_MAP_SPEC> {
        PWM2_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "pwm2 interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM2_INTR_MAP_SPEC;
impl crate::RegisterSpec for PWM2_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2_intr_map::R`](R) reader structure"]
impl crate::Readable for PWM2_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm2_intr_map::W`](W) writer structure"]
impl crate::Writable for PWM2_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM2_INTR_MAP to value 0x10"]
impl crate::Resettable for PWM2_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
