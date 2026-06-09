#[doc = "Register `SAR2DATA_STATUS` reader"]
pub type R = crate::R<SAR2DATA_STATUS_SPEC>;
#[doc = "Field `ADC2_DATA` reader - "]
pub type ADC2_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn adc2_data(&self) -> ADC2_DATA_R {
        ADC2_DATA_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2DATA_STATUS")
            .field("adc2_data", &self.adc2_data())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2data_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR2DATA_STATUS_SPEC;
impl crate::RegisterSpec for SAR2DATA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2data_status::R`](R) reader structure"]
impl crate::Readable for SAR2DATA_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR2DATA_STATUS to value 0"]
impl crate::Resettable for SAR2DATA_STATUS_SPEC {}
