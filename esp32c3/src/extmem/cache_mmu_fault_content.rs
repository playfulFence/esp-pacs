#[doc = "Register `CACHE_MMU_FAULT_CONTENT` reader"]
pub type R = crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>;
#[doc = "Field `CACHE_MMU_FAULT_CONTENT` reader - The bits are used to indicate the content of mmu entry which cause mmu fault.."]
pub type CACHE_MMU_FAULT_CONTENT_R = crate::FieldReader<u16>;
#[doc = "Field `CACHE_MMU_FAULT_CODE` reader - The right-most 3 bits are used to indicate the operations which"]
pub type CACHE_MMU_FAULT_CODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - The bits are used to indicate the content of mmu entry which cause mmu fault.."]
    #[inline(always)]
    pub fn cache_mmu_fault_content(&self) -> CACHE_MMU_FAULT_CONTENT_R {
        CACHE_MMU_FAULT_CONTENT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:13 - The right-most 3 bits are used to indicate the operations which"]
    #[inline(always)]
    pub fn cache_mmu_fault_code(&self) -> CACHE_MMU_FAULT_CODE_R {
        CACHE_MMU_FAULT_CODE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MMU_FAULT_CONTENT")
            .field("cache_mmu_fault_code", &self.cache_mmu_fault_code())
            .field("cache_mmu_fault_content", &self.cache_mmu_fault_content())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mmu_fault_content::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_MMU_FAULT_CONTENT_SPEC;
impl crate::RegisterSpec for CACHE_MMU_FAULT_CONTENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_mmu_fault_content::R`](R) reader structure"]
impl crate::Readable for CACHE_MMU_FAULT_CONTENT_SPEC {}
#[doc = "`reset()` method sets CACHE_MMU_FAULT_CONTENT to value 0"]
impl crate::Resettable for CACHE_MMU_FAULT_CONTENT_SPEC {}
