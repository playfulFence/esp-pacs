#[doc = "Register `TO` reader"]
pub type R = crate::R<TO_SPEC>;
#[doc = "Register `TO` writer"]
pub type W = crate::W<TO_SPEC>;
#[doc = "Field `TIME_OUT_REG` reader - ."]
pub type TIME_OUT_REG_R = crate::FieldReader;
#[doc = "Field `TIME_OUT_REG` writer - ."]
pub type TIME_OUT_REG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TIME_OUT_EN` reader - ."]
pub type TIME_OUT_EN_R = crate::BitReader;
#[doc = "Field `TIME_OUT_EN` writer - ."]
pub type TIME_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - ."]
    #[inline(always)]
    pub fn time_out_reg(&self) -> TIME_OUT_REG_R {
        TIME_OUT_REG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - ."]
    #[inline(always)]
    pub fn time_out_en(&self) -> TIME_OUT_EN_R {
        TIME_OUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO")
            .field("time_out_en", &self.time_out_en())
            .field("time_out_reg", &self.time_out_reg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - ."]
    #[inline(always)]
    pub fn time_out_reg(&mut self) -> TIME_OUT_REG_W<'_, TO_SPEC> {
        TIME_OUT_REG_W::new(self, 0)
    }
    #[doc = "Bit 5 - ."]
    #[inline(always)]
    pub fn time_out_en(&mut self) -> TIME_OUT_EN_W<'_, TO_SPEC> {
        TIME_OUT_EN_W::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`to::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`to::R`](R) reader structure"]
impl crate::Readable for TO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`to::W`](W) writer structure"]
impl crate::Writable for TO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TO to value 0"]
impl crate::Resettable for TO_SPEC {}
