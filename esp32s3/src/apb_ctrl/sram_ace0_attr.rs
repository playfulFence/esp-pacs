#[doc = "Register `SRAM_ACE0_ATTR` reader"]
pub type R = crate::R<SRAM_ACE0_ATTR_SPEC>;
#[doc = "Register `SRAM_ACE0_ATTR` writer"]
pub type W = crate::W<SRAM_ACE0_ATTR_SPEC>;
#[doc = "Field `SRAM_ACE0_ATTR` reader - ******* Description ***********"]
pub type SRAM_ACE0_ATTR_R = crate::FieldReader<u16>;
#[doc = "Field `SRAM_ACE0_ATTR` writer - ******* Description ***********"]
pub type SRAM_ACE0_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_ace0_attr(&self) -> SRAM_ACE0_ATTR_R {
        SRAM_ACE0_ATTR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_ACE0_ATTR")
            .field("sram_ace0_attr", &self.sram_ace0_attr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_ace0_attr(&mut self) -> SRAM_ACE0_ATTR_W<SRAM_ACE0_ATTR_SPEC> {
        SRAM_ACE0_ATTR_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ace0_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ace0_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ACE0_ATTR_SPEC;
impl crate::RegisterSpec for SRAM_ACE0_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ace0_attr::R`](R) reader structure"]
impl crate::Readable for SRAM_ACE0_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ace0_attr::W`](W) writer structure"]
impl crate::Writable for SRAM_ACE0_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_ACE0_ATTR to value 0xff"]
impl crate::Resettable for SRAM_ACE0_ATTR_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
