#[doc = "Register `RD_MAC_SPI_SYS_3` reader"]
pub type R = crate::R<RD_MAC_SPI_SYS_3_SPEC>;
#[doc = "Field `SPI_PAD_CONFIG_D5` reader - SPI PAD D5 /"]
pub type SPI_PAD_CONFIG_D5_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_D6` reader - SPI PAD D6 /"]
pub type SPI_PAD_CONFIG_D6_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_D7` reader - SPI PAD D7 /"]
pub type SPI_PAD_CONFIG_D7_R = crate::FieldReader;
#[doc = "Field `WAFER_VERSION_MINOR_LO` reader - WAFER_VERSION_MINOR least significant bits /"]
pub type WAFER_VERSION_MINOR_LO_R = crate::FieldReader;
#[doc = "Field `PKG_VERSION` reader - Package version /"]
pub type PKG_VERSION_R = crate::FieldReader;
#[doc = "Field `BLK_VERSION_MINOR` reader - BLK_VERSION_MINOR /"]
pub type BLK_VERSION_MINOR_R = crate::FieldReader;
#[doc = "Field `FLASH_CAP` reader - Flash capacity /"]
pub type FLASH_CAP_R = crate::FieldReader;
#[doc = "Field `FLASH_TEMP` reader - Flash temperature /"]
pub type FLASH_TEMP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - SPI PAD D5 /"]
    #[inline(always)]
    pub fn spi_pad_config_d5(&self) -> SPI_PAD_CONFIG_D5_R {
        SPI_PAD_CONFIG_D5_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - SPI PAD D6 /"]
    #[inline(always)]
    pub fn spi_pad_config_d6(&self) -> SPI_PAD_CONFIG_D6_R {
        SPI_PAD_CONFIG_D6_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - SPI PAD D7 /"]
    #[inline(always)]
    pub fn spi_pad_config_d7(&self) -> SPI_PAD_CONFIG_D7_R {
        SPI_PAD_CONFIG_D7_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:20 - WAFER_VERSION_MINOR least significant bits /"]
    #[inline(always)]
    pub fn wafer_version_minor_lo(&self) -> WAFER_VERSION_MINOR_LO_R {
        WAFER_VERSION_MINOR_LO_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Package version /"]
    #[inline(always)]
    pub fn pkg_version(&self) -> PKG_VERSION_R {
        PKG_VERSION_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - BLK_VERSION_MINOR /"]
    #[inline(always)]
    pub fn blk_version_minor(&self) -> BLK_VERSION_MINOR_R {
        BLK_VERSION_MINOR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Flash capacity /"]
    #[inline(always)]
    pub fn flash_cap(&self) -> FLASH_CAP_R {
        FLASH_CAP_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Flash temperature /"]
    #[inline(always)]
    pub fn flash_temp(&self) -> FLASH_TEMP_R {
        FLASH_TEMP_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_3")
            .field("spi_pad_config_d5", &self.spi_pad_config_d5())
            .field("spi_pad_config_d6", &self.spi_pad_config_d6())
            .field("spi_pad_config_d7", &self.spi_pad_config_d7())
            .field("wafer_version_minor_lo", &self.wafer_version_minor_lo())
            .field("pkg_version", &self.pkg_version())
            .field("blk_version_minor", &self.blk_version_minor())
            .field("flash_cap", &self.flash_cap())
            .field("flash_temp", &self.flash_temp())
            .finish()
    }
}
#[doc = "BLOCK1 data register 3. /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_spi_sys_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SPI_SYS_3_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_spi_sys_3::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_3_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_3 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_3_SPEC {}
