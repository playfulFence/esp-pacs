#[doc = "Register `LPBUS` reader"]
pub type R = crate::R<LPBUS_SPEC>;
#[doc = "Register `LPBUS` writer"]
pub type W = crate::W<LPBUS_SPEC>;
#[doc = "Field `FAST_MEM_MUX_FSM_IDLE` reader - get current lp memory bus fsm status"]
pub type FAST_MEM_MUX_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `FAST_MEM_MUX_SEL_STATUS` reader - get current lp memory bus mode"]
pub type FAST_MEM_MUX_SEL_STATUS_R = crate::BitReader;
#[doc = "Field `FAST_MEM_MUX_SEL_UPDATE` writer - enable reg_fast_mem_sel configure 1: enable 0: no operation"]
pub type FAST_MEM_MUX_SEL_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_MEM_MUX_SEL` reader - select lp memory bus is high speed mode or low speed mode 1: high speed from hp system ahb 0: low speed from lp system"]
pub type FAST_MEM_MUX_SEL_R = crate::BitReader;
#[doc = "Field `FAST_MEM_MUX_SEL` writer - select lp memory bus is high speed mode or low speed mode 1: high speed from hp system ahb 0: low speed from lp system"]
pub type FAST_MEM_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - get current lp memory bus fsm status"]
    #[inline(always)]
    pub fn fast_mem_mux_fsm_idle(&self) -> FAST_MEM_MUX_FSM_IDLE_R {
        FAST_MEM_MUX_FSM_IDLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - get current lp memory bus mode"]
    #[inline(always)]
    pub fn fast_mem_mux_sel_status(&self) -> FAST_MEM_MUX_SEL_STATUS_R {
        FAST_MEM_MUX_SEL_STATUS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - select lp memory bus is high speed mode or low speed mode 1: high speed from hp system ahb 0: low speed from lp system"]
    #[inline(always)]
    pub fn fast_mem_mux_sel(&self) -> FAST_MEM_MUX_SEL_R {
        FAST_MEM_MUX_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPBUS")
            .field("fast_mem_mux_fsm_idle", &self.fast_mem_mux_fsm_idle())
            .field("fast_mem_mux_sel_status", &self.fast_mem_mux_sel_status())
            .field("fast_mem_mux_sel", &self.fast_mem_mux_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - enable reg_fast_mem_sel configure 1: enable 0: no operation"]
    #[inline(always)]
    pub fn fast_mem_mux_sel_update(&mut self) -> FAST_MEM_MUX_SEL_UPDATE_W<LPBUS_SPEC> {
        FAST_MEM_MUX_SEL_UPDATE_W::new(self, 30)
    }
    #[doc = "Bit 31 - select lp memory bus is high speed mode or low speed mode 1: high speed from hp system ahb 0: low speed from lp system"]
    #[inline(always)]
    pub fn fast_mem_mux_sel(&mut self) -> FAST_MEM_MUX_SEL_W<LPBUS_SPEC> {
        FAST_MEM_MUX_SEL_W::new(self, 31)
    }
}
#[doc = "Select lp memory bus\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPBUS_SPEC;
impl crate::RegisterSpec for LPBUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbus::R`](R) reader structure"]
impl crate::Readable for LPBUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpbus::W`](W) writer structure"]
impl crate::Writable for LPBUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPBUS to value 0xb000_0000"]
impl crate::Resettable for LPBUS_SPEC {
    const RESET_VALUE: u32 = 0xb000_0000;
}
