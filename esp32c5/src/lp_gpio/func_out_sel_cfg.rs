#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Field `FUNC_OUT_INV_SEL` reader - Configures whether or not to invert the output value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC_OUT_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC_OUT_INV_SEL` writer - Configures whether or not to invert the output value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC_OUT_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUNC_OE_INV_SEL` reader - Configures whether or not to invert the output enable signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC_OE_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC_OE_INV_SEL` writer - Configures whether or not to invert the output enable signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC_OE_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to invert the output value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func_out_inv_sel(&self) -> FUNC_OUT_INV_SEL_R {
        FUNC_OUT_INV_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to invert the output enable signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func_oe_inv_sel(&self) -> FUNC_OE_INV_SEL_R {
        FUNC_OE_INV_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_OUT_SEL_CFG")
            .field("func_out_inv_sel", &self.func_out_inv_sel())
            .field("func_oe_inv_sel", &self.func_oe_inv_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to invert the output value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func_out_inv_sel(&mut self) -> FUNC_OUT_INV_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        FUNC_OUT_INV_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configures whether or not to invert the output enable signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func_oe_inv_sel(&mut self) -> FUNC_OE_INV_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        FUNC_OE_INV_SEL_W::new(self, 2)
    }
}
#[doc = "Configuration register for GPIO%s output\n\nYou can [`read`](crate::Reg::read) this register and get [`func_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {}
