#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `SDA_FORCE_OUT` reader - ."]
pub type SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SDA_FORCE_OUT` writer - ."]
pub type SDA_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_FORCE_OUT` reader - ."]
pub type SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SCL_FORCE_OUT` writer - ."]
pub type SCL_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_SCL_LEVEL` reader - ."]
pub type SAMPLE_SCL_LEVEL_R = crate::BitReader;
#[doc = "Field `SAMPLE_SCL_LEVEL` writer - ."]
pub type SAMPLE_SCL_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL_ACK_LEVEL` reader - ."]
pub type RX_FULL_ACK_LEVEL_R = crate::BitReader;
#[doc = "Field `RX_FULL_ACK_LEVEL` writer - ."]
pub type RX_FULL_ACK_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS_MODE` reader - ."]
pub type MS_MODE_R = crate::BitReader;
#[doc = "Field `MS_MODE` writer - ."]
pub type MS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START` writer - ."]
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_LSB_FIRST` reader - ."]
pub type TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST` writer - ."]
pub type TX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_LSB_FIRST` reader - ."]
pub type RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST` writer - ."]
pub type RX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - ."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - ."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_EN` reader - ."]
pub type ARBITRATION_EN_R = crate::BitReader;
#[doc = "Field `ARBITRATION_EN` writer - ."]
pub type ARBITRATION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSM_RST` writer - ."]
pub type FSM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_UPGATE` writer - ."]
pub type CONF_UPGATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_TX_AUTO_START_EN` reader - ."]
pub type SLV_TX_AUTO_START_EN_R = crate::BitReader;
#[doc = "Field `SLV_TX_AUTO_START_EN` writer - ."]
pub type SLV_TX_AUTO_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_10BIT_RW_CHECK_EN` reader - ."]
pub type ADDR_10BIT_RW_CHECK_EN_R = crate::BitReader;
#[doc = "Field `ADDR_10BIT_RW_CHECK_EN` writer - ."]
pub type ADDR_10BIT_RW_CHECK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_BROADCASTING_EN` reader - ."]
pub type ADDR_BROADCASTING_EN_R = crate::BitReader;
#[doc = "Field `ADDR_BROADCASTING_EN` writer - ."]
pub type ADDR_BROADCASTING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn sample_scl_level(&self) -> SAMPLE_SCL_LEVEL_R {
        SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn rx_full_ack_level(&self) -> RX_FULL_ACK_LEVEL_R {
        RX_FULL_ACK_LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - ."]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ."]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ."]
    #[inline(always)]
    pub fn arbitration_en(&self) -> ARBITRATION_EN_R {
        ARBITRATION_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    pub fn slv_tx_auto_start_en(&self) -> SLV_TX_AUTO_START_EN_R {
        SLV_TX_AUTO_START_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    pub fn addr_10bit_rw_check_en(&self) -> ADDR_10BIT_RW_CHECK_EN_R {
        ADDR_10BIT_RW_CHECK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ."]
    #[inline(always)]
    pub fn addr_broadcasting_en(&self) -> ADDR_BROADCASTING_EN_R {
        ADDR_BROADCASTING_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR")
            .field("addr_broadcasting_en", &self.addr_broadcasting_en())
            .field("addr_10bit_rw_check_en", &self.addr_10bit_rw_check_en())
            .field("slv_tx_auto_start_en", &self.slv_tx_auto_start_en())
            .field("arbitration_en", &self.arbitration_en())
            .field("clk_en", &self.clk_en())
            .field("rx_lsb_first", &self.rx_lsb_first())
            .field("tx_lsb_first", &self.tx_lsb_first())
            .field("ms_mode", &self.ms_mode())
            .field("rx_full_ack_level", &self.rx_full_ack_level())
            .field("sample_scl_level", &self.sample_scl_level())
            .field("scl_force_out", &self.scl_force_out())
            .field("sda_force_out", &self.sda_force_out())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<'_, CTR_SPEC> {
        SDA_FORCE_OUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<'_, CTR_SPEC> {
        SCL_FORCE_OUT_W::new(self, 1)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn sample_scl_level(&mut self) -> SAMPLE_SCL_LEVEL_W<'_, CTR_SPEC> {
        SAMPLE_SCL_LEVEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn rx_full_ack_level(&mut self) -> RX_FULL_ACK_LEVEL_W<'_, CTR_SPEC> {
        RX_FULL_ACK_LEVEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn ms_mode(&mut self) -> MS_MODE_W<'_, CTR_SPEC> {
        MS_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - ."]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TRANS_START_W<'_, CTR_SPEC> {
        TRANS_START_W::new(self, 5)
    }
    #[doc = "Bit 6 - ."]
    #[inline(always)]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<'_, CTR_SPEC> {
        TX_LSB_FIRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - ."]
    #[inline(always)]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<'_, CTR_SPEC> {
        RX_LSB_FIRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - ."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, CTR_SPEC> {
        CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - ."]
    #[inline(always)]
    pub fn arbitration_en(&mut self) -> ARBITRATION_EN_W<'_, CTR_SPEC> {
        ARBITRATION_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - ."]
    #[inline(always)]
    pub fn fsm_rst(&mut self) -> FSM_RST_W<'_, CTR_SPEC> {
        FSM_RST_W::new(self, 10)
    }
    #[doc = "Bit 11 - ."]
    #[inline(always)]
    pub fn conf_upgate(&mut self) -> CONF_UPGATE_W<'_, CTR_SPEC> {
        CONF_UPGATE_W::new(self, 11)
    }
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    pub fn slv_tx_auto_start_en(&mut self) -> SLV_TX_AUTO_START_EN_W<'_, CTR_SPEC> {
        SLV_TX_AUTO_START_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    pub fn addr_10bit_rw_check_en(&mut self) -> ADDR_10BIT_RW_CHECK_EN_W<'_, CTR_SPEC> {
        ADDR_10BIT_RW_CHECK_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - ."]
    #[inline(always)]
    pub fn addr_broadcasting_en(&mut self) -> ADDR_BROADCASTING_EN_W<'_, CTR_SPEC> {
        ADDR_BROADCASTING_EN_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CTR_SPEC {}
