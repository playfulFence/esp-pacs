#[doc = "Register `SWFC_CONF1` reader"]
pub type R = crate::R<SWFC_CONF1_SPEC>;
#[doc = "Register `SWFC_CONF1` writer"]
pub type W = crate::W<SWFC_CONF1_SPEC>;
#[doc = "Field `XON_THRESHOLD` reader - When the number of data bytes in RX FIFO is less than this register's value with UART_SW_FLOW_CON_EN set to 1, the transmitter sends an XON character."]
pub type XON_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `XON_THRESHOLD` writer - When the number of data bytes in RX FIFO is less than this register's value with UART_SW_FLOW_CON_EN set to 1, the transmitter sends an XON character."]
pub type XON_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `XON_CHAR` reader - This register stores the XON flow control character."]
pub type XON_CHAR_R = crate::FieldReader;
#[doc = "Field `XON_CHAR` writer - This register stores the XON flow control character."]
pub type XON_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:8 - When the number of data bytes in RX FIFO is less than this register's value with UART_SW_FLOW_CON_EN set to 1, the transmitter sends an XON character."]
    #[inline(always)]
    pub fn xon_threshold(&self) -> XON_THRESHOLD_R {
        XON_THRESHOLD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:16 - This register stores the XON flow control character."]
    #[inline(always)]
    pub fn xon_char(&self) -> XON_CHAR_R {
        XON_CHAR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWFC_CONF1")
            .field("xon_threshold", &self.xon_threshold())
            .field("xon_char", &self.xon_char())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - When the number of data bytes in RX FIFO is less than this register's value with UART_SW_FLOW_CON_EN set to 1, the transmitter sends an XON character."]
    #[inline(always)]
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W<SWFC_CONF1_SPEC> {
        XON_THRESHOLD_W::new(self, 0)
    }
    #[doc = "Bits 9:16 - This register stores the XON flow control character."]
    #[inline(always)]
    pub fn xon_char(&mut self) -> XON_CHAR_W<SWFC_CONF1_SPEC> {
        XON_CHAR_W::new(self, 9)
    }
}
#[doc = "Software flow-control character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWFC_CONF1_SPEC;
impl crate::RegisterSpec for SWFC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swfc_conf1::R`](R) reader structure"]
impl crate::Readable for SWFC_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swfc_conf1::W`](W) writer structure"]
impl crate::Writable for SWFC_CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWFC_CONF1 to value 0x2200"]
impl crate::Resettable for SWFC_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x2200;
}
