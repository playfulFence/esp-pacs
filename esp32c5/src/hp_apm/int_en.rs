#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `M_APM(0-4)` reader - Configures to enable APM M%s interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type M_APM_R = crate::BitReader;
#[doc = "Field `M_APM(0-4)` writer - Configures to enable APM M%s interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type M_APM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Configures to enable APM M(0-4) interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `M0_APM` field.</div>"]
    #[inline(always)]
    pub fn m_apm(&self, n: u8) -> M_APM_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        M_APM_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures to enable APM M(0-4) interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m_apm_iter(&self) -> impl Iterator<Item = M_APM_R> + '_ {
        (0..5).map(move |n| M_APM_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Configures to enable APM M0 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m0_apm(&self) -> M_APM_R {
        M_APM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures to enable APM M1 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m1_apm(&self) -> M_APM_R {
        M_APM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures to enable APM M2 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m2_apm(&self) -> M_APM_R {
        M_APM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures to enable APM M3 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m3_apm(&self) -> M_APM_R {
        M_APM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures to enable APM M4 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m4_apm(&self) -> M_APM_R {
        M_APM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_EN")
            .field("m0_apm", &self.m0_apm())
            .field("m1_apm", &self.m1_apm())
            .field("m2_apm", &self.m2_apm())
            .field("m3_apm", &self.m3_apm())
            .field("m4_apm", &self.m4_apm())
            .finish()
    }
}
impl W {
    #[doc = "Configures to enable APM M(0-4) interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `M0_APM` field.</div>"]
    #[inline(always)]
    pub fn m_apm(&mut self, n: u8) -> M_APM_W<INT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        M_APM_W::new(self, n)
    }
    #[doc = "Bit 0 - Configures to enable APM M0 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m0_apm(&mut self) -> M_APM_W<INT_EN_SPEC> {
        M_APM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures to enable APM M1 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m1_apm(&mut self) -> M_APM_W<INT_EN_SPEC> {
        M_APM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures to enable APM M2 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m2_apm(&mut self) -> M_APM_W<INT_EN_SPEC> {
        M_APM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures to enable APM M3 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m3_apm(&mut self) -> M_APM_W<INT_EN_SPEC> {
        M_APM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures to enable APM M4 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m4_apm(&mut self) -> M_APM_W<INT_EN_SPEC> {
        M_APM_W::new(self, 4)
    }
}
#[doc = "APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {}
