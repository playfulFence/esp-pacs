#[doc = "Register `VID_VSA_LINES_ACT` reader"]
pub type R = crate::R<VID_VSA_LINES_ACT_SPEC>;
#[doc = "Field `VSA_LINES_ACT` reader - NA"]
pub type VSA_LINES_ACT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn vsa_lines_act(&self) -> VSA_LINES_ACT_R {
        VSA_LINES_ACT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_VSA_LINES_ACT")
            .field("vsa_lines_act", &self.vsa_lines_act())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_vsa_lines_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_VSA_LINES_ACT_SPEC;
impl crate::RegisterSpec for VID_VSA_LINES_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vsa_lines_act::R`](R) reader structure"]
impl crate::Readable for VID_VSA_LINES_ACT_SPEC {}
#[doc = "`reset()` method sets VID_VSA_LINES_ACT to value 0"]
impl crate::Resettable for VID_VSA_LINES_ACT_SPEC {}
