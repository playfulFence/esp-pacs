#[doc = "Register `RD_USR_DATA7` reader"]
pub type R = crate::R<RD_USR_DATA7_SPEC>;
#[doc = "Field `CUSTOM_MAC_1` reader - Custom MAC address /"]
pub type CUSTOM_MAC_1_R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_3_248` reader - reserved /"]
pub type RESERVED_3_248_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Custom MAC address /"]
    #[inline(always)]
    pub fn custom_mac_1(&self) -> CUSTOM_MAC_1_R {
        CUSTOM_MAC_1_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved /"]
    #[inline(always)]
    pub fn reserved_3_248(&self) -> RESERVED_3_248_R {
        RESERVED_3_248_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_USR_DATA7")
            .field("custom_mac_1", &self.custom_mac_1())
            .field("reserved_3_248", &self.reserved_3_248())
            .finish()
    }
}
#[doc = "Register 7 of BLOCK3 (user). /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_usr_data7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_USR_DATA7_SPEC;
impl crate::RegisterSpec for RD_USR_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_usr_data7::R`](R) reader structure"]
impl crate::Readable for RD_USR_DATA7_SPEC {}
#[doc = "`reset()` method sets RD_USR_DATA7 to value 0"]
impl crate::Resettable for RD_USR_DATA7_SPEC {}
