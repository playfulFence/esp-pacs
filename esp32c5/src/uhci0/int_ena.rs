#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RX_START_INT_ENA` reader - Write 1 to enable UHCI_RX_START_INT."]
pub type RX_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_START_INT_ENA` writer - Write 1 to enable UHCI_RX_START_INT."]
pub type RX_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START_INT_ENA` reader - Write 1 to enable UHCI_TX_START_INT."]
pub type TX_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_START_INT_ENA` writer - Write 1 to enable UHCI_TX_START_INT."]
pub type TX_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG_INT_ENA` reader - Write 1 to enable UHCI_RX_HUNG_INT."]
pub type RX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_ENA` writer - Write 1 to enable UHCI_RX_HUNG_INT."]
pub type RX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG_INT_ENA` reader - Write 1 to enable UHCI_TX_HUNG_INT."]
pub type TX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_ENA` writer - Write 1 to enable UHCI_TX_HUNG_INT."]
pub type TX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q_INT_ENA` reader - Write 1 to enable UHCI_SEND_S_REG_Q_INT."]
pub type SEND_S_REG_Q_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q_INT_ENA` writer - Write 1 to enable UHCI_SEND_S_REG_Q_INT."]
pub type SEND_S_REG_Q_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q_INT_ENA` reader - Write 1 to enable UHCI_SEND_A_REG_Q_INT."]
pub type SEND_A_REG_Q_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q_INT_ENA` writer - Write 1 to enable UHCI_SEND_A_REG_Q_INT."]
pub type SEND_A_REG_Q_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` reader - Write 1 to enable UHCI_OUTLINK_EOF_ERR_INT."]
pub type OUTLINK_EOF_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` writer - Write 1 to enable UHCI_OUTLINK_EOF_ERR_INT."]
pub type OUTLINK_EOF_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL0_INT_ENA` reader - Write 1 to enable UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_ENA_R = crate::BitReader;
#[doc = "Field `APP_CTRL0_INT_ENA` writer - Write 1 to enable UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1_INT_ENA` reader - Write 1 to enable UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_ENA_R = crate::BitReader;
#[doc = "Field `APP_CTRL1_INT_ENA` writer - Write 1 to enable UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start_int_ena(&self) -> RX_START_INT_ENA_R {
        RX_START_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start_int_ena(&self) -> TX_START_INT_ENA_R {
        TX_START_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q_int_ena(&self) -> SEND_S_REG_Q_INT_ENA_R {
        SEND_S_REG_Q_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q_int_ena(&self) -> SEND_A_REG_Q_INT_ENA_R {
        SEND_A_REG_Q_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable UHCI_OUTLINK_EOF_ERR_INT."]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&self) -> OUTLINK_EOF_ERR_INT_ENA_R {
        OUTLINK_EOF_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to enable UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&self) -> APP_CTRL0_INT_ENA_R {
        APP_CTRL0_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to enable UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&self) -> APP_CTRL1_INT_ENA_R {
        APP_CTRL1_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("rx_start_int_ena", &self.rx_start_int_ena())
            .field("tx_start_int_ena", &self.tx_start_int_ena())
            .field("rx_hung_int_ena", &self.rx_hung_int_ena())
            .field("tx_hung_int_ena", &self.tx_hung_int_ena())
            .field("send_s_reg_q_int_ena", &self.send_s_reg_q_int_ena())
            .field("send_a_reg_q_int_ena", &self.send_a_reg_q_int_ena())
            .field("outlink_eof_err_int_ena", &self.outlink_eof_err_int_ena())
            .field("app_ctrl0_int_ena", &self.app_ctrl0_int_ena())
            .field("app_ctrl1_int_ena", &self.app_ctrl1_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start_int_ena(&mut self) -> RX_START_INT_ENA_W<INT_ENA_SPEC> {
        RX_START_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start_int_ena(&mut self) -> TX_START_INT_ENA_W<INT_ENA_SPEC> {
        TX_START_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        RX_HUNG_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        TX_HUNG_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q_int_ena(&mut self) -> SEND_S_REG_Q_INT_ENA_W<INT_ENA_SPEC> {
        SEND_S_REG_Q_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q_int_ena(&mut self) -> SEND_A_REG_Q_INT_ENA_W<INT_ENA_SPEC> {
        SEND_A_REG_Q_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable UHCI_OUTLINK_EOF_ERR_INT."]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&mut self) -> OUTLINK_EOF_ERR_INT_ENA_W<INT_ENA_SPEC> {
        OUTLINK_EOF_ERR_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to enable UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&mut self) -> APP_CTRL0_INT_ENA_W<INT_ENA_SPEC> {
        APP_CTRL0_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to enable UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&mut self) -> APP_CTRL1_INT_ENA_W<INT_ENA_SPEC> {
        APP_CTRL1_INT_ENA_W::new(self, 8)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
