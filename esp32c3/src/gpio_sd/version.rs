#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VERSION_SPEC>;
#[doc = "Register `VERSION` writer"]
pub type W = crate::W<VERSION_SPEC>;
#[doc = "Field `GPIO_SD_DATE` reader - Version control register."]
pub type GPIO_SD_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_SD_DATE` writer - Version control register."]
pub type GPIO_SD_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn gpio_sd_date(&self) -> GPIO_SD_DATE_R {
        GPIO_SD_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("gpio_sd_date", &self.gpio_sd_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn gpio_sd_date(&mut self) -> GPIO_SD_DATE_W<VERSION_SPEC> {
        GPIO_SD_DATE_W::new(self, 0)
    }
}
#[doc = "Version Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`version::W`](W) writer structure"]
impl crate::Writable for VERSION_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VERSION to value 0x0200_6230"]
impl crate::Resettable for VERSION_SPEC {
    const RESET_VALUE: u32 = 0x0200_6230;
}
