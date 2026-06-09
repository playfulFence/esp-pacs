#[doc = "Register `RD_SYS_PART1_DATA2` reader"]
pub type R = crate::R<RD_SYS_PART1_DATA2_SPEC>;
#[doc = "Field `OPTIONAL_UNIQUE_ID_2` reader - Optional unique 128-bit ID /"]
pub type OPTIONAL_UNIQUE_ID_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Optional unique 128-bit ID /"]
    #[inline(always)]
    pub fn optional_unique_id_2(&self) -> OPTIONAL_UNIQUE_ID_2_R {
        OPTIONAL_UNIQUE_ID_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA2")
            .field("optional_unique_id_2", &self.optional_unique_id_2())
            .finish()
    }
}
#[doc = "Register 2 of BLOCK2 (system). /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART1_DATA2_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data2::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA2 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA2_SPEC {}
