#[doc = "Register `CONSTANT_TIME` reader"]
pub struct R(crate::R<CONSTANT_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANT_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANT_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANT_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANT_TIME` writer"]
pub struct W(crate::W<CONSTANT_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANT_TIME_SPEC>;
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
impl From<crate::W<CONSTANT_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANT_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANT_TIME` reader - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
pub type CONSTANT_TIME_R = crate::BitReader<bool>;
#[doc = "Field `CONSTANT_TIME` writer - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
pub type CONSTANT_TIME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONSTANT_TIME_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
    #[inline(always)]
    pub fn constant_time(&self) -> CONSTANT_TIME_R {
        CONSTANT_TIME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
    #[inline(always)]
    #[must_use]
    pub fn constant_time(&mut self) -> CONSTANT_TIME_W<0> {
        CONSTANT_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSA constant time option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constant_time](index.html) module"]
pub struct CONSTANT_TIME_SPEC;
impl crate::RegisterSpec for CONSTANT_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constant_time::R](R) reader structure"]
impl crate::Readable for CONSTANT_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constant_time::W](W) writer structure"]
impl crate::Writable for CONSTANT_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANT_TIME to value 0x01"]
impl crate::Resettable for CONSTANT_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}