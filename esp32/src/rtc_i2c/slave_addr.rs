#[doc = "Register `SLAVE_ADDR` reader"]
pub type R = crate::R<SLAVE_ADDR_SPEC>;
#[doc = "Register `SLAVE_ADDR` writer"]
pub type W = crate::W<SLAVE_ADDR_SPEC>;
#[doc = "Field `SLAVE_ADDR` reader - local slave address"]
pub type SLAVE_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `SLAVE_ADDR` writer - local slave address"]
pub type SLAVE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `_10BIT` reader - Set if local slave address is 10-bit"]
pub type _10BIT_R = crate::BitReader;
#[doc = "Field `_10BIT` writer - Set if local slave address is 10-bit"]
pub type _10BIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - local slave address"]
    #[inline(always)]
    pub fn slave_addr(&self) -> SLAVE_ADDR_R {
        SLAVE_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Set if local slave address is 10-bit"]
    #[inline(always)]
    pub fn _10bit(&self) -> _10BIT_R {
        _10BIT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE_ADDR")
            .field("slave_addr", &self.slave_addr())
            .field("_10bit", &self._10bit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - local slave address"]
    #[inline(always)]
    pub fn slave_addr(&mut self) -> SLAVE_ADDR_W<SLAVE_ADDR_SPEC> {
        SLAVE_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 31 - Set if local slave address is 10-bit"]
    #[inline(always)]
    pub fn _10bit(&mut self) -> _10BIT_W<SLAVE_ADDR_SPEC> {
        _10BIT_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE_ADDR_SPEC;
impl crate::RegisterSpec for SLAVE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_addr::R`](R) reader structure"]
impl crate::Readable for SLAVE_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave_addr::W`](W) writer structure"]
impl crate::Writable for SLAVE_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLAVE_ADDR to value 0"]
impl crate::Resettable for SLAVE_ADDR_SPEC {}
