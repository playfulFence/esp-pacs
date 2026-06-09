#[doc = "Register `THRES_CTRL` reader"]
pub type R = crate::R<THRES_CTRL_SPEC>;
#[doc = "Register `THRES_CTRL` writer"]
pub type W = crate::W<THRES_CTRL_SPEC>;
#[doc = "Field `THRES1_EN` reader - "]
pub type THRES1_EN_R = crate::BitReader;
#[doc = "Field `THRES1_EN` writer - "]
pub type THRES1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_EN` reader - "]
pub type THRES0_EN_R = crate::BitReader;
#[doc = "Field `THRES0_EN` writer - "]
pub type THRES0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn thres1_en(&self) -> THRES1_EN_R {
        THRES1_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn thres0_en(&self) -> THRES0_EN_R {
        THRES0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES_CTRL")
            .field("thres0_en", &self.thres0_en())
            .field("thres1_en", &self.thres1_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn thres1_en(&mut self) -> THRES1_EN_W<'_, THRES_CTRL_SPEC> {
        THRES1_EN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn thres0_en(&mut self) -> THRES0_EN_W<'_, THRES_CTRL_SPEC> {
        THRES0_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres_ctrl::R`](R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres_ctrl::W`](W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {}
