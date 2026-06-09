#[doc = "Register `BACKUP_BUS_PMS_MONITOR_1` reader"]
pub type R = crate::R<BACKUP_BUS_PMS_MONITOR_1_SPEC>;
#[doc = "Register `BACKUP_BUS_PMS_MONITOR_1` writer"]
pub type W = crate::W<BACKUP_BUS_PMS_MONITOR_1_SPEC>;
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR` reader - "]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R = crate::BitReader;
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR` writer - "]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_EN` reader - "]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R = crate::BitReader;
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_EN` writer - "]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_clr(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_en(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_MONITOR_1")
            .field(
                "backup_bus_pms_monitor_violate_en",
                &self.backup_bus_pms_monitor_violate_en(),
            )
            .field(
                "backup_bus_pms_monitor_violate_clr",
                &self.backup_bus_pms_monitor_violate_clr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_clr(
        &mut self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W<'_, BACKUP_BUS_PMS_MONITOR_1_SPEC> {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_en(
        &mut self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W<'_, BACKUP_BUS_PMS_MONITOR_1_SPEC> {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_bus_pms_monitor_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_bus_pms_monitor_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_BUS_PMS_MONITOR_1_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_bus_pms_monitor_1::R`](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_MONITOR_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_bus_pms_monitor_1::W`](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_MONITOR_1 to value 0"]
impl crate::Resettable for BACKUP_BUS_PMS_MONITOR_1_SPEC {}
