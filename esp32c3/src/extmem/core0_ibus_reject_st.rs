#[doc = "Register `CORE0_IBUS_REJECT_ST` reader"]
pub type R = crate::R<CORE0_IBUS_REJECT_ST_SPEC>;
#[doc = "Field `CORE0_IBUS_ATTR` reader - The bits are used to indicate the attribute of CPU access ibus"]
pub type CORE0_IBUS_ATTR_R = crate::FieldReader;
#[doc = "Field `CORE0_IBUS_WORLD` reader - The bit is used to indicate the world of CPU access ibus when"]
pub type CORE0_IBUS_WORLD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - The bits are used to indicate the attribute of CPU access ibus"]
    #[inline(always)]
    pub fn core0_ibus_attr(&self) -> CORE0_IBUS_ATTR_R {
        CORE0_IBUS_ATTR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - The bit is used to indicate the world of CPU access ibus when"]
    #[inline(always)]
    pub fn core0_ibus_world(&self) -> CORE0_IBUS_WORLD_R {
        CORE0_IBUS_WORLD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_IBUS_REJECT_ST")
            .field("core0_ibus_world", &self.core0_ibus_world())
            .field("core0_ibus_attr", &self.core0_ibus_attr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_ibus_reject_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_IBUS_REJECT_ST_SPEC;
impl crate::RegisterSpec for CORE0_IBUS_REJECT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_ibus_reject_st::R`](R) reader structure"]
impl crate::Readable for CORE0_IBUS_REJECT_ST_SPEC {}
#[doc = "`reset()` method sets CORE0_IBUS_REJECT_ST to value 0"]
impl crate::Resettable for CORE0_IBUS_REJECT_ST_SPEC {}
