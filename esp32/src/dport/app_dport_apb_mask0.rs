#[doc = "Register `APP_DPORT_APB_MASK0` reader"]
pub type R = crate::R<APP_DPORT_APB_MASK0_SPEC>;
#[doc = "Register `APP_DPORT_APB_MASK0` writer"]
pub type W = crate::W<APP_DPORT_APB_MASK0_SPEC>;
#[doc = "Field `APPDPORT_APB_MASK0` reader - "]
pub type APPDPORT_APB_MASK0_R = crate::FieldReader<u32>;
#[doc = "Field `APPDPORT_APB_MASK0` writer - "]
pub type APPDPORT_APB_MASK0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn appdport_apb_mask0(&self) -> APPDPORT_APB_MASK0_R {
        APPDPORT_APB_MASK0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DPORT_APB_MASK0")
            .field("appdport_apb_mask0", &self.appdport_apb_mask0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn appdport_apb_mask0(&mut self) -> APPDPORT_APB_MASK0_W<APP_DPORT_APB_MASK0_SPEC> {
        APPDPORT_APB_MASK0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_dport_apb_mask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_dport_apb_mask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_DPORT_APB_MASK0_SPEC;
impl crate::RegisterSpec for APP_DPORT_APB_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_dport_apb_mask0::R`](R) reader structure"]
impl crate::Readable for APP_DPORT_APB_MASK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_dport_apb_mask0::W`](W) writer structure"]
impl crate::Writable for APP_DPORT_APB_MASK0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APP_DPORT_APB_MASK0 to value 0"]
impl crate::Resettable for APP_DPORT_APB_MASK0_SPEC {}
