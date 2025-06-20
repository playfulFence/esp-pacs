#[doc = "Register `M%s_MODE_CTRL` reader"]
pub type R = crate::R<M_MODE_CTRL_SPEC>;
#[doc = "Register `M%s_MODE_CTRL` writer"]
pub type W = crate::W<M_MODE_CTRL_SPEC>;
#[doc = "Field `M_MODE` reader - Configures M%s security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
pub type M_MODE_R = crate::FieldReader;
#[doc = "Field `M_MODE` writer - Configures M%s security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
pub type M_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M_LOCK` reader - Set 1 to lock m0 tee configuration"]
pub type M_LOCK_R = crate::BitReader;
#[doc = "Field `M_LOCK` writer - Set 1 to lock m0 tee configuration"]
pub type M_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures M%s security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
    #[inline(always)]
    pub fn m_mode(&self) -> M_MODE_R {
        M_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set 1 to lock m0 tee configuration"]
    #[inline(always)]
    pub fn m_lock(&self) -> M_LOCK_R {
        M_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M_MODE_CTRL")
            .field("m_mode", &self.m_mode())
            .field("m_lock", &self.m_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures M%s security level mode.\\\\ 0: tee_mode \\\\ 1: ree_mode0 \\\\ 2: ree_mode1 \\\\ 3: ree_mode2 \\\\"]
    #[inline(always)]
    pub fn m_mode(&mut self) -> M_MODE_W<M_MODE_CTRL_SPEC> {
        M_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set 1 to lock m0 tee configuration"]
    #[inline(always)]
    pub fn m_lock(&mut self) -> M_LOCK_W<M_MODE_CTRL_SPEC> {
        M_LOCK_W::new(self, 2)
    }
}
#[doc = "TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m_mode_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_mode_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M_MODE_CTRL_SPEC;
impl crate::RegisterSpec for M_MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for M_MODE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for M_MODE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M%s_MODE_CTRL to value 0"]
impl crate::Resettable for M_MODE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
