#[doc = "Register `DMA_OUT_CH2_INT_MAP` reader"]
pub type R = crate::R<DMA_OUT_CH2_INT_MAP_SPEC>;
#[doc = "Register `DMA_OUT_CH2_INT_MAP` writer"]
pub type W = crate::W<DMA_OUT_CH2_INT_MAP_SPEC>;
#[doc = "Field `DMA_OUT_CH2_INT_MAP` reader - this register used to map dma_out_ch2 interrupt to one of core0's external interrupt"]
pub type DMA_OUT_CH2_INT_MAP_R = crate::FieldReader;
#[doc = "Field `DMA_OUT_CH2_INT_MAP` writer - this register used to map dma_out_ch2 interrupt to one of core0's external interrupt"]
pub type DMA_OUT_CH2_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map dma_out_ch2 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn dma_out_ch2_int_map(&self) -> DMA_OUT_CH2_INT_MAP_R {
        DMA_OUT_CH2_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_CH2_INT_MAP")
            .field("dma_out_ch2_int_map", &self.dma_out_ch2_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map dma_out_ch2 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn dma_out_ch2_int_map(&mut self) -> DMA_OUT_CH2_INT_MAP_W<DMA_OUT_CH2_INT_MAP_SPEC> {
        DMA_OUT_CH2_INT_MAP_W::new(self, 0)
    }
}
#[doc = "dma_out_ch2 interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_ch2_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_out_ch2_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUT_CH2_INT_MAP_SPEC;
impl crate::RegisterSpec for DMA_OUT_CH2_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_out_ch2_int_map::R`](R) reader structure"]
impl crate::Readable for DMA_OUT_CH2_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_out_ch2_int_map::W`](W) writer structure"]
impl crate::Writable for DMA_OUT_CH2_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_OUT_CH2_INT_MAP to value 0x10"]
impl crate::Resettable for DMA_OUT_CH2_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
