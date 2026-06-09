#[doc = "Register `FLASH_ACE0_ADDR` reader"]
pub type R = crate::R<FLASH_ACE0_ADDR_SPEC>;
#[doc = "Register `FLASH_ACE0_ADDR` writer"]
pub type W = crate::W<FLASH_ACE0_ADDR_SPEC>;
#[doc = "Field `FLASH_ACE0_ADDR_S` reader - "]
pub type FLASH_ACE0_ADDR_S_R = crate::FieldReader<u32>;
#[doc = "Field `FLASH_ACE0_ADDR_S` writer - "]
pub type FLASH_ACE0_ADDR_S_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn flash_ace0_addr_s(&self) -> FLASH_ACE0_ADDR_S_R {
        FLASH_ACE0_ADDR_S_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE0_ADDR")
            .field("flash_ace0_addr_s", &self.flash_ace0_addr_s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn flash_ace0_addr_s(&mut self) -> FLASH_ACE0_ADDR_S_W<'_, FLASH_ACE0_ADDR_SPEC> {
        FLASH_ACE0_ADDR_S_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ace0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ace0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACE0_ADDR_SPEC;
impl crate::RegisterSpec for FLASH_ACE0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ace0_addr::R`](R) reader structure"]
impl crate::Readable for FLASH_ACE0_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ace0_addr::W`](W) writer structure"]
impl crate::Writable for FLASH_ACE0_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_ACE0_ADDR to value 0"]
impl crate::Resettable for FLASH_ACE0_ADDR_SPEC {}
