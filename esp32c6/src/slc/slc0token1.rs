#[doc = "Register `SLC0TOKEN1` reader"]
pub type R = crate::R<SLC0TOKEN1_SPEC>;
#[doc = "Register `SLC0TOKEN1` writer"]
pub type W = crate::W<SLC0TOKEN1_SPEC>;
#[doc = "Field `SDIO_SLC0_TOKEN1_WDATA` writer - Configures SLC0 token 1 value."]
pub type SDIO_SLC0_TOKEN1_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SDIO_SLC0_TOKEN1_WR` writer - Configures this bit to 1 to write SDIO_SLC0_TOKEN1_WDATA into SDIO_SLC0_TOKEN1."]
pub type SDIO_SLC0_TOKEN1_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_SLC0_TOKEN1_INC` writer - Configures this bit to 1 to add 1 to SDIO_SLC0_TOKEN1."]
pub type SDIO_SLC0_TOKEN1_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_SLC0_TOKEN1_INC_MORE` writer - Configures this bit to 1 to add the value of SDIO_SLC0_TOKEN1_WDATA to SDIO_SLC0_TOKEN1."]
pub type SDIO_SLC0_TOKEN1_INC_MORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_SLC0_TOKEN1` reader - Represents the SLC0 accumulated number of buffers for receiving packets."]
pub type SDIO_SLC0_TOKEN1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:27 - Represents the SLC0 accumulated number of buffers for receiving packets."]
    #[inline(always)]
    pub fn sdio_slc0_token1(&self) -> SDIO_SLC0_TOKEN1_R {
        SDIO_SLC0_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0TOKEN1")
            .field("sdio_slc0_token1", &self.sdio_slc0_token1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Configures SLC0 token 1 value."]
    #[inline(always)]
    pub fn sdio_slc0_token1_wdata(&mut self) -> SDIO_SLC0_TOKEN1_WDATA_W<SLC0TOKEN1_SPEC> {
        SDIO_SLC0_TOKEN1_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 12 - Configures this bit to 1 to write SDIO_SLC0_TOKEN1_WDATA into SDIO_SLC0_TOKEN1."]
    #[inline(always)]
    pub fn sdio_slc0_token1_wr(&mut self) -> SDIO_SLC0_TOKEN1_WR_W<SLC0TOKEN1_SPEC> {
        SDIO_SLC0_TOKEN1_WR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures this bit to 1 to add 1 to SDIO_SLC0_TOKEN1."]
    #[inline(always)]
    pub fn sdio_slc0_token1_inc(&mut self) -> SDIO_SLC0_TOKEN1_INC_W<SLC0TOKEN1_SPEC> {
        SDIO_SLC0_TOKEN1_INC_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures this bit to 1 to add the value of SDIO_SLC0_TOKEN1_WDATA to SDIO_SLC0_TOKEN1."]
    #[inline(always)]
    pub fn sdio_slc0_token1_inc_more(&mut self) -> SDIO_SLC0_TOKEN1_INC_MORE_W<SLC0TOKEN1_SPEC> {
        SDIO_SLC0_TOKEN1_INC_MORE_W::new(self, 14)
    }
}
#[doc = "SLC0 receiving buffer configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`slc0token1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slc0token1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0TOKEN1_SPEC;
impl crate::RegisterSpec for SLC0TOKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0token1::R`](R) reader structure"]
impl crate::Readable for SLC0TOKEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0token1::W`](W) writer structure"]
impl crate::Writable for SLC0TOKEN1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLC0TOKEN1 to value 0"]
impl crate::Resettable for SLC0TOKEN1_SPEC {}
