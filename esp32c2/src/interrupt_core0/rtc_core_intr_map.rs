#[doc = "Register `RTC_CORE_INTR_MAP` reader"]
pub type R = crate::R<RTC_CORE_INTR_MAP_SPEC>;
#[doc = "Register `RTC_CORE_INTR_MAP` writer"]
pub type W = crate::W<RTC_CORE_INTR_MAP_SPEC>;
#[doc = "Field `RTC_CORE_INTR_MAP` reader - Need add description"]
pub type RTC_CORE_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `RTC_CORE_INTR_MAP` writer - Need add description"]
pub type RTC_CORE_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn rtc_core_intr_map(&self) -> RTC_CORE_INTR_MAP_R {
        RTC_CORE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CORE_INTR_MAP")
            .field("rtc_core_intr_map", &self.rtc_core_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn rtc_core_intr_map(&mut self) -> RTC_CORE_INTR_MAP_W<RTC_CORE_INTR_MAP_SPEC> {
        RTC_CORE_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_core_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_core_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_CORE_INTR_MAP_SPEC;
impl crate::RegisterSpec for RTC_CORE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_core_intr_map::R`](R) reader structure"]
impl crate::Readable for RTC_CORE_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_core_intr_map::W`](W) writer structure"]
impl crate::Writable for RTC_CORE_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CORE_INTR_MAP to value 0"]
impl crate::Resettable for RTC_CORE_INTR_MAP_SPEC {}
