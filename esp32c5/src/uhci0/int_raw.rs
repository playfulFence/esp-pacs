#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `RX_START_INT_RAW` reader - The raw interrupt status of UHCI_RX_START_INT."]
pub type RX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_START_INT_RAW` writer - The raw interrupt status of UHCI_RX_START_INT."]
pub type RX_START_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START_INT_RAW` reader - The raw interrupt status of UHCI_TX_START_INT."]
pub type TX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_START_INT_RAW` writer - The raw interrupt status of UHCI_TX_START_INT."]
pub type TX_START_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG_INT_RAW` reader - The raw interrupt status of UHCI_RX_HUNG_INT."]
pub type RX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_RAW` writer - The raw interrupt status of UHCI_RX_HUNG_INT."]
pub type RX_HUNG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG_INT_RAW` reader - The raw interrupt status of UHCI_TX_HUNG_INT."]
pub type TX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_RAW` writer - The raw interrupt status of UHCI_TX_HUNG_INT."]
pub type TX_HUNG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q_INT_RAW` reader - The raw interrupt status of UHCI_SEND_S_REG_Q_INT."]
pub type SEND_S_REG_Q_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q_INT_RAW` writer - The raw interrupt status of UHCI_SEND_S_REG_Q_INT."]
pub type SEND_S_REG_Q_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q_INT_RAW` reader - The raw interrupt status of UHCI_SEND_A_REG_Q_INT."]
pub type SEND_A_REG_Q_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q_INT_RAW` writer - The raw interrupt status of UHCI_SEND_A_REG_Q_INT."]
pub type SEND_A_REG_Q_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_INT_RAW` reader - The raw interrupt status of UHCI_OUT_EOF_INT."]
pub type OUT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_RAW` writer - The raw interrupt status of UHCI_OUT_EOF_INT."]
pub type OUT_EOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL0_INT_RAW` reader - The raw interrupt status of UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_RAW_R = crate::BitReader;
#[doc = "Field `APP_CTRL0_INT_RAW` writer - The raw interrupt status of UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1_INT_RAW` reader - The raw interrupt status of UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_RAW_R = crate::BitReader;
#[doc = "Field `APP_CTRL1_INT_RAW` writer - The raw interrupt status of UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start_int_raw(&self) -> RX_START_INT_RAW_R {
        RX_START_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start_int_raw(&self) -> TX_START_INT_RAW_R {
        TX_START_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> RX_HUNG_INT_RAW_R {
        RX_HUNG_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> TX_HUNG_INT_RAW_R {
        TX_HUNG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q_int_raw(&self) -> SEND_S_REG_Q_INT_RAW_R {
        SEND_S_REG_Q_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q_int_raw(&self) -> SEND_A_REG_Q_INT_RAW_R {
        SEND_A_REG_Q_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of UHCI_OUT_EOF_INT."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_raw(&self) -> APP_CTRL0_INT_RAW_R {
        APP_CTRL0_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_raw(&self) -> APP_CTRL1_INT_RAW_R {
        APP_CTRL1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("rx_start_int_raw", &self.rx_start_int_raw())
            .field("tx_start_int_raw", &self.tx_start_int_raw())
            .field("rx_hung_int_raw", &self.rx_hung_int_raw())
            .field("tx_hung_int_raw", &self.tx_hung_int_raw())
            .field("send_s_reg_q_int_raw", &self.send_s_reg_q_int_raw())
            .field("send_a_reg_q_int_raw", &self.send_a_reg_q_int_raw())
            .field("out_eof_int_raw", &self.out_eof_int_raw())
            .field("app_ctrl0_int_raw", &self.app_ctrl0_int_raw())
            .field("app_ctrl1_int_raw", &self.app_ctrl1_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start_int_raw(&mut self) -> RX_START_INT_RAW_W<INT_RAW_SPEC> {
        RX_START_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start_int_raw(&mut self) -> TX_START_INT_RAW_W<INT_RAW_SPEC> {
        TX_START_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status of UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung_int_raw(&mut self) -> RX_HUNG_INT_RAW_W<INT_RAW_SPEC> {
        RX_HUNG_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status of UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung_int_raw(&mut self) -> TX_HUNG_INT_RAW_W<INT_RAW_SPEC> {
        TX_HUNG_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt status of UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q_int_raw(&mut self) -> SEND_S_REG_Q_INT_RAW_W<INT_RAW_SPEC> {
        SEND_S_REG_Q_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt status of UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q_int_raw(&mut self) -> SEND_A_REG_Q_INT_RAW_W<INT_RAW_SPEC> {
        SEND_A_REG_Q_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt status of UHCI_OUT_EOF_INT."]
    #[inline(always)]
    pub fn out_eof_int_raw(&mut self) -> OUT_EOF_INT_RAW_W<INT_RAW_SPEC> {
        OUT_EOF_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt status of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_raw(&mut self) -> APP_CTRL0_INT_RAW_W<INT_RAW_SPEC> {
        APP_CTRL0_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt status of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_raw(&mut self) -> APP_CTRL1_INT_RAW_W<INT_RAW_SPEC> {
        APP_CTRL1_INT_RAW_W::new(self, 8)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
