#[doc = "Register `SIGMADELTA2` reader"]
pub type R = crate::R<SIGMADELTA2_SPEC>;
#[doc = "Register `SIGMADELTA2` writer"]
pub type W = crate::W<SIGMADELTA2_SPEC>;
#[doc = "Field `SD2_IN` reader - "]
pub type SD2_IN_R = crate::FieldReader;
#[doc = "Field `SD2_IN` writer - "]
pub type SD2_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SD2_PRESCALE` reader - "]
pub type SD2_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD2_PRESCALE` writer - "]
pub type SD2_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd2_in(&self) -> SD2_IN_R {
        SD2_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd2_prescale(&self) -> SD2_PRESCALE_R {
        SD2_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA2")
            .field("sd2_prescale", &self.sd2_prescale())
            .field("sd2_in", &self.sd2_in())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd2_in(&mut self) -> SD2_IN_W<'_, SIGMADELTA2_SPEC> {
        SD2_IN_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd2_prescale(&mut self) -> SD2_PRESCALE_W<'_, SIGMADELTA2_SPEC> {
        SD2_PRESCALE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA2_SPEC;
impl crate::RegisterSpec for SIGMADELTA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta2::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta2::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIGMADELTA2 to value 0"]
impl crate::Resettable for SIGMADELTA2_SPEC {}
