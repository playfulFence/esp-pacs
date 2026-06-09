#[doc = "Register `RD_SYS_PART1_DATA6` reader"]
pub type R = crate::R<RD_SYS_PART1_DATA6_SPEC>;
#[doc = "Field `ADC1_CAL_VOL_ATTEN0_1` reader - ADC1 calibration voltage at atten0 /"]
pub type ADC1_CAL_VOL_ATTEN0_1_R = crate::FieldReader;
#[doc = "Field `ADC1_CAL_VOL_ATTEN1` reader - ADC1 calibration voltage at atten1 /"]
pub type ADC1_CAL_VOL_ATTEN1_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_CAL_VOL_ATTEN2` reader - ADC1 calibration voltage at atten2 /"]
pub type ADC1_CAL_VOL_ATTEN2_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_CAL_VOL_ATTEN3` reader - ADC1 calibration voltage at atten3 /"]
pub type ADC1_CAL_VOL_ATTEN3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - ADC1 calibration voltage at atten0 /"]
    #[inline(always)]
    pub fn adc1_cal_vol_atten0_1(&self) -> ADC1_CAL_VOL_ATTEN0_1_R {
        ADC1_CAL_VOL_ATTEN0_1_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:15 - ADC1 calibration voltage at atten1 /"]
    #[inline(always)]
    pub fn adc1_cal_vol_atten1(&self) -> ADC1_CAL_VOL_ATTEN1_R {
        ADC1_CAL_VOL_ATTEN1_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - ADC1 calibration voltage at atten2 /"]
    #[inline(always)]
    pub fn adc1_cal_vol_atten2(&self) -> ADC1_CAL_VOL_ATTEN2_R {
        ADC1_CAL_VOL_ATTEN2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - ADC1 calibration voltage at atten3 /"]
    #[inline(always)]
    pub fn adc1_cal_vol_atten3(&self) -> ADC1_CAL_VOL_ATTEN3_R {
        ADC1_CAL_VOL_ATTEN3_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA6")
            .field("adc1_cal_vol_atten0_1", &self.adc1_cal_vol_atten0_1())
            .field("adc1_cal_vol_atten1", &self.adc1_cal_vol_atten1())
            .field("adc1_cal_vol_atten2", &self.adc1_cal_vol_atten2())
            .field("adc1_cal_vol_atten3", &self.adc1_cal_vol_atten3())
            .finish()
    }
}
#[doc = "Register 6 of BLOCK2 (system). /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART1_DATA6_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data6::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA6_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA6 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA6_SPEC {}
