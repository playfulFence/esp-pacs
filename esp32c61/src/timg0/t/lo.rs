#[doc = "Register `LO` reader"]
pub type R = crate::R<LO_SPEC>;
#[doc = "Field `T0_LO` reader - Represents the low 32 bits of the time-base counter of timer 0. Valid only after writing to TIMG_T0UPDATE_REG. \\\\ Measurement unit: T0_clk \\\\"]
pub type T0_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the low 32 bits of the time-base counter of timer 0. Valid only after writing to TIMG_T0UPDATE_REG. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t0_lo(&self) -> T0_LO_R {
        T0_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LO").field("t0_lo", &self.t0_lo()).finish()
    }
}
#[doc = "Timer 0 current value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LO_SPEC;
impl crate::RegisterSpec for LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LO_SPEC {}
#[doc = "`reset()` method sets LO to value 0"]
impl crate::Resettable for LO_SPEC {}
