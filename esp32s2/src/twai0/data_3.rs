#[doc = "Register `DATA_3` reader"]
pub type R = crate::R<DATA_3_SPEC>;
#[doc = "Register `DATA_3` writer"]
pub type W = crate::W<DATA_3_SPEC>;
#[doc = "Field `TX_BYTE_3` reader - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_3_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_3` writer - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte_3(&self) -> TX_BYTE_3_R {
        TX_BYTE_3_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_3")
            .field("tx_byte_3", &self.tx_byte_3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte_3(&mut self) -> TX_BYTE_3_W<DATA_3_SPEC> {
        TX_BYTE_3_W::new(self, 0)
    }
}
#[doc = "Data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_3_SPEC;
impl crate::RegisterSpec for DATA_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_3::R`](R) reader structure"]
impl crate::Readable for DATA_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_3::W`](W) writer structure"]
impl crate::Writable for DATA_3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_3 to value 0"]
impl crate::Resettable for DATA_3_SPEC {}
