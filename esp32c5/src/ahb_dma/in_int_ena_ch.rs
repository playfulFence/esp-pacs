#[doc = "Register `IN_INT_ENA_CH%s` reader"]
pub type R = crate::R<IN_INT_ENA_CH_SPEC>;
#[doc = "Register `IN_INT_ENA_CH%s` writer"]
pub type W = crate::W<IN_INT_ENA_CH_SPEC>;
#[doc = "Field `IN_DONE` reader - Write 1 to enable AHB_DMA_IN_DONE_CH%s_INT."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_DONE` writer - Write 1 to enable AHB_DMA_IN_DONE_CH%s_INT."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH%s_INT."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH%s_INT."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` reader - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH%s_INT."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` writer - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH%s_INT."]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` reader - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` writer - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` reader - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` writer - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF` reader - Write 1 to enable AHB_DMA_INFIFO_OVF_CH%s_INT."]
pub type INFIFO_OVF_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF` writer - Write 1 to enable AHB_DMA_INFIFO_OVF_CH%s_INT."]
pub type INFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF` reader - Write 1 to enable AHB_DMA_INFIFO_UDF_CH%s_INT."]
pub type INFIFO_UDF_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF` writer - Write 1 to enable AHB_DMA_INFIFO_UDF_CH%s_INT."]
pub type INFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_IN_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_INFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_ovf(&self) -> INFIFO_OVF_R {
        INFIFO_OVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_INFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_udf(&self) -> INFIFO_UDF_R {
        INFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_INT_ENA_CH")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("infifo_ovf", &self.infifo_ovf())
            .field("infifo_udf", &self.infifo_udf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_IN_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn in_done(&mut self) -> IN_DONE_W<IN_INT_ENA_CH_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<IN_INT_ENA_CH_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<IN_INT_ENA_CH_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<IN_INT_ENA_CH_SPEC> {
        IN_DSCR_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<IN_INT_ENA_CH_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_INFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_ovf(&mut self) -> INFIFO_OVF_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_OVF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_INFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_udf(&mut self) -> INFIFO_UDF_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_UDF_W::new(self, 6)
    }
}
#[doc = "Interrupt enable bits of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_ena_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_ena_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_ENA_CH_SPEC;
impl crate::RegisterSpec for IN_INT_ENA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_int_ena_ch::R`](R) reader structure"]
impl crate::Readable for IN_INT_ENA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_int_ena_ch::W`](W) writer structure"]
impl crate::Writable for IN_INT_ENA_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_INT_ENA_CH%s to value 0"]
impl crate::Resettable for IN_INT_ENA_CH_SPEC {}
