#[doc = "Register `ASSIST_INFO_MEM[%s]` reader"]
pub type R = crate::R<ASSIST_INFO_MEM_SPEC>;
#[doc = "Register `ASSIST_INFO_MEM[%s]` writer"]
pub type W = crate::W<ASSIST_INFO_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores assist key info.\n\nYou can [`read`](crate::Reg::read) this register and get [`assist_info_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assist_info_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASSIST_INFO_MEM_SPEC;
impl crate::RegisterSpec for ASSIST_INFO_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`assist_info_mem::R`](R) reader structure"]
impl crate::Readable for ASSIST_INFO_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`assist_info_mem::W`](W) writer structure"]
impl crate::Writable for ASSIST_INFO_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASSIST_INFO_MEM[%s] to value 0"]
impl crate::Resettable for ASSIST_INFO_MEM_SPEC {}
