#[doc = "Register `SYSCLK_CONF` reader"]
pub type R = crate::R<SYSCLK_CONF_SPEC>;
#[doc = "Register `SYSCLK_CONF` writer"]
pub type W = crate::W<SYSCLK_CONF_SPEC>;
#[doc = "Field `PRE_DIV_CNT` reader - "]
pub type PRE_DIV_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `PRE_DIV_CNT` writer - "]
pub type PRE_DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SOC_CLK_SEL` reader - "]
pub type SOC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SOC_CLK_SEL` writer - "]
pub type SOC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_XTAL_FREQ` reader - "]
pub type CLK_XTAL_FREQ_R = crate::FieldReader;
#[doc = "Field `CLK_DIV_EN` reader - "]
pub type CLK_DIV_EN_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn pre_div_cnt(&self) -> PRE_DIV_CNT_R {
        PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn soc_clk_sel(&self) -> SOC_CLK_SEL_R {
        SOC_CLK_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:18"]
    #[inline(always)]
    pub fn clk_xtal_freq(&self) -> CLK_XTAL_FREQ_R {
        CLK_XTAL_FREQ_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_div_en(&self) -> CLK_DIV_EN_R {
        CLK_DIV_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCLK_CONF")
            .field("clk_div_en", &self.clk_div_en())
            .field("clk_xtal_freq", &self.clk_xtal_freq())
            .field("soc_clk_sel", &self.soc_clk_sel())
            .field("pre_div_cnt", &self.pre_div_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn pre_div_cnt(&mut self) -> PRE_DIV_CNT_W<'_, SYSCLK_CONF_SPEC> {
        PRE_DIV_CNT_W::new(self, 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn soc_clk_sel(&mut self) -> SOC_CLK_SEL_W<'_, SYSCLK_CONF_SPEC> {
        SOC_CLK_SEL_W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCLK_CONF_SPEC;
impl crate::RegisterSpec for SYSCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclk_conf::R`](R) reader structure"]
impl crate::Readable for SYSCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysclk_conf::W`](W) writer structure"]
impl crate::Writable for SYSCLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCLK_CONF to value 0"]
impl crate::Resettable for SYSCLK_CONF_SPEC {}
