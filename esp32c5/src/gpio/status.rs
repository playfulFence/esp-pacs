#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `INTERRUPT` reader - The interrupt status of GPIO0 ~ GPIO31, can be configured by the software. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
pub type INTERRUPT_R = crate::FieldReader<u32>;
#[doc = "Field `INTERRUPT` writer - The interrupt status of GPIO0 ~ GPIO31, can be configured by the software. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
pub type INTERRUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The interrupt status of GPIO0 ~ GPIO31, can be configured by the software. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("interrupt", &self.interrupt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The interrupt status of GPIO0 ~ GPIO31, can be configured by the software. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W<STATUS_SPEC> {
        INTERRUPT_W::new(self, 0)
    }
}
#[doc = "GPIO interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {}
