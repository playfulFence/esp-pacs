#[doc = "Register `LP_INT_ST` reader"]
pub type R = crate::R<LP_INT_ST_SPEC>;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_ST` reader - Status of RTC main timer overflow interrupt ."]
pub type MAIN_TIMER_OVERFLOW_LP_INT_ST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_LP_INT_ST` reader - Status of RTC main timer interrupt ."]
pub type MAIN_TIMER_LP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 30 - Status of RTC main timer overflow interrupt ."]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_st(&self) -> MAIN_TIMER_OVERFLOW_LP_INT_ST_R {
        MAIN_TIMER_OVERFLOW_LP_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Status of RTC main timer interrupt ."]
    #[inline(always)]
    pub fn main_timer_lp_int_st(&self) -> MAIN_TIMER_LP_INT_ST_R {
        MAIN_TIMER_LP_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ST")
            .field(
                "main_timer_overflow_lp_int_st",
                &self.main_timer_overflow_lp_int_st(),
            )
            .field("main_timer_lp_int_st", &self.main_timer_lp_int_st())
            .finish()
    }
}
#[doc = "RTC timer interrupt status register(For ULP)\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_st::R`](R) reader structure"]
impl crate::Readable for LP_INT_ST_SPEC {}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LP_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
