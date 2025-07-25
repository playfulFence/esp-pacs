#[doc = "Register `BUS_MONITOR_INTR_MAP` reader"]
pub type R = crate::R<BUS_MONITOR_INTR_MAP_SPEC>;
#[doc = "Register `BUS_MONITOR_INTR_MAP` writer"]
pub type W = crate::W<BUS_MONITOR_INTR_MAP_SPEC>;
#[doc = "Field `BUS_MONITOR_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type BUS_MONITOR_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `BUS_MONITOR_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type BUS_MONITOR_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn bus_monitor_intr_map(&self) -> BUS_MONITOR_INTR_MAP_R {
        BUS_MONITOR_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_MONITOR_INTR_MAP")
            .field("bus_monitor_intr_map", &self.bus_monitor_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn bus_monitor_intr_map(&mut self) -> BUS_MONITOR_INTR_MAP_W<BUS_MONITOR_INTR_MAP_SPEC> {
        BUS_MONITOR_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "BUS_MONITOR_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_monitor_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_monitor_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_MONITOR_INTR_MAP_SPEC;
impl crate::RegisterSpec for BUS_MONITOR_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_monitor_intr_map::R`](R) reader structure"]
impl crate::Readable for BUS_MONITOR_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_monitor_intr_map::W`](W) writer structure"]
impl crate::Writable for BUS_MONITOR_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_MONITOR_INTR_MAP to value 0"]
impl crate::Resettable for BUS_MONITOR_INTR_MAP_SPEC {}
