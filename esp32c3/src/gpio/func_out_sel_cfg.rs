#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Field `OUT_SEL` reader - "]
pub type OUT_SEL_R = crate::FieldReader;
#[doc = "Field `OUT_SEL` writer - "]
pub type OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INV_SEL` reader - "]
pub type INV_SEL_R = crate::BitReader;
#[doc = "Field `INV_SEL` writer - "]
pub type INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN_SEL` reader - "]
pub type OEN_SEL_R = crate::BitReader;
#[doc = "Field `OEN_SEL` writer - "]
pub type OEN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN_INV_SEL` reader - "]
pub type OEN_INV_SEL_R = crate::BitReader;
#[doc = "Field `OEN_INV_SEL` writer - "]
pub type OEN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inv_sel(&self) -> INV_SEL_R {
        INV_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn oen_sel(&self) -> OEN_SEL_R {
        OEN_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oen_inv_sel(&self) -> OEN_INV_SEL_R {
        OEN_INV_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_OUT_SEL_CFG")
            .field("oen_inv_sel", &self.oen_inv_sel())
            .field("oen_sel", &self.oen_sel())
            .field("inv_sel", &self.inv_sel())
            .field("out_sel", &self.out_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        OUT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inv_sel(&mut self) -> INV_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        INV_SEL_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn oen_sel(&mut self) -> OEN_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        OEN_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oen_inv_sel(&mut self) -> OEN_INV_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        OEN_INV_SEL_W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`func_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {}
