#[doc = "Register `I2C_MST_INT_MAP` reader"]
pub type R = crate::R<I2C_MST_INT_MAP_SPEC>;
#[doc = "Register `I2C_MST_INT_MAP` writer"]
pub type W = crate::W<I2C_MST_INT_MAP_SPEC>;
#[doc = "Field `I2C_MST_INT_MAP` reader - reg_core0_i2c_mst_int_map"]
pub type I2C_MST_INT_MAP_R = crate::FieldReader;
#[doc = "Field `I2C_MST_INT_MAP` writer - reg_core0_i2c_mst_int_map"]
pub type I2C_MST_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_i2c_mst_int_map"]
    #[inline(always)]
    pub fn i2c_mst_int_map(&self) -> I2C_MST_INT_MAP_R {
        I2C_MST_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_MST_INT_MAP")
            .field("i2c_mst_int_map", &self.i2c_mst_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_i2c_mst_int_map"]
    #[inline(always)]
    pub fn i2c_mst_int_map(&mut self) -> I2C_MST_INT_MAP_W<I2C_MST_INT_MAP_SPEC> {
        I2C_MST_INT_MAP_W::new(self, 0)
    }
}
#[doc = "i2c intr map register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_mst_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_mst_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_MST_INT_MAP_SPEC;
impl crate::RegisterSpec for I2C_MST_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_mst_int_map::R`](R) reader structure"]
impl crate::Readable for I2C_MST_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_mst_int_map::W`](W) writer structure"]
impl crate::Writable for I2C_MST_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_MST_INT_MAP to value 0"]
impl crate::Resettable for I2C_MST_INT_MAP_SPEC {}
