#[doc = "Register `RTCCALICFG2` reader"]
pub type R = crate::R<RTCCALICFG2_SPEC>;
#[doc = "Register `RTCCALICFG2` writer"]
pub type W = crate::W<RTCCALICFG2_SPEC>;
#[doc = "Field `RTC_CALI_TIMEOUT` reader - Represents whether RTC frequency calculation is timeout. \\\\ 0: No timeout \\\\ 1: Timeout \\\\"]
pub type RTC_CALI_TIMEOUT_R = crate::BitReader;
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` reader - Configures the cycles that reset frequency calculation timeout. \\\\ Measurement unit: XTAL_CLK \\\\"]
pub type RTC_CALI_TIMEOUT_RST_CNT_R = crate::FieldReader;
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` writer - Configures the cycles that reset frequency calculation timeout. \\\\ Measurement unit: XTAL_CLK \\\\"]
pub type RTC_CALI_TIMEOUT_RST_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` reader - Configures the threshold value for the RTC frequency calculation timer. If the timer's value exceeds this threshold, a timeout is triggered.\\\\ Measurement unit: XTAL_CLK \\\\"]
pub type RTC_CALI_TIMEOUT_THRES_R = crate::FieldReader<u32>;
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` writer - Configures the threshold value for the RTC frequency calculation timer. If the timer's value exceeds this threshold, a timeout is triggered.\\\\ Measurement unit: XTAL_CLK \\\\"]
pub type RTC_CALI_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - Represents whether RTC frequency calculation is timeout. \\\\ 0: No timeout \\\\ 1: Timeout \\\\"]
    #[inline(always)]
    pub fn rtc_cali_timeout(&self) -> RTC_CALI_TIMEOUT_R {
        RTC_CALI_TIMEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:6 - Configures the cycles that reset frequency calculation timeout. \\\\ Measurement unit: XTAL_CLK \\\\"]
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&self) -> RTC_CALI_TIMEOUT_RST_CNT_R {
        RTC_CALI_TIMEOUT_RST_CNT_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:31 - Configures the threshold value for the RTC frequency calculation timer. If the timer's value exceeds this threshold, a timeout is triggered.\\\\ Measurement unit: XTAL_CLK \\\\"]
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&self) -> RTC_CALI_TIMEOUT_THRES_R {
        RTC_CALI_TIMEOUT_THRES_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG2")
            .field("rtc_cali_timeout", &self.rtc_cali_timeout())
            .field("rtc_cali_timeout_rst_cnt", &self.rtc_cali_timeout_rst_cnt())
            .field("rtc_cali_timeout_thres", &self.rtc_cali_timeout_thres())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:6 - Configures the cycles that reset frequency calculation timeout. \\\\ Measurement unit: XTAL_CLK \\\\"]
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&mut self) -> RTC_CALI_TIMEOUT_RST_CNT_W<RTCCALICFG2_SPEC> {
        RTC_CALI_TIMEOUT_RST_CNT_W::new(self, 3)
    }
    #[doc = "Bits 7:31 - Configures the threshold value for the RTC frequency calculation timer. If the timer's value exceeds this threshold, a timeout is triggered.\\\\ Measurement unit: XTAL_CLK \\\\"]
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&mut self) -> RTC_CALI_TIMEOUT_THRES_W<RTCCALICFG2_SPEC> {
        RTC_CALI_TIMEOUT_THRES_W::new(self, 7)
    }
}
#[doc = "RTC frequency calculation configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccalicfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccalicfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCALICFG2_SPEC;
impl crate::RegisterSpec for RTCCALICFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccalicfg2::R`](R) reader structure"]
impl crate::Readable for RTCCALICFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccalicfg2::W`](W) writer structure"]
impl crate::Writable for RTCCALICFG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCALICFG2 to value 0xffff_ff98"]
impl crate::Resettable for RTCCALICFG2_SPEC {
    const RESET_VALUE: u32 = 0xffff_ff98;
}
