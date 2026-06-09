#[doc = "Register `FIFO_CONF` reader"]
pub type R = crate::R<FIFO_CONF_SPEC>;
#[doc = "Register `FIFO_CONF` writer"]
pub type W = crate::W<FIFO_CONF_SPEC>;
#[doc = "Field `RXFIFO_WM_THRHD` reader - ."]
pub type RXFIFO_WM_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_WM_THRHD` writer - ."]
pub type RXFIFO_WM_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TXFIFO_WM_THRHD` reader - ."]
pub type TXFIFO_WM_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_WM_THRHD` writer - ."]
pub type TXFIFO_WM_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NONFIFO_EN` reader - ."]
pub type NONFIFO_EN_R = crate::BitReader;
#[doc = "Field `NONFIFO_EN` writer - ."]
pub type NONFIFO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_ADDR_CFG_EN` reader - ."]
pub type FIFO_ADDR_CFG_EN_R = crate::BitReader;
#[doc = "Field `FIFO_ADDR_CFG_EN` writer - ."]
pub type FIFO_ADDR_CFG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RST` reader - ."]
pub type RX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RST` writer - ."]
pub type RX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_RST` reader - ."]
pub type TX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RST` writer - ."]
pub type TX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_PRT_EN` reader - ."]
pub type FIFO_PRT_EN_R = crate::BitReader;
#[doc = "Field `FIFO_PRT_EN` writer - ."]
pub type FIFO_PRT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - ."]
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&self) -> RXFIFO_WM_THRHD_R {
        RXFIFO_WM_THRHD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - ."]
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&self) -> TXFIFO_WM_THRHD_R {
        TXFIFO_WM_THRHD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - ."]
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NONFIFO_EN_R {
        NONFIFO_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> FIFO_ADDR_CFG_EN_R {
        FIFO_ADDR_CFG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TX_FIFO_RST_R {
        TX_FIFO_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ."]
    #[inline(always)]
    pub fn fifo_prt_en(&self) -> FIFO_PRT_EN_R {
        FIFO_PRT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field("fifo_prt_en", &self.fifo_prt_en())
            .field("tx_fifo_rst", &self.tx_fifo_rst())
            .field("rx_fifo_rst", &self.rx_fifo_rst())
            .field("fifo_addr_cfg_en", &self.fifo_addr_cfg_en())
            .field("nonfifo_en", &self.nonfifo_en())
            .field("txfifo_wm_thrhd", &self.txfifo_wm_thrhd())
            .field("rxfifo_wm_thrhd", &self.rxfifo_wm_thrhd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - ."]
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&mut self) -> RXFIFO_WM_THRHD_W<'_, FIFO_CONF_SPEC> {
        RXFIFO_WM_THRHD_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - ."]
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&mut self) -> TXFIFO_WM_THRHD_W<'_, FIFO_CONF_SPEC> {
        TXFIFO_WM_THRHD_W::new(self, 5)
    }
    #[doc = "Bit 10 - ."]
    #[inline(always)]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W<'_, FIFO_CONF_SPEC> {
        NONFIFO_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - ."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&mut self) -> FIFO_ADDR_CFG_EN_W<'_, FIFO_CONF_SPEC> {
        FIFO_ADDR_CFG_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<'_, FIFO_CONF_SPEC> {
        RX_FIFO_RST_W::new(self, 12)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<'_, FIFO_CONF_SPEC> {
        TX_FIFO_RST_W::new(self, 13)
    }
    #[doc = "Bit 14 - ."]
    #[inline(always)]
    pub fn fifo_prt_en(&mut self) -> FIFO_PRT_EN_W<'_, FIFO_CONF_SPEC> {
        FIFO_PRT_EN_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_conf::R`](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_conf::W`](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0"]
impl crate::Resettable for FIFO_CONF_SPEC {}
