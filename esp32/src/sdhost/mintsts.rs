#[doc = "Register `MINTSTS` reader"]
pub type R = crate::R<MINTSTS_SPEC>;
#[doc = "Field `INT_STATUS_MSK` reader - Interrupt enabled only if corresponding bit in interrupt mask register is set. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub type INT_STATUS_MSK_R = crate::FieldReader<u16>;
#[doc = "Field `SDIO_INTERRUPT_MSK` reader - Interrupt from SDIO card, one bit for each card. Bit\\[17:16\\] correspond to card1 and card0, respectively. SDIO interrupt for card is enabled only if corresponding sdhost_sdio_int_mask bit is set in Interrupt mask register (Setting mask bit enables interrupt)."]
pub type SDIO_INTERRUPT_MSK_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Interrupt enabled only if corresponding bit in interrupt mask register is set. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
    #[inline(always)]
    pub fn int_status_msk(&self) -> INT_STATUS_MSK_R {
        INT_STATUS_MSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Interrupt from SDIO card, one bit for each card. Bit\\[17:16\\] correspond to card1 and card0, respectively. SDIO interrupt for card is enabled only if corresponding sdhost_sdio_int_mask bit is set in Interrupt mask register (Setting mask bit enables interrupt)."]
    #[inline(always)]
    pub fn sdio_interrupt_msk(&self) -> SDIO_INTERRUPT_MSK_R {
        SDIO_INTERRUPT_MSK_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MINTSTS")
            .field("int_status_msk", &self.int_status_msk())
            .field("sdio_interrupt_msk", &self.sdio_interrupt_msk())
            .finish()
    }
}
#[doc = "Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MINTSTS_SPEC;
impl crate::RegisterSpec for MINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mintsts::R`](R) reader structure"]
impl crate::Readable for MINTSTS_SPEC {}
#[doc = "`reset()` method sets MINTSTS to value 0"]
impl crate::Resettable for MINTSTS_SPEC {}
