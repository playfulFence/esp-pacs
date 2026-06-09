#[doc = "Register `RD_MAC_SPI_SYS_2` reader"]
pub type R = crate::R<RD_MAC_SPI_SYS_2_SPEC>;
#[doc = "Field `SPI_PAD_CONFIG_D_1` reader - SPI PAD D(D0) /"]
pub type SPI_PAD_CONFIG_D_1_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_CS` reader - SPI PAD CS /"]
pub type SPI_PAD_CONFIG_CS_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_HD` reader - SPI PAD HD(D3) /"]
pub type SPI_PAD_CONFIG_HD_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_WP` reader - SPI PAD WP(D2) /"]
pub type SPI_PAD_CONFIG_WP_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_DQS` reader - SPI PAD DQS /"]
pub type SPI_PAD_CONFIG_DQS_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_D4` reader - SPI PAD D4 /"]
pub type SPI_PAD_CONFIG_D4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - SPI PAD D(D0) /"]
    #[inline(always)]
    pub fn spi_pad_config_d_1(&self) -> SPI_PAD_CONFIG_D_1_R {
        SPI_PAD_CONFIG_D_1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - SPI PAD CS /"]
    #[inline(always)]
    pub fn spi_pad_config_cs(&self) -> SPI_PAD_CONFIG_CS_R {
        SPI_PAD_CONFIG_CS_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - SPI PAD HD(D3) /"]
    #[inline(always)]
    pub fn spi_pad_config_hd(&self) -> SPI_PAD_CONFIG_HD_R {
        SPI_PAD_CONFIG_HD_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - SPI PAD WP(D2) /"]
    #[inline(always)]
    pub fn spi_pad_config_wp(&self) -> SPI_PAD_CONFIG_WP_R {
        SPI_PAD_CONFIG_WP_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - SPI PAD DQS /"]
    #[inline(always)]
    pub fn spi_pad_config_dqs(&self) -> SPI_PAD_CONFIG_DQS_R {
        SPI_PAD_CONFIG_DQS_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - SPI PAD D4 /"]
    #[inline(always)]
    pub fn spi_pad_config_d4(&self) -> SPI_PAD_CONFIG_D4_R {
        SPI_PAD_CONFIG_D4_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_2")
            .field("spi_pad_config_d_1", &self.spi_pad_config_d_1())
            .field("spi_pad_config_cs", &self.spi_pad_config_cs())
            .field("spi_pad_config_hd", &self.spi_pad_config_hd())
            .field("spi_pad_config_wp", &self.spi_pad_config_wp())
            .field("spi_pad_config_dqs", &self.spi_pad_config_dqs())
            .field("spi_pad_config_d4", &self.spi_pad_config_d4())
            .finish()
    }
}
#[doc = "BLOCK1 data register 2. /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_spi_sys_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SPI_SYS_2_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_spi_sys_2::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_2_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_2 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_2_SPEC {}
