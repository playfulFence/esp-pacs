#[doc = "Register `RD_USR_DATA6` reader"]
pub type R = crate::R<RD_USR_DATA6_SPEC>;
#[doc = "Field `RESERVED_3_192` reader - reserved /"]
pub type RESERVED_3_192_R = crate::FieldReader;
#[doc = "Field `CUSTOM_MAC` reader - Custom MAC address /"]
pub type CUSTOM_MAC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - reserved /"]
    #[inline(always)]
    pub fn reserved_3_192(&self) -> RESERVED_3_192_R {
        RESERVED_3_192_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Custom MAC address /"]
    #[inline(always)]
    pub fn custom_mac(&self) -> CUSTOM_MAC_R {
        CUSTOM_MAC_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_USR_DATA6")
            .field("reserved_3_192", &self.reserved_3_192())
            .field("custom_mac", &self.custom_mac())
            .finish()
    }
}
#[doc = "Register 6 of BLOCK3 (user). /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_usr_data6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_USR_DATA6_SPEC;
impl crate::RegisterSpec for RD_USR_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_usr_data6::R`](R) reader structure"]
impl crate::Readable for RD_USR_DATA6_SPEC {}
#[doc = "`reset()` method sets RD_USR_DATA6 to value 0"]
impl crate::Resettable for RD_USR_DATA6_SPEC {}
