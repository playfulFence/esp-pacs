#[doc = "Register `DCACHE_PRELOCK_SCT1_ADDR` reader"]
pub type R = crate::R<DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Register `DCACHE_PRELOCK_SCT1_ADDR` writer"]
pub type W = crate::W<DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Field `CACHE_PRELOCK_SCT1_ADDR` reader - Those bits are used to configure the start address of the second section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOCK_SCT1_SIZE_REG"]
pub type CACHE_PRELOCK_SCT1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_PRELOCK_SCT1_ADDR` writer - Those bits are used to configure the start address of the second section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOCK_SCT1_SIZE_REG"]
pub type CACHE_PRELOCK_SCT1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of the second section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    pub fn cache_prelock_sct1_addr(&self) -> CACHE_PRELOCK_SCT1_ADDR_R {
        CACHE_PRELOCK_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOCK_SCT1_ADDR")
            .field("cache_prelock_sct1_addr", &self.cache_prelock_sct1_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of the second section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    pub fn cache_prelock_sct1_addr(
        &mut self,
    ) -> CACHE_PRELOCK_SCT1_ADDR_W<DCACHE_PRELOCK_SCT1_ADDR_SPEC> {
        CACHE_PRELOCK_SCT1_ADDR_W::new(self, 0)
    }
}
#[doc = "L1 Cache prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_PRELOCK_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOCK_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_prelock_sct1_addr::R`](R) reader structure"]
impl crate::Readable for DCACHE_PRELOCK_SCT1_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_prelock_sct1_addr::W`](W) writer structure"]
impl crate::Writable for DCACHE_PRELOCK_SCT1_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCACHE_PRELOCK_SCT1_ADDR to value 0"]
impl crate::Resettable for DCACHE_PRELOCK_SCT1_ADDR_SPEC {}
