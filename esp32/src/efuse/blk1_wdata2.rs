#[doc = "Register `BLK1_WDATA2` reader"]
pub type R = crate::R<BLK1_WDATA2_SPEC>;
#[doc = "Register `BLK1_WDATA2` writer"]
pub type W = crate::W<BLK1_WDATA2_SPEC>;
#[doc = "Field `BLK1_DIN2` reader - "]
pub type BLK1_DIN2_R = crate::FieldReader<u32>;
#[doc = "Field `BLK1_DIN2` writer - "]
pub type BLK1_DIN2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk1_din2(&self) -> BLK1_DIN2_R {
        BLK1_DIN2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_WDATA2")
            .field("blk1_din2", &self.blk1_din2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk1_din2(&mut self) -> BLK1_DIN2_W<BLK1_WDATA2_SPEC> {
        BLK1_DIN2_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`blk1_wdata2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk1_wdata2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK1_WDATA2_SPEC;
impl crate::RegisterSpec for BLK1_WDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk1_wdata2::R`](R) reader structure"]
impl crate::Readable for BLK1_WDATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk1_wdata2::W`](W) writer structure"]
impl crate::Writable for BLK1_WDATA2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK1_WDATA2 to value 0"]
impl crate::Resettable for BLK1_WDATA2_SPEC {}
