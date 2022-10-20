#[doc = "Register `IMMU_TABLE14` reader"]
pub struct R(crate::R<IMMU_TABLE14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMMU_TABLE14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMMU_TABLE14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMMU_TABLE14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMMU_TABLE14` writer"]
pub struct W(crate::W<IMMU_TABLE14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMMU_TABLE14_SPEC>;
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
impl From<crate::W<IMMU_TABLE14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMMU_TABLE14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMMU_TABLE14` reader - "]
pub type IMMU_TABLE14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMMU_TABLE14` writer - "]
pub type IMMU_TABLE14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IMMU_TABLE14_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table14(&self) -> IMMU_TABLE14_R {
        IMMU_TABLE14_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table14(&mut self) -> IMMU_TABLE14_W<0> {
        IMMU_TABLE14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [immu_table14](index.html) module"]
pub struct IMMU_TABLE14_SPEC;
impl crate::RegisterSpec for IMMU_TABLE14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [immu_table14::R](R) reader structure"]
impl crate::Readable for IMMU_TABLE14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [immu_table14::W](W) writer structure"]
impl crate::Writable for IMMU_TABLE14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMMU_TABLE14 to value 0x0e"]
impl crate::Resettable for IMMU_TABLE14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e
    }
}