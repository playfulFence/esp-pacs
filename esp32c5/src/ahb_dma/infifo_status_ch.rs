#[doc = "Register `INFIFO_STATUS_CH%s` reader"]
pub type R = crate::R<INFIFO_STATUS_CH_SPEC>;
#[doc = "Field `INFIFO_FULL_CH` reader - Represents whether or not L1 RX FIFO is full.\\\\0: Not Full\\\\1: Full\\\\"]
pub type INFIFO_FULL_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_CH` reader - Represents whether or not L1 RX FIFO is empty.\\\\0: Not empty\\\\1: Empty\\\\"]
pub type INFIFO_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_CH` reader - Represents the number of data bytes in L1 RX FIFO for RX channel %s."]
pub type INFIFO_CNT_CH_R = crate::FieldReader;
#[doc = "Field `IN_REMAIN_UNDER_1B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_1B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_2B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_2B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_3B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_3B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_4B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_4B_CH_R = crate::BitReader;
#[doc = "Field `IN_BUF_HUNGRY_CH` reader - reserved"]
pub type IN_BUF_HUNGRY_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not L1 RX FIFO is full.\\\\0: Not Full\\\\1: Full\\\\"]
    #[inline(always)]
    pub fn infifo_full_ch(&self) -> INFIFO_FULL_CH_R {
        INFIFO_FULL_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether or not L1 RX FIFO is empty.\\\\0: Not empty\\\\1: Empty\\\\"]
    #[inline(always)]
    pub fn infifo_empty_ch(&self) -> INFIFO_EMPTY_CH_R {
        INFIFO_EMPTY_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Represents the number of data bytes in L1 RX FIFO for RX channel %s."]
    #[inline(always)]
    pub fn infifo_cnt_ch(&self) -> INFIFO_CNT_CH_R {
        INFIFO_CNT_CH_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_1b_ch(&self) -> IN_REMAIN_UNDER_1B_CH_R {
        IN_REMAIN_UNDER_1B_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_2b_ch(&self) -> IN_REMAIN_UNDER_2B_CH_R {
        IN_REMAIN_UNDER_2B_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_3b_ch(&self) -> IN_REMAIN_UNDER_3B_CH_R {
        IN_REMAIN_UNDER_3B_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_4b_ch(&self) -> IN_REMAIN_UNDER_4B_CH_R {
        IN_REMAIN_UNDER_4B_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn in_buf_hungry_ch(&self) -> IN_BUF_HUNGRY_CH_R {
        IN_BUF_HUNGRY_CH_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS_CH")
            .field("infifo_full_ch", &self.infifo_full_ch())
            .field("infifo_empty_ch", &self.infifo_empty_ch())
            .field("infifo_cnt_ch", &self.infifo_cnt_ch())
            .field("in_remain_under_1b_ch", &self.in_remain_under_1b_ch())
            .field("in_remain_under_2b_ch", &self.in_remain_under_2b_ch())
            .field("in_remain_under_3b_ch", &self.in_remain_under_3b_ch())
            .field("in_remain_under_4b_ch", &self.in_remain_under_4b_ch())
            .field("in_buf_hungry_ch", &self.in_buf_hungry_ch())
            .finish()
    }
}
#[doc = "Receive FIFO status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status_ch::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_CH_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS_CH%s to value 0x0780_0003"]
impl crate::Resettable for INFIFO_STATUS_CH_SPEC {
    const RESET_VALUE: u32 = 0x0780_0003;
}
