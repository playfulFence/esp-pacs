#[doc = "Register `RD_MAC_SPI_SYS_1` reader"]
pub type R = crate::R<RD_MAC_SPI_SYS_1_SPEC>;
#[doc = "Field `MAC_1` reader - Stores the high 16 bits of MAC address. /"]
pub type MAC_1_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_PAD_CONFIG_CLK` reader - SPI PAD CLK /"]
pub type SPI_PAD_CONFIG_CLK_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_Q` reader - SPI PAD Q(D1) /"]
pub type SPI_PAD_CONFIG_Q_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_D` reader - SPI PAD D(D0) /"]
pub type SPI_PAD_CONFIG_D_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Stores the high 16 bits of MAC address. /"]
    #[inline(always)]
    pub fn mac_1(&self) -> MAC_1_R {
        MAC_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - SPI PAD CLK /"]
    #[inline(always)]
    pub fn spi_pad_config_clk(&self) -> SPI_PAD_CONFIG_CLK_R {
        SPI_PAD_CONFIG_CLK_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - SPI PAD Q(D1) /"]
    #[inline(always)]
    pub fn spi_pad_config_q(&self) -> SPI_PAD_CONFIG_Q_R {
        SPI_PAD_CONFIG_Q_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - SPI PAD D(D0) /"]
    #[inline(always)]
    pub fn spi_pad_config_d(&self) -> SPI_PAD_CONFIG_D_R {
        SPI_PAD_CONFIG_D_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_1")
            .field("mac_1", &self.mac_1())
            .field("spi_pad_config_clk", &self.spi_pad_config_clk())
            .field("spi_pad_config_q", &self.spi_pad_config_q())
            .field("spi_pad_config_d", &self.spi_pad_config_d())
            .finish()
    }
}
#[doc = "BLOCK1 data register 1. /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_spi_sys_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SPI_SYS_1_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_spi_sys_1::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_1_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_1 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_1_SPEC {}
