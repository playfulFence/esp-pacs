#[doc = "Register `EXT_WAKEUP1` reader"]
pub type R = crate::R<EXT_WAKEUP1_SPEC>;
#[doc = "Register `EXT_WAKEUP1` writer"]
pub type W = crate::W<EXT_WAKEUP1_SPEC>;
#[doc = "Field `SEL` reader - Bitmap to select RTC pads for ext wakeup1"]
pub type SEL_R = crate::FieldReader<u16>;
#[doc = "Field `SEL` writer - Bitmap to select RTC pads for ext wakeup1"]
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STATUS_CLR` writer - clear ext wakeup1 status"]
pub type STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Bitmap to select RTC pads for ext wakeup1"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP1")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Bitmap to select RTC pads for ext wakeup1"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<EXT_WAKEUP1_SPEC> {
        SEL_W::new(self, 0)
    }
    #[doc = "Bit 16 - clear ext wakeup1 status"]
    #[inline(always)]
    pub fn status_clr(&mut self) -> STATUS_CLR_W<EXT_WAKEUP1_SPEC> {
        STATUS_CLR_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP1_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup1::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup1::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_WAKEUP1 to value 0"]
impl crate::Resettable for EXT_WAKEUP1_SPEC {}
