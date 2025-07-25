#[doc = "Register `COMD%s` reader"]
pub type R = crate::R<COMD_SPEC>;
#[doc = "Register `COMD%s` writer"]
pub type W = crate::W<COMD_SPEC>;
#[doc = "Field `COMMAND` reader - Configures command 0. \\\\ It consists of three parts:\\\\ op_code is the command\\\\ 1: WRITE\\\\ 2: STOP\\\\ 3: READ\\\\ 4: END\\\\ 6: RSTART\\\\ Byte_num represents the number of bytes that need to be sent or received.\\\\ ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure <a href=fig:i2c-cmd-structure\">link</a> for more information. \\\\\\tododone{for CJ, please add a hyperlink for I2C CMD structure.CJ: done.}\""]
pub type COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND` writer - Configures command 0. \\\\ It consists of three parts:\\\\ op_code is the command\\\\ 1: WRITE\\\\ 2: STOP\\\\ 3: READ\\\\ 4: END\\\\ 6: RSTART\\\\ Byte_num represents the number of bytes that need to be sent or received.\\\\ ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure <a href=fig:i2c-cmd-structure\">link</a> for more information. \\\\\\tododone{for CJ, please add a hyperlink for I2C CMD structure.CJ: done.}\""]
pub type COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND_DONE` reader - Represents whether command 0 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type COMMAND_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND_DONE` writer - Represents whether command 0 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type COMMAND_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Configures command 0. \\\\ It consists of three parts:\\\\ op_code is the command\\\\ 1: WRITE\\\\ 2: STOP\\\\ 3: READ\\\\ 4: END\\\\ 6: RSTART\\\\ Byte_num represents the number of bytes that need to be sent or received.\\\\ ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure <a href=fig:i2c-cmd-structure\">link</a> for more information. \\\\\\tododone{for CJ, please add a hyperlink for I2C CMD structure.CJ: done.}\""]
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Represents whether command 0 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD")
            .field("command", &self.command())
            .field("command_done", &self.command_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Configures command 0. \\\\ It consists of three parts:\\\\ op_code is the command\\\\ 1: WRITE\\\\ 2: STOP\\\\ 3: READ\\\\ 4: END\\\\ 6: RSTART\\\\ Byte_num represents the number of bytes that need to be sent or received.\\\\ ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure <a href=fig:i2c-cmd-structure\">link</a> for more information. \\\\\\tododone{for CJ, please add a hyperlink for I2C CMD structure.CJ: done.}\""]
    #[inline(always)]
    pub fn command(&mut self) -> COMMAND_W<COMD_SPEC> {
        COMMAND_W::new(self, 0)
    }
    #[doc = "Bit 31 - Represents whether command 0 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn command_done(&mut self) -> COMMAND_DONE_W<COMD_SPEC> {
        COMMAND_DONE_W::new(self, 31)
    }
}
#[doc = "I2C command register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`comd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD_SPEC;
impl crate::RegisterSpec for COMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd::R`](R) reader structure"]
impl crate::Readable for COMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd::W`](W) writer structure"]
impl crate::Writable for COMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMD%s to value 0"]
impl crate::Resettable for COMD_SPEC {}
