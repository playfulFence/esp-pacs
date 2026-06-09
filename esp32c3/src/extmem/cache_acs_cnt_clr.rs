#[doc = "Register `CACHE_ACS_CNT_CLR` reader"]
pub type R = crate::R<CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "Register `CACHE_ACS_CNT_CLR` writer"]
pub type W = crate::W<CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "Field `IBUS_ACS_CNT_CLR` reader - The bit is used to clear ibus counter."]
pub type IBUS_ACS_CNT_CLR_R = crate::BitReader;
#[doc = "Field `IBUS_ACS_CNT_CLR` writer - The bit is used to clear ibus counter."]
pub type IBUS_ACS_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS_ACS_CNT_CLR` reader - The bit is used to clear dbus counter."]
pub type DBUS_ACS_CNT_CLR_R = crate::BitReader;
#[doc = "Field `DBUS_ACS_CNT_CLR` writer - The bit is used to clear dbus counter."]
pub type DBUS_ACS_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to clear ibus counter."]
    #[inline(always)]
    pub fn ibus_acs_cnt_clr(&self) -> IBUS_ACS_CNT_CLR_R {
        IBUS_ACS_CNT_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to clear dbus counter."]
    #[inline(always)]
    pub fn dbus_acs_cnt_clr(&self) -> DBUS_ACS_CNT_CLR_R {
        DBUS_ACS_CNT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_CNT_CLR")
            .field("dbus_acs_cnt_clr", &self.dbus_acs_cnt_clr())
            .field("ibus_acs_cnt_clr", &self.ibus_acs_cnt_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear ibus counter."]
    #[inline(always)]
    pub fn ibus_acs_cnt_clr(&mut self) -> IBUS_ACS_CNT_CLR_W<'_, CACHE_ACS_CNT_CLR_SPEC> {
        IBUS_ACS_CNT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear dbus counter."]
    #[inline(always)]
    pub fn dbus_acs_cnt_clr(&mut self) -> DBUS_ACS_CNT_CLR_W<'_, CACHE_ACS_CNT_CLR_SPEC> {
        DBUS_ACS_CNT_CLR_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_cnt_clr::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_CNT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_clr::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_CLR to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_CLR_SPEC {}
