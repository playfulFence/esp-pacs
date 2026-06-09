#[doc = "Register `DMA_CONF` reader"]
pub type R = crate::R<DMA_CONF_SPEC>;
#[doc = "Register `DMA_CONF` writer"]
pub type W = crate::W<DMA_CONF_SPEC>;
#[doc = "Field `EOF_NUM` reader - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
pub type EOF_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `EOF_NUM` writer - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
pub type EOF_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESET_FSM` reader - reset_apb_adc_state"]
pub type RESET_FSM_R = crate::BitReader;
#[doc = "Field `RESET_FSM` writer - reset_apb_adc_state"]
pub type RESET_FSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS` reader - enable apb_adc use spi_dma"]
pub type TRANS_R = crate::BitReader;
#[doc = "Field `TRANS` writer - enable apb_adc use spi_dma"]
pub type TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
    #[inline(always)]
    pub fn eof_num(&self) -> EOF_NUM_R {
        EOF_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - reset_apb_adc_state"]
    #[inline(always)]
    pub fn reset_fsm(&self) -> RESET_FSM_R {
        RESET_FSM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable apb_adc use spi_dma"]
    #[inline(always)]
    pub fn trans(&self) -> TRANS_R {
        TRANS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CONF")
            .field("trans", &self.trans())
            .field("reset_fsm", &self.reset_fsm())
            .field("eof_num", &self.eof_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
    #[inline(always)]
    pub fn eof_num(&mut self) -> EOF_NUM_W<'_, DMA_CONF_SPEC> {
        EOF_NUM_W::new(self, 0)
    }
    #[doc = "Bit 30 - reset_apb_adc_state"]
    #[inline(always)]
    pub fn reset_fsm(&mut self) -> RESET_FSM_W<'_, DMA_CONF_SPEC> {
        RESET_FSM_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable apb_adc use spi_dma"]
    #[inline(always)]
    pub fn trans(&mut self) -> TRANS_W<'_, DMA_CONF_SPEC> {
        TRANS_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_conf::R`](R) reader structure"]
impl crate::Readable for DMA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_conf::W`](W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CONF to value 0"]
impl crate::Resettable for DMA_CONF_SPEC {}
