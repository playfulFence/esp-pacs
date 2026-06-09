#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `RESP_REC` reader - ."]
pub type RESP_REC_R = crate::BitReader;
#[doc = "Field `SLAVE_RW` reader - ."]
pub type SLAVE_RW_R = crate::BitReader;
#[doc = "Field `ARB_LOST` reader - ."]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` reader - ."]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDRESSED` reader - ."]
pub type SLAVE_ADDRESSED_R = crate::BitReader;
#[doc = "Field `RXFIFO_CNT` reader - ."]
pub type RXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `STRETCH_CAUSE` reader - ."]
pub type STRETCH_CAUSE_R = crate::FieldReader;
#[doc = "Field `TXFIFO_CNT` reader - ."]
pub type TXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - ."]
pub type SCL_MAIN_STATE_LAST_R = crate::FieldReader;
#[doc = "Field `SCL_STATE_LAST` reader - ."]
pub type SCL_STATE_LAST_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn resp_rec(&self) -> RESP_REC_R {
        RESP_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ."]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - ."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - ."]
    #[inline(always)]
    pub fn stretch_cause(&self) -> STRETCH_CAUSE_R {
        STRETCH_CAUSE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:23 - ."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - ."]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - ."]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("scl_state_last", &self.scl_state_last())
            .field("scl_main_state_last", &self.scl_main_state_last())
            .field("txfifo_cnt", &self.txfifo_cnt())
            .field("stretch_cause", &self.stretch_cause())
            .field("rxfifo_cnt", &self.rxfifo_cnt())
            .field("slave_addressed", &self.slave_addressed())
            .field("bus_busy", &self.bus_busy())
            .field("arb_lost", &self.arb_lost())
            .field("slave_rw", &self.slave_rw())
            .field("resp_rec", &self.resp_rec())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {}
