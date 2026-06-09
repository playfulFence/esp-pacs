#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `CS0_DIS` reader - SPI_CS0 pin enable 1: disable SPI_CS0 0: SPI_CS0 pin is active"]
pub type CS0_DIS_R = crate::BitReader;
#[doc = "Field `CS0_DIS` writer - SPI_CS0 pin enable 1: disable SPI_CS0 0: SPI_CS0 pin is active"]
pub type CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1_DIS` reader - SPI_CS1 pin enable 1: disable SPI_CS1 0: SPI_CS1 pin is active"]
pub type CS1_DIS_R = crate::BitReader;
#[doc = "Field `CS1_DIS` writer - SPI_CS1 pin enable 1: disable SPI_CS1 0: SPI_CS1 pin is active"]
pub type CS1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of SPI0 em_st controlled"]
pub type TRANS_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of SPI0 em_st controlled"]
pub type TRANS_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI_ST_TRANS_END` reader - The bit is used to indicate the SPI0 cspi_st controlled transmitting is done."]
pub type CSPI_ST_TRANS_END_R = crate::BitReader;
#[doc = "Field `CSPI_ST_TRANS_END` writer - The bit is used to indicate the SPI0 cspi_st controlled transmitting is done."]
pub type CSPI_ST_TRANS_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI_ST_TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of SPI0 cspi_st controlled"]
pub type CSPI_ST_TRANS_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CSPI_ST_TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of SPI0 cspi_st controlled"]
pub type CSPI_ST_TRANS_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_ST_TRANS_END` reader - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
pub type SLV_ST_TRANS_END_R = crate::BitReader;
#[doc = "Field `SLV_ST_TRANS_END` writer - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
pub type SLV_ST_TRANS_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_ST_TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of spi0_slv_st controlled"]
pub type SLV_ST_TRANS_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_ST_TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of spi0_slv_st controlled"]
pub type SLV_ST_TRANS_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI_CS0 pin enable 1: disable SPI_CS0 0: SPI_CS0 pin is active"]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable 1: disable SPI_CS1 0: SPI_CS1 pin is active"]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable the interrupt of SPI0 em_st controlled"]
    #[inline(always)]
    pub fn trans_end_int_ena(&self) -> TRANS_END_INT_ENA_R {
        TRANS_END_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate the SPI0 cspi_st controlled transmitting is done."]
    #[inline(always)]
    pub fn cspi_st_trans_end(&self) -> CSPI_ST_TRANS_END_R {
        CSPI_ST_TRANS_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt of SPI0 cspi_st controlled"]
    #[inline(always)]
    pub fn cspi_st_trans_end_int_ena(&self) -> CSPI_ST_TRANS_END_INT_ENA_R {
        CSPI_ST_TRANS_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn slv_st_trans_end(&self) -> SLV_ST_TRANS_END_R {
        SLV_ST_TRANS_END_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to enable the interrupt of spi0_slv_st controlled"]
    #[inline(always)]
    pub fn slv_st_trans_end_int_ena(&self) -> SLV_ST_TRANS_END_INT_ENA_R {
        SLV_ST_TRANS_END_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("cs_keep_active", &self.cs_keep_active())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("slv_st_trans_end_int_ena", &self.slv_st_trans_end_int_ena())
            .field("slv_st_trans_end", &self.slv_st_trans_end())
            .field(
                "cspi_st_trans_end_int_ena",
                &self.cspi_st_trans_end_int_ena(),
            )
            .field("cspi_st_trans_end", &self.cspi_st_trans_end())
            .field("trans_end_int_ena", &self.trans_end_int_ena())
            .field("cs1_dis", &self.cs1_dis())
            .field("cs0_dis", &self.cs0_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI_CS0 pin enable 1: disable SPI_CS0 0: SPI_CS0 pin is active"]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W<'_, MISC_SPEC> {
        CS0_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable 1: disable SPI_CS1 0: SPI_CS1 pin is active"]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W<'_, MISC_SPEC> {
        CS1_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to enable the interrupt of SPI0 em_st controlled"]
    #[inline(always)]
    pub fn trans_end_int_ena(&mut self) -> TRANS_END_INT_ENA_W<'_, MISC_SPEC> {
        TRANS_END_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to indicate the SPI0 cspi_st controlled transmitting is done."]
    #[inline(always)]
    pub fn cspi_st_trans_end(&mut self) -> CSPI_ST_TRANS_END_W<'_, MISC_SPEC> {
        CSPI_ST_TRANS_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt of SPI0 cspi_st controlled"]
    #[inline(always)]
    pub fn cspi_st_trans_end_int_ena(&mut self) -> CSPI_ST_TRANS_END_INT_ENA_W<'_, MISC_SPEC> {
        CSPI_ST_TRANS_END_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn slv_st_trans_end(&mut self) -> SLV_ST_TRANS_END_W<'_, MISC_SPEC> {
        SLV_ST_TRANS_END_W::new(self, 5)
    }
    #[doc = "Bit 6 - The bit is used to enable the interrupt of spi0_slv_st controlled"]
    #[inline(always)]
    pub fn slv_st_trans_end_int_ena(&mut self) -> SLV_ST_TRANS_END_INT_ENA_W<'_, MISC_SPEC> {
        SLV_ST_TRANS_END_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<'_, MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<'_, MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {}
