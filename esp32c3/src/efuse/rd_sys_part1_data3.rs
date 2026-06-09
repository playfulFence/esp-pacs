#[doc = "Register `RD_SYS_PART1_DATA3` reader"]
pub type R = crate::R<RD_SYS_PART1_DATA3_SPEC>;
#[doc = "Field `OPTIONAL_UNIQUE_ID_3` reader - Optional unique 128-bit ID /"]
pub type OPTIONAL_UNIQUE_ID_3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Optional unique 128-bit ID /"]
    #[inline(always)]
    pub fn optional_unique_id_3(&self) -> OPTIONAL_UNIQUE_ID_3_R {
        OPTIONAL_UNIQUE_ID_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA3")
            .field("optional_unique_id_3", &self.optional_unique_id_3())
            .finish()
    }
}
#[doc = "Register 3 of BLOCK2 (system). /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART1_DATA3_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data3::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA3_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA3 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA3_SPEC {}
