#[doc = "Register `HP_SLEEP_LP_REGULATOR1` reader"]
pub type R = crate::R<HP_SLEEP_LP_REGULATOR1_SPEC>;
#[doc = "Register `HP_SLEEP_LP_REGULATOR1` writer"]
pub type W = crate::W<HP_SLEEP_LP_REGULATOR1_SPEC>;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_DRV_B` reader - need_des"]
pub type HP_SLEEP_LP_REGULATOR_DRV_B_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_DRV_B` writer - need_des"]
pub type HP_SLEEP_LP_REGULATOR_DRV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_drv_b(&self) -> HP_SLEEP_LP_REGULATOR_DRV_B_R {
        HP_SLEEP_LP_REGULATOR_DRV_B_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_LP_REGULATOR1")
            .field(
                "hp_sleep_lp_regulator_drv_b",
                &self.hp_sleep_lp_regulator_drv_b(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_drv_b(
        &mut self,
    ) -> HP_SLEEP_LP_REGULATOR_DRV_B_W<HP_SLEEP_LP_REGULATOR1_SPEC> {
        HP_SLEEP_LP_REGULATOR_DRV_B_W::new(self, 26)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_regulator1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_regulator1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_SLEEP_LP_REGULATOR1_SPEC;
impl crate::RegisterSpec for HP_SLEEP_LP_REGULATOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_lp_regulator1::R`](R) reader structure"]
impl crate::Readable for HP_SLEEP_LP_REGULATOR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_lp_regulator1::W`](W) writer structure"]
impl crate::Writable for HP_SLEEP_LP_REGULATOR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_SLEEP_LP_REGULATOR1 to value 0"]
impl crate::Resettable for HP_SLEEP_LP_REGULATOR1_SPEC {}
