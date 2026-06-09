#[doc = "Register `SIGMADELTA1` reader"]
pub type R = crate::R<SIGMADELTA1_SPEC>;
#[doc = "Register `SIGMADELTA1` writer"]
pub type W = crate::W<SIGMADELTA1_SPEC>;
#[doc = "Field `SD1_IN` reader - "]
pub type SD1_IN_R = crate::FieldReader;
#[doc = "Field `SD1_IN` writer - "]
pub type SD1_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SD1_PRESCALE` reader - "]
pub type SD1_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD1_PRESCALE` writer - "]
pub type SD1_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd1_in(&self) -> SD1_IN_R {
        SD1_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd1_prescale(&self) -> SD1_PRESCALE_R {
        SD1_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA1")
            .field("sd1_prescale", &self.sd1_prescale())
            .field("sd1_in", &self.sd1_in())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd1_in(&mut self) -> SD1_IN_W<'_, SIGMADELTA1_SPEC> {
        SD1_IN_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd1_prescale(&mut self) -> SD1_PRESCALE_W<'_, SIGMADELTA1_SPEC> {
        SD1_PRESCALE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA1_SPEC;
impl crate::RegisterSpec for SIGMADELTA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta1::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta1::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIGMADELTA1 to value 0"]
impl crate::Resettable for SIGMADELTA1_SPEC {}
