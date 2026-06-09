#[doc = "Register `REF_CNT_RST` writer"]
pub type W = crate::W<REF_CNT_RST_SPEC>;
#[doc = "Field `REF_CNT_RST_CH0` writer - "]
pub type REF_CNT_RST_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_CNT_RST_CH1` writer - "]
pub type REF_CNT_RST_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_CNT_RST_CH2` writer - "]
pub type REF_CNT_RST_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_CNT_RST_CH3` writer - "]
pub type REF_CNT_RST_CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CNT_RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ref_cnt_rst_ch0(&mut self) -> REF_CNT_RST_CH0_W<'_, REF_CNT_RST_SPEC> {
        REF_CNT_RST_CH0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ref_cnt_rst_ch1(&mut self) -> REF_CNT_RST_CH1_W<'_, REF_CNT_RST_SPEC> {
        REF_CNT_RST_CH1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ref_cnt_rst_ch2(&mut self) -> REF_CNT_RST_CH2_W<'_, REF_CNT_RST_SPEC> {
        REF_CNT_RST_CH2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ref_cnt_rst_ch3(&mut self) -> REF_CNT_RST_CH3_W<'_, REF_CNT_RST_SPEC> {
        REF_CNT_RST_CH3_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_cnt_rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ref_cnt_rst::W`](W) writer structure"]
impl crate::Writable for REF_CNT_RST_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {}
