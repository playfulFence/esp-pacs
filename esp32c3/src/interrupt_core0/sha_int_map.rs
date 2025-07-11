#[doc = "Register `SHA_INT_MAP` reader"]
pub type R = crate::R<SHA_INT_MAP_SPEC>;
#[doc = "Register `SHA_INT_MAP` writer"]
pub type W = crate::W<SHA_INT_MAP_SPEC>;
#[doc = "Field `SHA_INT_MAP` reader - reg_core0_sha_int_map"]
pub type SHA_INT_MAP_R = crate::FieldReader;
#[doc = "Field `SHA_INT_MAP` writer - reg_core0_sha_int_map"]
pub type SHA_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_sha_int_map"]
    #[inline(always)]
    pub fn sha_int_map(&self) -> SHA_INT_MAP_R {
        SHA_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA_INT_MAP")
            .field("sha_int_map", &self.sha_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_sha_int_map"]
    #[inline(always)]
    pub fn sha_int_map(&mut self) -> SHA_INT_MAP_W<SHA_INT_MAP_SPEC> {
        SHA_INT_MAP_W::new(self, 0)
    }
}
#[doc = "sha intr map register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA_INT_MAP_SPEC;
impl crate::RegisterSpec for SHA_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_int_map::R`](R) reader structure"]
impl crate::Readable for SHA_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sha_int_map::W`](W) writer structure"]
impl crate::Writable for SHA_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_INT_MAP to value 0"]
impl crate::Resettable for SHA_INT_MAP_SPEC {}
