#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `COMP_NEG_0_INT_CLR` writer - analog comparator pos edge interrupt clear"]
pub type COMP_NEG_0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_POS_0_INT_CLR` writer - analog comparator neg edge interrupt clear"]
pub type COMP_POS_0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_ALL_0_INT_CLR` writer - analog comparator neg or pos edge interrupt clear"]
pub type COMP_ALL_0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp_neg_0_int_clr(&mut self) -> COMP_NEG_0_INT_CLR_W<INT_CLR_SPEC> {
        COMP_NEG_0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    pub fn comp_pos_0_int_clr(&mut self) -> COMP_POS_0_INT_CLR_W<INT_CLR_SPEC> {
        COMP_POS_0_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp_all_0_int_clr(&mut self) -> COMP_ALL_0_INT_CLR_W<INT_CLR_SPEC> {
        COMP_ALL_0_INT_CLR_W::new(self, 2)
    }
}
#[doc = "GPIO_EXT interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
