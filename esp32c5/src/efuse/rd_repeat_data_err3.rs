#[doc = "Register `RD_REPEAT_DATA_ERR3` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR3_SPEC>;
#[doc = "Field `DIS_DOWNLOAD_MODE_ERR` reader - Represents the programming error of EFUSE_DIS_DOWNLOAD_MODE"]
pub type DIS_DOWNLOAD_MODE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DIRECT_BOOT_ERR` reader - Represents the programming error of EFUSE_DIS_DIRECT_BOOT"]
pub type DIS_DIRECT_BOOT_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR` reader - Represents the programming error of EFUSE_DIS_USB_SERIAL_JTAG_ROM_PRINT"]
pub type DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_R = crate::BitReader;
#[doc = "Field `LOCK_KM_KEY_ERR` reader - Represents the programming error of EFUSE_LOCK_KM_KEY"]
pub type LOCK_KM_KEY_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR` reader - Represents the programming error of EFUSE_DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE"]
pub type DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD_ERR` reader - Represents the programming error of EFUSE_ENABLE_SECURITY_DOWNLOAD"]
pub type ENABLE_SECURITY_DOWNLOAD_ERR_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL_ERR` reader - Represents the programming error of EFUSE_UART_PRINT_CONTROL"]
pub type UART_PRINT_CONTROL_ERR_R = crate::FieldReader;
#[doc = "Field `FORCE_SEND_RESUME_ERR` reader - Represents the programming error of EFUSE_FORCE_SEND_RESUME"]
pub type FORCE_SEND_RESUME_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_VERSION_ERR` reader - Represents the programming error of EFUSE_SECURE_VERSION"]
pub type SECURE_VERSION_ERR_R = crate::FieldReader<u16>;
#[doc = "Field `SECURE_BOOT_DISABLE_FAST_WAKE_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_DISABLE_FAST_WAKE"]
pub type SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R = crate::BitReader;
#[doc = "Field `HYS_EN_PAD_ERR` reader - Represents the programming error of EFUSE_HYS_EN_PAD"]
pub type HYS_EN_PAD_ERR_R = crate::BitReader;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL_ERR` reader - Represents the programming error of EFUSE_XTS_DPA_PSEUDO_LEVEL"]
pub type XTS_DPA_PSEUDO_LEVEL_ERR_R = crate::FieldReader;
#[doc = "Field `XTS_DPA_CLK_ENABLE_ERR` reader - Represents the programming error of EFUSE_XTS_DPA_CLK_ENABLE"]
pub type XTS_DPA_CLK_ENABLE_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents the programming error of EFUSE_DIS_DOWNLOAD_MODE"]
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DIS_DOWNLOAD_MODE_ERR_R {
        DIS_DOWNLOAD_MODE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents the programming error of EFUSE_DIS_DIRECT_BOOT"]
    #[inline(always)]
    pub fn dis_direct_boot_err(&self) -> DIS_DIRECT_BOOT_ERR_R {
        DIS_DIRECT_BOOT_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents the programming error of EFUSE_DIS_USB_SERIAL_JTAG_ROM_PRINT"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_rom_print_err(&self) -> DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_R {
        DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents the programming error of EFUSE_LOCK_KM_KEY"]
    #[inline(always)]
    pub fn lock_km_key_err(&self) -> LOCK_KM_KEY_ERR_R {
        LOCK_KM_KEY_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the programming error of EFUSE_DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode_err(&self) -> DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R {
        DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents the programming error of EFUSE_ENABLE_SECURITY_DOWNLOAD"]
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> ENABLE_SECURITY_DOWNLOAD_ERR_R {
        ENABLE_SECURITY_DOWNLOAD_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Represents the programming error of EFUSE_UART_PRINT_CONTROL"]
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> UART_PRINT_CONTROL_ERR_R {
        UART_PRINT_CONTROL_ERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Represents the programming error of EFUSE_FORCE_SEND_RESUME"]
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> FORCE_SEND_RESUME_ERR_R {
        FORCE_SEND_RESUME_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:24 - Represents the programming error of EFUSE_SECURE_VERSION"]
    #[inline(always)]
    pub fn secure_version_err(&self) -> SECURE_VERSION_ERR_R {
        SECURE_VERSION_ERR_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
    #[doc = "Bit 25 - Represents the programming error of EFUSE_SECURE_BOOT_DISABLE_FAST_WAKE"]
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake_err(&self) -> SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R {
        SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents the programming error of EFUSE_HYS_EN_PAD"]
    #[inline(always)]
    pub fn hys_en_pad_err(&self) -> HYS_EN_PAD_ERR_R {
        HYS_EN_PAD_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Represents the programming error of EFUSE_XTS_DPA_PSEUDO_LEVEL"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level_err(&self) -> XTS_DPA_PSEUDO_LEVEL_ERR_R {
        XTS_DPA_PSEUDO_LEVEL_ERR_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Represents the programming error of EFUSE_XTS_DPA_CLK_ENABLE"]
    #[inline(always)]
    pub fn xts_dpa_clk_enable_err(&self) -> XTS_DPA_CLK_ENABLE_ERR_R {
        XTS_DPA_CLK_ENABLE_ERR_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR3")
            .field("dis_download_mode_err", &self.dis_download_mode_err())
            .field("dis_direct_boot_err", &self.dis_direct_boot_err())
            .field(
                "dis_usb_serial_jtag_rom_print_err",
                &self.dis_usb_serial_jtag_rom_print_err(),
            )
            .field("lock_km_key_err", &self.lock_km_key_err())
            .field(
                "dis_usb_serial_jtag_download_mode_err",
                &self.dis_usb_serial_jtag_download_mode_err(),
            )
            .field(
                "enable_security_download_err",
                &self.enable_security_download_err(),
            )
            .field("uart_print_control_err", &self.uart_print_control_err())
            .field("force_send_resume_err", &self.force_send_resume_err())
            .field("secure_version_err", &self.secure_version_err())
            .field(
                "secure_boot_disable_fast_wake_err",
                &self.secure_boot_disable_fast_wake_err(),
            )
            .field("hys_en_pad_err", &self.hys_en_pad_err())
            .field("xts_dpa_pseudo_level_err", &self.xts_dpa_pseudo_level_err())
            .field("xts_dpa_clk_enable_err", &self.xts_dpa_clk_enable_err())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err3::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR3_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR3 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR3_SPEC {}
