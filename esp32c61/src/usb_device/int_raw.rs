#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `JTAG_IN_FLUSH_INT_RAW` reader - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
pub type JTAG_IN_FLUSH_INT_RAW_R = crate::BitReader;
#[doc = "Field `JTAG_IN_FLUSH_INT_RAW` writer - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
pub type JTAG_IN_FLUSH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_INT_RAW` reader - The raw interrupt bit turns to high level when SOF frame is received."]
pub type SOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SOF_INT_RAW` writer - The raw interrupt bit turns to high level when SOF frame is received."]
pub type SOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_RAW` reader - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
pub type SERIAL_OUT_RECV_PKT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_RAW` writer - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
pub type SERIAL_OUT_RECV_PKT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_IN_EMPTY_INT_RAW` reader - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
pub type SERIAL_IN_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_EMPTY_INT_RAW` writer - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
pub type SERIAL_IN_EMPTY_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when pid error is detected."]
pub type PID_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `PID_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when pid error is detected."]
pub type PID_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC5_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when CRC5 error is detected."]
pub type CRC5_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CRC5_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when CRC5 error is detected."]
pub type CRC5_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when CRC16 error is detected."]
pub type CRC16_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CRC16_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when CRC16 error is detected."]
pub type CRC16_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUFF_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when stuff error is detected."]
pub type STUFF_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `STUFF_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when stuff error is detected."]
pub type STUFF_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_RAW` reader - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
pub type IN_TOKEN_REC_IN_EP1_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_RAW` writer - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
pub type IN_TOKEN_REC_IN_EP1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_BUS_RESET_INT_RAW` reader - The raw interrupt bit turns to high level when usb bus reset is detected."]
pub type USB_BUS_RESET_INT_RAW_R = crate::BitReader;
#[doc = "Field `USB_BUS_RESET_INT_RAW` writer - The raw interrupt bit turns to high level when usb bus reset is detected."]
pub type USB_BUS_RESET_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_RAW` reader - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_RAW` writer - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_RAW` reader - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_RAW` writer - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_CHG_INT_RAW` reader - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed."]
pub type RTS_CHG_INT_RAW_R = crate::BitReader;
#[doc = "Field `RTS_CHG_INT_RAW` writer - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed."]
pub type RTS_CHG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR_CHG_INT_RAW` reader - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed."]
pub type DTR_CHG_INT_RAW_R = crate::BitReader;
#[doc = "Field `DTR_CHG_INT_RAW` writer - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed."]
pub type DTR_CHG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GET_LINE_CODE_INT_RAW` reader - The raw interrupt bit turns to high level when level of GET LINE CODING request is received."]
pub type GET_LINE_CODE_INT_RAW_R = crate::BitReader;
#[doc = "Field `GET_LINE_CODE_INT_RAW` writer - The raw interrupt bit turns to high level when level of GET LINE CODING request is received."]
pub type GET_LINE_CODE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_LINE_CODE_INT_RAW` reader - The raw interrupt bit turns to high level when level of SET LINE CODING request is received."]
pub type SET_LINE_CODE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SET_LINE_CODE_INT_RAW` writer - The raw interrupt bit turns to high level when level of SET LINE CODING request is received."]
pub type SET_LINE_CODE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
    #[inline(always)]
    pub fn jtag_in_flush_int_raw(&self) -> JTAG_IN_FLUSH_INT_RAW_R {
        JTAG_IN_FLUSH_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when SOF frame is received."]
    #[inline(always)]
    pub fn sof_int_raw(&self) -> SOF_INT_RAW_R {
        SOF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_raw(&self) -> SERIAL_OUT_RECV_PKT_INT_RAW_R {
        SERIAL_OUT_RECV_PKT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
    #[inline(always)]
    pub fn serial_in_empty_int_raw(&self) -> SERIAL_IN_EMPTY_INT_RAW_R {
        SERIAL_IN_EMPTY_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when pid error is detected."]
    #[inline(always)]
    pub fn pid_err_int_raw(&self) -> PID_ERR_INT_RAW_R {
        PID_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when CRC5 error is detected."]
    #[inline(always)]
    pub fn crc5_err_int_raw(&self) -> CRC5_ERR_INT_RAW_R {
        CRC5_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when CRC16 error is detected."]
    #[inline(always)]
    pub fn crc16_err_int_raw(&self) -> CRC16_ERR_INT_RAW_R {
        CRC16_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when stuff error is detected."]
    #[inline(always)]
    pub fn stuff_err_int_raw(&self) -> STUFF_ERR_INT_RAW_R {
        STUFF_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_raw(&self) -> IN_TOKEN_REC_IN_EP1_INT_RAW_R {
        IN_TOKEN_REC_IN_EP1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when usb bus reset is detected."]
    #[inline(always)]
    pub fn usb_bus_reset_int_raw(&self) -> USB_BUS_RESET_INT_RAW_R {
        USB_BUS_RESET_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_raw(&self) -> OUT_EP1_ZERO_PAYLOAD_INT_RAW_R {
        OUT_EP1_ZERO_PAYLOAD_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_raw(&self) -> OUT_EP2_ZERO_PAYLOAD_INT_RAW_R {
        OUT_EP2_ZERO_PAYLOAD_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed."]
    #[inline(always)]
    pub fn rts_chg_int_raw(&self) -> RTS_CHG_INT_RAW_R {
        RTS_CHG_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed."]
    #[inline(always)]
    pub fn dtr_chg_int_raw(&self) -> DTR_CHG_INT_RAW_R {
        DTR_CHG_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt bit turns to high level when level of GET LINE CODING request is received."]
    #[inline(always)]
    pub fn get_line_code_int_raw(&self) -> GET_LINE_CODE_INT_RAW_R {
        GET_LINE_CODE_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt bit turns to high level when level of SET LINE CODING request is received."]
    #[inline(always)]
    pub fn set_line_code_int_raw(&self) -> SET_LINE_CODE_INT_RAW_R {
        SET_LINE_CODE_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("jtag_in_flush_int_raw", &self.jtag_in_flush_int_raw())
            .field("sof_int_raw", &self.sof_int_raw())
            .field(
                "serial_out_recv_pkt_int_raw",
                &self.serial_out_recv_pkt_int_raw(),
            )
            .field("serial_in_empty_int_raw", &self.serial_in_empty_int_raw())
            .field("pid_err_int_raw", &self.pid_err_int_raw())
            .field("crc5_err_int_raw", &self.crc5_err_int_raw())
            .field("crc16_err_int_raw", &self.crc16_err_int_raw())
            .field("stuff_err_int_raw", &self.stuff_err_int_raw())
            .field(
                "in_token_rec_in_ep1_int_raw",
                &self.in_token_rec_in_ep1_int_raw(),
            )
            .field("usb_bus_reset_int_raw", &self.usb_bus_reset_int_raw())
            .field(
                "out_ep1_zero_payload_int_raw",
                &self.out_ep1_zero_payload_int_raw(),
            )
            .field(
                "out_ep2_zero_payload_int_raw",
                &self.out_ep2_zero_payload_int_raw(),
            )
            .field("rts_chg_int_raw", &self.rts_chg_int_raw())
            .field("dtr_chg_int_raw", &self.dtr_chg_int_raw())
            .field("get_line_code_int_raw", &self.get_line_code_int_raw())
            .field("set_line_code_int_raw", &self.set_line_code_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
    #[inline(always)]
    pub fn jtag_in_flush_int_raw(&mut self) -> JTAG_IN_FLUSH_INT_RAW_W<INT_RAW_SPEC> {
        JTAG_IN_FLUSH_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when SOF frame is received."]
    #[inline(always)]
    pub fn sof_int_raw(&mut self) -> SOF_INT_RAW_W<INT_RAW_SPEC> {
        SOF_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_raw(&mut self) -> SERIAL_OUT_RECV_PKT_INT_RAW_W<INT_RAW_SPEC> {
        SERIAL_OUT_RECV_PKT_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
    #[inline(always)]
    pub fn serial_in_empty_int_raw(&mut self) -> SERIAL_IN_EMPTY_INT_RAW_W<INT_RAW_SPEC> {
        SERIAL_IN_EMPTY_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when pid error is detected."]
    #[inline(always)]
    pub fn pid_err_int_raw(&mut self) -> PID_ERR_INT_RAW_W<INT_RAW_SPEC> {
        PID_ERR_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when CRC5 error is detected."]
    #[inline(always)]
    pub fn crc5_err_int_raw(&mut self) -> CRC5_ERR_INT_RAW_W<INT_RAW_SPEC> {
        CRC5_ERR_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when CRC16 error is detected."]
    #[inline(always)]
    pub fn crc16_err_int_raw(&mut self) -> CRC16_ERR_INT_RAW_W<INT_RAW_SPEC> {
        CRC16_ERR_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when stuff error is detected."]
    #[inline(always)]
    pub fn stuff_err_int_raw(&mut self) -> STUFF_ERR_INT_RAW_W<INT_RAW_SPEC> {
        STUFF_ERR_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_raw(&mut self) -> IN_TOKEN_REC_IN_EP1_INT_RAW_W<INT_RAW_SPEC> {
        IN_TOKEN_REC_IN_EP1_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when usb bus reset is detected."]
    #[inline(always)]
    pub fn usb_bus_reset_int_raw(&mut self) -> USB_BUS_RESET_INT_RAW_W<INT_RAW_SPEC> {
        USB_BUS_RESET_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_raw(&mut self) -> OUT_EP1_ZERO_PAYLOAD_INT_RAW_W<INT_RAW_SPEC> {
        OUT_EP1_ZERO_PAYLOAD_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_raw(&mut self) -> OUT_EP2_ZERO_PAYLOAD_INT_RAW_W<INT_RAW_SPEC> {
        OUT_EP2_ZERO_PAYLOAD_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12 - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed."]
    #[inline(always)]
    pub fn rts_chg_int_raw(&mut self) -> RTS_CHG_INT_RAW_W<INT_RAW_SPEC> {
        RTS_CHG_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 13 - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed."]
    #[inline(always)]
    pub fn dtr_chg_int_raw(&mut self) -> DTR_CHG_INT_RAW_W<INT_RAW_SPEC> {
        DTR_CHG_INT_RAW_W::new(self, 13)
    }
    #[doc = "Bit 14 - The raw interrupt bit turns to high level when level of GET LINE CODING request is received."]
    #[inline(always)]
    pub fn get_line_code_int_raw(&mut self) -> GET_LINE_CODE_INT_RAW_W<INT_RAW_SPEC> {
        GET_LINE_CODE_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 15 - The raw interrupt bit turns to high level when level of SET LINE CODING request is received."]
    #[inline(always)]
    pub fn set_line_code_int_raw(&mut self) -> SET_LINE_CODE_INT_RAW_W<INT_RAW_SPEC> {
        SET_LINE_CODE_INT_RAW_W::new(self, 15)
    }
}
#[doc = "Interrupt raw status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0x08"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
