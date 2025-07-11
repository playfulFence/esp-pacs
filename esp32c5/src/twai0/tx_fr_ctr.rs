#[doc = "Register `TX_FR_CTR` reader"]
pub type R = crate::R<TX_FR_CTR_SPEC>;
#[doc = "Field `TX_CTR_VAL` reader - Number of transmitted frames by CTU CAN FD."]
pub type TX_CTR_VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of transmitted frames by CTU CAN FD."]
    #[inline(always)]
    pub fn tx_ctr_val(&self) -> TX_CTR_VAL_R {
        TX_CTR_VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_FR_CTR")
            .field("tx_ctr_val", &self.tx_ctr_val())
            .finish()
    }
}
#[doc = "TWAI FD transmitted frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fr_ctr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_FR_CTR_SPEC;
impl crate::RegisterSpec for TX_FR_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fr_ctr::R`](R) reader structure"]
impl crate::Readable for TX_FR_CTR_SPEC {}
#[doc = "`reset()` method sets TX_FR_CTR to value 0"]
impl crate::Resettable for TX_FR_CTR_SPEC {}
