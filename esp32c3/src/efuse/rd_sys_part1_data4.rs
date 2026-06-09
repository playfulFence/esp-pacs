#[doc = "Register `RD_SYS_PART1_DATA4` reader"]
pub type R = crate::R<RD_SYS_PART1_DATA4_SPEC>;
#[doc = "Field `BLK_VERSION_MAJOR` reader - BLK_VERSION_MAJOR of BLOCK2 /"]
pub type BLK_VERSION_MAJOR_R = crate::FieldReader;
#[doc = "Field `RESERVED_2_130` reader - reserved /"]
pub type RESERVED_2_130_R = crate::BitReader;
#[doc = "Field `TEMP_CALIB` reader - Temperature calibration data /"]
pub type TEMP_CALIB_R = crate::FieldReader<u16>;
#[doc = "Field `OCODE` reader - ADC OCode /"]
pub type OCODE_R = crate::FieldReader;
#[doc = "Field `ADC1_INIT_CODE_ATTEN0` reader - ADC1 init code at atten0 /"]
pub type ADC1_INIT_CODE_ATTEN0_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_INIT_CODE_ATTEN1` reader - ADC1 init code at atten1 /"]
pub type ADC1_INIT_CODE_ATTEN1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - BLK_VERSION_MAJOR of BLOCK2 /"]
    #[inline(always)]
    pub fn blk_version_major(&self) -> BLK_VERSION_MAJOR_R {
        BLK_VERSION_MAJOR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reserved /"]
    #[inline(always)]
    pub fn reserved_2_130(&self) -> RESERVED_2_130_R {
        RESERVED_2_130_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Temperature calibration data /"]
    #[inline(always)]
    pub fn temp_calib(&self) -> TEMP_CALIB_R {
        TEMP_CALIB_R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:19 - ADC OCode /"]
    #[inline(always)]
    pub fn ocode(&self) -> OCODE_R {
        OCODE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:29 - ADC1 init code at atten0 /"]
    #[inline(always)]
    pub fn adc1_init_code_atten0(&self) -> ADC1_INIT_CODE_ATTEN0_R {
        ADC1_INIT_CODE_ATTEN0_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - ADC1 init code at atten1 /"]
    #[inline(always)]
    pub fn adc1_init_code_atten1(&self) -> ADC1_INIT_CODE_ATTEN1_R {
        ADC1_INIT_CODE_ATTEN1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA4")
            .field("blk_version_major", &self.blk_version_major())
            .field("reserved_2_130", &self.reserved_2_130())
            .field("temp_calib", &self.temp_calib())
            .field("ocode", &self.ocode())
            .field("adc1_init_code_atten0", &self.adc1_init_code_atten0())
            .field("adc1_init_code_atten1", &self.adc1_init_code_atten1())
            .finish()
    }
}
#[doc = "Register 4 of BLOCK2 (system). /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART1_DATA4_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data4::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA4 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA4_SPEC {}
