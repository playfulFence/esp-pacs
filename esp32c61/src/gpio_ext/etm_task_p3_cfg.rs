#[doc = "Register `ETM_TASK_P3_CFG` reader"]
pub type R = crate::R<ETM_TASK_P3_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P3_CFG` writer"]
pub type W = crate::W<ETM_TASK_P3_CFG_SPEC>;
#[doc = "Field `GPIO_SEL(15-19)` reader - GPIO choose a etm task channel."]
pub type GPIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SEL(15-19)` writer - GPIO choose a etm task channel."]
pub type GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EN(15-19)` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EN(15-19)` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO15_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GPIO_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GPIO_SEL_R> + '_ {
        (0..5).map(move |n| GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio15_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio16_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio17_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 18:20 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio18_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:26 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio19_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO15_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GPIO_EN_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GPIO_EN_R> + '_ {
        (0..5).map(move |n| GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio15_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio16_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio17_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio18_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio19_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P3_CFG")
            .field("gpio15_sel", &self.gpio15_sel())
            .field("gpio16_sel", &self.gpio16_sel())
            .field("gpio17_sel", &self.gpio17_sel())
            .field("gpio18_sel", &self.gpio18_sel())
            .field("gpio19_sel", &self.gpio19_sel())
            .field("gpio15_en", &self.gpio15_en())
            .field("gpio16_en", &self.gpio16_en())
            .field("gpio17_en", &self.gpio17_en())
            .field("gpio18_en", &self.gpio18_en())
            .field("gpio19_en", &self.gpio19_en())
            .finish()
    }
}
impl W {
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO15_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GPIO_SEL_W<ETM_TASK_P3_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_W::new(self, n * 6)
    }
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio15_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_SEL_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio16_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_SEL_W::new(self, 6)
    }
    #[doc = "Bits 12:14 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio17_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_SEL_W::new(self, 12)
    }
    #[doc = "Bits 18:20 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio18_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_SEL_W::new(self, 18)
    }
    #[doc = "Bits 24:26 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio19_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_SEL_W::new(self, 24)
    }
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO15_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GPIO_EN_W<ETM_TASK_P3_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_W::new(self, n * 6 + 5)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio15_en(&mut self) -> GPIO_EN_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio16_en(&mut self) -> GPIO_EN_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_EN_W::new(self, 11)
    }
    #[doc = "Bit 17 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio17_en(&mut self) -> GPIO_EN_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_EN_W::new(self, 17)
    }
    #[doc = "Bit 23 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio18_en(&mut self) -> GPIO_EN_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_EN_W::new(self, 23)
    }
    #[doc = "Bit 29 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio19_en(&mut self) -> GPIO_EN_W<ETM_TASK_P3_CFG_SPEC> {
        GPIO_EN_W::new(self, 29)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p3_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p3_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P3_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P3_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p3_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P3_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p3_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P3_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P3_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P3_CFG_SPEC {}
