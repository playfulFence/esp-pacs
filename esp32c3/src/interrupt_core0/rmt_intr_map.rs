#[doc = "Register `RMT_INTR_MAP` reader"]
pub type R = crate::R<RMT_INTR_MAP_SPEC>;
#[doc = "Register `RMT_INTR_MAP` writer"]
pub type W = crate::W<RMT_INTR_MAP_SPEC>;
#[doc = "Field `RMT_INTR_MAP` reader - reg_core0_rmt_intr_map"]
pub type RMT_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `RMT_INTR_MAP` writer - reg_core0_rmt_intr_map"]
pub type RMT_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_rmt_intr_map"]
    #[inline(always)]
    pub fn rmt_intr_map(&self) -> RMT_INTR_MAP_R {
        RMT_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMT_INTR_MAP")
            .field("rmt_intr_map", &self.rmt_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_rmt_intr_map"]
    #[inline(always)]
    pub fn rmt_intr_map(&mut self) -> RMT_INTR_MAP_W<RMT_INTR_MAP_SPEC> {
        RMT_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "rmt intr map register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMT_INTR_MAP_SPEC;
impl crate::RegisterSpec for RMT_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmt_intr_map::R`](R) reader structure"]
impl crate::Readable for RMT_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmt_intr_map::W`](W) writer structure"]
impl crate::Writable for RMT_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMT_INTR_MAP to value 0"]
impl crate::Resettable for RMT_INTR_MAP_SPEC {}
