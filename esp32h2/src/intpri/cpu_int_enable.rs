#[doc = "Register `CPU_INT_ENABLE` reader"]
pub type R = crate::R<CPU_INT_ENABLE_SPEC>;
#[doc = "Register `CPU_INT_ENABLE` writer"]
pub type W = crate::W<CPU_INT_ENABLE_SPEC>;
#[doc = "Field `CPU_INT_ENABLE` reader - Need add description"]
pub type CPU_INT_ENABLE_R = crate::FieldReader<u32>;
#[doc = "Field `CPU_INT_ENABLE` writer - Need add description"]
pub type CPU_INT_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn cpu_int_enable(&self) -> CPU_INT_ENABLE_R {
        CPU_INT_ENABLE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_ENABLE")
            .field("cpu_int_enable", &self.cpu_int_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn cpu_int_enable(&mut self) -> CPU_INT_ENABLE_W<CPU_INT_ENABLE_SPEC> {
        CPU_INT_ENABLE_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INT_ENABLE_SPEC;
impl crate::RegisterSpec for CPU_INT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_int_enable::R`](R) reader structure"]
impl crate::Readable for CPU_INT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_int_enable::W`](W) writer structure"]
impl crate::Writable for CPU_INT_ENABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_INT_ENABLE to value 0"]
impl crate::Resettable for CPU_INT_ENABLE_SPEC {}
