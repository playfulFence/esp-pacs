#[doc = "Register `HWFC_CONF_SYNC` reader"]
pub type R = crate::R<HWFC_CONF_SYNC_SPEC>;
#[doc = "Register `HWFC_CONF_SYNC` writer"]
pub type W = crate::W<HWFC_CONF_SYNC_SPEC>;
#[doc = "Field `RX_FLOW_THRHD` reader - Configures the maximum number of data bytes that can be received during hardware flow control.\\\\Measurement unit: byte."]
pub type RX_FLOW_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_FLOW_THRHD` writer - Configures the maximum number of data bytes that can be received during hardware flow control.\\\\Measurement unit: byte."]
pub type RX_FLOW_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RX_FLOW_EN` reader - Configures whether or not to enable the LP UART receiver.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type RX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `RX_FLOW_EN` writer - Configures whether or not to enable the LP UART receiver.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type RX_FLOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 3:7 - Configures the maximum number of data bytes that can be received during hardware flow control.\\\\Measurement unit: byte."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Configures whether or not to enable the LP UART receiver.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWFC_CONF_SYNC")
            .field("rx_flow_thrhd", &self.rx_flow_thrhd())
            .field("rx_flow_en", &self.rx_flow_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:7 - Configures the maximum number of data bytes that can be received during hardware flow control.\\\\Measurement unit: byte."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<HWFC_CONF_SYNC_SPEC> {
        RX_FLOW_THRHD_W::new(self, 3)
    }
    #[doc = "Bit 8 - Configures whether or not to enable the LP UART receiver.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W<HWFC_CONF_SYNC_SPEC> {
        RX_FLOW_EN_W::new(self, 8)
    }
}
#[doc = "Hardware flow control configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hwfc_conf_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwfc_conf_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWFC_CONF_SYNC_SPEC;
impl crate::RegisterSpec for HWFC_CONF_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwfc_conf_sync::R`](R) reader structure"]
impl crate::Readable for HWFC_CONF_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hwfc_conf_sync::W`](W) writer structure"]
impl crate::Writable for HWFC_CONF_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HWFC_CONF_SYNC to value 0"]
impl crate::Resettable for HWFC_CONF_SYNC_SPEC {}
