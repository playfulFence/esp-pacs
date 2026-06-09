#[doc = "Register `SCL_STRETCH_CONF` reader"]
pub type R = crate::R<SCL_STRETCH_CONF_SPEC>;
#[doc = "Register `SCL_STRETCH_CONF` writer"]
pub type W = crate::W<SCL_STRETCH_CONF_SPEC>;
#[doc = "Field `STRETCH_PROTECT_NUM` reader - ."]
pub type STRETCH_PROTECT_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `STRETCH_PROTECT_NUM` writer - ."]
pub type STRETCH_PROTECT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` reader - ."]
pub type SLAVE_SCL_STRETCH_EN_R = crate::BitReader;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` writer - ."]
pub type SLAVE_SCL_STRETCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_SCL_STRETCH_CLR` writer - ."]
pub type SLAVE_SCL_STRETCH_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` reader - ."]
pub type SLAVE_BYTE_ACK_CTL_EN_R = crate::BitReader;
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` writer - ."]
pub type SLAVE_BYTE_ACK_CTL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_BYTE_ACK_LVL` reader - ."]
pub type SLAVE_BYTE_ACK_LVL_R = crate::BitReader;
#[doc = "Field `SLAVE_BYTE_ACK_LVL` writer - ."]
pub type SLAVE_BYTE_ACK_LVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - ."]
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> STRETCH_PROTECT_NUM_R {
        STRETCH_PROTECT_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - ."]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> SLAVE_SCL_STRETCH_EN_R {
        SLAVE_SCL_STRETCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&self) -> SLAVE_BYTE_ACK_CTL_EN_R {
        SLAVE_BYTE_ACK_CTL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&self) -> SLAVE_BYTE_ACK_LVL_R {
        SLAVE_BYTE_ACK_LVL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STRETCH_CONF")
            .field("slave_byte_ack_lvl", &self.slave_byte_ack_lvl())
            .field("slave_byte_ack_ctl_en", &self.slave_byte_ack_ctl_en())
            .field("slave_scl_stretch_en", &self.slave_scl_stretch_en())
            .field("stretch_protect_num", &self.stretch_protect_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - ."]
    #[inline(always)]
    pub fn stretch_protect_num(&mut self) -> STRETCH_PROTECT_NUM_W<'_, SCL_STRETCH_CONF_SPEC> {
        STRETCH_PROTECT_NUM_W::new(self, 0)
    }
    #[doc = "Bit 10 - ."]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&mut self) -> SLAVE_SCL_STRETCH_EN_W<'_, SCL_STRETCH_CONF_SPEC> {
        SLAVE_SCL_STRETCH_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - ."]
    #[inline(always)]
    pub fn slave_scl_stretch_clr(&mut self) -> SLAVE_SCL_STRETCH_CLR_W<'_, SCL_STRETCH_CONF_SPEC> {
        SLAVE_SCL_STRETCH_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&mut self) -> SLAVE_BYTE_ACK_CTL_EN_W<'_, SCL_STRETCH_CONF_SPEC> {
        SLAVE_BYTE_ACK_CTL_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&mut self) -> SLAVE_BYTE_ACK_LVL_W<'_, SCL_STRETCH_CONF_SPEC> {
        SLAVE_BYTE_ACK_LVL_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_stretch_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_stretch_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_STRETCH_CONF_SPEC;
impl crate::RegisterSpec for SCL_STRETCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stretch_conf::R`](R) reader structure"]
impl crate::Readable for SCL_STRETCH_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_stretch_conf::W`](W) writer structure"]
impl crate::Writable for SCL_STRETCH_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_STRETCH_CONF to value 0"]
impl crate::Resettable for SCL_STRETCH_CONF_SPEC {}
