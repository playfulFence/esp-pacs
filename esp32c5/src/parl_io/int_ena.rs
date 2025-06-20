#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `TX_FIFO_REMPTY_INT_ENA` reader - Write 1 to enable TX_FIFO_REMPTY_INT."]
pub type TX_FIFO_REMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_FIFO_REMPTY_INT_ENA` writer - Write 1 to enable TX_FIFO_REMPTY_INT."]
pub type TX_FIFO_REMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_WOVF_INT_ENA` reader - Write 1 to enable RX_FIFO_WOVF_INT."]
pub type RX_FIFO_WOVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_FIFO_WOVF_INT_ENA` writer - Write 1 to enable RX_FIFO_WOVF_INT."]
pub type RX_FIFO_WOVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EOF_INT_ENA` reader - Write 1 to enable TX_EOF_INT."]
pub type TX_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_EOF_INT_ENA` writer - Write 1 to enable TX_EOF_INT."]
pub type TX_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    pub fn tx_fifo_rempty_int_ena(&self) -> TX_FIFO_REMPTY_INT_ENA_R {
        TX_FIFO_REMPTY_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable RX_FIFO_WOVF_INT."]
    #[inline(always)]
    pub fn rx_fifo_wovf_int_ena(&self) -> RX_FIFO_WOVF_INT_ENA_R {
        RX_FIFO_WOVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable TX_EOF_INT."]
    #[inline(always)]
    pub fn tx_eof_int_ena(&self) -> TX_EOF_INT_ENA_R {
        TX_EOF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("tx_fifo_rempty_int_ena", &self.tx_fifo_rempty_int_ena())
            .field("rx_fifo_wovf_int_ena", &self.rx_fifo_wovf_int_ena())
            .field("tx_eof_int_ena", &self.tx_eof_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    pub fn tx_fifo_rempty_int_ena(&mut self) -> TX_FIFO_REMPTY_INT_ENA_W<INT_ENA_SPEC> {
        TX_FIFO_REMPTY_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable RX_FIFO_WOVF_INT."]
    #[inline(always)]
    pub fn rx_fifo_wovf_int_ena(&mut self) -> RX_FIFO_WOVF_INT_ENA_W<INT_ENA_SPEC> {
        RX_FIFO_WOVF_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable TX_EOF_INT."]
    #[inline(always)]
    pub fn tx_eof_int_ena(&mut self) -> TX_EOF_INT_ENA_W<INT_ENA_SPEC> {
        TX_EOF_INT_ENA_W::new(self, 2)
    }
}
#[doc = "Parallel IO interrupt enable singal configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
