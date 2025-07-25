#[doc = "Register `I2C_DATE` reader"]
pub type R = crate::R<I2C_DATE_SPEC>;
#[doc = "Register `I2C_DATE` writer"]
pub type W = crate::W<I2C_DATE_SPEC>;
#[doc = "Field `I2C_DATE` reader - This is the the version register."]
pub type I2C_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `I2C_DATE` writer - This is the the version register."]
pub type I2C_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is the the version register."]
    #[inline(always)]
    pub fn i2c_date(&self) -> I2C_DATE_R {
        I2C_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_DATE")
            .field("i2c_date", &self.i2c_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the the version register."]
    #[inline(always)]
    pub fn i2c_date(&mut self) -> I2C_DATE_W<I2C_DATE_SPEC> {
        I2C_DATE_W::new(self, 0)
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_DATE_SPEC;
impl crate::RegisterSpec for I2C_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_date::R`](R) reader structure"]
impl crate::Readable for I2C_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_date::W`](W) writer structure"]
impl crate::Writable for I2C_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_DATE to value 0x0240_1040"]
impl crate::Resettable for I2C_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0240_1040;
}
