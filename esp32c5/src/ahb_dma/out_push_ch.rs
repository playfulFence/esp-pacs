#[doc = "Register `OUT_PUSH_CH%s` reader"]
pub type R = crate::R<OUT_PUSH_CH_SPEC>;
#[doc = "Register `OUT_PUSH_CH%s` writer"]
pub type W = crate::W<OUT_PUSH_CH_SPEC>;
#[doc = "Field `OUTFIFO_WDATA` reader - Configures the data that need to be pushed into AHB_DMA FIFO."]
pub type OUTFIFO_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA` writer - Configures the data that need to be pushed into AHB_DMA FIFO."]
pub type OUTFIFO_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `OUTFIFO_PUSH` writer - Configures whether or not to push data into AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Push\\\\"]
pub type OUTFIFO_PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Configures the data that need to be pushed into AHB_DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PUSH_CH")
            .field("outfifo_wdata", &self.outfifo_wdata())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Configures the data that need to be pushed into AHB_DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W<OUT_PUSH_CH_SPEC> {
        OUTFIFO_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 9 - Configures whether or not to push data into AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Push\\\\"]
    #[inline(always)]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W<OUT_PUSH_CH_SPEC> {
        OUTFIFO_PUSH_W::new(self, 9)
    }
}
#[doc = "Push control register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PUSH_CH_SPEC;
impl crate::RegisterSpec for OUT_PUSH_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_push_ch::R`](R) reader structure"]
impl crate::Readable for OUT_PUSH_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_push_ch::W`](W) writer structure"]
impl crate::Writable for OUT_PUSH_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_PUSH_CH%s to value 0"]
impl crate::Resettable for OUT_PUSH_CH_SPEC {}
