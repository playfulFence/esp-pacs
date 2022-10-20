#[doc = "Register `HOST_SLCHOSTDATE` reader"]
pub struct R(crate::R<HOST_SLCHOSTDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOSTDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOSTDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOSTDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOSTDATE` writer"]
pub struct W(crate::W<HOST_SLCHOSTDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOSTDATE_SPEC>;
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
impl From<crate::W<HOST_SLCHOSTDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOSTDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_DATE` reader - "]
pub type HOST_SLCHOST_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HOST_SLCHOST_DATE` writer - "]
pub type HOST_SLCHOST_DATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOSTDATE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_date(&self) -> HOST_SLCHOST_DATE_R {
        HOST_SLCHOST_DATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_date(&mut self) -> HOST_SLCHOST_DATE_W<0> {
        HOST_SLCHOST_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchostdate](index.html) module"]
pub struct HOST_SLCHOSTDATE_SPEC;
impl crate::RegisterSpec for HOST_SLCHOSTDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchostdate::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOSTDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchostdate::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOSTDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOSTDATE to value 0x1602_2500"]
impl crate::Resettable for HOST_SLCHOSTDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1602_2500
    }
}