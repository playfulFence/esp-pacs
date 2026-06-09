#[doc = "Register `PIN%s` reader"]
pub type R = crate::R<PIN_SPEC>;
#[doc = "Register `PIN%s` writer"]
pub type W = crate::W<PIN_SPEC>;
#[doc = "Field `PIN0_SYNC2_BYPASS` reader - "]
pub type PIN0_SYNC2_BYPASS_R = crate::FieldReader;
#[doc = "Field `PIN0_SYNC2_BYPASS` writer - "]
pub type PIN0_SYNC2_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN0_PAD_DRIVER` reader - "]
pub type PIN0_PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PIN0_PAD_DRIVER` writer - "]
pub type PIN0_PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN0_SYNC1_BYPASS` reader - "]
pub type PIN0_SYNC1_BYPASS_R = crate::FieldReader;
#[doc = "Field `PIN0_SYNC1_BYPASS` writer - "]
pub type PIN0_SYNC1_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN0_INT_TYPE` reader - "]
pub type PIN0_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `PIN0_INT_TYPE` writer - "]
pub type PIN0_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PIN0_WAKEUP_ENABLE` reader - "]
pub type PIN0_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `PIN0_WAKEUP_ENABLE` writer - "]
pub type PIN0_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN0_CONFIG` reader - "]
pub type PIN0_CONFIG_R = crate::FieldReader;
#[doc = "Field `PIN0_CONFIG` writer - "]
pub type PIN0_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN0_INT_ENA` reader - "]
pub type PIN0_INT_ENA_R = crate::FieldReader;
#[doc = "Field `PIN0_INT_ENA` writer - "]
pub type PIN0_INT_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pin0_sync2_bypass(&self) -> PIN0_SYNC2_BYPASS_R {
        PIN0_SYNC2_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin0_pad_driver(&self) -> PIN0_PAD_DRIVER_R {
        PIN0_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn pin0_sync1_bypass(&self) -> PIN0_SYNC1_BYPASS_R {
        PIN0_SYNC1_BYPASS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn pin0_int_type(&self) -> PIN0_INT_TYPE_R {
        PIN0_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin0_wakeup_enable(&self) -> PIN0_WAKEUP_ENABLE_R {
        PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn pin0_config(&self) -> PIN0_CONFIG_R {
        PIN0_CONFIG_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn pin0_int_ena(&self) -> PIN0_INT_ENA_R {
        PIN0_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN")
            .field("pin0_int_ena", &self.pin0_int_ena())
            .field("pin0_config", &self.pin0_config())
            .field("pin0_wakeup_enable", &self.pin0_wakeup_enable())
            .field("pin0_int_type", &self.pin0_int_type())
            .field("pin0_sync1_bypass", &self.pin0_sync1_bypass())
            .field("pin0_pad_driver", &self.pin0_pad_driver())
            .field("pin0_sync2_bypass", &self.pin0_sync2_bypass())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pin0_sync2_bypass(&mut self) -> PIN0_SYNC2_BYPASS_W<'_, PIN_SPEC> {
        PIN0_SYNC2_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin0_pad_driver(&mut self) -> PIN0_PAD_DRIVER_W<'_, PIN_SPEC> {
        PIN0_PAD_DRIVER_W::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn pin0_sync1_bypass(&mut self) -> PIN0_SYNC1_BYPASS_W<'_, PIN_SPEC> {
        PIN0_SYNC1_BYPASS_W::new(self, 3)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn pin0_int_type(&mut self) -> PIN0_INT_TYPE_W<'_, PIN_SPEC> {
        PIN0_INT_TYPE_W::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin0_wakeup_enable(&mut self) -> PIN0_WAKEUP_ENABLE_W<'_, PIN_SPEC> {
        PIN0_WAKEUP_ENABLE_W::new(self, 10)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn pin0_config(&mut self) -> PIN0_CONFIG_W<'_, PIN_SPEC> {
        PIN0_CONFIG_W::new(self, 11)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn pin0_int_ena(&mut self) -> PIN0_INT_ENA_W<'_, PIN_SPEC> {
        PIN0_INT_ENA_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin::R`](R) reader structure"]
impl crate::Readable for PIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin::W`](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {}
