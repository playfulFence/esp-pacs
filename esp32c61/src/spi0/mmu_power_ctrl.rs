#[doc = "Register `MMU_POWER_CTRL` reader"]
pub type R = crate::R<MMU_POWER_CTRL_SPEC>;
#[doc = "Register `MMU_POWER_CTRL` writer"]
pub type W = crate::W<MMU_POWER_CTRL_SPEC>;
#[doc = "Field `MMU_MEM_FORCE_ON` reader - Set this bit to enable mmu-memory clock force on"]
pub type MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MMU_MEM_FORCE_ON` writer - Set this bit to enable mmu-memory clock force on"]
pub type MMU_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_MEM_FORCE_PD` reader - Set this bit to force mmu-memory powerdown"]
pub type MMU_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MMU_MEM_FORCE_PD` writer - Set this bit to force mmu-memory powerdown"]
pub type MMU_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_MEM_FORCE_PU` reader - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
pub type MMU_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MMU_MEM_FORCE_PU` writer - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
pub type MMU_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_PAGE_SIZE` reader - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
pub type MMU_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `MMU_PAGE_SIZE` writer - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
pub type MMU_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AUX_CTRL` reader - MMU PSRAM aux control register"]
pub type AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `AUX_CTRL` writer - MMU PSRAM aux control register"]
pub type AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `RDN_ENA` reader - ECO register enable bit"]
pub type RDN_ENA_R = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - ECO register enable bit"]
pub type RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDN_RESULT` reader - MSPI module clock domain and AXI clock domain ECO register result register"]
pub type RDN_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to enable mmu-memory clock force on"]
    #[inline(always)]
    pub fn mmu_mem_force_on(&self) -> MMU_MEM_FORCE_ON_R {
        MMU_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force mmu-memory powerdown"]
    #[inline(always)]
    pub fn mmu_mem_force_pd(&self) -> MMU_MEM_FORCE_PD_R {
        MMU_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
    #[inline(always)]
    pub fn mmu_mem_force_pu(&self) -> MMU_MEM_FORCE_PU_R {
        MMU_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
    #[inline(always)]
    pub fn mmu_page_size(&self) -> MMU_PAGE_SIZE_R {
        MMU_PAGE_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 16:29 - MMU PSRAM aux control register"]
    #[inline(always)]
    pub fn aux_ctrl(&self) -> AUX_CTRL_R {
        AUX_CTRL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - ECO register enable bit"]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RDN_ENA_R {
        RDN_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MSPI module clock domain and AXI clock domain ECO register result register"]
    #[inline(always)]
    pub fn rdn_result(&self) -> RDN_RESULT_R {
        RDN_RESULT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMU_POWER_CTRL")
            .field("mmu_mem_force_on", &self.mmu_mem_force_on())
            .field("mmu_mem_force_pd", &self.mmu_mem_force_pd())
            .field("mmu_mem_force_pu", &self.mmu_mem_force_pu())
            .field("mmu_page_size", &self.mmu_page_size())
            .field("aux_ctrl", &self.aux_ctrl())
            .field("rdn_ena", &self.rdn_ena())
            .field("rdn_result", &self.rdn_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable mmu-memory clock force on"]
    #[inline(always)]
    pub fn mmu_mem_force_on(&mut self) -> MMU_MEM_FORCE_ON_W<MMU_POWER_CTRL_SPEC> {
        MMU_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force mmu-memory powerdown"]
    #[inline(always)]
    pub fn mmu_mem_force_pd(&mut self) -> MMU_MEM_FORCE_PD_W<MMU_POWER_CTRL_SPEC> {
        MMU_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
    #[inline(always)]
    pub fn mmu_mem_force_pu(&mut self) -> MMU_MEM_FORCE_PU_W<MMU_POWER_CTRL_SPEC> {
        MMU_MEM_FORCE_PU_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
    #[inline(always)]
    pub fn mmu_page_size(&mut self) -> MMU_PAGE_SIZE_W<MMU_POWER_CTRL_SPEC> {
        MMU_PAGE_SIZE_W::new(self, 3)
    }
    #[doc = "Bits 16:29 - MMU PSRAM aux control register"]
    #[inline(always)]
    pub fn aux_ctrl(&mut self) -> AUX_CTRL_W<MMU_POWER_CTRL_SPEC> {
        AUX_CTRL_W::new(self, 16)
    }
    #[doc = "Bit 30 - ECO register enable bit"]
    #[inline(always)]
    pub fn rdn_ena(&mut self) -> RDN_ENA_W<MMU_POWER_CTRL_SPEC> {
        RDN_ENA_W::new(self, 30)
    }
}
#[doc = "MSPI MMU power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_power_ctrl::R`](R) reader structure"]
impl crate::Readable for MMU_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmu_power_ctrl::W`](W) writer structure"]
impl crate::Writable for MMU_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMU_POWER_CTRL to value 0x1320_0004"]
impl crate::Resettable for MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1320_0004;
}
