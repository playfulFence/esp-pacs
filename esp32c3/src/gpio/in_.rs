#[doc = "Register `IN` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Field `IN_DATA` reader - "]
pub type IN_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn in_data(&self) -> IN_DATA_R {
        IN_DATA_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN")
            .field("in_data", &self.in_data())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {}
