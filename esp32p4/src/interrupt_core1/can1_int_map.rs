#[doc = "Register `CAN1_INT_MAP` reader"]
pub type R = crate::R<CAN1_INT_MAP_SPEC>;
#[doc = "Register `CAN1_INT_MAP` writer"]
pub type W = crate::W<CAN1_INT_MAP_SPEC>;
#[doc = "Field `CORE1_CAN1_INT_MAP` reader - NA"]
pub type CORE1_CAN1_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CORE1_CAN1_INT_MAP` writer - NA"]
pub type CORE1_CAN1_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core1_can1_int_map(&self) -> CORE1_CAN1_INT_MAP_R {
        CORE1_CAN1_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN1_INT_MAP")
            .field("core1_can1_int_map", &self.core1_can1_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core1_can1_int_map(&mut self) -> CORE1_CAN1_INT_MAP_W<CAN1_INT_MAP_SPEC> {
        CORE1_CAN1_INT_MAP_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`can1_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can1_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN1_INT_MAP_SPEC;
impl crate::RegisterSpec for CAN1_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can1_int_map::R`](R) reader structure"]
impl crate::Readable for CAN1_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can1_int_map::W`](W) writer structure"]
impl crate::Writable for CAN1_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN1_INT_MAP to value 0"]
impl crate::Resettable for CAN1_INT_MAP_SPEC {}
