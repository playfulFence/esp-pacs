#[doc = "Register `CACHE_CORE0_ACS_INT_MAP` reader"]
pub type R = crate::R<CACHE_CORE0_ACS_INT_MAP_SPEC>;
#[doc = "Register `CACHE_CORE0_ACS_INT_MAP` writer"]
pub type W = crate::W<CACHE_CORE0_ACS_INT_MAP_SPEC>;
#[doc = "Field `CACHE_CORE0_ACS_INT_MAP` reader - Need add description"]
pub type CACHE_CORE0_ACS_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CACHE_CORE0_ACS_INT_MAP` writer - Need add description"]
pub type CACHE_CORE0_ACS_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn cache_core0_acs_int_map(&self) -> CACHE_CORE0_ACS_INT_MAP_R {
        CACHE_CORE0_ACS_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CORE0_ACS_INT_MAP")
            .field("cache_core0_acs_int_map", &self.cache_core0_acs_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn cache_core0_acs_int_map(
        &mut self,
    ) -> CACHE_CORE0_ACS_INT_MAP_W<CACHE_CORE0_ACS_INT_MAP_SPEC> {
        CACHE_CORE0_ACS_INT_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_core0_acs_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_core0_acs_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CORE0_ACS_INT_MAP_SPEC;
impl crate::RegisterSpec for CACHE_CORE0_ACS_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_core0_acs_int_map::R`](R) reader structure"]
impl crate::Readable for CACHE_CORE0_ACS_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_core0_acs_int_map::W`](W) writer structure"]
impl crate::Writable for CACHE_CORE0_ACS_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_CORE0_ACS_INT_MAP to value 0"]
impl crate::Resettable for CACHE_CORE0_ACS_INT_MAP_SPEC {}
