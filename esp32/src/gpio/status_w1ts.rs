#[doc = "Register `STATUS_W1TS` reader"]
pub type R = crate::R<STATUS_W1TS_SPEC>;
#[doc = "Register `STATUS_W1TS` writer"]
pub type W = crate::W<STATUS_W1TS_SPEC>;
#[doc = "Field `STATUS_INT_W1TS` reader - GPIO0~31 interrupt status write 1 to set"]
pub type STATUS_INT_W1TS_R = crate::FieldReader<u32>;
#[doc = "Field `STATUS_INT_W1TS` writer - GPIO0~31 interrupt status write 1 to set"]
pub type STATUS_INT_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status_int_w1ts(&self) -> STATUS_INT_W1TS_R {
        STATUS_INT_W1TS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_W1TS")
            .field("status_int_w1ts", &self.status_int_w1ts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status_int_w1ts(&mut self) -> STATUS_INT_W1TS_W<STATUS_W1TS_SPEC> {
        STATUS_INT_W1TS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status_w1ts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_W1TS_SPEC;
impl crate::RegisterSpec for STATUS_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_w1ts::R`](R) reader structure"]
impl crate::Readable for STATUS_W1TS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status_w1ts::W`](W) writer structure"]
impl crate::Writable for STATUS_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS_W1TS to value 0"]
impl crate::Resettable for STATUS_W1TS_SPEC {}
