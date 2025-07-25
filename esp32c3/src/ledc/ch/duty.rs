#[doc = "Register `DUTY` reader"]
pub type R = crate::R<DUTY_SPEC>;
#[doc = "Register `DUTY` writer"]
pub type W = crate::W<DUTY_SPEC>;
#[doc = "Field `DUTY` reader - reg_duty_lsch0."]
pub type DUTY_R = crate::FieldReader<u32>;
#[doc = "Field `DUTY` writer - reg_duty_lsch0."]
pub type DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - reg_duty_lsch0."]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(self.bits & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DUTY").field("duty", &self.duty()).finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - reg_duty_lsch0."]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W<DUTY_SPEC> {
        DUTY_W::new(self, 0)
    }
}
#[doc = "LEDC_LSCH0_DUTY.\n\nYou can [`read`](crate::Reg::read) this register and get [`duty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUTY_SPEC;
impl crate::RegisterSpec for DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`duty::R`](R) reader structure"]
impl crate::Readable for DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`duty::W`](W) writer structure"]
impl crate::Writable for DUTY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUTY to value 0"]
impl crate::Resettable for DUTY_SPEC {}
