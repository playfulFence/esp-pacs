#[doc = "Register `SMEM_DIN_HEX_MODE` reader"]
pub type R = crate::R<SMEM_DIN_HEX_MODE_SPEC>;
#[doc = "Field `SMEM_DIN08_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DIN08_MODE_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN09_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DIN09_MODE_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN10_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DIN10_MODE_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN11_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DIN11_MODE_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN12_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DIN12_MODE_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN13_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DIN13_MODE_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN14_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DIN14_MODE_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN15_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DIN15_MODE_R = crate::FieldReader;
#[doc = "Field `SMEM_DINS_HEX_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SMEM_DINS_HEX_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_din08_mode(&self) -> SMEM_DIN08_MODE_R {
        SMEM_DIN08_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_din09_mode(&self) -> SMEM_DIN09_MODE_R {
        SMEM_DIN09_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_din10_mode(&self) -> SMEM_DIN10_MODE_R {
        SMEM_DIN10_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_din11_mode(&self) -> SMEM_DIN11_MODE_R {
        SMEM_DIN11_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_din12_mode(&self) -> SMEM_DIN12_MODE_R {
        SMEM_DIN12_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_din13_mode(&self) -> SMEM_DIN13_MODE_R {
        SMEM_DIN13_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_din14_mode(&self) -> SMEM_DIN14_MODE_R {
        SMEM_DIN14_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_din15_mode(&self) -> SMEM_DIN15_MODE_R {
        SMEM_DIN15_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dins_hex_mode(&self) -> SMEM_DINS_HEX_MODE_R {
        SMEM_DINS_HEX_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DIN_HEX_MODE")
            .field("smem_din08_mode", &self.smem_din08_mode())
            .field("smem_din09_mode", &self.smem_din09_mode())
            .field("smem_din10_mode", &self.smem_din10_mode())
            .field("smem_din11_mode", &self.smem_din11_mode())
            .field("smem_din12_mode", &self.smem_din12_mode())
            .field("smem_din13_mode", &self.smem_din13_mode())
            .field("smem_din14_mode", &self.smem_din14_mode())
            .field("smem_din15_mode", &self.smem_din15_mode())
            .field("smem_dins_hex_mode", &self.smem_dins_hex_mode())
            .finish()
    }
}
#[doc = "MSPI 16x external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_hex_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DIN_HEX_MODE_SPEC;
impl crate::RegisterSpec for SMEM_DIN_HEX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_din_hex_mode::R`](R) reader structure"]
impl crate::Readable for SMEM_DIN_HEX_MODE_SPEC {}
#[doc = "`reset()` method sets SMEM_DIN_HEX_MODE to value 0"]
impl crate::Resettable for SMEM_DIN_HEX_MODE_SPEC {}
