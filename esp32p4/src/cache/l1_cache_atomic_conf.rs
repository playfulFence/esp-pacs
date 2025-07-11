#[doc = "Register `L1_CACHE_ATOMIC_CONF` reader"]
pub type R = crate::R<L1_CACHE_ATOMIC_CONF_SPEC>;
#[doc = "Register `L1_CACHE_ATOMIC_CONF` writer"]
pub type W = crate::W<L1_CACHE_ATOMIC_CONF_SPEC>;
#[doc = "Field `L1_DCACHE_ATOMIC_EN` reader - The bit is used to enable atomic feature on L1-DCache when multiple cores access L1-DCache. 1: disable, 1: enable."]
pub type L1_DCACHE_ATOMIC_EN_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_ATOMIC_EN` writer - The bit is used to enable atomic feature on L1-DCache when multiple cores access L1-DCache. 1: disable, 1: enable."]
pub type L1_DCACHE_ATOMIC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable atomic feature on L1-DCache when multiple cores access L1-DCache. 1: disable, 1: enable."]
    #[inline(always)]
    pub fn l1_dcache_atomic_en(&self) -> L1_DCACHE_ATOMIC_EN_R {
        L1_DCACHE_ATOMIC_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ATOMIC_CONF")
            .field("l1_dcache_atomic_en", &self.l1_dcache_atomic_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable atomic feature on L1-DCache when multiple cores access L1-DCache. 1: disable, 1: enable."]
    #[inline(always)]
    pub fn l1_dcache_atomic_en(&mut self) -> L1_DCACHE_ATOMIC_EN_W<L1_CACHE_ATOMIC_CONF_SPEC> {
        L1_DCACHE_ATOMIC_EN_W::new(self, 0)
    }
}
#[doc = "L1 Cache atomic feature configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_atomic_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_atomic_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_ATOMIC_CONF_SPEC;
impl crate::RegisterSpec for L1_CACHE_ATOMIC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_atomic_conf::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_ATOMIC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_atomic_conf::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_ATOMIC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_ATOMIC_CONF to value 0x01"]
impl crate::Resettable for L1_CACHE_ATOMIC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
