#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `STATE` reader - Those bits indicates the status of the Manual Encryption block. 0X0 (XTS_AES_IDLE): idle. 0X1 (XTS_AES_BUSY): busy with encryption. 0X2 (XTS_AES_DONE): encryption is completed, but the encrypted result is not accessible to SPI. 0X3 (XTS_AES_AVAILABLE) encrypted result is accessible and available to SPI."]
pub type STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Those bits indicates the status of the Manual Encryption block. 0X0 (XTS_AES_IDLE): idle. 0X1 (XTS_AES_BUSY): busy with encryption. 0X2 (XTS_AES_DONE): encryption is completed, but the encrypted result is not accessible to SPI. 0X3 (XTS_AES_AVAILABLE) encrypted result is accessible and available to SPI."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("state", &self.state())
            .finish()
    }
}
#[doc = "XTS-AES status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {}
