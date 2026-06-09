#[doc = "Register `PERIP_RST_EN1` reader"]
pub type R = crate::R<PERIP_RST_EN1_SPEC>;
#[doc = "Register `PERIP_RST_EN1` writer"]
pub type W = crate::W<PERIP_RST_EN1_SPEC>;
#[doc = "Field `CRYPTO_AES_RST` reader - "]
pub type CRYPTO_AES_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_RST` writer - "]
pub type CRYPTO_AES_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_RST` reader - "]
pub type CRYPTO_SHA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_RST` writer - "]
pub type CRYPTO_SHA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_RST` reader - "]
pub type CRYPTO_RSA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_RST` writer - "]
pub type CRYPTO_RSA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DS_RST` reader - "]
pub type CRYPTO_DS_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_RST` writer - "]
pub type CRYPTO_DS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_HMAC_RST` reader - "]
pub type CRYPTO_HMAC_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_RST` writer - "]
pub type CRYPTO_HMAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RST` reader - "]
pub type DMA_RST_R = crate::BitReader;
#[doc = "Field `DMA_RST` writer - "]
pub type DMA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HOST_RST` reader - "]
pub type SDIO_HOST_RST_R = crate::BitReader;
#[doc = "Field `SDIO_HOST_RST` writer - "]
pub type SDIO_HOST_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CAM_RST` reader - "]
pub type LCD_CAM_RST_R = crate::BitReader;
#[doc = "Field `LCD_CAM_RST` writer - "]
pub type LCD_CAM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_RST` reader - "]
pub type TSENS_RST_R = crate::BitReader;
#[doc = "Field `TSENS_RST` writer - "]
pub type TSENS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crypto_aes_rst(&self) -> CRYPTO_AES_RST_R {
        CRYPTO_AES_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crypto_sha_rst(&self) -> CRYPTO_SHA_RST_R {
        CRYPTO_SHA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn crypto_rsa_rst(&self) -> CRYPTO_RSA_RST_R {
        CRYPTO_RSA_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn crypto_ds_rst(&self) -> CRYPTO_DS_RST_R {
        CRYPTO_DS_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crypto_hmac_rst(&self) -> CRYPTO_HMAC_RST_R {
        CRYPTO_HMAC_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sdio_host_rst(&self) -> SDIO_HOST_RST_R {
        SDIO_HOST_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lcd_cam_rst(&self) -> LCD_CAM_RST_R {
        LCD_CAM_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tsens_rst(&self) -> TSENS_RST_R {
        TSENS_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_RST_EN1")
            .field("tsens_rst", &self.tsens_rst())
            .field("lcd_cam_rst", &self.lcd_cam_rst())
            .field("sdio_host_rst", &self.sdio_host_rst())
            .field("dma_rst", &self.dma_rst())
            .field("crypto_hmac_rst", &self.crypto_hmac_rst())
            .field("crypto_ds_rst", &self.crypto_ds_rst())
            .field("crypto_rsa_rst", &self.crypto_rsa_rst())
            .field("crypto_sha_rst", &self.crypto_sha_rst())
            .field("crypto_aes_rst", &self.crypto_aes_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crypto_aes_rst(&mut self) -> CRYPTO_AES_RST_W<'_, PERIP_RST_EN1_SPEC> {
        CRYPTO_AES_RST_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W<'_, PERIP_RST_EN1_SPEC> {
        CRYPTO_SHA_RST_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn crypto_rsa_rst(&mut self) -> CRYPTO_RSA_RST_W<'_, PERIP_RST_EN1_SPEC> {
        CRYPTO_RSA_RST_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn crypto_ds_rst(&mut self) -> CRYPTO_DS_RST_W<'_, PERIP_RST_EN1_SPEC> {
        CRYPTO_DS_RST_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crypto_hmac_rst(&mut self) -> CRYPTO_HMAC_RST_W<'_, PERIP_RST_EN1_SPEC> {
        CRYPTO_HMAC_RST_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_rst(&mut self) -> DMA_RST_W<'_, PERIP_RST_EN1_SPEC> {
        DMA_RST_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sdio_host_rst(&mut self) -> SDIO_HOST_RST_W<'_, PERIP_RST_EN1_SPEC> {
        SDIO_HOST_RST_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lcd_cam_rst(&mut self) -> LCD_CAM_RST_W<'_, PERIP_RST_EN1_SPEC> {
        LCD_CAM_RST_W::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tsens_rst(&mut self) -> TSENS_RST_W<'_, PERIP_RST_EN1_SPEC> {
        TSENS_RST_W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`perip_rst_en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perip_rst_en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_RST_EN1_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_rst_en1::R`](R) reader structure"]
impl crate::Readable for PERIP_RST_EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_rst_en1::W`](W) writer structure"]
impl crate::Writable for PERIP_RST_EN1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIP_RST_EN1 to value 0"]
impl crate::Resettable for PERIP_RST_EN1_SPEC {}
