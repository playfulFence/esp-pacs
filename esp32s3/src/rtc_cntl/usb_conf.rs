#[doc = "Register `USB_CONF` reader"]
pub type R = crate::R<USB_CONF_SPEC>;
#[doc = "Register `USB_CONF` writer"]
pub type W = crate::W<USB_CONF_SPEC>;
#[doc = "Field `USB_VREFH` reader - reg_usb_vrefh"]
pub type USB_VREFH_R = crate::FieldReader;
#[doc = "Field `USB_VREFH` writer - reg_usb_vrefh"]
pub type USB_VREFH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_VREFL` reader - reg_usb_vrefl"]
pub type USB_VREFL_R = crate::FieldReader;
#[doc = "Field `USB_VREFL` writer - reg_usb_vrefl"]
pub type USB_VREFL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_VREF_OVERRIDE` reader - reg_usb_vref_override"]
pub type USB_VREF_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_VREF_OVERRIDE` writer - reg_usb_vref_override"]
pub type USB_VREF_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_PAD_PULL_OVERRIDE` reader - reg_usb_pad_pull_override"]
pub type USB_PAD_PULL_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_PAD_PULL_OVERRIDE` writer - reg_usb_pad_pull_override"]
pub type USB_PAD_PULL_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DP_PULLUP` reader - reg_usb_dp_pullup"]
pub type USB_DP_PULLUP_R = crate::BitReader;
#[doc = "Field `USB_DP_PULLUP` writer - reg_usb_dp_pullup"]
pub type USB_DP_PULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DP_PULLDOWN` reader - reg_usb_dp_pulldown"]
pub type USB_DP_PULLDOWN_R = crate::BitReader;
#[doc = "Field `USB_DP_PULLDOWN` writer - reg_usb_dp_pulldown"]
pub type USB_DP_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DM_PULLUP` reader - reg_usb_dm_pullup"]
pub type USB_DM_PULLUP_R = crate::BitReader;
#[doc = "Field `USB_DM_PULLUP` writer - reg_usb_dm_pullup"]
pub type USB_DM_PULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DM_PULLDOWN` reader - reg_usb_dm_pulldown"]
pub type USB_DM_PULLDOWN_R = crate::BitReader;
#[doc = "Field `USB_DM_PULLDOWN` writer - reg_usb_dm_pulldown"]
pub type USB_DM_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_PULLUP_VALUE` reader - reg_usb_pullup_value"]
pub type USB_PULLUP_VALUE_R = crate::BitReader;
#[doc = "Field `USB_PULLUP_VALUE` writer - reg_usb_pullup_value"]
pub type USB_PULLUP_VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_PAD_ENABLE_OVERRIDE` reader - reg_usb_pad_enable_override"]
pub type USB_PAD_ENABLE_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_PAD_ENABLE_OVERRIDE` writer - reg_usb_pad_enable_override"]
pub type USB_PAD_ENABLE_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_PAD_ENABLE` reader - reg_usb_pad_enable"]
pub type USB_PAD_ENABLE_R = crate::BitReader;
#[doc = "Field `USB_PAD_ENABLE` writer - reg_usb_pad_enable"]
pub type USB_PAD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_TXM` reader - reg_usb_txm"]
pub type USB_TXM_R = crate::BitReader;
#[doc = "Field `USB_TXM` writer - reg_usb_txm"]
pub type USB_TXM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_TXP` reader - reg_usb_txp"]
pub type USB_TXP_R = crate::BitReader;
#[doc = "Field `USB_TXP` writer - reg_usb_txp"]
pub type USB_TXP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_TX_EN` reader - reg_usb_tx_en"]
pub type USB_TX_EN_R = crate::BitReader;
#[doc = "Field `USB_TX_EN` writer - reg_usb_tx_en"]
pub type USB_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_TX_EN_OVERRIDE` reader - reg_usb_tx_en_override"]
pub type USB_TX_EN_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_TX_EN_OVERRIDE` writer - reg_usb_tx_en_override"]
pub type USB_TX_EN_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_RESET_DISABLE` reader - reg_usb_reset_disable"]
pub type USB_RESET_DISABLE_R = crate::BitReader;
#[doc = "Field `USB_RESET_DISABLE` writer - reg_usb_reset_disable"]
pub type USB_RESET_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - reg_io_mux_reset_disable"]
pub type IO_MUX_RESET_DISABLE_R = crate::BitReader;
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - reg_io_mux_reset_disable"]
pub type IO_MUX_RESET_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_USB_PHY_SEL` reader - reg_sw_usb_phy_sel"]
pub type SW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_USB_PHY_SEL` writer - reg_sw_usb_phy_sel"]
pub type SW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HW_USB_PHY_SEL` reader - reg_sw_hw_usb_phy_sel"]
pub type SW_HW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_HW_USB_PHY_SEL` writer - reg_sw_hw_usb_phy_sel"]
pub type SW_HW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - reg_usb_vrefh"]
    #[inline(always)]
    pub fn usb_vrefh(&self) -> USB_VREFH_R {
        USB_VREFH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reg_usb_vrefl"]
    #[inline(always)]
    pub fn usb_vrefl(&self) -> USB_VREFL_R {
        USB_VREFL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - reg_usb_vref_override"]
    #[inline(always)]
    pub fn usb_vref_override(&self) -> USB_VREF_OVERRIDE_R {
        USB_VREF_OVERRIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_usb_pad_pull_override"]
    #[inline(always)]
    pub fn usb_pad_pull_override(&self) -> USB_PAD_PULL_OVERRIDE_R {
        USB_PAD_PULL_OVERRIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_usb_dp_pullup"]
    #[inline(always)]
    pub fn usb_dp_pullup(&self) -> USB_DP_PULLUP_R {
        USB_DP_PULLUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_usb_dp_pulldown"]
    #[inline(always)]
    pub fn usb_dp_pulldown(&self) -> USB_DP_PULLDOWN_R {
        USB_DP_PULLDOWN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_usb_dm_pullup"]
    #[inline(always)]
    pub fn usb_dm_pullup(&self) -> USB_DM_PULLUP_R {
        USB_DM_PULLUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_usb_dm_pulldown"]
    #[inline(always)]
    pub fn usb_dm_pulldown(&self) -> USB_DM_PULLDOWN_R {
        USB_DM_PULLDOWN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_usb_pullup_value"]
    #[inline(always)]
    pub fn usb_pullup_value(&self) -> USB_PULLUP_VALUE_R {
        USB_PULLUP_VALUE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_usb_pad_enable_override"]
    #[inline(always)]
    pub fn usb_pad_enable_override(&self) -> USB_PAD_ENABLE_OVERRIDE_R {
        USB_PAD_ENABLE_OVERRIDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_usb_pad_enable"]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> USB_PAD_ENABLE_R {
        USB_PAD_ENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_usb_txm"]
    #[inline(always)]
    pub fn usb_txm(&self) -> USB_TXM_R {
        USB_TXM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_usb_txp"]
    #[inline(always)]
    pub fn usb_txp(&self) -> USB_TXP_R {
        USB_TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_usb_tx_en"]
    #[inline(always)]
    pub fn usb_tx_en(&self) -> USB_TX_EN_R {
        USB_TX_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - reg_usb_tx_en_override"]
    #[inline(always)]
    pub fn usb_tx_en_override(&self) -> USB_TX_EN_OVERRIDE_R {
        USB_TX_EN_OVERRIDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_usb_reset_disable"]
    #[inline(always)]
    pub fn usb_reset_disable(&self) -> USB_RESET_DISABLE_R {
        USB_RESET_DISABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - reg_io_mux_reset_disable"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reg_sw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&self) -> SW_USB_PHY_SEL_R {
        SW_USB_PHY_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - reg_sw_hw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&self) -> SW_HW_USB_PHY_SEL_R {
        SW_HW_USB_PHY_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_CONF")
            .field("usb_vrefh", &self.usb_vrefh())
            .field("usb_vrefl", &self.usb_vrefl())
            .field("usb_vref_override", &self.usb_vref_override())
            .field("usb_pad_pull_override", &self.usb_pad_pull_override())
            .field("usb_dp_pullup", &self.usb_dp_pullup())
            .field("usb_dp_pulldown", &self.usb_dp_pulldown())
            .field("usb_dm_pullup", &self.usb_dm_pullup())
            .field("usb_dm_pulldown", &self.usb_dm_pulldown())
            .field("usb_pullup_value", &self.usb_pullup_value())
            .field("usb_pad_enable_override", &self.usb_pad_enable_override())
            .field("usb_pad_enable", &self.usb_pad_enable())
            .field("usb_txm", &self.usb_txm())
            .field("usb_txp", &self.usb_txp())
            .field("usb_tx_en", &self.usb_tx_en())
            .field("usb_tx_en_override", &self.usb_tx_en_override())
            .field("usb_reset_disable", &self.usb_reset_disable())
            .field("io_mux_reset_disable", &self.io_mux_reset_disable())
            .field("sw_usb_phy_sel", &self.sw_usb_phy_sel())
            .field("sw_hw_usb_phy_sel", &self.sw_hw_usb_phy_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_usb_vrefh"]
    #[inline(always)]
    pub fn usb_vrefh(&mut self) -> USB_VREFH_W<USB_CONF_SPEC> {
        USB_VREFH_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - reg_usb_vrefl"]
    #[inline(always)]
    pub fn usb_vrefl(&mut self) -> USB_VREFL_W<USB_CONF_SPEC> {
        USB_VREFL_W::new(self, 2)
    }
    #[doc = "Bit 4 - reg_usb_vref_override"]
    #[inline(always)]
    pub fn usb_vref_override(&mut self) -> USB_VREF_OVERRIDE_W<USB_CONF_SPEC> {
        USB_VREF_OVERRIDE_W::new(self, 4)
    }
    #[doc = "Bit 5 - reg_usb_pad_pull_override"]
    #[inline(always)]
    pub fn usb_pad_pull_override(&mut self) -> USB_PAD_PULL_OVERRIDE_W<USB_CONF_SPEC> {
        USB_PAD_PULL_OVERRIDE_W::new(self, 5)
    }
    #[doc = "Bit 6 - reg_usb_dp_pullup"]
    #[inline(always)]
    pub fn usb_dp_pullup(&mut self) -> USB_DP_PULLUP_W<USB_CONF_SPEC> {
        USB_DP_PULLUP_W::new(self, 6)
    }
    #[doc = "Bit 7 - reg_usb_dp_pulldown"]
    #[inline(always)]
    pub fn usb_dp_pulldown(&mut self) -> USB_DP_PULLDOWN_W<USB_CONF_SPEC> {
        USB_DP_PULLDOWN_W::new(self, 7)
    }
    #[doc = "Bit 8 - reg_usb_dm_pullup"]
    #[inline(always)]
    pub fn usb_dm_pullup(&mut self) -> USB_DM_PULLUP_W<USB_CONF_SPEC> {
        USB_DM_PULLUP_W::new(self, 8)
    }
    #[doc = "Bit 9 - reg_usb_dm_pulldown"]
    #[inline(always)]
    pub fn usb_dm_pulldown(&mut self) -> USB_DM_PULLDOWN_W<USB_CONF_SPEC> {
        USB_DM_PULLDOWN_W::new(self, 9)
    }
    #[doc = "Bit 10 - reg_usb_pullup_value"]
    #[inline(always)]
    pub fn usb_pullup_value(&mut self) -> USB_PULLUP_VALUE_W<USB_CONF_SPEC> {
        USB_PULLUP_VALUE_W::new(self, 10)
    }
    #[doc = "Bit 11 - reg_usb_pad_enable_override"]
    #[inline(always)]
    pub fn usb_pad_enable_override(&mut self) -> USB_PAD_ENABLE_OVERRIDE_W<USB_CONF_SPEC> {
        USB_PAD_ENABLE_OVERRIDE_W::new(self, 11)
    }
    #[doc = "Bit 12 - reg_usb_pad_enable"]
    #[inline(always)]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W<USB_CONF_SPEC> {
        USB_PAD_ENABLE_W::new(self, 12)
    }
    #[doc = "Bit 13 - reg_usb_txm"]
    #[inline(always)]
    pub fn usb_txm(&mut self) -> USB_TXM_W<USB_CONF_SPEC> {
        USB_TXM_W::new(self, 13)
    }
    #[doc = "Bit 14 - reg_usb_txp"]
    #[inline(always)]
    pub fn usb_txp(&mut self) -> USB_TXP_W<USB_CONF_SPEC> {
        USB_TXP_W::new(self, 14)
    }
    #[doc = "Bit 15 - reg_usb_tx_en"]
    #[inline(always)]
    pub fn usb_tx_en(&mut self) -> USB_TX_EN_W<USB_CONF_SPEC> {
        USB_TX_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - reg_usb_tx_en_override"]
    #[inline(always)]
    pub fn usb_tx_en_override(&mut self) -> USB_TX_EN_OVERRIDE_W<USB_CONF_SPEC> {
        USB_TX_EN_OVERRIDE_W::new(self, 16)
    }
    #[doc = "Bit 17 - reg_usb_reset_disable"]
    #[inline(always)]
    pub fn usb_reset_disable(&mut self) -> USB_RESET_DISABLE_W<USB_CONF_SPEC> {
        USB_RESET_DISABLE_W::new(self, 17)
    }
    #[doc = "Bit 18 - reg_io_mux_reset_disable"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W<USB_CONF_SPEC> {
        IO_MUX_RESET_DISABLE_W::new(self, 18)
    }
    #[doc = "Bit 19 - reg_sw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&mut self) -> SW_USB_PHY_SEL_W<USB_CONF_SPEC> {
        SW_USB_PHY_SEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - reg_sw_hw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&mut self) -> SW_HW_USB_PHY_SEL_W<USB_CONF_SPEC> {
        SW_HW_USB_PHY_SEL_W::new(self, 20)
    }
}
#[doc = "usb configure\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_CONF_SPEC;
impl crate::RegisterSpec for USB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_conf::R`](R) reader structure"]
impl crate::Readable for USB_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_conf::W`](W) writer structure"]
impl crate::Writable for USB_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_CONF to value 0"]
impl crate::Resettable for USB_CONF_SPEC {}
