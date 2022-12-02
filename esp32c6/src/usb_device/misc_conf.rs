#[doc = "Register `MISC_CONF` reader"]
pub struct R(crate::R<MISC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CONF` writer"]
pub struct W(crate::W<MISC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MISC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type USB_SERIAL_JTAG_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type USB_SERIAL_JTAG_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn usb_serial_jtag_clk_en(&self) -> USB_SERIAL_JTAG_CLK_EN_R {
        USB_SERIAL_JTAG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_clk_en(&mut self) -> USB_SERIAL_JTAG_CLK_EN_W<0> {
        USB_SERIAL_JTAG_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock enable control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf](index.html) module"]
pub struct MISC_CONF_SPEC;
impl crate::RegisterSpec for MISC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_conf::R](R) reader structure"]
impl crate::Readable for MISC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_conf::W](W) writer structure"]
impl crate::Writable for MISC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC_CONF to value 0"]
impl crate::Resettable for MISC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}