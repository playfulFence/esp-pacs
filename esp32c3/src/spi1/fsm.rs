#[doc = "Register `FSM` reader"]
pub type R = crate::R<FSM_SPEC>;
#[doc = "Register `FSM` writer"]
pub type W = crate::W<FSM_SPEC>;
#[doc = "Field `CSPI_ST` reader - The current status of SPI0 slave FSM: cspi_st. 0: idle state"]
pub type CSPI_ST_R = crate::FieldReader;
#[doc = "Field `EM_ST` reader - The current status of SPI0 master FSM: cspi_st. 0: idle state"]
pub type EM_ST_R = crate::FieldReader;
#[doc = "Field `CSPI_LOCK_DELAY_TIME` reader - The lock delay time of SPI0/1 arbiter by cspi_st after PER is sent by SPI1."]
pub type CSPI_LOCK_DELAY_TIME_R = crate::FieldReader;
#[doc = "Field `CSPI_LOCK_DELAY_TIME` writer - The lock delay time of SPI0/1 arbiter by cspi_st after PER is sent by SPI1."]
pub type CSPI_LOCK_DELAY_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - The current status of SPI0 slave FSM: cspi_st. 0: idle state"]
    #[inline(always)]
    pub fn cspi_st(&self) -> CSPI_ST_R {
        CSPI_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - The current status of SPI0 master FSM: cspi_st. 0: idle state"]
    #[inline(always)]
    pub fn em_st(&self) -> EM_ST_R {
        EM_ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by cspi_st after PER is sent by SPI1."]
    #[inline(always)]
    pub fn cspi_lock_delay_time(&self) -> CSPI_LOCK_DELAY_TIME_R {
        CSPI_LOCK_DELAY_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM")
            .field("cspi_lock_delay_time", &self.cspi_lock_delay_time())
            .field("em_st", &self.em_st())
            .field("cspi_st", &self.cspi_st())
            .finish()
    }
}
impl W {
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by cspi_st after PER is sent by SPI1."]
    #[inline(always)]
    pub fn cspi_lock_delay_time(&mut self) -> CSPI_LOCK_DELAY_TIME_W<'_, FSM_SPEC> {
        CSPI_LOCK_DELAY_TIME_W::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm::R`](R) reader structure"]
impl crate::Readable for FSM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsm::W`](W) writer structure"]
impl crate::Writable for FSM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSM to value 0"]
impl crate::Resettable for FSM_SPEC {}
