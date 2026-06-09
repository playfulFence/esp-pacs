#[doc = "Register `CH%s_TX_CONF0` reader"]
pub type R = crate::R<CH_TX_CONF0_SPEC>;
#[doc = "Register `CH%s_TX_CONF0` writer"]
pub type W = crate::W<CH_TX_CONF0_SPEC>;
#[doc = "Field `TX_START` writer - "]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_RD_RST` writer - "]
pub type MEM_RD_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_MEM_RST` writer - "]
pub type APB_MEM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CONTI_MODE` reader - "]
pub type TX_CONTI_MODE_R = crate::BitReader;
#[doc = "Field `TX_CONTI_MODE` writer - "]
pub type TX_CONTI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TX_WRAP_EN` reader - "]
pub type MEM_TX_WRAP_EN_R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN` writer - "]
pub type MEM_TX_WRAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_LV` reader - "]
pub type IDLE_OUT_LV_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_LV` writer - "]
pub type IDLE_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_EN` reader - "]
pub type IDLE_OUT_EN_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_EN` writer - "]
pub type IDLE_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STOP` reader - "]
pub type TX_STOP_R = crate::BitReader;
#[doc = "Field `TX_STOP` writer - "]
pub type TX_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_CNT` reader - "]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - "]
pub type DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_SIZE` reader - "]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - "]
pub type MEM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CARRIER_EFF_EN` reader - "]
pub type CARRIER_EFF_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EFF_EN` writer - "]
pub type CARRIER_EFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_EN` reader - "]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - "]
pub type CARRIER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV` reader - "]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - "]
pub type CARRIER_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFIFO_RST` writer - "]
pub type AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_UPDATE` writer - "]
pub type CONF_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_conti_mode(&self) -> TX_CONTI_MODE_R {
        TX_CONTI_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn idle_out_lv(&self) -> IDLE_OUT_LV_R {
        IDLE_OUT_LV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn idle_out_en(&self) -> IDLE_OUT_EN_R {
        IDLE_OUT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_stop(&self) -> TX_STOP_R {
        TX_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn carrier_eff_en(&self) -> CARRIER_EFF_EN_R {
        CARRIER_EFF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_CONF0")
            .field("carrier_out_lv", &self.carrier_out_lv())
            .field("carrier_en", &self.carrier_en())
            .field("carrier_eff_en", &self.carrier_eff_en())
            .field("mem_size", &self.mem_size())
            .field("div_cnt", &self.div_cnt())
            .field("tx_stop", &self.tx_stop())
            .field("idle_out_en", &self.idle_out_en())
            .field("idle_out_lv", &self.idle_out_lv())
            .field("mem_tx_wrap_en", &self.mem_tx_wrap_en())
            .field("tx_conti_mode", &self.tx_conti_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W<'_, CH_TX_CONF0_SPEC> {
        TX_START_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mem_rd_rst(&mut self) -> MEM_RD_RST_W<'_, CH_TX_CONF0_SPEC> {
        MEM_RD_RST_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W<'_, CH_TX_CONF0_SPEC> {
        APB_MEM_RST_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_conti_mode(&mut self) -> TX_CONTI_MODE_W<'_, CH_TX_CONF0_SPEC> {
        TX_CONTI_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W<'_, CH_TX_CONF0_SPEC> {
        MEM_TX_WRAP_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn idle_out_lv(&mut self) -> IDLE_OUT_LV_W<'_, CH_TX_CONF0_SPEC> {
        IDLE_OUT_LV_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn idle_out_en(&mut self) -> IDLE_OUT_EN_W<'_, CH_TX_CONF0_SPEC> {
        IDLE_OUT_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_stop(&mut self) -> TX_STOP_W<'_, CH_TX_CONF0_SPEC> {
        TX_STOP_W::new(self, 7)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<'_, CH_TX_CONF0_SPEC> {
        DIV_CNT_W::new(self, 8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<'_, CH_TX_CONF0_SPEC> {
        MEM_SIZE_W::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn carrier_eff_en(&mut self) -> CARRIER_EFF_EN_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_EFF_EN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_EN_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_OUT_LV_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W<'_, CH_TX_CONF0_SPEC> {
        AFIFO_RST_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W<'_, CH_TX_CONF0_SPEC> {
        CONF_UPDATE_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_tx_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_tx_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_TX_CONF0_SPEC;
impl crate::RegisterSpec for CH_TX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_tx_conf0::R`](R) reader structure"]
impl crate::Readable for CH_TX_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_tx_conf0::W`](W) writer structure"]
impl crate::Writable for CH_TX_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%s_TX_CONF0 to value 0"]
impl crate::Resettable for CH_TX_CONF0_SPEC {}
