#[doc = "Register `OUTLINK_DSCR` reader"]
pub type R = crate::R<OUTLINK_DSCR_SPEC>;
#[doc = "Field `DMA_OUTLINK_DSCR` reader - The content of current out descriptor pointer."]
pub type DMA_OUTLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current out descriptor pointer."]
    #[inline(always)]
    pub fn dma_outlink_dscr(&self) -> DMA_OUTLINK_DSCR_R {
        DMA_OUTLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTLINK_DSCR")
            .field("dma_outlink_dscr", &self.dma_outlink_dscr())
            .finish()
    }
}
#[doc = "Current SPI DMA TX descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`outlink_dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTLINK_DSCR_SPEC;
impl crate::RegisterSpec for OUTLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outlink_dscr::R`](R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_SPEC {}
#[doc = "`reset()` method sets OUTLINK_DSCR to value 0"]
impl crate::Resettable for OUTLINK_DSCR_SPEC {}
