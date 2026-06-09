#[doc = "Register `RD_MAC_SPI_SYS_5` reader"]
pub type R = crate::R<RD_MAC_SPI_SYS_5_SPEC>;
#[doc = "Field `V_DIG_DBIAS20_1` reader - BLOCK1 voltage of digital dbias20 /"]
pub type V_DIG_DBIAS20_1_R = crate::FieldReader;
#[doc = "Field `DIG_DBIAS_HVT` reader - BLOCK1 digital dbias when hvt /"]
pub type DIG_DBIAS_HVT_R = crate::FieldReader;
#[doc = "Field `THRES_HVT` reader - BLOCK1 pvt threshold when hvt /"]
pub type THRES_HVT_R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED_1_180` reader - reserved /"]
pub type RESERVED_1_180_R = crate::FieldReader;
#[doc = "Field `WAFER_VERSION_MINOR_HI` reader - WAFER_VERSION_MINOR most significant bit /"]
pub type WAFER_VERSION_MINOR_HI_R = crate::BitReader;
#[doc = "Field `WAFER_VERSION_MAJOR` reader - WAFER_VERSION_MAJOR /"]
pub type WAFER_VERSION_MAJOR_R = crate::FieldReader;
#[doc = "Field `RESERVED_1_186` reader - reserved /"]
pub type RESERVED_1_186_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - BLOCK1 voltage of digital dbias20 /"]
    #[inline(always)]
    pub fn v_dig_dbias20_1(&self) -> V_DIG_DBIAS20_1_R {
        V_DIG_DBIAS20_1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - BLOCK1 digital dbias when hvt /"]
    #[inline(always)]
    pub fn dig_dbias_hvt(&self) -> DIG_DBIAS_HVT_R {
        DIG_DBIAS_HVT_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:19 - BLOCK1 pvt threshold when hvt /"]
    #[inline(always)]
    pub fn thres_hvt(&self) -> THRES_HVT_R {
        THRES_HVT_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:22 - reserved /"]
    #[inline(always)]
    pub fn reserved_1_180(&self) -> RESERVED_1_180_R {
        RESERVED_1_180_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - WAFER_VERSION_MINOR most significant bit /"]
    #[inline(always)]
    pub fn wafer_version_minor_hi(&self) -> WAFER_VERSION_MINOR_HI_R {
        WAFER_VERSION_MINOR_HI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - WAFER_VERSION_MAJOR /"]
    #[inline(always)]
    pub fn wafer_version_major(&self) -> WAFER_VERSION_MAJOR_R {
        WAFER_VERSION_MAJOR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - reserved /"]
    #[inline(always)]
    pub fn reserved_1_186(&self) -> RESERVED_1_186_R {
        RESERVED_1_186_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_5")
            .field("v_dig_dbias20_1", &self.v_dig_dbias20_1())
            .field("dig_dbias_hvt", &self.dig_dbias_hvt())
            .field("thres_hvt", &self.thres_hvt())
            .field("reserved_1_180", &self.reserved_1_180())
            .field("wafer_version_minor_hi", &self.wafer_version_minor_hi())
            .field("wafer_version_major", &self.wafer_version_major())
            .field("reserved_1_186", &self.reserved_1_186())
            .finish()
    }
}
#[doc = "BLOCK1 data register 5. /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_spi_sys_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SPI_SYS_5_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_spi_sys_5::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_5_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_5 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_5_SPEC {}
