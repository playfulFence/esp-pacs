#[doc = "Register `ICACHE_FREEZE` reader"]
pub type R = crate::R<ICACHE_FREEZE_SPEC>;
#[doc = "Register `ICACHE_FREEZE` writer"]
pub type W = crate::W<ICACHE_FREEZE_SPEC>;
#[doc = "Field `ICACHE_FREEZE_ENA` reader - The bit is used to enable icache freeze mode"]
pub type ICACHE_FREEZE_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_FREEZE_ENA` writer - The bit is used to enable icache freeze mode"]
pub type ICACHE_FREEZE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_FREEZE_MODE` reader - The bit is used to configure freeze mode 0: assert busy if"]
pub type ICACHE_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE_FREEZE_MODE` writer - The bit is used to configure freeze mode 0: assert busy if"]
pub type ICACHE_FREEZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_FREEZE_DONE` reader - The bit is used to indicate icache freeze success"]
pub type ICACHE_FREEZE_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable icache freeze mode"]
    #[inline(always)]
    pub fn icache_freeze_ena(&self) -> ICACHE_FREEZE_ENA_R {
        ICACHE_FREEZE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure freeze mode 0: assert busy if"]
    #[inline(always)]
    pub fn icache_freeze_mode(&self) -> ICACHE_FREEZE_MODE_R {
        ICACHE_FREEZE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate icache freeze success"]
    #[inline(always)]
    pub fn icache_freeze_done(&self) -> ICACHE_FREEZE_DONE_R {
        ICACHE_FREEZE_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_FREEZE")
            .field("icache_freeze_done", &self.icache_freeze_done())
            .field("icache_freeze_mode", &self.icache_freeze_mode())
            .field("icache_freeze_ena", &self.icache_freeze_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable icache freeze mode"]
    #[inline(always)]
    pub fn icache_freeze_ena(&mut self) -> ICACHE_FREEZE_ENA_W<'_, ICACHE_FREEZE_SPEC> {
        ICACHE_FREEZE_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to configure freeze mode 0: assert busy if"]
    #[inline(always)]
    pub fn icache_freeze_mode(&mut self) -> ICACHE_FREEZE_MODE_W<'_, ICACHE_FREEZE_SPEC> {
        ICACHE_FREEZE_MODE_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_freeze::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_freeze::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_FREEZE_SPEC;
impl crate::RegisterSpec for ICACHE_FREEZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_freeze::R`](R) reader structure"]
impl crate::Readable for ICACHE_FREEZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache_freeze::W`](W) writer structure"]
impl crate::Writable for ICACHE_FREEZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICACHE_FREEZE to value 0"]
impl crate::Resettable for ICACHE_FREEZE_SPEC {}
