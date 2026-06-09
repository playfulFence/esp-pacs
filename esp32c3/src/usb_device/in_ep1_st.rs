#[doc = "Register `IN_EP1_ST` reader"]
pub type R = crate::R<IN_EP1_ST_SPEC>;
#[doc = "Field `IN_EP1_STATE` reader - State of IN Endpoint 1. /"]
pub type IN_EP1_STATE_R = crate::FieldReader;
#[doc = "Field `IN_EP1_WR_ADDR` reader - Write data address of IN endpoint 1. /"]
pub type IN_EP1_WR_ADDR_R = crate::FieldReader;
#[doc = "Field `IN_EP1_RD_ADDR` reader - Read data address of IN endpoint 1. /"]
pub type IN_EP1_RD_ADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - State of IN Endpoint 1. /"]
    #[inline(always)]
    pub fn in_ep1_state(&self) -> IN_EP1_STATE_R {
        IN_EP1_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 2:9 - Write data address of IN endpoint 1. /"]
    #[inline(always)]
    pub fn in_ep1_wr_addr(&self) -> IN_EP1_WR_ADDR_R {
        IN_EP1_WR_ADDR_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - Read data address of IN endpoint 1. /"]
    #[inline(always)]
    pub fn in_ep1_rd_addr(&self) -> IN_EP1_RD_ADDR_R {
        IN_EP1_RD_ADDR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_EP1_ST")
            .field("in_ep1_state", &self.in_ep1_state())
            .field("in_ep1_wr_addr", &self.in_ep1_wr_addr())
            .field("in_ep1_rd_addr", &self.in_ep1_rd_addr())
            .finish()
    }
}
#[doc = "USB_SERIAL_JTAG_IN_EP1_ST_REG. /\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ep1_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_EP1_ST_SPEC;
impl crate::RegisterSpec for IN_EP1_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ep1_st::R`](R) reader structure"]
impl crate::Readable for IN_EP1_ST_SPEC {}
#[doc = "`reset()` method sets IN_EP1_ST to value 0"]
impl crate::Resettable for IN_EP1_ST_SPEC {}
