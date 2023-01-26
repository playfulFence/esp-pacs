#[doc = "Register `HP_ACTIVE_HP_SYS_CNTL` reader"]
pub struct R(crate::R<HP_ACTIVE_HP_SYS_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_ACTIVE_HP_SYS_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_ACTIVE_HP_SYS_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_ACTIVE_HP_SYS_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_ACTIVE_HP_SYS_CNTL` writer"]
pub struct W(crate::W<HP_ACTIVE_HP_SYS_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_ACTIVE_HP_SYS_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HP_ACTIVE_HP_SYS_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_ACTIVE_HP_SYS_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_ACTIVE_UART_WAKEUP_EN` reader - need_des"]
pub type HP_ACTIVE_UART_WAKEUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HP_ACTIVE_UART_WAKEUP_EN` writer - need_des"]
pub type HP_ACTIVE_UART_WAKEUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_HP_SYS_CNTL_SPEC, bool, O>;
#[doc = "Field `HP_ACTIVE_LP_PAD_HOLD_ALL` reader - need_des"]
pub type HP_ACTIVE_LP_PAD_HOLD_ALL_R = crate::BitReader<bool>;
#[doc = "Field `HP_ACTIVE_LP_PAD_HOLD_ALL` writer - need_des"]
pub type HP_ACTIVE_LP_PAD_HOLD_ALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_HP_SYS_CNTL_SPEC, bool, O>;
#[doc = "Field `HP_ACTIVE_HP_PAD_HOLD_ALL` reader - need_des"]
pub type HP_ACTIVE_HP_PAD_HOLD_ALL_R = crate::BitReader<bool>;
#[doc = "Field `HP_ACTIVE_HP_PAD_HOLD_ALL` writer - need_des"]
pub type HP_ACTIVE_HP_PAD_HOLD_ALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_HP_SYS_CNTL_SPEC, bool, O>;
#[doc = "Field `HP_ACTIVE_DIG_PAD_SLP_SEL` reader - need_des"]
pub type HP_ACTIVE_DIG_PAD_SLP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `HP_ACTIVE_DIG_PAD_SLP_SEL` writer - need_des"]
pub type HP_ACTIVE_DIG_PAD_SLP_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_HP_SYS_CNTL_SPEC, bool, O>;
#[doc = "Field `HP_ACTIVE_DIG_PAUSE_WDT` reader - need_des"]
pub type HP_ACTIVE_DIG_PAUSE_WDT_R = crate::BitReader<bool>;
#[doc = "Field `HP_ACTIVE_DIG_PAUSE_WDT` writer - need_des"]
pub type HP_ACTIVE_DIG_PAUSE_WDT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_HP_SYS_CNTL_SPEC, bool, O>;
#[doc = "Field `HP_ACTIVE_DIG_CPU_STALL` reader - need_des"]
pub type HP_ACTIVE_DIG_CPU_STALL_R = crate::BitReader<bool>;
#[doc = "Field `HP_ACTIVE_DIG_CPU_STALL` writer - need_des"]
pub type HP_ACTIVE_DIG_CPU_STALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_HP_SYS_CNTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn hp_active_uart_wakeup_en(&self) -> HP_ACTIVE_UART_WAKEUP_EN_R {
        HP_ACTIVE_UART_WAKEUP_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_active_lp_pad_hold_all(&self) -> HP_ACTIVE_LP_PAD_HOLD_ALL_R {
        HP_ACTIVE_LP_PAD_HOLD_ALL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_pad_hold_all(&self) -> HP_ACTIVE_HP_PAD_HOLD_ALL_R {
        HP_ACTIVE_HP_PAD_HOLD_ALL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_active_dig_pad_slp_sel(&self) -> HP_ACTIVE_DIG_PAD_SLP_SEL_R {
        HP_ACTIVE_DIG_PAD_SLP_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn hp_active_dig_pause_wdt(&self) -> HP_ACTIVE_DIG_PAUSE_WDT_R {
        HP_ACTIVE_DIG_PAUSE_WDT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_active_dig_cpu_stall(&self) -> HP_ACTIVE_DIG_CPU_STALL_R {
        HP_ACTIVE_DIG_CPU_STALL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_uart_wakeup_en(&mut self) -> HP_ACTIVE_UART_WAKEUP_EN_W<24> {
        HP_ACTIVE_UART_WAKEUP_EN_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_lp_pad_hold_all(&mut self) -> HP_ACTIVE_LP_PAD_HOLD_ALL_W<25> {
        HP_ACTIVE_LP_PAD_HOLD_ALL_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_hp_pad_hold_all(&mut self) -> HP_ACTIVE_HP_PAD_HOLD_ALL_W<26> {
        HP_ACTIVE_HP_PAD_HOLD_ALL_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_dig_pad_slp_sel(&mut self) -> HP_ACTIVE_DIG_PAD_SLP_SEL_W<27> {
        HP_ACTIVE_DIG_PAD_SLP_SEL_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_dig_pause_wdt(&mut self) -> HP_ACTIVE_DIG_PAUSE_WDT_W<28> {
        HP_ACTIVE_DIG_PAUSE_WDT_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_dig_cpu_stall(&mut self) -> HP_ACTIVE_DIG_CPU_STALL_W<29> {
        HP_ACTIVE_DIG_CPU_STALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_active_hp_sys_cntl](index.html) module"]
pub struct HP_ACTIVE_HP_SYS_CNTL_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_HP_SYS_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_active_hp_sys_cntl::R](R) reader structure"]
impl crate::Readable for HP_ACTIVE_HP_SYS_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_active_hp_sys_cntl::W](W) writer structure"]
impl crate::Writable for HP_ACTIVE_HP_SYS_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_HP_SYS_CNTL to value 0"]
impl crate::Resettable for HP_ACTIVE_HP_SYS_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}