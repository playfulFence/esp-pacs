#[doc = "Register `RD_REPEAT_ERR3` reader"]
pub struct R(crate::R<RD_REPEAT_ERR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIS_DOWNLOAD_MODE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_DOWNLOAD_MODE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_LEGACY_SPI_BOOT_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_LEGACY_SPI_BOOT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `UART_PRINT_CHANNEL_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type UART_PRINT_CHANNEL_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_ECC_MODE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type FLASH_ECC_MODE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_USB_DOWNLOAD_MODE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_USB_DOWNLOAD_MODE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type ENABLE_SECURITY_DOWNLOAD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `UART_PRINT_CONTROL_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type UART_PRINT_CONTROL_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_POWER_SELECTION_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type PIN_POWER_SELECTION_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_TYPE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type FLASH_TYPE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_PAGE_SIZE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type FLASH_PAGE_SIZE_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_ECC_EN_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type FLASH_ECC_EN_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_SEND_RESUME_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type FORCE_SEND_RESUME_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_VERSION_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type SECURE_VERSION_ERR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `POWERGLITCH_EN_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type POWERGLITCH_EN_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RPT4_RESERVED1_ERR` reader - Reserved."]
pub type RPT4_RESERVED1_ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DIS_DOWNLOAD_MODE_ERR_R {
        DIS_DOWNLOAD_MODE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_legacy_spi_boot_err(&self) -> DIS_LEGACY_SPI_BOOT_ERR_R {
        DIS_LEGACY_SPI_BOOT_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn uart_print_channel_err(&self) -> UART_PRINT_CHANNEL_ERR_R {
        UART_PRINT_CHANNEL_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn flash_ecc_mode_err(&self) -> FLASH_ECC_MODE_ERR_R {
        FLASH_ECC_MODE_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_usb_download_mode_err(&self) -> DIS_USB_DOWNLOAD_MODE_ERR_R {
        DIS_USB_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> ENABLE_SECURITY_DOWNLOAD_ERR_R {
        ENABLE_SECURITY_DOWNLOAD_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> UART_PRINT_CONTROL_ERR_R {
        UART_PRINT_CONTROL_ERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn pin_power_selection_err(&self) -> PIN_POWER_SELECTION_ERR_R {
        PIN_POWER_SELECTION_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn flash_type_err(&self) -> FLASH_TYPE_ERR_R {
        FLASH_TYPE_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn flash_page_size_err(&self) -> FLASH_PAGE_SIZE_ERR_R {
        FLASH_PAGE_SIZE_ERR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn flash_ecc_en_err(&self) -> FLASH_ECC_EN_ERR_R {
        FLASH_ECC_EN_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> FORCE_SEND_RESUME_ERR_R {
        FORCE_SEND_RESUME_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:29 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_version_err(&self) -> SECURE_VERSION_ERR_R {
        SECURE_VERSION_ERR_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 30 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn powerglitch_en_err(&self) -> POWERGLITCH_EN_ERR_R {
        POWERGLITCH_EN_ERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved1_err(&self) -> RPT4_RESERVED1_ERR_R {
        RPT4_RESERVED1_ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Programming error record register 3 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err3](index.html) module"]
pub struct RD_REPEAT_ERR3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err3::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR3 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}