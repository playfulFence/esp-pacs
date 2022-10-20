#[doc = "Register `FRC2_COUNT` reader"]
pub struct R(crate::R<FRC2_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRC2_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRC2_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRC2_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `frc2_count` reader - the current value of the counter. It is a increasingcounter."]
pub type FRC2_COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - the current value of the counter. It is a increasingcounter."]
    #[inline(always)]
    pub fn frc2_count(&self) -> FRC2_COUNT_R {
        FRC2_COUNT_R::new(self.bits)
    }
}
#[doc = "the current value of the counter. It is a increasingcounter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frc2_count](index.html) module"]
pub struct FRC2_COUNT_SPEC;
impl crate::RegisterSpec for FRC2_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frc2_count::R](R) reader structure"]
impl crate::Readable for FRC2_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRC2_COUNT to value 0"]
impl crate::Resettable for FRC2_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}