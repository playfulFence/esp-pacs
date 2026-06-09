#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `RXFIFO_WM` reader - ."]
pub type RXFIFO_WM_R = crate::BitReader;
#[doc = "Field `TXFIFO_WM` reader - ."]
pub type TXFIFO_WM_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - ."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `END_DETECT` reader - ."]
pub type END_DETECT_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE` reader - ."]
pub type BYTE_TRANS_DONE_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` reader - ."]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF` reader - ."]
pub type MST_TXFIFO_UDF_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` reader - ."]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TIME_OUT` reader - ."]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TRANS_START` reader - ."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `NACK` reader - ."]
pub type NACK_R = crate::BitReader;
#[doc = "Field `TXFIFO_OVF` reader - ."]
pub type TXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `RXFIFO_UDF` reader - ."]
pub type RXFIFO_UDF_R = crate::BitReader;
#[doc = "Field `SCL_ST_TO` reader - ."]
pub type SCL_ST_TO_R = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO` reader - ."]
pub type SCL_MAIN_ST_TO_R = crate::BitReader;
#[doc = "Field `DET_START` reader - ."]
pub type DET_START_R = crate::BitReader;
#[doc = "Field `SLAVE_STRETCH` reader - ."]
pub type SLAVE_STRETCH_R = crate::BitReader;
#[doc = "Field `GENERAL_CALL` reader - ."]
pub type GENERAL_CALL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn rxfifo_wm(&self) -> RXFIFO_WM_R {
        RXFIFO_WM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn txfifo_wm(&self) -> TXFIFO_WM_R {
        TXFIFO_WM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn end_detect(&self) -> END_DETECT_R {
        END_DETECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn byte_trans_done(&self) -> BYTE_TRANS_DONE_R {
        BYTE_TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ."]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ."]
    #[inline(always)]
    pub fn mst_txfifo_udf(&self) -> MST_TXFIFO_UDF_R {
        MST_TXFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ."]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ."]
    #[inline(always)]
    pub fn txfifo_ovf(&self) -> TXFIFO_OVF_R {
        TXFIFO_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    pub fn rxfifo_udf(&self) -> RXFIFO_UDF_R {
        RXFIFO_UDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    pub fn scl_st_to(&self) -> SCL_ST_TO_R {
        SCL_ST_TO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ."]
    #[inline(always)]
    pub fn scl_main_st_to(&self) -> SCL_MAIN_ST_TO_R {
        SCL_MAIN_ST_TO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ."]
    #[inline(always)]
    pub fn det_start(&self) -> DET_START_R {
        DET_START_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ."]
    #[inline(always)]
    pub fn slave_stretch(&self) -> SLAVE_STRETCH_R {
        SLAVE_STRETCH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ."]
    #[inline(always)]
    pub fn general_call(&self) -> GENERAL_CALL_R {
        GENERAL_CALL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("general_call", &self.general_call())
            .field("slave_stretch", &self.slave_stretch())
            .field("det_start", &self.det_start())
            .field("scl_main_st_to", &self.scl_main_st_to())
            .field("scl_st_to", &self.scl_st_to())
            .field("rxfifo_udf", &self.rxfifo_udf())
            .field("txfifo_ovf", &self.txfifo_ovf())
            .field("nack", &self.nack())
            .field("trans_start", &self.trans_start())
            .field("time_out", &self.time_out())
            .field("trans_complete", &self.trans_complete())
            .field("mst_txfifo_udf", &self.mst_txfifo_udf())
            .field("arbitration_lost", &self.arbitration_lost())
            .field("byte_trans_done", &self.byte_trans_done())
            .field("end_detect", &self.end_detect())
            .field("rxfifo_ovf", &self.rxfifo_ovf())
            .field("txfifo_wm", &self.txfifo_wm())
            .field("rxfifo_wm", &self.rxfifo_wm())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
