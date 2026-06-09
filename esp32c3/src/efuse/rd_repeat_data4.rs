#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub type R = crate::R<RD_REPEAT_DATA4_SPEC>;
#[doc = "Field `DISABLE_WAFER_VERSION_MAJOR` reader - Disables check of wafer version major /"]
pub type DISABLE_WAFER_VERSION_MAJOR_R = crate::BitReader;
#[doc = "Field `DISABLE_BLK_VERSION_MAJOR` reader - Disables check of blk version major /"]
pub type DISABLE_BLK_VERSION_MAJOR_R = crate::BitReader;
#[doc = "Field `RESERVED_0_162` reader - reserved /"]
pub type RESERVED_0_162_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Disables check of wafer version major /"]
    #[inline(always)]
    pub fn disable_wafer_version_major(&self) -> DISABLE_WAFER_VERSION_MAJOR_R {
        DISABLE_WAFER_VERSION_MAJOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disables check of blk version major /"]
    #[inline(always)]
    pub fn disable_blk_version_major(&self) -> DISABLE_BLK_VERSION_MAJOR_R {
        DISABLE_BLK_VERSION_MAJOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:23 - reserved /"]
    #[inline(always)]
    pub fn reserved_0_162(&self) -> RESERVED_0_162_R {
        RESERVED_0_162_R::new((self.bits >> 2) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA4")
            .field(
                "disable_wafer_version_major",
                &self.disable_wafer_version_major(),
            )
            .field(
                "disable_blk_version_major",
                &self.disable_blk_version_major(),
            )
            .field("reserved_0_162", &self.reserved_0_162())
            .finish()
    }
}
#[doc = "BLOCK0 data register 5. /\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data4::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA4 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA4_SPEC {}
