#[doc = "Register `CH%s_RX_CONF0` reader"]
pub type R = crate::R<CH_RX_CONF0_SPEC>;
#[doc = "Register `CH%s_RX_CONF0` writer"]
pub type W = crate::W<CH_RX_CONF0_SPEC>;
#[doc = "Field `DIV_CNT` reader - "]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - "]
pub type DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDLE_THRES` reader - "]
pub type IDLE_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `IDLE_THRES` writer - "]
pub type IDLE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MEM_SIZE` reader - "]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - "]
pub type MEM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CARRIER_EN` reader - "]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - "]
pub type CARRIER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV` reader - "]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - "]
pub type CARRIER_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22"]
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CONF0")
            .field("carrier_out_lv", &self.carrier_out_lv())
            .field("carrier_en", &self.carrier_en())
            .field("mem_size", &self.mem_size())
            .field("idle_thres", &self.idle_thres())
            .field("div_cnt", &self.div_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<'_, CH_RX_CONF0_SPEC> {
        DIV_CNT_W::new(self, 0)
    }
    #[doc = "Bits 8:22"]
    #[inline(always)]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W<'_, CH_RX_CONF0_SPEC> {
        IDLE_THRES_W::new(self, 8)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<'_, CH_RX_CONF0_SPEC> {
        MEM_SIZE_W::new(self, 23)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<'_, CH_RX_CONF0_SPEC> {
        CARRIER_EN_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<'_, CH_RX_CONF0_SPEC> {
        CARRIER_OUT_LV_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_rx_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_rx_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_CONF0_SPEC;
impl crate::RegisterSpec for CH_RX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_conf0::R`](R) reader structure"]
impl crate::Readable for CH_RX_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_conf0::W`](W) writer structure"]
impl crate::Writable for CH_RX_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%s_RX_CONF0 to value 0"]
impl crate::Resettable for CH_RX_CONF0_SPEC {}
