#[doc = "Register `DATA_12` reader"]
pub type R = crate::R<DATA_12_SPEC>;
#[doc = "Register `DATA_12` writer"]
pub type W = crate::W<DATA_12_SPEC>;
#[doc = "Field `DATA_12` reader - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12."]
pub type DATA_12_R = crate::FieldReader;
#[doc = "Field `DATA_12` writer - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12."]
pub type DATA_12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12."]
    #[inline(always)]
    pub fn data_12(&self) -> DATA_12_R {
        DATA_12_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_12")
            .field("data_12", &self.data_12())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12."]
    #[inline(always)]
    pub fn data_12(&mut self) -> DATA_12_W<DATA_12_SPEC> {
        DATA_12_W::new(self, 0)
    }
}
#[doc = "Data register 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_12_SPEC;
impl crate::RegisterSpec for DATA_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_12::R`](R) reader structure"]
impl crate::Readable for DATA_12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_12::W`](W) writer structure"]
impl crate::Writable for DATA_12_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_12 to value 0"]
impl crate::Resettable for DATA_12_SPEC {}
