#[doc = "Register `MEM_PVT` reader"]
pub type R = crate::R<MEM_PVT_SPEC>;
#[doc = "Register `MEM_PVT` writer"]
pub type W = crate::W<MEM_PVT_SPEC>;
#[doc = "Field `MEM_PATH_LEN` reader - "]
pub type MEM_PATH_LEN_R = crate::FieldReader;
#[doc = "Field `MEM_PATH_LEN` writer - "]
pub type MEM_PATH_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MEM_ERR_CNT_CLR` writer - "]
pub type MEM_ERR_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_PVT_MONITOR_EN` reader - "]
pub type MEM_PVT_MONITOR_EN_R = crate::BitReader;
#[doc = "Field `MEM_PVT_MONITOR_EN` writer - "]
pub type MEM_PVT_MONITOR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TIMING_ERR_CNT` reader - "]
pub type MEM_TIMING_ERR_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_VT_SEL` reader - "]
pub type MEM_VT_SEL_R = crate::FieldReader;
#[doc = "Field `MEM_VT_SEL` writer - "]
pub type MEM_VT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mem_path_len(&self) -> MEM_PATH_LEN_R {
        MEM_PATH_LEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mem_pvt_monitor_en(&self) -> MEM_PVT_MONITOR_EN_R {
        MEM_PVT_MONITOR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:21"]
    #[inline(always)]
    pub fn mem_timing_err_cnt(&self) -> MEM_TIMING_ERR_CNT_R {
        MEM_TIMING_ERR_CNT_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn mem_vt_sel(&self) -> MEM_VT_SEL_R {
        MEM_VT_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_PVT")
            .field("mem_vt_sel", &self.mem_vt_sel())
            .field("mem_timing_err_cnt", &self.mem_timing_err_cnt())
            .field("mem_pvt_monitor_en", &self.mem_pvt_monitor_en())
            .field("mem_path_len", &self.mem_path_len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mem_path_len(&mut self) -> MEM_PATH_LEN_W<'_, MEM_PVT_SPEC> {
        MEM_PATH_LEN_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mem_err_cnt_clr(&mut self) -> MEM_ERR_CNT_CLR_W<'_, MEM_PVT_SPEC> {
        MEM_ERR_CNT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mem_pvt_monitor_en(&mut self) -> MEM_PVT_MONITOR_EN_W<'_, MEM_PVT_SPEC> {
        MEM_PVT_MONITOR_EN_W::new(self, 5)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn mem_vt_sel(&mut self) -> MEM_VT_SEL_W<'_, MEM_PVT_SPEC> {
        MEM_VT_SEL_W::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_pvt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_pvt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_PVT_SPEC;
impl crate::RegisterSpec for MEM_PVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_pvt::R`](R) reader structure"]
impl crate::Readable for MEM_PVT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_pvt::W`](W) writer structure"]
impl crate::Writable for MEM_PVT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_PVT to value 0"]
impl crate::Resettable for MEM_PVT_SPEC {}
