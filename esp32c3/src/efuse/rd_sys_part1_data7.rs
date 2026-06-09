#[doc = "Register `RD_SYS_PART1_DATA7` reader"]
pub type R = crate::R<RD_SYS_PART1_DATA7_SPEC>;
#[doc = "Field `ADC1_CAL_VOL_ATTEN3_1` reader - ADC1 calibration voltage at atten3 /"]
pub type ADC1_CAL_VOL_ATTEN3_1_R = crate::FieldReader;
#[doc = "Field `RESERVED_2_228` reader - reserved /"]
pub type RESERVED_2_228_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - ADC1 calibration voltage at atten3 /"]
    #[inline(always)]
    pub fn adc1_cal_vol_atten3_1(&self) -> ADC1_CAL_VOL_ATTEN3_1_R {
        ADC1_CAL_VOL_ATTEN3_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - reserved /"]
    #[inline(always)]
    pub fn reserved_2_228(&self) -> RESERVED_2_228_R {
        RESERVED_2_228_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA7")
            .field("adc1_cal_vol_atten3_1", &self.adc1_cal_vol_atten3_1())
            .field("reserved_2_228", &self.reserved_2_228())
            .finish()
    }
}
#[doc = "Register 7 of BLOCK2 (system). /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART1_DATA7_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data7::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA7_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA7 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA7_SPEC {}
