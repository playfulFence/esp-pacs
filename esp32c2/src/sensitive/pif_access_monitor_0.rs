#[doc = "Register `PIF_ACCESS_MONITOR_0` reader"]
pub struct R(crate::R<PIF_ACCESS_MONITOR_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIF_ACCESS_MONITOR_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIF_ACCESS_MONITOR_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIF_ACCESS_MONITOR_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIF_ACCESS_MONITOR_0` writer"]
pub struct W(crate::W<PIF_ACCESS_MONITOR_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIF_ACCESS_MONITOR_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PIF_ACCESS_MONITOR_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIF_ACCESS_MONITOR_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIF_ACCESS_MONITOR_LOCK` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PIF_ACCESS_MONITOR_LOCK` writer - Need add description"]
pub type PIF_ACCESS_MONITOR_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PIF_ACCESS_MONITOR_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_lock(&self) -> PIF_ACCESS_MONITOR_LOCK_R {
        PIF_ACCESS_MONITOR_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_lock(&mut self) -> PIF_ACCESS_MONITOR_LOCK_W<0> {
        PIF_ACCESS_MONITOR_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pif_access_monitor_0](index.html) module"]
pub struct PIF_ACCESS_MONITOR_0_SPEC;
impl crate::RegisterSpec for PIF_ACCESS_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pif_access_monitor_0::R](R) reader structure"]
impl crate::Readable for PIF_ACCESS_MONITOR_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pif_access_monitor_0::W](W) writer structure"]
impl crate::Writable for PIF_ACCESS_MONITOR_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIF_ACCESS_MONITOR_0 to value 0"]
impl crate::Resettable for PIF_ACCESS_MONITOR_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}