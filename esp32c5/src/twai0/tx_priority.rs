#[doc = "Register `TX_PRIORITY` reader"]
pub type R = crate::R<TX_PRIORITY_SPEC>;
#[doc = "Register `TX_PRIORITY` writer"]
pub type W = crate::W<TX_PRIORITY_SPEC>;
#[doc = "Field `TXT1P` reader - Priority of TXT buffer 1."]
pub type TXT1P_R = crate::FieldReader;
#[doc = "Field `TXT1P` writer - Priority of TXT buffer 1."]
pub type TXT1P_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXT2P` reader - Priority of TXT buffer 2."]
pub type TXT2P_R = crate::FieldReader;
#[doc = "Field `TXT2P` writer - Priority of TXT buffer 2."]
pub type TXT2P_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXT3P` reader - Priority of TXT buffer 3. If number of TXT Buffers is less than 3, this field is reserved and has no function."]
pub type TXT3P_R = crate::FieldReader;
#[doc = "Field `TXT3P` writer - Priority of TXT buffer 3. If number of TXT Buffers is less than 3, this field is reserved and has no function."]
pub type TXT3P_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXT4P` reader - Priority of TXT buffer 4. If number of TXT Buffers is less than 4, this field is reserved and has no function."]
pub type TXT4P_R = crate::FieldReader;
#[doc = "Field `TXT4P` writer - Priority of TXT buffer 4. If number of TXT Buffers is less than 4, this field is reserved and has no function."]
pub type TXT4P_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXT5P` reader - Priority of TXT buffer 5. If number of TXT Buffers is less than 5, this field is reserved and has no function."]
pub type TXT5P_R = crate::FieldReader;
#[doc = "Field `TXT5P` writer - Priority of TXT buffer 5. If number of TXT Buffers is less than 5, this field is reserved and has no function."]
pub type TXT5P_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXT6P` reader - Priority of TXT buffer 6. If number of TXT Buffers is less than 6, this field is reserved and has no function."]
pub type TXT6P_R = crate::FieldReader;
#[doc = "Field `TXT6P` writer - Priority of TXT buffer 6. If number of TXT Buffers is less than 6, this field is reserved and has no function."]
pub type TXT6P_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXT7P` reader - Priority of TXT buffer 7. If number of TXT Buffers is less than 7, this field is reserved and has no function."]
pub type TXT7P_R = crate::FieldReader;
#[doc = "Field `TXT7P` writer - Priority of TXT buffer 7. If number of TXT Buffers is less than 7, this field is reserved and has no function."]
pub type TXT7P_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXT8P` reader - Priority of TXT buffer 8. If number of TXT Buffers is less than 8, this field is reserved and has no function."]
pub type TXT8P_R = crate::FieldReader;
#[doc = "Field `TXT8P` writer - Priority of TXT buffer 8. If number of TXT Buffers is less than 8, this field is reserved and has no function."]
pub type TXT8P_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Priority of TXT buffer 1."]
    #[inline(always)]
    pub fn txt1p(&self) -> TXT1P_R {
        TXT1P_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Priority of TXT buffer 2."]
    #[inline(always)]
    pub fn txt2p(&self) -> TXT2P_R {
        TXT2P_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Priority of TXT buffer 3. If number of TXT Buffers is less than 3, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt3p(&self) -> TXT3P_R {
        TXT3P_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Priority of TXT buffer 4. If number of TXT Buffers is less than 4, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt4p(&self) -> TXT4P_R {
        TXT4P_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Priority of TXT buffer 5. If number of TXT Buffers is less than 5, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt5p(&self) -> TXT5P_R {
        TXT5P_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Priority of TXT buffer 6. If number of TXT Buffers is less than 6, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt6p(&self) -> TXT6P_R {
        TXT6P_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Priority of TXT buffer 7. If number of TXT Buffers is less than 7, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt7p(&self) -> TXT7P_R {
        TXT7P_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Priority of TXT buffer 8. If number of TXT Buffers is less than 8, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt8p(&self) -> TXT8P_R {
        TXT8P_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PRIORITY")
            .field("txt1p", &self.txt1p())
            .field("txt2p", &self.txt2p())
            .field("txt3p", &self.txt3p())
            .field("txt4p", &self.txt4p())
            .field("txt5p", &self.txt5p())
            .field("txt6p", &self.txt6p())
            .field("txt7p", &self.txt7p())
            .field("txt8p", &self.txt8p())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Priority of TXT buffer 1."]
    #[inline(always)]
    pub fn txt1p(&mut self) -> TXT1P_W<TX_PRIORITY_SPEC> {
        TXT1P_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Priority of TXT buffer 2."]
    #[inline(always)]
    pub fn txt2p(&mut self) -> TXT2P_W<TX_PRIORITY_SPEC> {
        TXT2P_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Priority of TXT buffer 3. If number of TXT Buffers is less than 3, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt3p(&mut self) -> TXT3P_W<TX_PRIORITY_SPEC> {
        TXT3P_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Priority of TXT buffer 4. If number of TXT Buffers is less than 4, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt4p(&mut self) -> TXT4P_W<TX_PRIORITY_SPEC> {
        TXT4P_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Priority of TXT buffer 5. If number of TXT Buffers is less than 5, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt5p(&mut self) -> TXT5P_W<TX_PRIORITY_SPEC> {
        TXT5P_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Priority of TXT buffer 6. If number of TXT Buffers is less than 6, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt6p(&mut self) -> TXT6P_W<TX_PRIORITY_SPEC> {
        TXT6P_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Priority of TXT buffer 7. If number of TXT Buffers is less than 7, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt7p(&mut self) -> TXT7P_W<TX_PRIORITY_SPEC> {
        TXT7P_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Priority of TXT buffer 8. If number of TXT Buffers is less than 8, this field is reserved and has no function."]
    #[inline(always)]
    pub fn txt8p(&mut self) -> TXT8P_W<TX_PRIORITY_SPEC> {
        TXT8P_W::new(self, 28)
    }
}
#[doc = "TWAI FD TXT buffer command & information register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_priority::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_priority::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_PRIORITY_SPEC;
impl crate::RegisterSpec for TX_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_priority::R`](R) reader structure"]
impl crate::Readable for TX_PRIORITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_priority::W`](W) writer structure"]
impl crate::Writable for TX_PRIORITY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_PRIORITY to value 0x01"]
impl crate::Resettable for TX_PRIORITY_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
