#[doc = "Register `XTS_RELEASE` writer"]
pub type W = crate::W<XTS_RELEASE_SPEC>;
#[doc = "Field `SPI_XTS_RELEASE` writer - Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3."]
pub type SPI_XTS_RELEASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTS_RELEASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3."]
    #[inline(always)]
    pub fn spi_xts_release(&mut self) -> SPI_XTS_RELEASE_W<XTS_RELEASE_SPEC> {
        SPI_XTS_RELEASE_W::new(self, 0)
    }
}
#[doc = "Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_release::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTS_RELEASE_SPEC;
impl crate::RegisterSpec for XTS_RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`xts_release::W`](W) writer structure"]
impl crate::Writable for XTS_RELEASE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTS_RELEASE to value 0"]
impl crate::Resettable for XTS_RELEASE_SPEC {}
