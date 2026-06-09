#[doc = "Register `CORE0_IBUS_REJECT_VADDR` reader"]
pub type R = crate::R<CORE0_IBUS_REJECT_VADDR_SPEC>;
#[doc = "Field `CORE0_IBUS_VADDR` reader - The bits are used to indicate the virtual address of CPU access"]
pub type CORE0_IBUS_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address of CPU access"]
    #[inline(always)]
    pub fn core0_ibus_vaddr(&self) -> CORE0_IBUS_VADDR_R {
        CORE0_IBUS_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_IBUS_REJECT_VADDR")
            .field("core0_ibus_vaddr", &self.core0_ibus_vaddr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_ibus_reject_vaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_IBUS_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for CORE0_IBUS_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_ibus_reject_vaddr::R`](R) reader structure"]
impl crate::Readable for CORE0_IBUS_REJECT_VADDR_SPEC {}
#[doc = "`reset()` method sets CORE0_IBUS_REJECT_VADDR to value 0"]
impl crate::Resettable for CORE0_IBUS_REJECT_VADDR_SPEC {}
