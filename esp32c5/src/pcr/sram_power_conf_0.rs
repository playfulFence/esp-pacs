#[doc = "Register `SRAM_POWER_CONF_0` reader"]
pub type R = crate::R<SRAM_POWER_CONF_0_SPEC>;
#[doc = "Register `SRAM_POWER_CONF_0` writer"]
pub type W = crate::W<SRAM_POWER_CONF_0_SPEC>;
#[doc = "Field `ROM_FORCE_PU` reader - Set this bit to force power up ROM"]
pub type ROM_FORCE_PU_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PU` writer - Set this bit to force power up ROM"]
pub type ROM_FORCE_PU_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ROM_FORCE_PD` reader - Set this bit to force power down ROM."]
pub type ROM_FORCE_PD_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PD` writer - Set this bit to force power down ROM."]
pub type ROM_FORCE_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` reader - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
pub type ROM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` writer - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
pub type ROM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Set this bit to force power up ROM"]
    #[inline(always)]
    pub fn rom_force_pu(&self) -> ROM_FORCE_PU_R {
        ROM_FORCE_PU_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Set this bit to force power down ROM."]
    #[inline(always)]
    pub fn rom_force_pd(&self) -> ROM_FORCE_PD_R {
        ROM_FORCE_PD_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new(((self.bits >> 6) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_POWER_CONF_0")
            .field("rom_force_pu", &self.rom_force_pu())
            .field("rom_force_pd", &self.rom_force_pd())
            .field("rom_clkgate_force_on", &self.rom_clkgate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Set this bit to force power up ROM"]
    #[inline(always)]
    pub fn rom_force_pu(&mut self) -> ROM_FORCE_PU_W<SRAM_POWER_CONF_0_SPEC> {
        ROM_FORCE_PU_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Set this bit to force power down ROM."]
    #[inline(always)]
    pub fn rom_force_pd(&mut self) -> ROM_FORCE_PD_W<SRAM_POWER_CONF_0_SPEC> {
        ROM_FORCE_PD_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W<SRAM_POWER_CONF_0_SPEC> {
        ROM_CLKGATE_FORCE_ON_W::new(self, 6)
    }
}
#[doc = "HP SRAM/ROM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_power_conf_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_power_conf_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_POWER_CONF_0_SPEC;
impl crate::RegisterSpec for SRAM_POWER_CONF_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_power_conf_0::R`](R) reader structure"]
impl crate::Readable for SRAM_POWER_CONF_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_power_conf_0::W`](W) writer structure"]
impl crate::Writable for SRAM_POWER_CONF_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_POWER_CONF_0 to value 0x07"]
impl crate::Resettable for SRAM_POWER_CONF_0_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
