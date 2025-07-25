#[doc = "Register `DURATION%s` reader"]
pub type R = crate::R<DURATION_SPEC>;
#[doc = "Register `DURATION%s` writer"]
pub type W = crate::W<DURATION_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "duration of the frame exchange\n\nYou can [`read`](crate::Reg::read) this register and get [`duration::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duration::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DURATION_SPEC;
impl crate::RegisterSpec for DURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`duration::R`](R) reader structure"]
impl crate::Readable for DURATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`duration::W`](W) writer structure"]
impl crate::Writable for DURATION_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DURATION%s to value 0"]
impl crate::Resettable for DURATION_SPEC {}
