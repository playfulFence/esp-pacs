#[doc = "Register `SAR_DAC_CTRL2` reader"]
pub struct R(crate::R<SAR_DAC_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_DAC_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_DAC_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_DAC_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_DAC_CTRL2` writer"]
pub struct W(crate::W<SAR_DAC_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_DAC_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SAR_DAC_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_DAC_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_DC1` reader - DC offset for DAC1 CW generator"]
pub type DAC_DC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_DC1` writer - DC offset for DAC1 CW generator"]
pub type DAC_DC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_DAC_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `DAC_DC2` reader - DC offset for DAC2 CW generator"]
pub type DAC_DC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_DC2` writer - DC offset for DAC2 CW generator"]
pub type DAC_DC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_DAC_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `DAC_SCALE1` reader - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub type DAC_SCALE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_SCALE1` writer - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub type DAC_SCALE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_DAC_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `DAC_SCALE2` reader - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub type DAC_SCALE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_SCALE2` writer - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub type DAC_SCALE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_DAC_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `DAC_INV1` reader - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub type DAC_INV1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_INV1` writer - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub type DAC_INV1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_DAC_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `DAC_INV2` reader - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub type DAC_INV2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_INV2` writer - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub type DAC_INV2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_DAC_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `DAC_CW_EN1` reader - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\]
0: to select register reg_pdac1_dac\\[7:0\\]
as source to PDAC1_DAC\\[7:0\\]"]
pub type DAC_CW_EN1_R = crate::BitReader<bool>;
#[doc = "Field `DAC_CW_EN1` writer - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\]
0: to select register reg_pdac1_dac\\[7:0\\]
as source to PDAC1_DAC\\[7:0\\]"]
pub type DAC_CW_EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_DAC_CTRL2_SPEC, bool, O>;
#[doc = "Field `DAC_CW_EN2` reader - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\]
0: to select register reg_pdac2_dac\\[7:0\\]
as source to PDAC2_DAC\\[7:0\\]"]
pub type DAC_CW_EN2_R = crate::BitReader<bool>;
#[doc = "Field `DAC_CW_EN2` writer - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\]
0: to select register reg_pdac2_dac\\[7:0\\]
as source to PDAC2_DAC\\[7:0\\]"]
pub type DAC_CW_EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_DAC_CTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - DC offset for DAC1 CW generator"]
    #[inline(always)]
    pub fn dac_dc1(&self) -> DAC_DC1_R {
        DAC_DC1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DC offset for DAC2 CW generator"]
    #[inline(always)]
    pub fn dac_dc2(&self) -> DAC_DC2_R {
        DAC_DC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale1(&self) -> DAC_SCALE1_R {
        DAC_SCALE1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale2(&self) -> DAC_SCALE2_R {
        DAC_SCALE2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv1(&self) -> DAC_INV1_R {
        DAC_INV1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv2(&self) -> DAC_INV2_R {
        DAC_INV2_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\]
0: to select register reg_pdac1_dac\\[7:0\\]
as source to PDAC1_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en1(&self) -> DAC_CW_EN1_R {
        DAC_CW_EN1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\]
0: to select register reg_pdac2_dac\\[7:0\\]
as source to PDAC2_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en2(&self) -> DAC_CW_EN2_R {
        DAC_CW_EN2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DC offset for DAC1 CW generator"]
    #[inline(always)]
    pub fn dac_dc1(&mut self) -> DAC_DC1_W<0> {
        DAC_DC1_W::new(self)
    }
    #[doc = "Bits 8:15 - DC offset for DAC2 CW generator"]
    #[inline(always)]
    pub fn dac_dc2(&mut self) -> DAC_DC2_W<8> {
        DAC_DC2_W::new(self)
    }
    #[doc = "Bits 16:17 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale1(&mut self) -> DAC_SCALE1_W<16> {
        DAC_SCALE1_W::new(self)
    }
    #[doc = "Bits 18:19 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale2(&mut self) -> DAC_SCALE2_W<18> {
        DAC_SCALE2_W::new(self)
    }
    #[doc = "Bits 20:21 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv1(&mut self) -> DAC_INV1_W<20> {
        DAC_INV1_W::new(self)
    }
    #[doc = "Bits 22:23 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv2(&mut self) -> DAC_INV2_W<22> {
        DAC_INV2_W::new(self)
    }
    #[doc = "Bit 24 - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\]
0: to select register reg_pdac1_dac\\[7:0\\]
as source to PDAC1_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en1(&mut self) -> DAC_CW_EN1_W<24> {
        DAC_CW_EN1_W::new(self)
    }
    #[doc = "Bit 25 - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\]
0: to select register reg_pdac2_dac\\[7:0\\]
as source to PDAC2_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en2(&mut self) -> DAC_CW_EN2_W<25> {
        DAC_CW_EN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_dac_ctrl2](index.html) module"]
pub struct SAR_DAC_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_DAC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_dac_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_DAC_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_dac_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_DAC_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_DAC_CTRL2 to value 0x0300_0000"]
impl crate::Resettable for SAR_DAC_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0000
    }
}