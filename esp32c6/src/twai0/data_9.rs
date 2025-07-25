#[doc = "Register `DATA_9` reader"]
pub type R = crate::R<DATA_9_SPEC>;
#[doc = "Register `DATA_9` writer"]
pub type W = crate::W<DATA_9_SPEC>;
#[doc = "Field `DATA_9` reader - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9."]
pub type DATA_9_R = crate::FieldReader;
#[doc = "Field `DATA_9` writer - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9."]
pub type DATA_9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9."]
    #[inline(always)]
    pub fn data_9(&self) -> DATA_9_R {
        DATA_9_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_9")
            .field("data_9", &self.data_9())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 9 and when software initiate read operation, it is rx data register 9."]
    #[inline(always)]
    pub fn data_9(&mut self) -> DATA_9_W<DATA_9_SPEC> {
        DATA_9_W::new(self, 0)
    }
}
#[doc = "Data register 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_9_SPEC;
impl crate::RegisterSpec for DATA_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_9::R`](R) reader structure"]
impl crate::Readable for DATA_9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_9::W`](W) writer structure"]
impl crate::Writable for DATA_9_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_9 to value 0"]
impl crate::Resettable for DATA_9_SPEC {}
