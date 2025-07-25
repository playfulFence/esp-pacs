#[doc = "Register `CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` reader"]
pub type R = crate::R<CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>;
#[doc = "Register `CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` writer"]
pub type W = crate::W<CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>;
#[doc = "Field `CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` reader - this register used to map core1_IRam0_pms_monitor_violatile interrupt to one of core0's external interrupt"]
pub type CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` writer - this register used to map core1_IRam0_pms_monitor_violatile interrupt to one of core0's external interrupt"]
pub type CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map core1_IRam0_pms_monitor_violatile interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn core_1_iram0_pms_monitor_violate_intr_map(
        &self,
    ) -> CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R {
        CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP")
            .field(
                "core_1_iram0_pms_monitor_violate_intr_map",
                &self.core_1_iram0_pms_monitor_violate_intr_map(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map core1_IRam0_pms_monitor_violatile interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn core_1_iram0_pms_monitor_violate_intr_map(
        &mut self,
    ) -> CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W<CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>
    {
        CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "core1_IRam0_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_iram0_pms_monitor_violate_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_iram0_pms_monitor_violate_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC;
impl crate::RegisterSpec for CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_iram0_pms_monitor_violate_intr_map::R`](R) reader structure"]
impl crate::Readable for CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_iram0_pms_monitor_violate_intr_map::W`](W) writer structure"]
impl crate::Writable for CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP to value 0x10"]
impl crate::Resettable for CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
