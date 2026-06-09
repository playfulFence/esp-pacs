#[doc = "Register `POSPULSE` reader"]
pub type R = crate::R<POSPULSE_SPEC>;
#[doc = "Field `POSEDGE_MIN_CNT` reader - This register stores the minimal input clock count between two"]
pub type POSEDGE_MIN_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - This register stores the minimal input clock count between two"]
    #[inline(always)]
    pub fn posedge_min_cnt(&self) -> POSEDGE_MIN_CNT_R {
        POSEDGE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POSPULSE")
            .field("posedge_min_cnt", &self.posedge_min_cnt())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pospulse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POSPULSE_SPEC;
impl crate::RegisterSpec for POSPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pospulse::R`](R) reader structure"]
impl crate::Readable for POSPULSE_SPEC {}
#[doc = "`reset()` method sets POSPULSE to value 0"]
impl crate::Resettable for POSPULSE_SPEC {}
