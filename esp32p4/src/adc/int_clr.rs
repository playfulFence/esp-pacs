#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `THRES1_LOW` writer - need_des"]
pub type THRES1_LOW_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `THRES0_LOW` writer - need_des"]
pub type THRES0_LOW_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `THRES1_HIGH` writer - need_des"]
pub type THRES1_HIGH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `THRES0_HIGH` writer - need_des"]
pub type THRES0_HIGH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ADC2_DONE` writer - need_des"]
pub type ADC2_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ADC1_DONE` writer - need_des"]
pub type ADC1_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn thres1_low(&mut self) -> THRES1_LOW_W<INT_CLR_SPEC> {
        THRES1_LOW_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres0_low(&mut self) -> THRES0_LOW_W<INT_CLR_SPEC> {
        THRES0_LOW_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres1_high(&mut self) -> THRES1_HIGH_W<INT_CLR_SPEC> {
        THRES1_HIGH_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres0_high(&mut self) -> THRES0_HIGH_W<INT_CLR_SPEC> {
        THRES0_HIGH_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn adc2_done(&mut self) -> ADC2_DONE_W<INT_CLR_SPEC> {
        ADC2_DONE_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn adc1_done(&mut self) -> ADC1_DONE_W<INT_CLR_SPEC> {
        ADC1_DONE_W::new(self, 31)
    }
}
#[doc = "Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfc00_0000;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
