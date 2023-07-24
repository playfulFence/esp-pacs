#[doc = "Register `GPIO_HOLD1` reader"]
pub struct R(crate::R<GPIO_HOLD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_HOLD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_HOLD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_HOLD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_HOLD1` writer"]
pub struct W(crate::W<GPIO_HOLD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_HOLD1_SPEC>;
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
impl From<crate::W<GPIO_HOLD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_HOLD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_HOLD1` reader - need_des"]
pub type GPIO_HOLD1_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_HOLD1` writer - need_des"]
pub type GPIO_HOLD1_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_HOLD1_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn gpio_hold1(&self) -> GPIO_HOLD1_R {
        GPIO_HOLD1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HOLD1")
            .field("gpio_hold1", &format_args!("{}", self.gpio_hold1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_HOLD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hold1(&mut self) -> GPIO_HOLD1_W<0> {
        GPIO_HOLD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_hold1](index.html) module"]
pub struct GPIO_HOLD1_SPEC;
impl crate::RegisterSpec for GPIO_HOLD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_hold1::R](R) reader structure"]
impl crate::Readable for GPIO_HOLD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_hold1::W](W) writer structure"]
impl crate::Writable for GPIO_HOLD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_HOLD1 to value 0"]
impl crate::Resettable for GPIO_HOLD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}