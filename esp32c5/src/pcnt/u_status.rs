#[doc = "Register `U%s_STATUS` reader"]
pub type R = crate::R<U_STATUS_SPEC>;
#[doc = "Field `CNT_THR_ZERO_MODE_U` reader - Represents the pulse counter status of PCNT_U%s corresponding to 0. \\\\ 0: pulse counter decreases from positive to 0\\\\ 1: pulse counter increases from negative to 0\\\\ 2: pulse counter is negative\\\\ 3: pulse counter is positive \\\\"]
pub type CNT_THR_ZERO_MODE_U_R = crate::FieldReader;
#[doc = "Field `CNT_THR_THRES1_LAT_U` reader - Represents the latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to thres1 and thres1 event is valid \\\\"]
pub type CNT_THR_THRES1_LAT_U_R = crate::BitReader;
#[doc = "Field `CNT_THR_THRES0_LAT_U` reader - Represents the latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to thres0 and thres0 event is valid \\\\"]
pub type CNT_THR_THRES0_LAT_U_R = crate::BitReader;
#[doc = "Field `CNT_THR_L_LIM_LAT_U` reader - Represents the latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to thr_l_lim and low limit event is valid. \\\\"]
pub type CNT_THR_L_LIM_LAT_U_R = crate::BitReader;
#[doc = "Field `CNT_THR_H_LIM_LAT_U` reader - Represents the latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to thr_h_lim and high limit event is valid. \\\\"]
pub type CNT_THR_H_LIM_LAT_U_R = crate::BitReader;
#[doc = "Field `CNT_THR_ZERO_LAT_U` reader - Represents the latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to 0 and zero threshold event is valid. \\\\"]
pub type CNT_THR_ZERO_LAT_U_R = crate::BitReader;
#[doc = "Field `CNT_THR_STEP_LIM_LAT_U` reader - The latched value of step counter limit event of PCNT_U%s when step counter event interrupt is valid. 1: the current pulse counter equals to reg_cnt_step_lim and step counter event is valid. 0: others"]
pub type CNT_THR_STEP_LIM_LAT_U_R = crate::BitReader;
#[doc = "Field `CNT_THR_STEP_LAT_U` reader - The latched value of step counter event of PCNT_U%s when step counter event interrupt is valid. 1: the current pulse counter increment equals to reg_cnt_step and step counter event is valid. 0: others"]
pub type CNT_THR_STEP_LAT_U_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Represents the pulse counter status of PCNT_U%s corresponding to 0. \\\\ 0: pulse counter decreases from positive to 0\\\\ 1: pulse counter increases from negative to 0\\\\ 2: pulse counter is negative\\\\ 3: pulse counter is positive \\\\"]
    #[inline(always)]
    pub fn cnt_thr_zero_mode_u(&self) -> CNT_THR_ZERO_MODE_U_R {
        CNT_THR_ZERO_MODE_U_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Represents the latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to thres1 and thres1 event is valid \\\\"]
    #[inline(always)]
    pub fn cnt_thr_thres1_lat_u(&self) -> CNT_THR_THRES1_LAT_U_R {
        CNT_THR_THRES1_LAT_U_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents the latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to thres0 and thres0 event is valid \\\\"]
    #[inline(always)]
    pub fn cnt_thr_thres0_lat_u(&self) -> CNT_THR_THRES0_LAT_U_R {
        CNT_THR_THRES0_LAT_U_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to thr_l_lim and low limit event is valid. \\\\"]
    #[inline(always)]
    pub fn cnt_thr_l_lim_lat_u(&self) -> CNT_THR_L_LIM_LAT_U_R {
        CNT_THR_L_LIM_LAT_U_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents the latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to thr_h_lim and high limit event is valid. \\\\"]
    #[inline(always)]
    pub fn cnt_thr_h_lim_lat_u(&self) -> CNT_THR_H_LIM_LAT_U_R {
        CNT_THR_H_LIM_LAT_U_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents the latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. \\\\ 0: others\\\\ 1: the current pulse counter equals to 0 and zero threshold event is valid. \\\\"]
    #[inline(always)]
    pub fn cnt_thr_zero_lat_u(&self) -> CNT_THR_ZERO_LAT_U_R {
        CNT_THR_ZERO_LAT_U_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The latched value of step counter limit event of PCNT_U%s when step counter event interrupt is valid. 1: the current pulse counter equals to reg_cnt_step_lim and step counter event is valid. 0: others"]
    #[inline(always)]
    pub fn cnt_thr_step_lim_lat_u(&self) -> CNT_THR_STEP_LIM_LAT_U_R {
        CNT_THR_STEP_LIM_LAT_U_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The latched value of step counter event of PCNT_U%s when step counter event interrupt is valid. 1: the current pulse counter increment equals to reg_cnt_step and step counter event is valid. 0: others"]
    #[inline(always)]
    pub fn cnt_thr_step_lat_u(&self) -> CNT_THR_STEP_LAT_U_R {
        CNT_THR_STEP_LAT_U_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_STATUS")
            .field("cnt_thr_zero_mode_u", &self.cnt_thr_zero_mode_u())
            .field("cnt_thr_thres1_lat_u", &self.cnt_thr_thres1_lat_u())
            .field("cnt_thr_thres0_lat_u", &self.cnt_thr_thres0_lat_u())
            .field("cnt_thr_l_lim_lat_u", &self.cnt_thr_l_lim_lat_u())
            .field("cnt_thr_h_lim_lat_u", &self.cnt_thr_h_lim_lat_u())
            .field("cnt_thr_zero_lat_u", &self.cnt_thr_zero_lat_u())
            .field("cnt_thr_step_lim_lat_u", &self.cnt_thr_step_lim_lat_u())
            .field("cnt_thr_step_lat_u", &self.cnt_thr_step_lat_u())
            .finish()
    }
}
#[doc = "PNCT UNIT%s status register\n\nYou can [`read`](crate::Reg::read) this register and get [`u_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U_STATUS_SPEC;
impl crate::RegisterSpec for U_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_status::R`](R) reader structure"]
impl crate::Readable for U_STATUS_SPEC {}
#[doc = "`reset()` method sets U%s_STATUS to value 0"]
impl crate::Resettable for U_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
