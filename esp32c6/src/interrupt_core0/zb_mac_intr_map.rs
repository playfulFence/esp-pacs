#[doc = "Register `ZB_MAC_INTR_MAP` reader"]
pub type R = crate::R<ZB_MAC_INTR_MAP_SPEC>;
#[doc = "Register `ZB_MAC_INTR_MAP` writer"]
pub type W = crate::W<ZB_MAC_INTR_MAP_SPEC>;
#[doc = "Field `ZB_MAC_INTR_MAP` reader - Need add description"]
pub type ZB_MAC_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `ZB_MAC_INTR_MAP` writer - Need add description"]
pub type ZB_MAC_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn zb_mac_intr_map(&self) -> ZB_MAC_INTR_MAP_R {
        ZB_MAC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZB_MAC_INTR_MAP")
            .field("zb_mac_intr_map", &self.zb_mac_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn zb_mac_intr_map(&mut self) -> ZB_MAC_INTR_MAP_W<ZB_MAC_INTR_MAP_SPEC> {
        ZB_MAC_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`zb_mac_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zb_mac_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZB_MAC_INTR_MAP_SPEC;
impl crate::RegisterSpec for ZB_MAC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zb_mac_intr_map::R`](R) reader structure"]
impl crate::Readable for ZB_MAC_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`zb_mac_intr_map::W`](W) writer structure"]
impl crate::Writable for ZB_MAC_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ZB_MAC_INTR_MAP to value 0"]
impl crate::Resettable for ZB_MAC_INTR_MAP_SPEC {}
