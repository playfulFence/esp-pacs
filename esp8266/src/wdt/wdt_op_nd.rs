#[doc = "Register `WDT_OP_ND` reader"]
pub struct R(crate::R<WDT_OP_ND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_OP_ND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_OP_ND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_OP_ND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_OP_ND` writer"]
pub struct W(crate::W<WDT_OP_ND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_OP_ND_SPEC>;
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
impl From<crate::W<WDT_OP_ND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_OP_ND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDT_OP_ND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&mut self) -> REGISTER_W<0> {
        REGISTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reload value for stage 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_op_nd](index.html) module"]
pub struct WDT_OP_ND_SPEC;
impl crate::RegisterSpec for WDT_OP_ND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_op_nd::R](R) reader structure"]
impl crate::Readable for WDT_OP_ND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_op_nd::W](W) writer structure"]
impl crate::Writable for WDT_OP_ND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_OP_ND to value 0"]
impl crate::Resettable for WDT_OP_ND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}