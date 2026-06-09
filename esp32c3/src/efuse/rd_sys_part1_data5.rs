#[doc = "Register `RD_SYS_PART1_DATA5` reader"]
pub type R = crate::R<RD_SYS_PART1_DATA5_SPEC>;
#[doc = "Field `ADC1_INIT_CODE_ATTEN1_1` reader - ADC1 init code at atten1 /"]
pub type ADC1_INIT_CODE_ATTEN1_1_R = crate::FieldReader;
#[doc = "Field `ADC1_INIT_CODE_ATTEN2` reader - ADC1 init code at atten2 /"]
pub type ADC1_INIT_CODE_ATTEN2_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_INIT_CODE_ATTEN3` reader - ADC1 init code at atten3 /"]
pub type ADC1_INIT_CODE_ATTEN3_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_CAL_VOL_ATTEN0` reader - ADC1 calibration voltage at atten0 /"]
pub type ADC1_CAL_VOL_ATTEN0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ADC1 init code at atten1 /"]
    #[inline(always)]
    pub fn adc1_init_code_atten1_1(&self) -> ADC1_INIT_CODE_ATTEN1_1_R {
        ADC1_INIT_CODE_ATTEN1_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - ADC1 init code at atten2 /"]
    #[inline(always)]
    pub fn adc1_init_code_atten2(&self) -> ADC1_INIT_CODE_ATTEN2_R {
        ADC1_INIT_CODE_ATTEN2_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 18:27 - ADC1 init code at atten3 /"]
    #[inline(always)]
    pub fn adc1_init_code_atten3(&self) -> ADC1_INIT_CODE_ATTEN3_R {
        ADC1_INIT_CODE_ATTEN3_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - ADC1 calibration voltage at atten0 /"]
    #[inline(always)]
    pub fn adc1_cal_vol_atten0(&self) -> ADC1_CAL_VOL_ATTEN0_R {
        ADC1_CAL_VOL_ATTEN0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA5")
            .field("adc1_init_code_atten1_1", &self.adc1_init_code_atten1_1())
            .field("adc1_init_code_atten2", &self.adc1_init_code_atten2())
            .field("adc1_init_code_atten3", &self.adc1_init_code_atten3())
            .field("adc1_cal_vol_atten0", &self.adc1_cal_vol_atten0())
            .finish()
    }
}
#[doc = "Register 5 of BLOCK2 (system). /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART1_DATA5_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data5::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA5_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA5 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA5_SPEC {}
