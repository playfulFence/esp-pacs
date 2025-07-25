#[doc = "Register `BLK3_WDATA0` reader"]
pub type R = crate::R<BLK3_WDATA0_SPEC>;
#[doc = "Register `BLK3_WDATA0` writer"]
pub type W = crate::W<BLK3_WDATA0_SPEC>;
#[doc = "Field `BLK3_DIN0` reader - "]
pub type BLK3_DIN0_R = crate::FieldReader<u32>;
#[doc = "Field `BLK3_DIN0` writer - "]
pub type BLK3_DIN0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk3_din0(&self) -> BLK3_DIN0_R {
        BLK3_DIN0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_WDATA0")
            .field("blk3_din0", &self.blk3_din0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk3_din0(&mut self) -> BLK3_DIN0_W<BLK3_WDATA0_SPEC> {
        BLK3_DIN0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`blk3_wdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk3_wdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_WDATA0_SPEC;
impl crate::RegisterSpec for BLK3_WDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_wdata0::R`](R) reader structure"]
impl crate::Readable for BLK3_WDATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk3_wdata0::W`](W) writer structure"]
impl crate::Writable for BLK3_WDATA0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK3_WDATA0 to value 0"]
impl crate::Resettable for BLK3_WDATA0_SPEC {}
