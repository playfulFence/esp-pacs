#[doc = "Register `SPI_SLAVE1` reader"]
pub type R = crate::R<SPI_SLAVE1_SPEC>;
#[doc = "Register `SPI_SLAVE1` writer"]
pub type W = crate::W<SPI_SLAVE1_SPEC>;
#[doc = "Field `SPI_SLV_DATA_BITLEN` reader - Configures the transferred data bit length in SPI slave full-/half-duplex modes."]
pub type SPI_SLV_DATA_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_SLV_DATA_BITLEN` writer - Configures the transferred data bit length in SPI slave full-/half-duplex modes."]
pub type SPI_SLV_DATA_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SPI_SLV_LAST_COMMAND` reader - Configures the command value in slave mode."]
pub type SPI_SLV_LAST_COMMAND_R = crate::FieldReader;
#[doc = "Field `SPI_SLV_LAST_COMMAND` writer - Configures the command value in slave mode."]
pub type SPI_SLV_LAST_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_SLV_LAST_ADDR` reader - Configures the address value in slave mode."]
pub type SPI_SLV_LAST_ADDR_R = crate::FieldReader;
#[doc = "Field `SPI_SLV_LAST_ADDR` writer - Configures the address value in slave mode."]
pub type SPI_SLV_LAST_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:17 - Configures the transferred data bit length in SPI slave full-/half-duplex modes."]
    #[inline(always)]
    pub fn spi_slv_data_bitlen(&self) -> SPI_SLV_DATA_BITLEN_R {
        SPI_SLV_DATA_BITLEN_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:25 - Configures the command value in slave mode."]
    #[inline(always)]
    pub fn spi_slv_last_command(&self) -> SPI_SLV_LAST_COMMAND_R {
        SPI_SLV_LAST_COMMAND_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - Configures the address value in slave mode."]
    #[inline(always)]
    pub fn spi_slv_last_addr(&self) -> SPI_SLV_LAST_ADDR_R {
        SPI_SLV_LAST_ADDR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SLAVE1")
            .field("spi_slv_data_bitlen", &self.spi_slv_data_bitlen())
            .field("spi_slv_last_command", &self.spi_slv_last_command())
            .field("spi_slv_last_addr", &self.spi_slv_last_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Configures the transferred data bit length in SPI slave full-/half-duplex modes."]
    #[inline(always)]
    pub fn spi_slv_data_bitlen(&mut self) -> SPI_SLV_DATA_BITLEN_W<SPI_SLAVE1_SPEC> {
        SPI_SLV_DATA_BITLEN_W::new(self, 0)
    }
    #[doc = "Bits 18:25 - Configures the command value in slave mode."]
    #[inline(always)]
    pub fn spi_slv_last_command(&mut self) -> SPI_SLV_LAST_COMMAND_W<SPI_SLAVE1_SPEC> {
        SPI_SLV_LAST_COMMAND_W::new(self, 18)
    }
    #[doc = "Bits 26:31 - Configures the address value in slave mode."]
    #[inline(always)]
    pub fn spi_slv_last_addr(&mut self) -> SPI_SLV_LAST_ADDR_W<SPI_SLAVE1_SPEC> {
        SPI_SLV_LAST_ADDR_W::new(self, 26)
    }
}
#[doc = "SPI slave control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_slave1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_slave1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SLAVE1_SPEC;
impl crate::RegisterSpec for SPI_SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_slave1::R`](R) reader structure"]
impl crate::Readable for SPI_SLAVE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_slave1::W`](W) writer structure"]
impl crate::Writable for SPI_SLAVE1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SLAVE1 to value 0"]
impl crate::Resettable for SPI_SLAVE1_SPEC {}
