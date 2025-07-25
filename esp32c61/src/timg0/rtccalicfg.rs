#[doc = "Register `RTCCALICFG` reader"]
pub type R = crate::R<RTCCALICFG_SPEC>;
#[doc = "Register `RTCCALICFG` writer"]
pub type W = crate::W<RTCCALICFG_SPEC>;
#[doc = "Field `RTC_CALI_START_CYCLING` reader - Configures the frequency calculation mode. \\\\ 0: one-shot frequency calculation \\\\ 1: periodic frequency calculation \\\\"]
pub type RTC_CALI_START_CYCLING_R = crate::BitReader;
#[doc = "Field `RTC_CALI_START_CYCLING` writer - Configures the frequency calculation mode. \\\\ 0: one-shot frequency calculation \\\\ 1: periodic frequency calculation \\\\"]
pub type RTC_CALI_START_CYCLING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_CALI_CLK_SEL` reader - Configures to select the clock to be calibrated\\\\ 0: RTC_SLOW_CLK\\\\ 1: RC_FAST_DIV_CLK\\\\ 2: XTAL32K_CLK\\\\"]
pub type RTC_CALI_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `RTC_CALI_CLK_SEL` writer - Configures to select the clock to be calibrated\\\\ 0: RTC_SLOW_CLK\\\\ 1: RC_FAST_DIV_CLK\\\\ 2: XTAL32K_CLK\\\\"]
pub type RTC_CALI_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTC_CALI_RDY` reader - Represents whether one-shot frequency calculation is done.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type RTC_CALI_RDY_R = crate::BitReader;
#[doc = "Field `RTC_CALI_MAX` reader - Configures the time to calculate RTC slow clock's frequency. \\\\ Measurement unit: XTAL_CLK \\\\"]
pub type RTC_CALI_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_CALI_MAX` writer - Configures the time to calculate RTC slow clock's frequency. \\\\ Measurement unit: XTAL_CLK \\\\"]
pub type RTC_CALI_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `RTC_CALI_START` reader - Configures whether to enable one-shot frequency calculation. \\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type RTC_CALI_START_R = crate::BitReader;
#[doc = "Field `RTC_CALI_START` writer - Configures whether to enable one-shot frequency calculation. \\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type RTC_CALI_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Configures the frequency calculation mode. \\\\ 0: one-shot frequency calculation \\\\ 1: periodic frequency calculation \\\\"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&self) -> RTC_CALI_START_CYCLING_R {
        RTC_CALI_START_CYCLING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Configures to select the clock to be calibrated\\\\ 0: RTC_SLOW_CLK\\\\ 1: RC_FAST_DIV_CLK\\\\ 2: XTAL32K_CLK\\\\"]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&self) -> RTC_CALI_CLK_SEL_R {
        RTC_CALI_CLK_SEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Represents whether one-shot frequency calculation is done.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn rtc_cali_rdy(&self) -> RTC_CALI_RDY_R {
        RTC_CALI_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Configures the time to calculate RTC slow clock's frequency. \\\\ Measurement unit: XTAL_CLK \\\\"]
    #[inline(always)]
    pub fn rtc_cali_max(&self) -> RTC_CALI_MAX_R {
        RTC_CALI_MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Configures whether to enable one-shot frequency calculation. \\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn rtc_cali_start(&self) -> RTC_CALI_START_R {
        RTC_CALI_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG")
            .field("rtc_cali_start_cycling", &self.rtc_cali_start_cycling())
            .field("rtc_cali_clk_sel", &self.rtc_cali_clk_sel())
            .field("rtc_cali_rdy", &self.rtc_cali_rdy())
            .field("rtc_cali_max", &self.rtc_cali_max())
            .field("rtc_cali_start", &self.rtc_cali_start())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Configures the frequency calculation mode. \\\\ 0: one-shot frequency calculation \\\\ 1: periodic frequency calculation \\\\"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&mut self) -> RTC_CALI_START_CYCLING_W<RTCCALICFG_SPEC> {
        RTC_CALI_START_CYCLING_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - Configures to select the clock to be calibrated\\\\ 0: RTC_SLOW_CLK\\\\ 1: RC_FAST_DIV_CLK\\\\ 2: XTAL32K_CLK\\\\"]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&mut self) -> RTC_CALI_CLK_SEL_W<RTCCALICFG_SPEC> {
        RTC_CALI_CLK_SEL_W::new(self, 13)
    }
    #[doc = "Bits 16:30 - Configures the time to calculate RTC slow clock's frequency. \\\\ Measurement unit: XTAL_CLK \\\\"]
    #[inline(always)]
    pub fn rtc_cali_max(&mut self) -> RTC_CALI_MAX_W<RTCCALICFG_SPEC> {
        RTC_CALI_MAX_W::new(self, 16)
    }
    #[doc = "Bit 31 - Configures whether to enable one-shot frequency calculation. \\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn rtc_cali_start(&mut self) -> RTC_CALI_START_W<RTCCALICFG_SPEC> {
        RTC_CALI_START_W::new(self, 31)
    }
}
#[doc = "RTC frequency calculation configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccalicfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccalicfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCALICFG_SPEC;
impl crate::RegisterSpec for RTCCALICFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccalicfg::R`](R) reader structure"]
impl crate::Readable for RTCCALICFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccalicfg::W`](W) writer structure"]
impl crate::Writable for RTCCALICFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCALICFG to value 0x0001_1000"]
impl crate::Resettable for RTCCALICFG_SPEC {
    const RESET_VALUE: u32 = 0x0001_1000;
}
