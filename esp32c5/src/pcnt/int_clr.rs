#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `CNT_THR_EVENT_U0_INT_CLR` writer - Set this bit to clear the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
pub type CNT_THR_EVENT_U0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U1_INT_CLR` writer - Set this bit to clear the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
pub type CNT_THR_EVENT_U1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U2_INT_CLR` writer - Set this bit to clear the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
pub type CNT_THR_EVENT_U2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U3_INT_CLR` writer - Set this bit to clear the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
pub type CNT_THR_EVENT_U3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_clr(&mut self) -> CNT_THR_EVENT_U0_INT_CLR_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_clr(&mut self) -> CNT_THR_EVENT_U1_INT_CLR_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_clr(&mut self) -> CNT_THR_EVENT_U2_INT_CLR_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U2_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_clr(&mut self) -> CNT_THR_EVENT_U3_INT_CLR_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U3_INT_CLR_W::new(self, 3)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
