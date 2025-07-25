#[doc = "Register `SAR_TOUCH_OUT3` reader"]
pub type R = crate::R<SAR_TOUCH_OUT3_SPEC>;
#[doc = "Field `TOUCH_MEAS_OUT5` reader - the counter for touch pad 5"]
pub type TOUCH_MEAS_OUT5_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_OUT4` reader - the counter for touch pad 4"]
pub type TOUCH_MEAS_OUT4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - the counter for touch pad 5"]
    #[inline(always)]
    pub fn touch_meas_out5(&self) -> TOUCH_MEAS_OUT5_R {
        TOUCH_MEAS_OUT5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the counter for touch pad 4"]
    #[inline(always)]
    pub fn touch_meas_out4(&self) -> TOUCH_MEAS_OUT4_R {
        TOUCH_MEAS_OUT4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_OUT3")
            .field("touch_meas_out5", &self.touch_meas_out5())
            .field("touch_meas_out4", &self.touch_meas_out4())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_out3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_OUT3_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_out3::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT3_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_OUT3 to value 0"]
impl crate::Resettable for SAR_TOUCH_OUT3_SPEC {}
