#[doc = "Register `INT_CLEAR` writer"]
pub type W = crate::W<INT_CLEAR_SPEC>;
#[doc = "Field `INT_CLEAR` writer - Configures whether or not to clear AES interrupt. \\\\ 0: No effect \\\\ 1: Clear \\\\"]
pub type INT_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLEAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear AES interrupt. \\\\ 0: No effect \\\\ 1: Clear \\\\"]
    #[inline(always)]
    pub fn int_clear(&mut self) -> INT_CLEAR_W<INT_CLEAR_SPEC> {
        INT_CLEAR_W::new(self, 0)
    }
}
#[doc = "DMA-AES interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLEAR_SPEC;
impl crate::RegisterSpec for INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for INT_CLEAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for INT_CLEAR_SPEC {}
