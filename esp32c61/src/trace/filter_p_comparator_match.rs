#[doc = "Register `FILTER_P_COMPARATOR_MATCH` reader"]
pub type R = crate::R<FILTER_P_COMPARATOR_MATCH_SPEC>;
#[doc = "Register `FILTER_P_COMPARATOR_MATCH` writer"]
pub type W = crate::W<FILTER_P_COMPARATOR_MATCH_SPEC>;
#[doc = "Field `P_MATCH` reader - Configures the match value for the primary comparator"]
pub type P_MATCH_R = crate::FieldReader<u32>;
#[doc = "Field `P_MATCH` writer - Configures the match value for the primary comparator"]
pub type P_MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the match value for the primary comparator"]
    #[inline(always)]
    pub fn p_match(&self) -> P_MATCH_R {
        P_MATCH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_P_COMPARATOR_MATCH")
            .field("p_match", &self.p_match())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the match value for the primary comparator"]
    #[inline(always)]
    pub fn p_match(&mut self) -> P_MATCH_W<FILTER_P_COMPARATOR_MATCH_SPEC> {
        P_MATCH_W::new(self, 0)
    }
}
#[doc = "primary comparator match value\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_p_comparator_match::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_p_comparator_match::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_P_COMPARATOR_MATCH_SPEC;
impl crate::RegisterSpec for FILTER_P_COMPARATOR_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_p_comparator_match::R`](R) reader structure"]
impl crate::Readable for FILTER_P_COMPARATOR_MATCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_p_comparator_match::W`](W) writer structure"]
impl crate::Writable for FILTER_P_COMPARATOR_MATCH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_P_COMPARATOR_MATCH to value 0"]
impl crate::Resettable for FILTER_P_COMPARATOR_MATCH_SPEC {}
