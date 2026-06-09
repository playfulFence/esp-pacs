#[doc = "Register `RD_MAC_SPI_SYS_4` reader"]
pub type R = crate::R<RD_MAC_SPI_SYS_4_SPEC>;
#[doc = "Field `FLASH_VENDOR` reader - Flash vendor /"]
pub type FLASH_VENDOR_R = crate::FieldReader;
#[doc = "Field `RESERVED_1_131` reader - reserved /"]
pub type RESERVED_1_131_R = crate::FieldReader;
#[doc = "Field `K_RTC_LDO` reader - BLOCK1 K_RTC_LDO /"]
pub type K_RTC_LDO_R = crate::FieldReader;
#[doc = "Field `K_DIG_LDO` reader - BLOCK1 K_DIG_LDO /"]
pub type K_DIG_LDO_R = crate::FieldReader;
#[doc = "Field `V_RTC_DBIAS20` reader - BLOCK1 voltage of rtc dbias20 /"]
pub type V_RTC_DBIAS20_R = crate::FieldReader;
#[doc = "Field `V_DIG_DBIAS20` reader - BLOCK1 voltage of digital dbias20 /"]
pub type V_DIG_DBIAS20_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Flash vendor /"]
    #[inline(always)]
    pub fn flash_vendor(&self) -> FLASH_VENDOR_R {
        FLASH_VENDOR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - reserved /"]
    #[inline(always)]
    pub fn reserved_1_131(&self) -> RESERVED_1_131_R {
        RESERVED_1_131_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:13 - BLOCK1 K_RTC_LDO /"]
    #[inline(always)]
    pub fn k_rtc_ldo(&self) -> K_RTC_LDO_R {
        K_RTC_LDO_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - BLOCK1 K_DIG_LDO /"]
    #[inline(always)]
    pub fn k_dig_ldo(&self) -> K_DIG_LDO_R {
        K_DIG_LDO_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:28 - BLOCK1 voltage of rtc dbias20 /"]
    #[inline(always)]
    pub fn v_rtc_dbias20(&self) -> V_RTC_DBIAS20_R {
        V_RTC_DBIAS20_R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bits 29:31 - BLOCK1 voltage of digital dbias20 /"]
    #[inline(always)]
    pub fn v_dig_dbias20(&self) -> V_DIG_DBIAS20_R {
        V_DIG_DBIAS20_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_4")
            .field("flash_vendor", &self.flash_vendor())
            .field("reserved_1_131", &self.reserved_1_131())
            .field("k_rtc_ldo", &self.k_rtc_ldo())
            .field("k_dig_ldo", &self.k_dig_ldo())
            .field("v_rtc_dbias20", &self.v_rtc_dbias20())
            .field("v_dig_dbias20", &self.v_dig_dbias20())
            .finish()
    }
}
#[doc = "BLOCK1 data register 4. /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_spi_sys_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SPI_SYS_4_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_spi_sys_4::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_4_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_4 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_4_SPEC {}
