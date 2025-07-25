#[doc = "Register `LP_CLK_EN` reader"]
pub type R = crate::R<LP_CLK_EN_SPEC>;
#[doc = "Register `LP_CLK_EN` writer"]
pub type W = crate::W<LP_CLK_EN_SPEC>;
#[doc = "Field `FAST_ORI_GATE` reader - Configures the clock gate to LP_FAST_CLK 0: Invalid. The clock gate controlled by hardware fsm 1: Force the clk pass clock gate"]
pub type FAST_ORI_GATE_R = crate::BitReader;
#[doc = "Field `FAST_ORI_GATE` writer - Configures the clock gate to LP_FAST_CLK 0: Invalid. The clock gate controlled by hardware fsm 1: Force the clk pass clock gate"]
pub type FAST_ORI_GATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Configures the clock gate to LP_FAST_CLK 0: Invalid. The clock gate controlled by hardware fsm 1: Force the clk pass clock gate"]
    #[inline(always)]
    pub fn fast_ori_gate(&self) -> FAST_ORI_GATE_R {
        FAST_ORI_GATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CLK_EN")
            .field("fast_ori_gate", &self.fast_ori_gate())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Configures the clock gate to LP_FAST_CLK 0: Invalid. The clock gate controlled by hardware fsm 1: Force the clk pass clock gate"]
    #[inline(always)]
    pub fn fast_ori_gate(&mut self) -> FAST_ORI_GATE_W<LP_CLK_EN_SPEC> {
        FAST_ORI_GATE_W::new(self, 31)
    }
}
#[doc = "Configure LP root clk source gate\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CLK_EN_SPEC;
impl crate::RegisterSpec for LP_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_clk_en::R`](R) reader structure"]
impl crate::Readable for LP_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_clk_en::W`](W) writer structure"]
impl crate::Writable for LP_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CLK_EN to value 0"]
impl crate::Resettable for LP_CLK_EN_SPEC {}
