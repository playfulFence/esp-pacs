#[doc = "Register `UART_RXD_CNT` reader"]
pub struct R(crate::R<UART_RXD_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RXD_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RXD_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RXD_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rxd_edge_cnt` reader - used in baudrate detect"]
pub type RXD_EDGE_CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - used in baudrate detect"]
    #[inline(always)]
    pub fn rxd_edge_cnt(&self) -> RXD_EDGE_CNT_R {
        RXD_EDGE_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "UART_RXD_CNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rxd_cnt](index.html) module"]
pub struct UART_RXD_CNT_SPEC;
impl crate::RegisterSpec for UART_RXD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rxd_cnt::R](R) reader structure"]
impl crate::Readable for UART_RXD_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_RXD_CNT to value 0"]
impl crate::Resettable for UART_RXD_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}