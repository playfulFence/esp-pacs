#[doc = "Register `M2_STATUS` reader"]
pub type R = crate::R<M2_STATUS_SPEC>;
#[doc = "Field `M2_EXCEPTION_STATUS` reader - Represents exception status.\\\\ bit0: 1 represents authority_exception \\\\ bit1: 1 represents space_exception \\\\"]
pub type M2_EXCEPTION_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents exception status.\\\\ bit0: 1 represents authority_exception \\\\ bit1: 1 represents space_exception \\\\"]
    #[inline(always)]
    pub fn m2_exception_status(&self) -> M2_EXCEPTION_STATUS_R {
        M2_EXCEPTION_STATUS_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2_STATUS")
            .field("m2_exception_status", &self.m2_exception_status())
            .finish()
    }
}
#[doc = "M2 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2_STATUS_SPEC;
impl crate::RegisterSpec for M2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2_status::R`](R) reader structure"]
impl crate::Readable for M2_STATUS_SPEC {}
#[doc = "`reset()` method sets M2_STATUS to value 0"]
impl crate::Resettable for M2_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
