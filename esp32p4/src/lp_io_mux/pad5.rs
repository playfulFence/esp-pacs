#[doc = "Register `PAD5` reader"]
pub type R = crate::R<PAD5_SPEC>;
#[doc = "Register `PAD5` writer"]
pub type W = crate::W<PAD5_SPEC>;
#[doc = "Field `REG_PAD5_DRV` reader - Reserved"]
pub type REG_PAD5_DRV_R = crate::FieldReader;
#[doc = "Field `REG_PAD5_DRV` writer - Reserved"]
pub type REG_PAD5_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD5_RDE` reader - Reserved"]
pub type REG_PAD5_RDE_R = crate::BitReader;
#[doc = "Field `REG_PAD5_RDE` writer - Reserved"]
pub type REG_PAD5_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD5_RUE` reader - Reserved"]
pub type REG_PAD5_RUE_R = crate::BitReader;
#[doc = "Field `REG_PAD5_RUE` writer - Reserved"]
pub type REG_PAD5_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD5_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type REG_PAD5_MUX_SEL_R = crate::BitReader;
#[doc = "Field `REG_PAD5_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type REG_PAD5_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD5_FUN_SEL` reader - function sel"]
pub type REG_PAD5_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PAD5_FUN_SEL` writer - function sel"]
pub type REG_PAD5_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD5_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type REG_PAD5_SLP_SEL_R = crate::BitReader;
#[doc = "Field `REG_PAD5_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type REG_PAD5_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD5_SLP_IE` reader - input enable in sleep mode"]
pub type REG_PAD5_SLP_IE_R = crate::BitReader;
#[doc = "Field `REG_PAD5_SLP_IE` writer - input enable in sleep mode"]
pub type REG_PAD5_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD5_SLP_OE` reader - output enable in sleep mode"]
pub type REG_PAD5_SLP_OE_R = crate::BitReader;
#[doc = "Field `REG_PAD5_SLP_OE` writer - output enable in sleep mode"]
pub type REG_PAD5_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD5_FUN_IE` reader - input enable in work mode"]
pub type REG_PAD5_FUN_IE_R = crate::BitReader;
#[doc = "Field `REG_PAD5_FUN_IE` writer - input enable in work mode"]
pub type REG_PAD5_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD5_FILTER_EN` reader - need des"]
pub type REG_PAD5_FILTER_EN_R = crate::BitReader;
#[doc = "Field `REG_PAD5_FILTER_EN` writer - need des"]
pub type REG_PAD5_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad5_drv(&self) -> REG_PAD5_DRV_R {
        REG_PAD5_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad5_rde(&self) -> REG_PAD5_RDE_R {
        REG_PAD5_RDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad5_rue(&self) -> REG_PAD5_RUE_R {
        REG_PAD5_RUE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad5_mux_sel(&self) -> REG_PAD5_MUX_SEL_R {
        REG_PAD5_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad5_fun_sel(&self) -> REG_PAD5_FUN_SEL_R {
        REG_PAD5_FUN_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad5_slp_sel(&self) -> REG_PAD5_SLP_SEL_R {
        REG_PAD5_SLP_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad5_slp_ie(&self) -> REG_PAD5_SLP_IE_R {
        REG_PAD5_SLP_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad5_slp_oe(&self) -> REG_PAD5_SLP_OE_R {
        REG_PAD5_SLP_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad5_fun_ie(&self) -> REG_PAD5_FUN_IE_R {
        REG_PAD5_FUN_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad5_filter_en(&self) -> REG_PAD5_FILTER_EN_R {
        REG_PAD5_FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD5")
            .field("reg_pad5_drv", &self.reg_pad5_drv())
            .field("reg_pad5_rde", &self.reg_pad5_rde())
            .field("reg_pad5_rue", &self.reg_pad5_rue())
            .field("reg_pad5_mux_sel", &self.reg_pad5_mux_sel())
            .field("reg_pad5_fun_sel", &self.reg_pad5_fun_sel())
            .field("reg_pad5_slp_sel", &self.reg_pad5_slp_sel())
            .field("reg_pad5_slp_ie", &self.reg_pad5_slp_ie())
            .field("reg_pad5_slp_oe", &self.reg_pad5_slp_oe())
            .field("reg_pad5_fun_ie", &self.reg_pad5_fun_ie())
            .field("reg_pad5_filter_en", &self.reg_pad5_filter_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad5_drv(&mut self) -> REG_PAD5_DRV_W<PAD5_SPEC> {
        REG_PAD5_DRV_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad5_rde(&mut self) -> REG_PAD5_RDE_W<PAD5_SPEC> {
        REG_PAD5_RDE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad5_rue(&mut self) -> REG_PAD5_RUE_W<PAD5_SPEC> {
        REG_PAD5_RUE_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad5_mux_sel(&mut self) -> REG_PAD5_MUX_SEL_W<PAD5_SPEC> {
        REG_PAD5_MUX_SEL_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad5_fun_sel(&mut self) -> REG_PAD5_FUN_SEL_W<PAD5_SPEC> {
        REG_PAD5_FUN_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad5_slp_sel(&mut self) -> REG_PAD5_SLP_SEL_W<PAD5_SPEC> {
        REG_PAD5_SLP_SEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad5_slp_ie(&mut self) -> REG_PAD5_SLP_IE_W<PAD5_SPEC> {
        REG_PAD5_SLP_IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad5_slp_oe(&mut self) -> REG_PAD5_SLP_OE_W<PAD5_SPEC> {
        REG_PAD5_SLP_OE_W::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad5_fun_ie(&mut self) -> REG_PAD5_FUN_IE_W<PAD5_SPEC> {
        REG_PAD5_FUN_IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad5_filter_en(&mut self) -> REG_PAD5_FILTER_EN_W<PAD5_SPEC> {
        REG_PAD5_FILTER_EN_W::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD5_SPEC;
impl crate::RegisterSpec for PAD5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad5::R`](R) reader structure"]
impl crate::Readable for PAD5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad5::W`](W) writer structure"]
impl crate::Writable for PAD5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD5 to value 0x02"]
impl crate::Resettable for PAD5_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
