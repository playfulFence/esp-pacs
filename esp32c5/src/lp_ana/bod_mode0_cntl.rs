#[doc = "Register `BOD_MODE0_CNTL` reader"]
pub type R = crate::R<BOD_MODE0_CNTL_SPEC>;
#[doc = "Register `BOD_MODE0_CNTL` writer"]
pub type W = crate::W<BOD_MODE0_CNTL_SPEC>;
#[doc = "Field `BOD_MODE0_CLOSE_FLASH_ENA` reader - enable suspend spi when brownout interrupt or not 1:enable 0:disable"]
pub type BOD_MODE0_CLOSE_FLASH_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_CLOSE_FLASH_ENA` writer - enable suspend spi when brownout interrupt or not 1:enable 0:disable"]
pub type BOD_MODE0_CLOSE_FLASH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0_PD_RF_ENA` reader - enable power down RF when brownout interrupt or not 1:enable 0:disable"]
pub type BOD_MODE0_PD_RF_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_PD_RF_ENA` writer - enable power down RF when brownout interrupt or not 1:enable 0:disable"]
pub type BOD_MODE0_PD_RF_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0_INTR_WAIT` reader - set the undervoltage hold time for triggering brownout interrupt"]
pub type BOD_MODE0_INTR_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `BOD_MODE0_INTR_WAIT` writer - set the undervoltage hold time for triggering brownout interrupt"]
pub type BOD_MODE0_INTR_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BOD_MODE0_RESET_WAIT` reader - set the undervoltage hold time for triggering brownout reset"]
pub type BOD_MODE0_RESET_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `BOD_MODE0_RESET_WAIT` writer - set the undervoltage hold time for triggering brownout reset"]
pub type BOD_MODE0_RESET_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BOD_MODE0_CNT_CLR` reader - clear brownout count or not 1: clear 0: no operation"]
pub type BOD_MODE0_CNT_CLR_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_CNT_CLR` writer - clear brownout count or not 1: clear 0: no operation"]
pub type BOD_MODE0_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0_INTR_ENA` reader - enable brownout interrupt or not 1: enable 0: disable"]
pub type BOD_MODE0_INTR_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_INTR_ENA` writer - enable brownout interrupt or not 1: enable 0: disable"]
pub type BOD_MODE0_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0_RESET_SEL` reader - select brownout reset level 1: system reset 0: chip reset"]
pub type BOD_MODE0_RESET_SEL_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_RESET_SEL` writer - select brownout reset level 1: system reset 0: chip reset"]
pub type BOD_MODE0_RESET_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0_RESET_ENA` reader - enable brownout reset or not 1: enable 0: disable"]
pub type BOD_MODE0_RESET_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_RESET_ENA` writer - enable brownout reset or not 1: enable 0: disable"]
pub type BOD_MODE0_RESET_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - enable suspend spi when brownout interrupt or not 1:enable 0:disable"]
    #[inline(always)]
    pub fn bod_mode0_close_flash_ena(&self) -> BOD_MODE0_CLOSE_FLASH_ENA_R {
        BOD_MODE0_CLOSE_FLASH_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable power down RF when brownout interrupt or not 1:enable 0:disable"]
    #[inline(always)]
    pub fn bod_mode0_pd_rf_ena(&self) -> BOD_MODE0_PD_RF_ENA_R {
        BOD_MODE0_PD_RF_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17 - set the undervoltage hold time for triggering brownout interrupt"]
    #[inline(always)]
    pub fn bod_mode0_intr_wait(&self) -> BOD_MODE0_INTR_WAIT_R {
        BOD_MODE0_INTR_WAIT_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 18:27 - set the undervoltage hold time for triggering brownout reset"]
    #[inline(always)]
    pub fn bod_mode0_reset_wait(&self) -> BOD_MODE0_RESET_WAIT_R {
        BOD_MODE0_RESET_WAIT_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - clear brownout count or not 1: clear 0: no operation"]
    #[inline(always)]
    pub fn bod_mode0_cnt_clr(&self) -> BOD_MODE0_CNT_CLR_R {
        BOD_MODE0_CNT_CLR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable brownout interrupt or not 1: enable 0: disable"]
    #[inline(always)]
    pub fn bod_mode0_intr_ena(&self) -> BOD_MODE0_INTR_ENA_R {
        BOD_MODE0_INTR_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - select brownout reset level 1: system reset 0: chip reset"]
    #[inline(always)]
    pub fn bod_mode0_reset_sel(&self) -> BOD_MODE0_RESET_SEL_R {
        BOD_MODE0_RESET_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable brownout reset or not 1: enable 0: disable"]
    #[inline(always)]
    pub fn bod_mode0_reset_ena(&self) -> BOD_MODE0_RESET_ENA_R {
        BOD_MODE0_RESET_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOD_MODE0_CNTL")
            .field(
                "bod_mode0_close_flash_ena",
                &self.bod_mode0_close_flash_ena(),
            )
            .field("bod_mode0_pd_rf_ena", &self.bod_mode0_pd_rf_ena())
            .field("bod_mode0_intr_wait", &self.bod_mode0_intr_wait())
            .field("bod_mode0_reset_wait", &self.bod_mode0_reset_wait())
            .field("bod_mode0_cnt_clr", &self.bod_mode0_cnt_clr())
            .field("bod_mode0_intr_ena", &self.bod_mode0_intr_ena())
            .field("bod_mode0_reset_sel", &self.bod_mode0_reset_sel())
            .field("bod_mode0_reset_ena", &self.bod_mode0_reset_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - enable suspend spi when brownout interrupt or not 1:enable 0:disable"]
    #[inline(always)]
    pub fn bod_mode0_close_flash_ena(
        &mut self,
    ) -> BOD_MODE0_CLOSE_FLASH_ENA_W<BOD_MODE0_CNTL_SPEC> {
        BOD_MODE0_CLOSE_FLASH_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - enable power down RF when brownout interrupt or not 1:enable 0:disable"]
    #[inline(always)]
    pub fn bod_mode0_pd_rf_ena(&mut self) -> BOD_MODE0_PD_RF_ENA_W<BOD_MODE0_CNTL_SPEC> {
        BOD_MODE0_PD_RF_ENA_W::new(self, 7)
    }
    #[doc = "Bits 8:17 - set the undervoltage hold time for triggering brownout interrupt"]
    #[inline(always)]
    pub fn bod_mode0_intr_wait(&mut self) -> BOD_MODE0_INTR_WAIT_W<BOD_MODE0_CNTL_SPEC> {
        BOD_MODE0_INTR_WAIT_W::new(self, 8)
    }
    #[doc = "Bits 18:27 - set the undervoltage hold time for triggering brownout reset"]
    #[inline(always)]
    pub fn bod_mode0_reset_wait(&mut self) -> BOD_MODE0_RESET_WAIT_W<BOD_MODE0_CNTL_SPEC> {
        BOD_MODE0_RESET_WAIT_W::new(self, 18)
    }
    #[doc = "Bit 28 - clear brownout count or not 1: clear 0: no operation"]
    #[inline(always)]
    pub fn bod_mode0_cnt_clr(&mut self) -> BOD_MODE0_CNT_CLR_W<BOD_MODE0_CNTL_SPEC> {
        BOD_MODE0_CNT_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - enable brownout interrupt or not 1: enable 0: disable"]
    #[inline(always)]
    pub fn bod_mode0_intr_ena(&mut self) -> BOD_MODE0_INTR_ENA_W<BOD_MODE0_CNTL_SPEC> {
        BOD_MODE0_INTR_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - select brownout reset level 1: system reset 0: chip reset"]
    #[inline(always)]
    pub fn bod_mode0_reset_sel(&mut self) -> BOD_MODE0_RESET_SEL_W<BOD_MODE0_CNTL_SPEC> {
        BOD_MODE0_RESET_SEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable brownout reset or not 1: enable 0: disable"]
    #[inline(always)]
    pub fn bod_mode0_reset_ena(&mut self) -> BOD_MODE0_RESET_ENA_W<BOD_MODE0_CNTL_SPEC> {
        BOD_MODE0_RESET_ENA_W::new(self, 31)
    }
}
#[doc = "Configure brownout mode0\n\nYou can [`read`](crate::Reg::read) this register and get [`bod_mode0_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod_mode0_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOD_MODE0_CNTL_SPEC;
impl crate::RegisterSpec for BOD_MODE0_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod_mode0_cntl::R`](R) reader structure"]
impl crate::Readable for BOD_MODE0_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bod_mode0_cntl::W`](W) writer structure"]
impl crate::Writable for BOD_MODE0_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOD_MODE0_CNTL to value 0x0ffc_0100"]
impl crate::Resettable for BOD_MODE0_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x0ffc_0100;
}
