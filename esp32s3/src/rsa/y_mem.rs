#[doc = "Register `Y_MEM[%s]` writer"]
pub type W = crate::W<Y_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<Y_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Memory Y\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y_mem::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Y_MEM_SPEC;
impl crate::RegisterSpec for Y_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`y_mem::W`](W) writer structure"]
impl crate::Writable for Y_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Y_MEM[%s] to value 0"]
impl crate::Resettable for Y_MEM_SPEC {}
