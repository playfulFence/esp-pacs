#[doc = "Register `STORE2` reader"]
pub struct R(crate::R<STORE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE2` writer"]
pub struct W(crate::W<STORE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE2_SPEC>;
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
impl From<crate::W<STORE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_AON_STORE2` reader - need_des"]
pub type LP_AON_STORE2_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AON_STORE2` writer - need_des"]
pub type LP_AON_STORE2_W<'a, const O: u8> = crate::FieldWriter<'a, STORE2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store2(&self) -> LP_AON_STORE2_R {
        LP_AON_STORE2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE2")
            .field(
                "lp_aon_store2",
                &format_args!("{}", self.lp_aon_store2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store2(&mut self) -> LP_AON_STORE2_W<0> {
        LP_AON_STORE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store2](index.html) module"]
pub struct STORE2_SPEC;
impl crate::RegisterSpec for STORE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store2::R](R) reader structure"]
impl crate::Readable for STORE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store2::W](W) writer structure"]
impl crate::Writable for STORE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE2 to value 0"]
impl crate::Resettable for STORE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}