#[doc = "Register `MISC_CONF` reader"]
pub type R = crate::R<MISC_CONF_SPEC>;
#[doc = "Register `MISC_CONF` writer"]
pub type W = crate::W<MISC_CONF_SPEC>;
#[doc = "Field `CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CONF")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<MISC_CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
#[doc = "USB_DEVICE_MISC_CONF_REG.\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_CONF_SPEC;
impl crate::RegisterSpec for MISC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_conf::R`](R) reader structure"]
impl crate::Readable for MISC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc_conf::W`](W) writer structure"]
impl crate::Writable for MISC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC_CONF to value 0"]
impl crate::Resettable for MISC_CONF_SPEC {}
