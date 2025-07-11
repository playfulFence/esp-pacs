#[doc = "Register `BYPASS_CHAIN` reader"]
pub type R = crate::R<BYPASS_CHAIN_SPEC>;
#[doc = "Register `BYPASS_CHAIN` writer"]
pub type W = crate::W<BYPASS_CHAIN_SPEC>;
#[doc = "Field `CLK_CHAIN_EN` reader - needs field desc"]
pub type CLK_CHAIN_EN_R = crate::FieldReader<u32>;
#[doc = "Field `CLK_CHAIN_EN` writer - needs field desc"]
pub type CLK_CHAIN_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - needs field desc"]
    #[inline(always)]
    pub fn clk_chain_en(&self) -> CLK_CHAIN_EN_R {
        CLK_CHAIN_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BYPASS_CHAIN")
            .field("clk_chain_en", &self.clk_chain_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - needs field desc"]
    #[inline(always)]
    pub fn clk_chain_en(&mut self) -> CLK_CHAIN_EN_W<BYPASS_CHAIN_SPEC> {
        CLK_CHAIN_EN_W::new(self, 0)
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::Reg::read) this register and get [`bypass_chain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bypass_chain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BYPASS_CHAIN_SPEC;
impl crate::RegisterSpec for BYPASS_CHAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bypass_chain::R`](R) reader structure"]
impl crate::Readable for BYPASS_CHAIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bypass_chain::W`](W) writer structure"]
impl crate::Writable for BYPASS_CHAIN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BYPASS_CHAIN to value 0xffff_ffff"]
impl crate::Resettable for BYPASS_CHAIN_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
