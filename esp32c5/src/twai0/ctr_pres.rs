#[doc = "Register `CTR_PRES` writer"]
pub type W = crate::W<CTR_PRES_SPEC>;
#[doc = "Field `CTPV` writer - Configures the pre-defined value to set the error counter."]
pub type CTPV_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PTX` writer - Configures whether or not to set the receiver error counter into the value of pre-defined value. 0: invalid 1: set"]
pub type PTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRX` writer - Configures whether or not to set the transmitter error counter into the value of pre-defined value. 0: invalid 1: set"]
pub type PRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENORM` writer - Configures whether or not to erase the error counter of nominal bit time. 0: invalid 1: erase"]
pub type ENORM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFD` writer - Configures whether or not to erase the error counter of data bit time. 0: invalid 1: erase"]
pub type EFD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTR_PRES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:8 - Configures the pre-defined value to set the error counter."]
    #[inline(always)]
    pub fn ctpv(&mut self) -> CTPV_W<CTR_PRES_SPEC> {
        CTPV_W::new(self, 0)
    }
    #[doc = "Bit 9 - Configures whether or not to set the receiver error counter into the value of pre-defined value. 0: invalid 1: set"]
    #[inline(always)]
    pub fn ptx(&mut self) -> PTX_W<CTR_PRES_SPEC> {
        PTX_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to set the transmitter error counter into the value of pre-defined value. 0: invalid 1: set"]
    #[inline(always)]
    pub fn prx(&mut self) -> PRX_W<CTR_PRES_SPEC> {
        PRX_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to erase the error counter of nominal bit time. 0: invalid 1: erase"]
    #[inline(always)]
    pub fn enorm(&mut self) -> ENORM_W<CTR_PRES_SPEC> {
        ENORM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to erase the error counter of data bit time. 0: invalid 1: erase"]
    #[inline(always)]
    pub fn efd(&mut self) -> EFD_W<CTR_PRES_SPEC> {
        EFD_W::new(self, 12)
    }
}
#[doc = "TWAI FD error counters pre-define configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr_pres::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_PRES_SPEC;
impl crate::RegisterSpec for CTR_PRES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctr_pres::W`](W) writer structure"]
impl crate::Writable for CTR_PRES_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTR_PRES to value 0"]
impl crate::Resettable for CTR_PRES_SPEC {}
