#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `RXNE` reader - RX buffer not empty. This bit is 1 when least one frame is stored in RX buffer. 0: empty 1: not empty"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `DOR` reader - Data Overrun flag. This bit is set when frame was dropped due to lack of space in RX buffer. This bit can be cleared by COMMAND\\[RRB\\]. 0: not overrun 1: overrun"]
pub type DOR_R = crate::BitReader;
#[doc = "Field `TXNF` reader - TXT buffers status. This bit is set if at least one TXT buffer is in \"Empty\" state. 0: not full 1: full"]
pub type TXNF_R = crate::BitReader;
#[doc = "Field `EFT` reader - Error frame is being transmitted at the moment. 0: not being transmitted 1: being transmitted"]
pub type EFT_R = crate::BitReader;
#[doc = "Field `RXS` reader - CTU CAN FD is receiver of CAN Frame. 0: not receiving 1: receiving"]
pub type RXS_R = crate::BitReader;
#[doc = "Field `TXS` reader - CTU CAN FD is transmitter of CAN Frame. 0: not transmitting 1: transmitting"]
pub type TXS_R = crate::BitReader;
#[doc = "Field `EWL` reader - TX Error counter (TEC) or RX Error counter (REC) is equal to, or higher than Error warning limit (EWL). 0: not reached 1: reached"]
pub type EWL_R = crate::BitReader;
#[doc = "Field `IDLE` reader - Bus is idle (no frame is being transmitted/received) or CTU CAN FD is bus-off. 0: active 1: not active"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `PEXS` reader - Protocol exception status (flag). Set when Protocol exception occurs. Cleared by writing COMMAND\\[CPEXS\\]=1."]
pub type PEXS_R = crate::BitReader;
#[doc = "Field `RXPE` reader - Set when parity error is detected during read of CAN frame from RX Buffer via RX_DATA register."]
pub type RXPE_R = crate::BitReader;
#[doc = "Field `TXPE` reader - TXT Buffers Parity Error flag. Set When Parity Error is detected in a TXT Buffer during transmission from this Buffer."]
pub type TXPE_R = crate::BitReader;
#[doc = "Field `TXDPE` reader - TXT Buffer double parity error. Set in TXT Buffer Backup mode when parity error is detected in \"backup\" TXT Buffer."]
pub type TXDPE_R = crate::BitReader;
#[doc = "Field `STCNT` reader - Support of Traffic counters. When this bit is 1, Traffic counters are present."]
pub type STCNT_R = crate::BitReader;
#[doc = "Field `STRGS` reader - Support of Test Registers for memory testability. When this bit is 1, Test Registers are present."]
pub type STRGS_R = crate::BitReader;
#[doc = "Field `SPRT` reader - Support of Parity protection on each word of TXT Buffer RAM and RX Buffer RAM."]
pub type SPRT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RX buffer not empty. This bit is 1 when least one frame is stored in RX buffer. 0: empty 1: not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Overrun flag. This bit is set when frame was dropped due to lack of space in RX buffer. This bit can be cleared by COMMAND\\[RRB\\]. 0: not overrun 1: overrun"]
    #[inline(always)]
    pub fn dor(&self) -> DOR_R {
        DOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXT buffers status. This bit is set if at least one TXT buffer is in \"Empty\" state. 0: not full 1: full"]
    #[inline(always)]
    pub fn txnf(&self) -> TXNF_R {
        TXNF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error frame is being transmitted at the moment. 0: not being transmitted 1: being transmitted"]
    #[inline(always)]
    pub fn eft(&self) -> EFT_R {
        EFT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CTU CAN FD is receiver of CAN Frame. 0: not receiving 1: receiving"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTU CAN FD is transmitter of CAN Frame. 0: not transmitting 1: transmitting"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Error counter (TEC) or RX Error counter (REC) is equal to, or higher than Error warning limit (EWL). 0: not reached 1: reached"]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus is idle (no frame is being transmitted/received) or CTU CAN FD is bus-off. 0: active 1: not active"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol exception status (flag). Set when Protocol exception occurs. Cleared by writing COMMAND\\[CPEXS\\]=1."]
    #[inline(always)]
    pub fn pexs(&self) -> PEXS_R {
        PEXS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set when parity error is detected during read of CAN frame from RX Buffer via RX_DATA register."]
    #[inline(always)]
    pub fn rxpe(&self) -> RXPE_R {
        RXPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXT Buffers Parity Error flag. Set When Parity Error is detected in a TXT Buffer during transmission from this Buffer."]
    #[inline(always)]
    pub fn txpe(&self) -> TXPE_R {
        TXPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXT Buffer double parity error. Set in TXT Buffer Backup mode when parity error is detected in \"backup\" TXT Buffer."]
    #[inline(always)]
    pub fn txdpe(&self) -> TXDPE_R {
        TXDPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Support of Traffic counters. When this bit is 1, Traffic counters are present."]
    #[inline(always)]
    pub fn stcnt(&self) -> STCNT_R {
        STCNT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Support of Test Registers for memory testability. When this bit is 1, Test Registers are present."]
    #[inline(always)]
    pub fn strgs(&self) -> STRGS_R {
        STRGS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Support of Parity protection on each word of TXT Buffer RAM and RX Buffer RAM."]
    #[inline(always)]
    pub fn sprt(&self) -> SPRT_R {
        SPRT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("rxne", &self.rxne())
            .field("dor", &self.dor())
            .field("txnf", &self.txnf())
            .field("eft", &self.eft())
            .field("rxs", &self.rxs())
            .field("txs", &self.txs())
            .field("ewl", &self.ewl())
            .field("idle", &self.idle())
            .field("pexs", &self.pexs())
            .field("rxpe", &self.rxpe())
            .field("txpe", &self.txpe())
            .field("txdpe", &self.txdpe())
            .field("stcnt", &self.stcnt())
            .field("strgs", &self.strgs())
            .field("sprt", &self.sprt())
            .finish()
    }
}
#[doc = "TWAI FD status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x0007_0084"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0007_0084;
}
