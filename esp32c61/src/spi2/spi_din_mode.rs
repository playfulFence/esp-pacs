#[doc = "Register `SPI_DIN_MODE` reader"]
pub type R = crate::R<SPI_DIN_MODE_SPEC>;
#[doc = "Register `SPI_DIN_MODE` writer"]
pub type W = crate::W<SPI_DIN_MODE_SPEC>;
#[doc = "Field `SPI_DIN0_MODE` reader - Configures the input mode for FSPID signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN0_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN0_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN0_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
pub type SPI_DIN0_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN0_MODE` writer - Configures the input mode for FSPID signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN0_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN0_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN0_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
pub type SPI_DIN0_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN1_MODE` reader - Configures the input mode for FSPIQ signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN1_NUM+1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN1_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN1_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
pub type SPI_DIN1_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN1_MODE` writer - Configures the input mode for FSPIQ signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN1_NUM+1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN1_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN1_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
pub type SPI_DIN1_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN2_MODE` reader - Configures the input mode for FSPIWP signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN2_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN2_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN2_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
pub type SPI_DIN2_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN2_MODE` writer - Configures the input mode for FSPIWP signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN2_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN2_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN2_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
pub type SPI_DIN2_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN3_MODE` reader - Configures the input mode for FSPIHD signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN3_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN3_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN3_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
pub type SPI_DIN3_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN3_MODE` writer - Configures the input mode for FSPIHD signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN3_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN3_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN3_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
pub type SPI_DIN3_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN4_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN4_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN5_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN5_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN6_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN6_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_DIN7_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type SPI_DIN7_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_TIMING_HCLK_ACTIVE` reader - Configures whether or not to enable HCLK (high-frequency clock) in SPI input timing module.\\\\ 0: Disable\\\\ 1: Enable\\\\ Can be configured in CONF state."]
pub type SPI_TIMING_HCLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_TIMING_HCLK_ACTIVE` writer - Configures whether or not to enable HCLK (high-frequency clock) in SPI input timing module.\\\\ 0: Disable\\\\ 1: Enable\\\\ Can be configured in CONF state."]
pub type SPI_TIMING_HCLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures the input mode for FSPID signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN0_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN0_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN0_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din0_mode(&self) -> SPI_DIN0_MODE_R {
        SPI_DIN0_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configures the input mode for FSPIQ signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN1_NUM+1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN1_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN1_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din1_mode(&self) -> SPI_DIN1_MODE_R {
        SPI_DIN1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configures the input mode for FSPIWP signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN2_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN2_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN2_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din2_mode(&self) -> SPI_DIN2_MODE_R {
        SPI_DIN2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configures the input mode for FSPIHD signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN3_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN3_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN3_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din3_mode(&self) -> SPI_DIN3_MODE_R {
        SPI_DIN3_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din4_mode(&self) -> SPI_DIN4_MODE_R {
        SPI_DIN4_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din5_mode(&self) -> SPI_DIN5_MODE_R {
        SPI_DIN5_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din6_mode(&self) -> SPI_DIN6_MODE_R {
        SPI_DIN6_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din7_mode(&self) -> SPI_DIN7_MODE_R {
        SPI_DIN7_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Configures whether or not to enable HCLK (high-frequency clock) in SPI input timing module.\\\\ 0: Disable\\\\ 1: Enable\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_timing_hclk_active(&self) -> SPI_TIMING_HCLK_ACTIVE_R {
        SPI_TIMING_HCLK_ACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DIN_MODE")
            .field("spi_din0_mode", &self.spi_din0_mode())
            .field("spi_din1_mode", &self.spi_din1_mode())
            .field("spi_din2_mode", &self.spi_din2_mode())
            .field("spi_din3_mode", &self.spi_din3_mode())
            .field("spi_din4_mode", &self.spi_din4_mode())
            .field("spi_din5_mode", &self.spi_din5_mode())
            .field("spi_din6_mode", &self.spi_din6_mode())
            .field("spi_din7_mode", &self.spi_din7_mode())
            .field("spi_timing_hclk_active", &self.spi_timing_hclk_active())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures the input mode for FSPID signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN0_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN0_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN0_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din0_mode(&mut self) -> SPI_DIN0_MODE_W<SPI_DIN_MODE_SPEC> {
        SPI_DIN0_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configures the input mode for FSPIQ signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN1_NUM+1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN1_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN1_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din1_mode(&mut self) -> SPI_DIN1_MODE_W<SPI_DIN_MODE_SPEC> {
        SPI_DIN1_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configures the input mode for FSPIWP signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN2_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN2_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN2_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din2_mode(&mut self) -> SPI_DIN2_MODE_W<SPI_DIN_MODE_SPEC> {
        SPI_DIN2_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Configures the input mode for FSPIHD signal.\\\\ 0: Input without delay\\\\ 1: Input at the (SPI_DIN3_NUM + 1)th falling edge of clk_spi_mst\\\\ 2: Input at the (SPI_DIN3_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst rising edge cycle\\\\ 3: Input at the (SPI_DIN3_NUM + 1)th rising edge of clk_hclk plus one clk_spi_mst falling edge cycle\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din3_mode(&mut self) -> SPI_DIN3_MODE_W<SPI_DIN_MODE_SPEC> {
        SPI_DIN3_MODE_W::new(self, 6)
    }
    #[doc = "Bit 16 - Configures whether or not to enable HCLK (high-frequency clock) in SPI input timing module.\\\\ 0: Disable\\\\ 1: Enable\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_timing_hclk_active(&mut self) -> SPI_TIMING_HCLK_ACTIVE_W<SPI_DIN_MODE_SPEC> {
        SPI_TIMING_HCLK_ACTIVE_W::new(self, 16)
    }
}
#[doc = "SPI input delay mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_din_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_din_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DIN_MODE_SPEC;
impl crate::RegisterSpec for SPI_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_din_mode::R`](R) reader structure"]
impl crate::Readable for SPI_DIN_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_din_mode::W`](W) writer structure"]
impl crate::Writable for SPI_DIN_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_DIN_MODE to value 0"]
impl crate::Resettable for SPI_DIN_MODE_SPEC {}
