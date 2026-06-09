#[doc = "Register `CH%s_RX_STATUS` reader"]
pub type R = crate::R<CH_RX_STATUS_SPEC>;
#[doc = "Field `MEM_WADDR_EX` reader - "]
pub type MEM_WADDR_EX_R = crate::FieldReader<u16>;
#[doc = "Field `APB_MEM_RADDR` reader - "]
pub type APB_MEM_RADDR_R = crate::FieldReader<u16>;
#[doc = "Field `STATE` reader - "]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `MEM_OWNER_ERR` reader - "]
pub type MEM_OWNER_ERR_R = crate::BitReader;
#[doc = "Field `MEM_FULL` reader - "]
pub type MEM_FULL_R = crate::BitReader;
#[doc = "Field `APB_MEM_RD_ERR` reader - "]
pub type APB_MEM_RD_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn mem_waddr_ex(&self) -> MEM_WADDR_EX_R {
        MEM_WADDR_EX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20"]
    #[inline(always)]
    pub fn apb_mem_raddr(&self) -> APB_MEM_RADDR_R {
        APB_MEM_RADDR_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mem_owner_err(&self) -> MEM_OWNER_ERR_R {
        MEM_OWNER_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mem_full(&self) -> MEM_FULL_R {
        MEM_FULL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_STATUS")
            .field("apb_mem_rd_err", &self.apb_mem_rd_err())
            .field("mem_full", &self.mem_full())
            .field("mem_owner_err", &self.mem_owner_err())
            .field("state", &self.state())
            .field("apb_mem_raddr", &self.apb_mem_raddr())
            .field("mem_waddr_ex", &self.mem_waddr_ex())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_rx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_STATUS_SPEC;
impl crate::RegisterSpec for CH_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_status::R`](R) reader structure"]
impl crate::Readable for CH_RX_STATUS_SPEC {}
#[doc = "`reset()` method sets CH%s_RX_STATUS to value 0"]
impl crate::Resettable for CH_RX_STATUS_SPEC {}
