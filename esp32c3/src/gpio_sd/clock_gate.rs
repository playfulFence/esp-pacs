#[doc = "Register `CLOCK_GATE` reader"]
pub type R = crate::R<CLOCK_GATE_SPEC>;
#[doc = "Register `CLOCK_GATE` writer"]
pub type W = crate::W<CLOCK_GATE_SPEC>;
#[doc = "Field `SD_CLK_EN` reader - "]
pub type SD_CLK_EN_R = crate::BitReader;
#[doc = "Field `SD_CLK_EN` writer - "]
pub type SD_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SD_CLK_EN_R {
        SD_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_GATE")
            .field("sd_clk_en", &self.sd_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sd_clk_en(&mut self) -> SD_CLK_EN_W<'_, CLOCK_GATE_SPEC> {
        SD_CLK_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_GATE_SPEC;
impl crate::RegisterSpec for CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_gate::R`](R) reader structure"]
impl crate::Readable for CLOCK_GATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_gate::W`](W) writer structure"]
impl crate::Writable for CLOCK_GATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK_GATE to value 0"]
impl crate::Resettable for CLOCK_GATE_SPEC {}
