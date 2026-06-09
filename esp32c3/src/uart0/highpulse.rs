#[doc = "Register `HIGHPULSE` reader"]
pub type R = crate::R<HIGHPULSE_SPEC>;
#[doc = "Field `HIGHPULSE_MIN_CNT` reader - This register stores the value of the maximum duration time"]
pub type HIGHPULSE_MIN_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - This register stores the value of the maximum duration time"]
    #[inline(always)]
    pub fn highpulse_min_cnt(&self) -> HIGHPULSE_MIN_CNT_R {
        HIGHPULSE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIGHPULSE")
            .field("highpulse_min_cnt", &self.highpulse_min_cnt())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`highpulse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIGHPULSE_SPEC;
impl crate::RegisterSpec for HIGHPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`highpulse::R`](R) reader structure"]
impl crate::Readable for HIGHPULSE_SPEC {}
#[doc = "`reset()` method sets HIGHPULSE to value 0"]
impl crate::Resettable for HIGHPULSE_SPEC {}
