#[doc = "Register `M1_EXCEPTION_INFO1` reader"]
pub type R = crate::R<M1_EXCEPTION_INFO1_SPEC>;
#[doc = "Field `M1_EXCEPTION_ADDR` reader - Represents exception addr."]
pub type M1_EXCEPTION_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents exception addr."]
    #[inline(always)]
    pub fn m1_exception_addr(&self) -> M1_EXCEPTION_ADDR_R {
        M1_EXCEPTION_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M1_EXCEPTION_INFO1")
            .field("m1_exception_addr", &self.m1_exception_addr())
            .finish()
    }
}
#[doc = "M1 exception_info1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1_exception_info1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1_EXCEPTION_INFO1_SPEC;
impl crate::RegisterSpec for M1_EXCEPTION_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1_exception_info1::R`](R) reader structure"]
impl crate::Readable for M1_EXCEPTION_INFO1_SPEC {}
#[doc = "`reset()` method sets M1_EXCEPTION_INFO1 to value 0"]
impl crate::Resettable for M1_EXCEPTION_INFO1_SPEC {
    const RESET_VALUE: u32 = 0;
}
