#[doc = "Register `INF3_EXTEND_ADDR0` reader"]
pub type R = crate::R<INF3_EXTEND_ADDR0_SPEC>;
#[doc = "Register `INF3_EXTEND_ADDR0` writer"]
pub type W = crate::W<INF3_EXTEND_ADDR0_SPEC>;
#[doc = "Field `MAC_INF3_EXTEND_ADDR0` reader - "]
pub type MAC_INF3_EXTEND_ADDR0_R = crate::FieldReader<u32>;
#[doc = "Field `MAC_INF3_EXTEND_ADDR0` writer - "]
pub type MAC_INF3_EXTEND_ADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac_inf3_extend_addr0(&self) -> MAC_INF3_EXTEND_ADDR0_R {
        MAC_INF3_EXTEND_ADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF3_EXTEND_ADDR0")
            .field("mac_inf3_extend_addr0", &self.mac_inf3_extend_addr0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac_inf3_extend_addr0(&mut self) -> MAC_INF3_EXTEND_ADDR0_W<INF3_EXTEND_ADDR0_SPEC> {
        MAC_INF3_EXTEND_ADDR0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`inf3_extend_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inf3_extend_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INF3_EXTEND_ADDR0_SPEC;
impl crate::RegisterSpec for INF3_EXTEND_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inf3_extend_addr0::R`](R) reader structure"]
impl crate::Readable for INF3_EXTEND_ADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inf3_extend_addr0::W`](W) writer structure"]
impl crate::Writable for INF3_EXTEND_ADDR0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INF3_EXTEND_ADDR0 to value 0"]
impl crate::Resettable for INF3_EXTEND_ADDR0_SPEC {}
