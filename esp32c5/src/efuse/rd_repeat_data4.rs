#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub type R = crate::R<RD_REPEAT_DATA4_SPEC>;
#[doc = "Field `HUK_GEN_STATE` reader - Set the bits to control validation of HUK generate mode.\\\\ Odd of 1 is invalid.\\\\ Even of 1 is valid.\\\\"]
pub type HUK_GEN_STATE_R = crate::FieldReader<u16>;
#[doc = "Field `XTAL_48M_SEL` reader - Represents whether XTAL frequency is 48MHz or not. If not, 40MHz XTAL will be used. If this field contains Odd number bit '1': Enable 48MHz XTAL\\\\ Even number bit '1': Enable 40MHz XTAL."]
pub type XTAL_48M_SEL_R = crate::FieldReader;
#[doc = "Field `XTAL_48M_SEL_MODE` reader - Specify the XTAL frequency selection is decided by eFuse or strapping-PAD-state. 1: eFuse\\\\ 0: strapping-PAD-state."]
pub type XTAL_48M_SEL_MODE_R = crate::BitReader;
#[doc = "Field `ECDSA_DISABLE_P192` reader - Represents whether to disable P192 curve in ECDSA.\\\\ 1: Disabled.\\\\ 0: Not disable."]
pub type ECDSA_DISABLE_P192_R = crate::BitReader;
#[doc = "Field `ECC_FORCE_CONST_TIME` reader - Represents whether to force ecc to use const-time calculation mode. \\\\ 1: Enable. \\\\ 0: Disable."]
pub type ECC_FORCE_CONST_TIME_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Set the bits to control validation of HUK generate mode.\\\\ Odd of 1 is invalid.\\\\ Even of 1 is valid.\\\\"]
    #[inline(always)]
    pub fn huk_gen_state(&self) -> HUK_GEN_STATE_R {
        HUK_GEN_STATE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - Represents whether XTAL frequency is 48MHz or not. If not, 40MHz XTAL will be used. If this field contains Odd number bit '1': Enable 48MHz XTAL\\\\ Even number bit '1': Enable 40MHz XTAL."]
    #[inline(always)]
    pub fn xtal_48m_sel(&self) -> XTAL_48M_SEL_R {
        XTAL_48M_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Specify the XTAL frequency selection is decided by eFuse or strapping-PAD-state. 1: eFuse\\\\ 0: strapping-PAD-state."]
    #[inline(always)]
    pub fn xtal_48m_sel_mode(&self) -> XTAL_48M_SEL_MODE_R {
        XTAL_48M_SEL_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents whether to disable P192 curve in ECDSA.\\\\ 1: Disabled.\\\\ 0: Not disable."]
    #[inline(always)]
    pub fn ecdsa_disable_p192(&self) -> ECDSA_DISABLE_P192_R {
        ECDSA_DISABLE_P192_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents whether to force ecc to use const-time calculation mode. \\\\ 1: Enable. \\\\ 0: Disable."]
    #[inline(always)]
    pub fn ecc_force_const_time(&self) -> ECC_FORCE_CONST_TIME_R {
        ECC_FORCE_CONST_TIME_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA4")
            .field("huk_gen_state", &self.huk_gen_state())
            .field("xtal_48m_sel", &self.xtal_48m_sel())
            .field("xtal_48m_sel_mode", &self.xtal_48m_sel_mode())
            .field("ecdsa_disable_p192", &self.ecdsa_disable_p192())
            .field("ecc_force_const_time", &self.ecc_force_const_time())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data4::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA4 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA4_SPEC {}
