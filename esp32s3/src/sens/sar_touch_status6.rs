#[doc = "Register `SAR_TOUCH_STATUS6` reader"]
pub type R = crate::R<SAR_TOUCH_STATUS6_SPEC>;
#[doc = "Field `SAR_TOUCH_PAD6_DATA` reader - touch data debounce of touch pad 6"]
pub type SAR_TOUCH_PAD6_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_TOUCH_PAD6_DEBOUNCE` reader - touch current debounce of touch pad 6"]
pub type SAR_TOUCH_PAD6_DEBOUNCE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - touch data debounce of touch pad 6"]
    #[inline(always)]
    pub fn sar_touch_pad6_data(&self) -> SAR_TOUCH_PAD6_DATA_R {
        SAR_TOUCH_PAD6_DATA_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 29:31 - touch current debounce of touch pad 6"]
    #[inline(always)]
    pub fn sar_touch_pad6_debounce(&self) -> SAR_TOUCH_PAD6_DEBOUNCE_R {
        SAR_TOUCH_PAD6_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS6")
            .field("sar_touch_pad6_data", &self.sar_touch_pad6_data())
            .field("sar_touch_pad6_debounce", &self.sar_touch_pad6_debounce())
            .finish()
    }
}
#[doc = "touch channel status of touch pad 6\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_STATUS6_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_status6::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS6_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS6 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS6_SPEC {}
