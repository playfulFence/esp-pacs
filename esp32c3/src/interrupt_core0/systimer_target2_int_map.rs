#[doc = "Register `SYSTIMER_TARGET2_INT_MAP` reader"]
pub type R = crate::R<SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "Register `SYSTIMER_TARGET2_INT_MAP` writer"]
pub type W = crate::W<SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "Field `SYSTIMER_TARGET2_INT_MAP` reader - reg_core0_systimer_target2_int_map"]
pub type SYSTIMER_TARGET2_INT_MAP_R = crate::FieldReader;
#[doc = "Field `SYSTIMER_TARGET2_INT_MAP` writer - reg_core0_systimer_target2_int_map"]
pub type SYSTIMER_TARGET2_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_systimer_target2_int_map"]
    #[inline(always)]
    pub fn systimer_target2_int_map(&self) -> SYSTIMER_TARGET2_INT_MAP_R {
        SYSTIMER_TARGET2_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER_TARGET2_INT_MAP")
            .field("systimer_target2_int_map", &self.systimer_target2_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_systimer_target2_int_map"]
    #[inline(always)]
    pub fn systimer_target2_int_map(
        &mut self,
    ) -> SYSTIMER_TARGET2_INT_MAP_W<SYSTIMER_TARGET2_INT_MAP_SPEC> {
        SYSTIMER_TARGET2_INT_MAP_W::new(self, 0)
    }
}
#[doc = "systimer target2 intr map register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target2_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target2_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTIMER_TARGET2_INT_MAP_SPEC;
impl crate::RegisterSpec for SYSTIMER_TARGET2_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimer_target2_int_map::R`](R) reader structure"]
impl crate::Readable for SYSTIMER_TARGET2_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`systimer_target2_int_map::W`](W) writer structure"]
impl crate::Writable for SYSTIMER_TARGET2_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTIMER_TARGET2_INT_MAP to value 0"]
impl crate::Resettable for SYSTIMER_TARGET2_INT_MAP_SPEC {}
