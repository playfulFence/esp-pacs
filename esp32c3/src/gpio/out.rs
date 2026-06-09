#[doc = "Register `OUT` reader"]
pub type R = crate::R<OUT_SPEC>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OUT_SPEC>;
#[doc = "Field `OUT_DATA` reader - "]
pub type OUT_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DATA` writer - "]
pub type OUT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn out_data(&self) -> OUT_DATA_R {
        OUT_DATA_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT")
            .field("out_data", &self.out_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn out_data(&mut self) -> OUT_DATA_W<'_, OUT_SPEC> {
        OUT_DATA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {}
