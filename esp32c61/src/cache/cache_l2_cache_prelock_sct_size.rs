#[doc = "Register `CACHE_L2_CACHE_PRELOCK_SCT_SIZE` reader"]
pub type R = crate::R<CACHE_L2_CACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_PRELOCK_SCT0_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT0_ADDR_REG"]
pub type CACHE_L2_CACHE_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `CACHE_L2_CACHE_PRELOCK_SCT1_SIZE` reader - Those bits are used to configure the size of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_ADDR_REG"]
pub type CACHE_L2_CACHE_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn cache_l2_cache_prelock_sct0_size(&self) -> CACHE_L2_CACHE_PRELOCK_SCT0_SIZE_R {
        CACHE_L2_CACHE_PRELOCK_SCT0_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Those bits are used to configure the size of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn cache_l2_cache_prelock_sct1_size(&self) -> CACHE_L2_CACHE_PRELOCK_SCT1_SIZE_R {
        CACHE_L2_CACHE_PRELOCK_SCT1_SIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_PRELOCK_SCT_SIZE")
            .field(
                "cache_l2_cache_prelock_sct0_size",
                &self.cache_l2_cache_prelock_sct0_size(),
            )
            .field(
                "cache_l2_cache_prelock_sct1_size",
                &self.cache_l2_cache_prelock_sct1_size(),
            )
            .finish()
    }
}
#[doc = "L2 Cache prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_prelock_sct_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_prelock_sct_size::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_PRELOCK_SCT_SIZE_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_PRELOCK_SCT_SIZE to value 0xffff_ffff"]
impl crate::Resettable for CACHE_L2_CACHE_PRELOCK_SCT_SIZE_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
