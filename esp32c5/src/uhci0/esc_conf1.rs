#[doc = "Register `ESC_CONF1` reader"]
pub type R = crate::R<ESC_CONF1_SPEC>;
#[doc = "Register `ESC_CONF1` writer"]
pub type W = crate::W<ESC_CONF1_SPEC>;
#[doc = "Field `ESC_SEQ0` reader - Configures the character that needs to be encoded. The default value is 0xDB used as the first character of SLIP escape sequence."]
pub type ESC_SEQ0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0` writer - Configures the character that needs to be encoded. The default value is 0xDB used as the first character of SLIP escape sequence."]
pub type ESC_SEQ0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ESC_SEQ0_CHAR0` reader - Configures the first character of SLIP escape sequence. The default value is 0xDB."]
pub type ESC_SEQ0_CHAR0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0_CHAR0` writer - Configures the first character of SLIP escape sequence. The default value is 0xDB."]
pub type ESC_SEQ0_CHAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ESC_SEQ0_CHAR1` reader - Configures the second character of SLIP escape sequence. The default value is 0xDD."]
pub type ESC_SEQ0_CHAR1_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0_CHAR1` writer - Configures the second character of SLIP escape sequence. The default value is 0xDD."]
pub type ESC_SEQ0_CHAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the character that needs to be encoded. The default value is 0xDB used as the first character of SLIP escape sequence."]
    #[inline(always)]
    pub fn esc_seq0(&self) -> ESC_SEQ0_R {
        ESC_SEQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the first character of SLIP escape sequence. The default value is 0xDB."]
    #[inline(always)]
    pub fn esc_seq0_char0(&self) -> ESC_SEQ0_CHAR0_R {
        ESC_SEQ0_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configures the second character of SLIP escape sequence. The default value is 0xDD."]
    #[inline(always)]
    pub fn esc_seq0_char1(&self) -> ESC_SEQ0_CHAR1_R {
        ESC_SEQ0_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF1")
            .field("esc_seq0", &self.esc_seq0())
            .field("esc_seq0_char0", &self.esc_seq0_char0())
            .field("esc_seq0_char1", &self.esc_seq0_char1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the character that needs to be encoded. The default value is 0xDB used as the first character of SLIP escape sequence."]
    #[inline(always)]
    pub fn esc_seq0(&mut self) -> ESC_SEQ0_W<ESC_CONF1_SPEC> {
        ESC_SEQ0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures the first character of SLIP escape sequence. The default value is 0xDB."]
    #[inline(always)]
    pub fn esc_seq0_char0(&mut self) -> ESC_SEQ0_CHAR0_W<ESC_CONF1_SPEC> {
        ESC_SEQ0_CHAR0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Configures the second character of SLIP escape sequence. The default value is 0xDD."]
    #[inline(always)]
    pub fn esc_seq0_char1(&mut self) -> ESC_SEQ0_CHAR1_W<ESC_CONF1_SPEC> {
        ESC_SEQ0_CHAR1_W::new(self, 16)
    }
}
#[doc = "Escape sequence configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esc_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESC_CONF1_SPEC;
impl crate::RegisterSpec for ESC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esc_conf1::R`](R) reader structure"]
impl crate::Readable for ESC_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`esc_conf1::W`](W) writer structure"]
impl crate::Writable for ESC_CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESC_CONF1 to value 0x00dd_dbdb"]
impl crate::Resettable for ESC_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x00dd_dbdb;
}
