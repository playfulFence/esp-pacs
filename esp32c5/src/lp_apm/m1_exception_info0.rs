#[doc = "Register `M1_EXCEPTION_INFO0` reader"]
pub type R = crate::R<M1_EXCEPTION_INFO0_SPEC>;
#[doc = "Field `M1_EXCEPTION_REGION` reader - Represents exception region."]
pub type M1_EXCEPTION_REGION_R = crate::FieldReader;
#[doc = "Field `M1_EXCEPTION_MODE` reader - Represents exception mode."]
pub type M1_EXCEPTION_MODE_R = crate::FieldReader;
#[doc = "Field `M1_EXCEPTION_ID` reader - Represents exception id information."]
pub type M1_EXCEPTION_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents exception region."]
    #[inline(always)]
    pub fn m1_exception_region(&self) -> M1_EXCEPTION_REGION_R {
        M1_EXCEPTION_REGION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents exception mode."]
    #[inline(always)]
    pub fn m1_exception_mode(&self) -> M1_EXCEPTION_MODE_R {
        M1_EXCEPTION_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Represents exception id information."]
    #[inline(always)]
    pub fn m1_exception_id(&self) -> M1_EXCEPTION_ID_R {
        M1_EXCEPTION_ID_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M1_EXCEPTION_INFO0")
            .field("m1_exception_region", &self.m1_exception_region())
            .field("m1_exception_mode", &self.m1_exception_mode())
            .field("m1_exception_id", &self.m1_exception_id())
            .finish()
    }
}
#[doc = "M1 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1_exception_info0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1_EXCEPTION_INFO0_SPEC;
impl crate::RegisterSpec for M1_EXCEPTION_INFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1_exception_info0::R`](R) reader structure"]
impl crate::Readable for M1_EXCEPTION_INFO0_SPEC {}
#[doc = "`reset()` method sets M1_EXCEPTION_INFO0 to value 0"]
impl crate::Resettable for M1_EXCEPTION_INFO0_SPEC {
    const RESET_VALUE: u32 = 0;
}
