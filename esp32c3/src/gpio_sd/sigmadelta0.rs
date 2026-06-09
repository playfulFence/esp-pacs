#[doc = "Register `SIGMADELTA0` reader"]
pub type R = crate::R<SIGMADELTA0_SPEC>;
#[doc = "Register `SIGMADELTA0` writer"]
pub type W = crate::W<SIGMADELTA0_SPEC>;
#[doc = "Field `SD0_IN` reader - "]
pub type SD0_IN_R = crate::FieldReader;
#[doc = "Field `SD0_IN` writer - "]
pub type SD0_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SD0_PRESCALE` reader - "]
pub type SD0_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD0_PRESCALE` writer - "]
pub type SD0_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd0_in(&self) -> SD0_IN_R {
        SD0_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd0_prescale(&self) -> SD0_PRESCALE_R {
        SD0_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA0")
            .field("sd0_prescale", &self.sd0_prescale())
            .field("sd0_in", &self.sd0_in())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd0_in(&mut self) -> SD0_IN_W<'_, SIGMADELTA0_SPEC> {
        SD0_IN_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd0_prescale(&mut self) -> SD0_PRESCALE_W<'_, SIGMADELTA0_SPEC> {
        SD0_PRESCALE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA0_SPEC;
impl crate::RegisterSpec for SIGMADELTA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta0::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta0::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIGMADELTA0 to value 0"]
impl crate::Resettable for SIGMADELTA0_SPEC {}
