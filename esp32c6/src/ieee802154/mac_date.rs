#[doc = "Register `MAC_DATE` reader"]
pub type R = crate::R<MAC_DATE_SPEC>;
#[doc = "Register `MAC_DATE` writer"]
pub type W = crate::W<MAC_DATE_SPEC>;
#[doc = "Field `MAC_DATE` reader - "]
pub type MAC_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `MAC_DATE` writer - "]
pub type MAC_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac_date(&self) -> MAC_DATE_R {
        MAC_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_DATE")
            .field("mac_date", &self.mac_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac_date(&mut self) -> MAC_DATE_W<MAC_DATE_SPEC> {
        MAC_DATE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_DATE_SPEC;
impl crate::RegisterSpec for MAC_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_date::R`](R) reader structure"]
impl crate::Readable for MAC_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_date::W`](W) writer structure"]
impl crate::Writable for MAC_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_DATE to value 0"]
impl crate::Resettable for MAC_DATE_SPEC {}
