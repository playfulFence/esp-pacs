#[doc = "Register `SAR_TOUCH_STATUS8` reader"]
pub type R = crate::R<SAR_TOUCH_STATUS8_SPEC>;
#[doc = "Field `TOUCH_PAD8_DATA` reader - The data of touch pad 8, depending on the setting of SENS_TOUCH_DATA_SEL."]
pub type TOUCH_PAD8_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_PAD8_DEBOUNCE` reader - Touch pad 8 debounce value."]
pub type TOUCH_PAD8_DEBOUNCE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - The data of touch pad 8, depending on the setting of SENS_TOUCH_DATA_SEL."]
    #[inline(always)]
    pub fn touch_pad8_data(&self) -> TOUCH_PAD8_DATA_R {
        TOUCH_PAD8_DATA_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 29:31 - Touch pad 8 debounce value."]
    #[inline(always)]
    pub fn touch_pad8_debounce(&self) -> TOUCH_PAD8_DEBOUNCE_R {
        TOUCH_PAD8_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS8")
            .field("touch_pad8_data", &self.touch_pad8_data())
            .field("touch_pad8_debounce", &self.touch_pad8_debounce())
            .finish()
    }
}
#[doc = "Touch pad 8 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_STATUS8_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_status8::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS8_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS8 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS8_SPEC {}
