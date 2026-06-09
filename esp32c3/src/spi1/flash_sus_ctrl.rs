#[doc = "Register `FLASH_SUS_CTRL` reader"]
pub type R = crate::R<FLASH_SUS_CTRL_SPEC>;
#[doc = "Register `FLASH_SUS_CTRL` writer"]
pub type W = crate::W<FLASH_SUS_CTRL_SPEC>;
#[doc = "Field `FLASH_PER` reader - program erase resume bit program erase suspend operation will"]
pub type FLASH_PER_R = crate::BitReader;
#[doc = "Field `FLASH_PER` writer - program erase resume bit program erase suspend operation will"]
pub type FLASH_PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES` reader - program erase suspend bit program erase suspend operation will"]
pub type FLASH_PES_R = crate::BitReader;
#[doc = "Field `FLASH_PES` writer - program erase suspend bit program erase suspend operation will"]
pub type FLASH_PES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PER_WAIT_EN` reader - Set this bit to enable SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]"]
pub type FLASH_PER_WAIT_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PER_WAIT_EN` writer - Set this bit to enable SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]"]
pub type FLASH_PER_WAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_WAIT_EN` reader - Set this bit to enable SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]"]
pub type FLASH_PES_WAIT_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PES_WAIT_EN` writer - Set this bit to enable SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]"]
pub type FLASH_PES_WAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_PER_EN` reader - Set this bit to enable PES transfer trigger PES transfer option."]
pub type PES_PER_EN_R = crate::BitReader;
#[doc = "Field `PES_PER_EN` writer - Set this bit to enable PES transfer trigger PES transfer option."]
pub type PES_PER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_EN` reader - Set this bit to enable Auto-suspending function."]
pub type FLASH_PES_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PES_EN` writer - Set this bit to enable Auto-suspending function."]
pub type FLASH_PES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PESR_END_MSK` reader - The mask value when check SUS/SUS1/SUS2 status bit. If the read"]
pub type PESR_END_MSK_R = crate::FieldReader<u16>;
#[doc = "Field `PESR_END_MSK` writer - The mask value when check SUS/SUS1/SUS2 status bit. If the read"]
pub type PESR_END_MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RD_SUS_2B` reader - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit."]
pub type RD_SUS_2B_R = crate::BitReader;
#[doc = "Field `RD_SUS_2B` writer - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit."]
pub type RD_SUS_2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER_END_EN` reader - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure"]
pub type PER_END_EN_R = crate::BitReader;
#[doc = "Field `PER_END_EN` writer - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure"]
pub type PER_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_END_EN` reader - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure"]
pub type PES_END_EN_R = crate::BitReader;
#[doc = "Field `PES_END_EN` writer - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure"]
pub type PES_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUS_TIMEOUT_CNT` reader - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\]"]
pub type SUS_TIMEOUT_CNT_R = crate::FieldReader;
#[doc = "Field `SUS_TIMEOUT_CNT` writer - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\]"]
pub type SUS_TIMEOUT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - program erase resume bit program erase suspend operation will"]
    #[inline(always)]
    pub fn flash_per(&self) -> FLASH_PER_R {
        FLASH_PER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - program erase suspend bit program erase suspend operation will"]
    #[inline(always)]
    pub fn flash_pes(&self) -> FLASH_PES_R {
        FLASH_PES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]"]
    #[inline(always)]
    pub fn flash_per_wait_en(&self) -> FLASH_PER_WAIT_EN_R {
        FLASH_PER_WAIT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]"]
    #[inline(always)]
    pub fn flash_pes_wait_en(&self) -> FLASH_PES_WAIT_EN_R {
        FLASH_PES_WAIT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable PES transfer trigger PES transfer option."]
    #[inline(always)]
    pub fn pes_per_en(&self) -> PES_PER_EN_R {
        PES_PER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable Auto-suspending function."]
    #[inline(always)]
    pub fn flash_pes_en(&self) -> FLASH_PES_EN_R {
        FLASH_PES_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:21 - The mask value when check SUS/SUS1/SUS2 status bit. If the read"]
    #[inline(always)]
    pub fn pesr_end_msk(&self) -> PESR_END_MSK_R {
        PESR_END_MSK_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22 - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit."]
    #[inline(always)]
    pub fn rd_sus_2b(&self) -> RD_SUS_2B_R {
        RD_SUS_2B_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure"]
    #[inline(always)]
    pub fn per_end_en(&self) -> PER_END_EN_R {
        PER_END_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure"]
    #[inline(always)]
    pub fn pes_end_en(&self) -> PES_END_EN_R {
        PES_END_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\]"]
    #[inline(always)]
    pub fn sus_timeout_cnt(&self) -> SUS_TIMEOUT_CNT_R {
        SUS_TIMEOUT_CNT_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SUS_CTRL")
            .field("sus_timeout_cnt", &self.sus_timeout_cnt())
            .field("pes_end_en", &self.pes_end_en())
            .field("per_end_en", &self.per_end_en())
            .field("rd_sus_2b", &self.rd_sus_2b())
            .field("pesr_end_msk", &self.pesr_end_msk())
            .field("flash_pes_en", &self.flash_pes_en())
            .field("pes_per_en", &self.pes_per_en())
            .field("flash_pes_wait_en", &self.flash_pes_wait_en())
            .field("flash_per_wait_en", &self.flash_per_wait_en())
            .field("flash_pes", &self.flash_pes())
            .field("flash_per", &self.flash_per())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - program erase resume bit program erase suspend operation will"]
    #[inline(always)]
    pub fn flash_per(&mut self) -> FLASH_PER_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PER_W::new(self, 0)
    }
    #[doc = "Bit 1 - program erase suspend bit program erase suspend operation will"]
    #[inline(always)]
    pub fn flash_pes(&mut self) -> FLASH_PES_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PES_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]"]
    #[inline(always)]
    pub fn flash_per_wait_en(&mut self) -> FLASH_PER_WAIT_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PER_WAIT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]"]
    #[inline(always)]
    pub fn flash_pes_wait_en(&mut self) -> FLASH_PES_WAIT_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PES_WAIT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable PES transfer trigger PES transfer option."]
    #[inline(always)]
    pub fn pes_per_en(&mut self) -> PES_PER_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        PES_PER_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable Auto-suspending function."]
    #[inline(always)]
    pub fn flash_pes_en(&mut self) -> FLASH_PES_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        FLASH_PES_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:21 - The mask value when check SUS/SUS1/SUS2 status bit. If the read"]
    #[inline(always)]
    pub fn pesr_end_msk(&mut self) -> PESR_END_MSK_W<'_, FLASH_SUS_CTRL_SPEC> {
        PESR_END_MSK_W::new(self, 6)
    }
    #[doc = "Bit 22 - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit."]
    #[inline(always)]
    pub fn rd_sus_2b(&mut self) -> RD_SUS_2B_W<'_, FLASH_SUS_CTRL_SPEC> {
        RD_SUS_2B_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure"]
    #[inline(always)]
    pub fn per_end_en(&mut self) -> PER_END_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        PER_END_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure"]
    #[inline(always)]
    pub fn pes_end_en(&mut self) -> PES_END_EN_W<'_, FLASH_SUS_CTRL_SPEC> {
        PES_END_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:31 - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\]"]
    #[inline(always)]
    pub fn sus_timeout_cnt(&mut self) -> SUS_TIMEOUT_CNT_W<'_, FLASH_SUS_CTRL_SPEC> {
        SUS_TIMEOUT_CNT_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SUS_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sus_ctrl::R`](R) reader structure"]
impl crate::Readable for FLASH_SUS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_sus_ctrl::W`](W) writer structure"]
impl crate::Writable for FLASH_SUS_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_SUS_CTRL to value 0"]
impl crate::Resettable for FLASH_SUS_CTRL_SPEC {}
