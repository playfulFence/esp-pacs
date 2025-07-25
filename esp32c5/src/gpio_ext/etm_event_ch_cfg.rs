#[doc = "Register `ETM_EVENT_CH%s_CFG` reader"]
pub type R = crate::R<ETM_EVENT_CH_CFG_SPEC>;
#[doc = "Register `ETM_EVENT_CH%s_CFG` writer"]
pub type W = crate::W<ETM_EVENT_CH_CFG_SPEC>;
#[doc = "Field `ETM_CH0_EVENT_SEL` reader - Configures to select GPIO for ETM event channel.\\\\ 0: Select GPIO0\\\\ 1: Select GPIO1\\\\ ......\\\\ 27: Select GPIO27\\\\ 28: Select GPIO28\\\\ 29 ~ 63: Reserved\\\\"]
pub type ETM_CH0_EVENT_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_CH0_EVENT_SEL` writer - Configures to select GPIO for ETM event channel.\\\\ 0: Select GPIO0\\\\ 1: Select GPIO1\\\\ ......\\\\ 27: Select GPIO27\\\\ 28: Select GPIO28\\\\ 29 ~ 63: Reserved\\\\"]
pub type ETM_CH0_EVENT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ETM_CH0_EVENT_EN` reader - Configures whether or not to enable ETM event send.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_CH0_EVENT_EN_R = crate::BitReader;
#[doc = "Field `ETM_CH0_EVENT_EN` writer - Configures whether or not to enable ETM event send.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_CH0_EVENT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures to select GPIO for ETM event channel.\\\\ 0: Select GPIO0\\\\ 1: Select GPIO1\\\\ ......\\\\ 27: Select GPIO27\\\\ 28: Select GPIO28\\\\ 29 ~ 63: Reserved\\\\"]
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> ETM_CH0_EVENT_SEL_R {
        ETM_CH0_EVENT_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ETM event send.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> ETM_CH0_EVENT_EN_R {
        ETM_CH0_EVENT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_EVENT_CH_CFG")
            .field("etm_ch0_event_sel", &self.etm_ch0_event_sel())
            .field("etm_ch0_event_en", &self.etm_ch0_event_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures to select GPIO for ETM event channel.\\\\ 0: Select GPIO0\\\\ 1: Select GPIO1\\\\ ......\\\\ 27: Select GPIO27\\\\ 28: Select GPIO28\\\\ 29 ~ 63: Reserved\\\\"]
    #[inline(always)]
    pub fn etm_ch0_event_sel(&mut self) -> ETM_CH0_EVENT_SEL_W<ETM_EVENT_CH_CFG_SPEC> {
        ETM_CH0_EVENT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ETM event send.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_ch0_event_en(&mut self) -> ETM_CH0_EVENT_EN_W<ETM_EVENT_CH_CFG_SPEC> {
        ETM_CH0_EVENT_EN_W::new(self, 7)
    }
}
#[doc = "Etm Config register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_event_ch_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_EVENT_CH_CFG_SPEC;
impl crate::RegisterSpec for ETM_EVENT_CH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_event_ch_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_EVENT_CH_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_event_ch_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_EVENT_CH_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_EVENT_CH%s_CFG to value 0"]
impl crate::Resettable for ETM_EVENT_CH_CFG_SPEC {}
