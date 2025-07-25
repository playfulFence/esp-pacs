#[doc = "Register `RD_STATUS` reader"]
pub type R = crate::R<RD_STATUS_SPEC>;
#[doc = "Register `RD_STATUS` writer"]
pub type W = crate::W<RD_STATUS_SPEC>;
#[doc = "Field `WB_MODE` reader - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type WB_MODE_R = crate::FieldReader;
#[doc = "Field `WB_MODE` writer - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type WB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WB_MODE_BITLEN` reader - Mode bits length for flash fast read mode."]
pub type WB_MODE_BITLEN_R = crate::FieldReader;
#[doc = "Field `WB_MODE_BITLEN` writer - Mode bits length for flash fast read mode."]
pub type WB_MODE_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WB_MODE_EN` reader - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
pub type WB_MODE_EN_R = crate::BitReader;
#[doc = "Field `WB_MODE_EN` writer - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
pub type WB_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn wb_mode(&self) -> WB_MODE_R {
        WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Mode bits length for flash fast read mode."]
    #[inline(always)]
    pub fn wb_mode_bitlen(&self) -> WB_MODE_BITLEN_R {
        WB_MODE_BITLEN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
    #[inline(always)]
    pub fn wb_mode_en(&self) -> WB_MODE_EN_R {
        WB_MODE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_STATUS")
            .field("wb_mode", &self.wb_mode())
            .field("wb_mode_bitlen", &self.wb_mode_bitlen())
            .field("wb_mode_en", &self.wb_mode_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn wb_mode(&mut self) -> WB_MODE_W<RD_STATUS_SPEC> {
        WB_MODE_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Mode bits length for flash fast read mode."]
    #[inline(always)]
    pub fn wb_mode_bitlen(&mut self) -> WB_MODE_BITLEN_W<RD_STATUS_SPEC> {
        WB_MODE_BITLEN_W::new(self, 24)
    }
    #[doc = "Bit 27 - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
    #[inline(always)]
    pub fn wb_mode_en(&mut self) -> WB_MODE_EN_W<RD_STATUS_SPEC> {
        WB_MODE_EN_W::new(self, 27)
    }
}
#[doc = "SPI0 read control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_STATUS_SPEC;
impl crate::RegisterSpec for RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_status::R`](R) reader structure"]
impl crate::Readable for RD_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_status::W`](W) writer structure"]
impl crate::Writable for RD_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_STATUS to value 0"]
impl crate::Resettable for RD_STATUS_SPEC {}
