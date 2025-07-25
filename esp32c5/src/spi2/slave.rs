#[doc = "Register `SLAVE` reader"]
pub type R = crate::R<SLAVE_SPEC>;
#[doc = "Register `SLAVE` writer"]
pub type W = crate::W<SLAVE_SPEC>;
#[doc = "Field `CLK_MODE` reader - Configures SPI clock mode.\\\\ 0: SPI clock is off when CS becomes inactive.\\\\ 1: SPI clock is delayed one cycle after CS becomes inactive.\\\\ 2: SPI clock is delayed two cycles after CS becomes inactive.\\\\ 3: SPI clock is always on.\\\\ Can be configured in CONF state."]
pub type CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - Configures SPI clock mode.\\\\ 0: SPI clock is off when CS becomes inactive.\\\\ 1: SPI clock is delayed one cycle after CS becomes inactive.\\\\ 2: SPI clock is delayed two cycles after CS becomes inactive.\\\\ 3: SPI clock is always on.\\\\ Can be configured in CONF state."]
pub type CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_MODE_13` reader - Configure clock mode.\\\\ 0: Support SPI clock mode 0 or 2. See Table <a href=\"table:clock_pol_slave\">link</a>.\\\\ 1: Support SPI clock mode 1 or 3. See Table <a href=\"table:clock_pol_slave\">link</a>.\\\\"]
pub type CLK_MODE_13_R = crate::BitReader;
#[doc = "Field `CLK_MODE_13` writer - Configure clock mode.\\\\ 0: Support SPI clock mode 0 or 2. See Table <a href=\"table:clock_pol_slave\">link</a>.\\\\ 1: Support SPI clock mode 1 or 3. See Table <a href=\"table:clock_pol_slave\">link</a>.\\\\"]
pub type CLK_MODE_13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSCK_DATA_OUT` reader - Configures the edge of output data.\\\\ 0: Output data at TSCK rising edge.\\\\ 1: Output data at RSCK rising edge.\\\\"]
pub type RSCK_DATA_OUT_R = crate::BitReader;
#[doc = "Field `RSCK_DATA_OUT` writer - Configures the edge of output data.\\\\ 0: Output data at TSCK rising edge.\\\\ 1: Output data at RSCK rising edge.\\\\"]
pub type RSCK_DATA_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RDDMA_BITLEN_EN` reader - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Rd_DMA transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
pub type SLV_RDDMA_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDDMA_BITLEN_EN` writer - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Rd_DMA transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
pub type SLV_RDDMA_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WRDMA_BITLEN_EN` reader - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Wr_DMA transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
pub type SLV_WRDMA_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRDMA_BITLEN_EN` writer - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Wr_DMA transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
pub type SLV_WRDMA_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RDBUF_BITLEN_EN` reader - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Rd_BUF transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
pub type SLV_RDBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDBUF_BITLEN_EN` writer - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Rd_BUF transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
pub type SLV_RDBUF_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WRBUF_BITLEN_EN` reader - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Wr_BUF transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
pub type SLV_WRBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRBUF_BITLEN_EN` writer - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Wr_BUF transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
pub type SLV_WRBUF_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_LAST_BYTE_STRB` reader - Represents the effective bit of the last received data byte in SPI slave FD and HD mode."]
pub type SLV_LAST_BYTE_STRB_R = crate::FieldReader;
#[doc = "Field `DMA_SEG_MAGIC_VALUE` reader - Configures the magic value of BM table in DMA-controlled configurable segmented transfer."]
pub type DMA_SEG_MAGIC_VALUE_R = crate::FieldReader;
#[doc = "Field `DMA_SEG_MAGIC_VALUE` writer - Configures the magic value of BM table in DMA-controlled configurable segmented transfer."]
pub type DMA_SEG_MAGIC_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE` reader - Configures SPI work mode.\\\\ 0: Master\\\\ 1: Slave\\\\"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Configures SPI work mode.\\\\ 0: Master\\\\ 1: Slave\\\\"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFT_RESET` writer - Configures whether to reset the SPI clock line, CS line, and data line via software.\\\\ 0: Not reset\\\\ 1: Reset\\\\ Can be configured in CONF state."]
pub type SOFT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_CONF` reader - Configures whether or not to enable the CONF state of current DMA-controlled configurable segmented transfer.\\\\ 0: No effect, which means the current transfer is not a configurable segmented transfer.\\\\ 1: Enable, which means a configurable segmented transfer is started.\\\\"]
pub type USR_CONF_R = crate::BitReader;
#[doc = "Field `USR_CONF` writer - Configures whether or not to enable the CONF state of current DMA-controlled configurable segmented transfer.\\\\ 0: No effect, which means the current transfer is not a configurable segmented transfer.\\\\ 1: Enable, which means a configurable segmented transfer is started.\\\\"]
pub type USR_CONF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_FD_WAIT_DMA_TX_DATA` reader - Configures whether or not to wait DMA TX data gets ready before starting SPI transfer in master full-duplex transfer.\\\\ 0: Not wait\\\\ 1: Wait\\\\"]
pub type MST_FD_WAIT_DMA_TX_DATA_R = crate::BitReader;
#[doc = "Field `MST_FD_WAIT_DMA_TX_DATA` writer - Configures whether or not to wait DMA TX data gets ready before starting SPI transfer in master full-duplex transfer.\\\\ 0: Not wait\\\\ 1: Wait\\\\"]
pub type MST_FD_WAIT_DMA_TX_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures SPI clock mode.\\\\ 0: SPI clock is off when CS becomes inactive.\\\\ 1: SPI clock is delayed one cycle after CS becomes inactive.\\\\ 2: SPI clock is delayed two cycles after CS becomes inactive.\\\\ 3: SPI clock is always on.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configure clock mode.\\\\ 0: Support SPI clock mode 0 or 2. See Table <a href=\"table:clock_pol_slave\">link</a>.\\\\ 1: Support SPI clock mode 1 or 3. See Table <a href=\"table:clock_pol_slave\">link</a>.\\\\"]
    #[inline(always)]
    pub fn clk_mode_13(&self) -> CLK_MODE_13_R {
        CLK_MODE_13_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the edge of output data.\\\\ 0: Output data at TSCK rising edge.\\\\ 1: Output data at RSCK rising edge.\\\\"]
    #[inline(always)]
    pub fn rsck_data_out(&self) -> RSCK_DATA_OUT_R {
        RSCK_DATA_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Rd_DMA transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
    #[inline(always)]
    pub fn slv_rddma_bitlen_en(&self) -> SLV_RDDMA_BITLEN_EN_R {
        SLV_RDDMA_BITLEN_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Wr_DMA transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
    #[inline(always)]
    pub fn slv_wrdma_bitlen_en(&self) -> SLV_WRDMA_BITLEN_EN_R {
        SLV_WRDMA_BITLEN_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Rd_BUF transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&self) -> SLV_RDBUF_BITLEN_EN_R {
        SLV_RDBUF_BITLEN_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Wr_BUF transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&self) -> SLV_WRBUF_BITLEN_EN_R {
        SLV_WRBUF_BITLEN_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - Represents the effective bit of the last received data byte in SPI slave FD and HD mode."]
    #[inline(always)]
    pub fn slv_last_byte_strb(&self) -> SLV_LAST_BYTE_STRB_R {
        SLV_LAST_BYTE_STRB_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 22:25 - Configures the magic value of BM table in DMA-controlled configurable segmented transfer."]
    #[inline(always)]
    pub fn dma_seg_magic_value(&self) -> DMA_SEG_MAGIC_VALUE_R {
        DMA_SEG_MAGIC_VALUE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - Configures SPI work mode.\\\\ 0: Master\\\\ 1: Slave\\\\"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether or not to enable the CONF state of current DMA-controlled configurable segmented transfer.\\\\ 0: No effect, which means the current transfer is not a configurable segmented transfer.\\\\ 1: Enable, which means a configurable segmented transfer is started.\\\\"]
    #[inline(always)]
    pub fn usr_conf(&self) -> USR_CONF_R {
        USR_CONF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to wait DMA TX data gets ready before starting SPI transfer in master full-duplex transfer.\\\\ 0: Not wait\\\\ 1: Wait\\\\"]
    #[inline(always)]
    pub fn mst_fd_wait_dma_tx_data(&self) -> MST_FD_WAIT_DMA_TX_DATA_R {
        MST_FD_WAIT_DMA_TX_DATA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE")
            .field("clk_mode", &self.clk_mode())
            .field("clk_mode_13", &self.clk_mode_13())
            .field("rsck_data_out", &self.rsck_data_out())
            .field("slv_rddma_bitlen_en", &self.slv_rddma_bitlen_en())
            .field("slv_wrdma_bitlen_en", &self.slv_wrdma_bitlen_en())
            .field("slv_rdbuf_bitlen_en", &self.slv_rdbuf_bitlen_en())
            .field("slv_wrbuf_bitlen_en", &self.slv_wrbuf_bitlen_en())
            .field("slv_last_byte_strb", &self.slv_last_byte_strb())
            .field("dma_seg_magic_value", &self.dma_seg_magic_value())
            .field("mode", &self.mode())
            .field("usr_conf", &self.usr_conf())
            .field("mst_fd_wait_dma_tx_data", &self.mst_fd_wait_dma_tx_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures SPI clock mode.\\\\ 0: SPI clock is off when CS becomes inactive.\\\\ 1: SPI clock is delayed one cycle after CS becomes inactive.\\\\ 2: SPI clock is delayed two cycles after CS becomes inactive.\\\\ 3: SPI clock is always on.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<SLAVE_SPEC> {
        CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configure clock mode.\\\\ 0: Support SPI clock mode 0 or 2. See Table <a href=\"table:clock_pol_slave\">link</a>.\\\\ 1: Support SPI clock mode 1 or 3. See Table <a href=\"table:clock_pol_slave\">link</a>.\\\\"]
    #[inline(always)]
    pub fn clk_mode_13(&mut self) -> CLK_MODE_13_W<SLAVE_SPEC> {
        CLK_MODE_13_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the edge of output data.\\\\ 0: Output data at TSCK rising edge.\\\\ 1: Output data at RSCK rising edge.\\\\"]
    #[inline(always)]
    pub fn rsck_data_out(&mut self) -> RSCK_DATA_OUT_W<SLAVE_SPEC> {
        RSCK_DATA_OUT_W::new(self, 3)
    }
    #[doc = "Bit 8 - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Rd_DMA transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
    #[inline(always)]
    pub fn slv_rddma_bitlen_en(&mut self) -> SLV_RDDMA_BITLEN_EN_W<SLAVE_SPEC> {
        SLV_RDDMA_BITLEN_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Wr_DMA transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
    #[inline(always)]
    pub fn slv_wrdma_bitlen_en(&mut self) -> SLV_WRDMA_BITLEN_EN_W<SLAVE_SPEC> {
        SLV_WRDMA_BITLEN_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Rd_BUF transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&mut self) -> SLV_RDBUF_BITLEN_EN_W<SLAVE_SPEC> {
        SLV_RDBUF_BITLEN_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to use SPI_SLV_DATA_BITLEN to store the data bit length of Wr_BUF transfer.\\\\ 0: Not use\\\\ 1: Use\\\\"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&mut self) -> SLV_WRBUF_BITLEN_EN_W<SLAVE_SPEC> {
        SLV_WRBUF_BITLEN_EN_W::new(self, 11)
    }
    #[doc = "Bits 22:25 - Configures the magic value of BM table in DMA-controlled configurable segmented transfer."]
    #[inline(always)]
    pub fn dma_seg_magic_value(&mut self) -> DMA_SEG_MAGIC_VALUE_W<SLAVE_SPEC> {
        DMA_SEG_MAGIC_VALUE_W::new(self, 22)
    }
    #[doc = "Bit 26 - Configures SPI work mode.\\\\ 0: Master\\\\ 1: Slave\\\\"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<SLAVE_SPEC> {
        MODE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether to reset the SPI clock line, CS line, and data line via software.\\\\ 0: Not reset\\\\ 1: Reset\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<SLAVE_SPEC> {
        SOFT_RESET_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to enable the CONF state of current DMA-controlled configurable segmented transfer.\\\\ 0: No effect, which means the current transfer is not a configurable segmented transfer.\\\\ 1: Enable, which means a configurable segmented transfer is started.\\\\"]
    #[inline(always)]
    pub fn usr_conf(&mut self) -> USR_CONF_W<SLAVE_SPEC> {
        USR_CONF_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to wait DMA TX data gets ready before starting SPI transfer in master full-duplex transfer.\\\\ 0: Not wait\\\\ 1: Wait\\\\"]
    #[inline(always)]
    pub fn mst_fd_wait_dma_tx_data(&mut self) -> MST_FD_WAIT_DMA_TX_DATA_W<SLAVE_SPEC> {
        MST_FD_WAIT_DMA_TX_DATA_W::new(self, 29)
    }
}
#[doc = "SPI slave control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slave::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave::R`](R) reader structure"]
impl crate::Readable for SLAVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave::W`](W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLAVE to value 0x0280_0000"]
impl crate::Resettable for SLAVE_SPEC {
    const RESET_VALUE: u32 = 0x0280_0000;
}
