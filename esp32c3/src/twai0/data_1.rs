#[doc = "Register `DATA_1` reader"]
pub type R = crate::R<DATA_1_SPEC>;
#[doc = "Register `DATA_1` writer"]
pub type W = crate::W<DATA_1_SPEC>;
#[doc = "Field `TX_BYTE_1` reader - In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, it stores the 1st byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_1_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_1` writer - In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, it stores the 1st byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, it stores the 1st byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_1(&self) -> TX_BYTE_1_R {
        TX_BYTE_1_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_1")
            .field("tx_byte_1", &self.tx_byte_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, it stores the 1st byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_1(&mut self) -> TX_BYTE_1_W<DATA_1_SPEC> {
        TX_BYTE_1_W::new(self, 0)
    }
}
#[doc = "Data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`data_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_1_SPEC;
impl crate::RegisterSpec for DATA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_1::R`](R) reader structure"]
impl crate::Readable for DATA_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_1::W`](W) writer structure"]
impl crate::Writable for DATA_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_1 to value 0"]
impl crate::Resettable for DATA_1_SPEC {}
