#[doc = "Register `DMA_INT_SET` writer"]
pub type W = crate::W<DMA_INT_SET_SPEC>;
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_SET` writer - Write 1 to set SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DMA_INFIFO_FULL_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_SET` writer - Write 1 to set SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DMA_OUTFIFO_EMPTY_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_EX_QPI_INT_SET` writer - Write 1 to set SPI_SLV_EX_QPI_INT interrupt."]
pub type SLV_EX_QPI_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_EN_QPI_INT_SET` writer - Write 1 to set SPI_SLV_EN_QPI_INT interrupt."]
pub type SLV_EN_QPI_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD7_INT_SET` writer - Write 1 to set SPI_SLV_CMD7_INT interrupt."]
pub type SLV_CMD7_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD8_INT_SET` writer - Write 1 to set SPI_SLV_CMD8_INT interrupt."]
pub type SLV_CMD8_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD9_INT_SET` writer - Write 1 to set SPI_SLV_CMD9_INT interrupt."]
pub type SLV_CMD9_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMDA_INT_SET` writer - Write 1 to set SPI_SLV_CMDA_INT interrupt."]
pub type SLV_CMDA_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_DMA_DONE_INT_SET` writer - Write 1 to set SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SLV_RD_DMA_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_DMA_DONE_INT_SET` writer - Write 1 to set SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SLV_WR_DMA_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_BUF_DONE_INT_SET` writer - Write 1 to set SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SLV_RD_BUF_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_BUF_DONE_INT_SET` writer - Write 1 to set SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SLV_WR_BUF_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE_INT_SET` writer - Write 1 to set SPI_TRANS_DONE_INT interrupt."]
pub type TRANS_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_SET` writer - Write 1 to set SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DMA_SEG_TRANS_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEG_MAGIC_ERR_INT_SET` writer - Write 1 to set SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SEG_MAGIC_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_SET` writer - The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SLV_BUF_ADDR_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD_ERR_INT_SET` writer - Write 1 to set SPI_SLV_CMD_ERR_INT interrupt."]
pub type SLV_CMD_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_SET` writer - Write 1 to set SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MST_RX_AFIFO_WFULL_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_SET` writer - Write 1 to set SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP2_INT_SET` writer - Write 1 to set SPI_APP2_INT interrupt."]
pub type APP2_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP1_INT_SET` writer - Write 1 to set SPI_APP1_INT interrupt."]
pub type APP1_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to set SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_set(
        &mut self,
    ) -> DMA_INFIFO_FULL_ERR_INT_SET_W<DMA_INT_SET_SPEC> {
        DMA_INFIFO_FULL_ERR_INT_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to set SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_set(
        &mut self,
    ) -> DMA_OUTFIFO_EMPTY_ERR_INT_SET_W<DMA_INT_SET_SPEC> {
        DMA_OUTFIFO_EMPTY_ERR_INT_SET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to set SPI_SLV_EX_QPI_INT interrupt."]
    #[inline(always)]
    pub fn slv_ex_qpi_int_set(&mut self) -> SLV_EX_QPI_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_EX_QPI_INT_SET_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to set SPI_SLV_EN_QPI_INT interrupt."]
    #[inline(always)]
    pub fn slv_en_qpi_int_set(&mut self) -> SLV_EN_QPI_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_EN_QPI_INT_SET_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to set SPI_SLV_CMD7_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_set(&mut self) -> SLV_CMD7_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_CMD7_INT_SET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to set SPI_SLV_CMD8_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_set(&mut self) -> SLV_CMD8_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_CMD8_INT_SET_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to set SPI_SLV_CMD9_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_set(&mut self) -> SLV_CMD9_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_CMD9_INT_SET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to set SPI_SLV_CMDA_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_set(&mut self) -> SLV_CMDA_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_CMDA_INT_SET_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to set SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_set(&mut self) -> SLV_RD_DMA_DONE_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_RD_DMA_DONE_INT_SET_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to set SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_set(&mut self) -> SLV_WR_DMA_DONE_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_WR_DMA_DONE_INT_SET_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to set SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_set(&mut self) -> SLV_RD_BUF_DONE_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_RD_BUF_DONE_INT_SET_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to set SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_set(&mut self) -> SLV_WR_BUF_DONE_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_WR_BUF_DONE_INT_SET_W::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 to set SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn trans_done_int_set(&mut self) -> TRANS_DONE_INT_SET_W<DMA_INT_SET_SPEC> {
        TRANS_DONE_INT_SET_W::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1 to set SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_set(&mut self) -> DMA_SEG_TRANS_DONE_INT_SET_W<DMA_INT_SET_SPEC> {
        DMA_SEG_TRANS_DONE_INT_SET_W::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to set SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn seg_magic_err_int_set(&mut self) -> SEG_MAGIC_ERR_INT_SET_W<DMA_INT_SET_SPEC> {
        SEG_MAGIC_ERR_INT_SET_W::new(self, 14)
    }
    #[doc = "Bit 15 - The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_set(&mut self) -> SLV_BUF_ADDR_ERR_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_BUF_ADDR_ERR_INT_SET_W::new(self, 15)
    }
    #[doc = "Bit 16 - Write 1 to set SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd_err_int_set(&mut self) -> SLV_CMD_ERR_INT_SET_W<DMA_INT_SET_SPEC> {
        SLV_CMD_ERR_INT_SET_W::new(self, 16)
    }
    #[doc = "Bit 17 - Write 1 to set SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_set(
        &mut self,
    ) -> MST_RX_AFIFO_WFULL_ERR_INT_SET_W<DMA_INT_SET_SPEC> {
        MST_RX_AFIFO_WFULL_ERR_INT_SET_W::new(self, 17)
    }
    #[doc = "Bit 18 - Write 1 to set SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_set(
        &mut self,
    ) -> MST_TX_AFIFO_REMPTY_ERR_INT_SET_W<DMA_INT_SET_SPEC> {
        MST_TX_AFIFO_REMPTY_ERR_INT_SET_W::new(self, 18)
    }
    #[doc = "Bit 19 - Write 1 to set SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn app2_int_set(&mut self) -> APP2_INT_SET_W<DMA_INT_SET_SPEC> {
        APP2_INT_SET_W::new(self, 19)
    }
    #[doc = "Bit 20 - Write 1 to set SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn app1_int_set(&mut self) -> APP1_INT_SET_W<DMA_INT_SET_SPEC> {
        APP1_INT_SET_W::new(self, 20)
    }
}
#[doc = "SPI interrupt software set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_SET_SPEC;
impl crate::RegisterSpec for DMA_INT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_int_set::W`](W) writer structure"]
impl crate::Writable for DMA_INT_SET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_SET to value 0"]
impl crate::Resettable for DMA_INT_SET_SPEC {}
