#[doc = "Register `SHARP_MATRIX_CTRL` reader"]
pub type R = crate::R<SHARP_MATRIX_CTRL_SPEC>;
#[doc = "Register `SHARP_MATRIX_CTRL` writer"]
pub type W = crate::W<SHARP_MATRIX_CTRL_SPEC>;
#[doc = "Field `SHARP_TAIL_PIXEN_PULSE_TL` reader - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th < reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
pub type SHARP_TAIL_PIXEN_PULSE_TL_R = crate::FieldReader;
#[doc = "Field `SHARP_TAIL_PIXEN_PULSE_TL` writer - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th < reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
pub type SHARP_TAIL_PIXEN_PULSE_TL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_TAIL_PIXEN_PULSE_TH` reader - matrix tail pixen high level threshold, must < hnum-1, only reg_sharp_tail_pixen_pulse_th!=0 and reg_sharp_tail_pixen_pulse_tl!=0 and reg_sharp_tail_pixen_pulse_th < reg_sharp_tail_pixen_pulse_tl will enable tail pulse function"]
pub type SHARP_TAIL_PIXEN_PULSE_TH_R = crate::FieldReader;
#[doc = "Field `SHARP_TAIL_PIXEN_PULSE_TH` writer - matrix tail pixen high level threshold, must < hnum-1, only reg_sharp_tail_pixen_pulse_th!=0 and reg_sharp_tail_pixen_pulse_tl!=0 and reg_sharp_tail_pixen_pulse_th < reg_sharp_tail_pixen_pulse_tl will enable tail pulse function"]
pub type SHARP_TAIL_PIXEN_PULSE_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_PADDING_DATA` reader - this field configures sharp padding data"]
pub type SHARP_PADDING_DATA_R = crate::FieldReader;
#[doc = "Field `SHARP_PADDING_DATA` writer - this field configures sharp padding data"]
pub type SHARP_PADDING_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_PADDING_MODE` reader - this field configures sharp padding mode"]
pub type SHARP_PADDING_MODE_R = crate::BitReader;
#[doc = "Field `SHARP_PADDING_MODE` writer - this field configures sharp padding mode"]
pub type SHARP_PADDING_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th < reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn sharp_tail_pixen_pulse_tl(&self) -> SHARP_TAIL_PIXEN_PULSE_TL_R {
        SHARP_TAIL_PIXEN_PULSE_TL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - matrix tail pixen high level threshold, must < hnum-1, only reg_sharp_tail_pixen_pulse_th!=0 and reg_sharp_tail_pixen_pulse_tl!=0 and reg_sharp_tail_pixen_pulse_th < reg_sharp_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn sharp_tail_pixen_pulse_th(&self) -> SHARP_TAIL_PIXEN_PULSE_TH_R {
        SHARP_TAIL_PIXEN_PULSE_TH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures sharp padding data"]
    #[inline(always)]
    pub fn sharp_padding_data(&self) -> SHARP_PADDING_DATA_R {
        SHARP_PADDING_DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - this field configures sharp padding mode"]
    #[inline(always)]
    pub fn sharp_padding_mode(&self) -> SHARP_PADDING_MODE_R {
        SHARP_PADDING_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHARP_MATRIX_CTRL")
            .field(
                "sharp_tail_pixen_pulse_tl",
                &self.sharp_tail_pixen_pulse_tl(),
            )
            .field(
                "sharp_tail_pixen_pulse_th",
                &self.sharp_tail_pixen_pulse_th(),
            )
            .field("sharp_padding_data", &self.sharp_padding_data())
            .field("sharp_padding_mode", &self.sharp_padding_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th < reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn sharp_tail_pixen_pulse_tl(
        &mut self,
    ) -> SHARP_TAIL_PIXEN_PULSE_TL_W<SHARP_MATRIX_CTRL_SPEC> {
        SHARP_TAIL_PIXEN_PULSE_TL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - matrix tail pixen high level threshold, must < hnum-1, only reg_sharp_tail_pixen_pulse_th!=0 and reg_sharp_tail_pixen_pulse_tl!=0 and reg_sharp_tail_pixen_pulse_th < reg_sharp_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn sharp_tail_pixen_pulse_th(
        &mut self,
    ) -> SHARP_TAIL_PIXEN_PULSE_TH_W<SHARP_MATRIX_CTRL_SPEC> {
        SHARP_TAIL_PIXEN_PULSE_TH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures sharp padding data"]
    #[inline(always)]
    pub fn sharp_padding_data(&mut self) -> SHARP_PADDING_DATA_W<SHARP_MATRIX_CTRL_SPEC> {
        SHARP_PADDING_DATA_W::new(self, 16)
    }
    #[doc = "Bit 24 - this field configures sharp padding mode"]
    #[inline(always)]
    pub fn sharp_padding_mode(&mut self) -> SHARP_PADDING_MODE_W<SHARP_MATRIX_CTRL_SPEC> {
        SHARP_PADDING_MODE_W::new(self, 24)
    }
}
#[doc = "sharp pix2matrix ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_matrix_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharp_matrix_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHARP_MATRIX_CTRL_SPEC;
impl crate::RegisterSpec for SHARP_MATRIX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sharp_matrix_ctrl::R`](R) reader structure"]
impl crate::Readable for SHARP_MATRIX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sharp_matrix_ctrl::W`](W) writer structure"]
impl crate::Writable for SHARP_MATRIX_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHARP_MATRIX_CTRL to value 0"]
impl crate::Resettable for SHARP_MATRIX_CTRL_SPEC {}
