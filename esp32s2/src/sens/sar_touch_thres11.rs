#[doc = "Register `SAR_TOUCH_THRES11` reader"]
pub type R = crate::R<SAR_TOUCH_THRES11_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES11` writer"]
pub type W = crate::W<SAR_TOUCH_THRES11_SPEC>;
#[doc = "Field `TOUCH_OUT_TH11` reader - Finger threshold for touch pad 11"]
pub type TOUCH_OUT_TH11_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_OUT_TH11` writer - Finger threshold for touch pad 11"]
pub type TOUCH_OUT_TH11_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 11"]
    #[inline(always)]
    pub fn touch_out_th11(&self) -> TOUCH_OUT_TH11_R {
        TOUCH_OUT_TH11_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES11")
            .field("touch_out_th11", &self.touch_out_th11())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 11"]
    #[inline(always)]
    pub fn touch_out_th11(&mut self) -> TOUCH_OUT_TH11_W<SAR_TOUCH_THRES11_SPEC> {
        TOUCH_OUT_TH11_W::new(self, 0)
    }
}
#[doc = "Finger threshold for touch pad 11\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES11_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres11::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres11::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES11_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES11 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES11_SPEC {}
