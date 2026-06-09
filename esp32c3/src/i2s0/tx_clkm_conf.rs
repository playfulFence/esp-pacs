#[doc = "Register `TX_CLKM_CONF` reader"]
pub type R = crate::R<TX_CLKM_CONF_SPEC>;
#[doc = "Register `TX_CLKM_CONF` writer"]
pub type W = crate::W<TX_CLKM_CONF_SPEC>;
#[doc = "Field `TX_CLKM_DIV_NUM` reader - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a)."]
pub type TX_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `TX_CLKM_DIV_NUM` writer - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a)."]
pub type TX_CLKM_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TX_CLK_ACTIVE` reader - I2S Tx module clock enable signal."]
pub type TX_CLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `TX_CLK_ACTIVE` writer - I2S Tx module clock enable signal."]
pub type TX_CLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CLK_SEL` reader - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2:"]
pub type TX_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `TX_CLK_SEL` writer - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2:"]
pub type TX_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_EN` reader - Set this bit to enable clk gate"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set this bit to enable clk gate"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a)."]
    #[inline(always)]
    pub fn tx_clkm_div_num(&self) -> TX_CLKM_DIV_NUM_R {
        TX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 26 - I2S Tx module clock enable signal."]
    #[inline(always)]
    pub fn tx_clk_active(&self) -> TX_CLK_ACTIVE_R {
        TX_CLK_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2:"]
    #[inline(always)]
    pub fn tx_clk_sel(&self) -> TX_CLK_SEL_R {
        TX_CLK_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CLKM_CONF")
            .field("clk_en", &self.clk_en())
            .field("tx_clk_sel", &self.tx_clk_sel())
            .field("tx_clk_active", &self.tx_clk_active())
            .field("tx_clkm_div_num", &self.tx_clkm_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a)."]
    #[inline(always)]
    pub fn tx_clkm_div_num(&mut self) -> TX_CLKM_DIV_NUM_W<'_, TX_CLKM_CONF_SPEC> {
        TX_CLKM_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 26 - I2S Tx module clock enable signal."]
    #[inline(always)]
    pub fn tx_clk_active(&mut self) -> TX_CLK_ACTIVE_W<'_, TX_CLKM_CONF_SPEC> {
        TX_CLK_ACTIVE_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2:"]
    #[inline(always)]
    pub fn tx_clk_sel(&mut self) -> TX_CLK_SEL_W<'_, TX_CLKM_CONF_SPEC> {
        TX_CLK_SEL_W::new(self, 27)
    }
    #[doc = "Bit 29 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, TX_CLKM_CONF_SPEC> {
        CLK_EN_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_clkm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_clkm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CLKM_CONF_SPEC;
impl crate::RegisterSpec for TX_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_clkm_conf::R`](R) reader structure"]
impl crate::Readable for TX_CLKM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_clkm_conf::W`](W) writer structure"]
impl crate::Writable for TX_CLKM_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CLKM_CONF to value 0"]
impl crate::Resettable for TX_CLKM_CONF_SPEC {}
