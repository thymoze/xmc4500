#[doc = "Register `EMUXSEL` reader"]
pub type R = crate::R<EMUXSEL_SPEC>;
#[doc = "Register `EMUXSEL` writer"]
pub type W = crate::W<EMUXSEL_SPEC>;
#[doc = "Field `EMUXGRP0` reader - External Multiplexer Group for Interface x"]
pub type EMUXGRP0_R = crate::FieldReader;
#[doc = "Field `EMUXGRP0` writer - External Multiplexer Group for Interface x"]
pub type EMUXGRP0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EMUXGRP1` reader - External Multiplexer Group for Interface x"]
pub type EMUXGRP1_R = crate::FieldReader;
#[doc = "Field `EMUXGRP1` writer - External Multiplexer Group for Interface x"]
pub type EMUXGRP1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp0(&self) -> EMUXGRP0_R {
        EMUXGRP0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp1(&self) -> EMUXGRP1_R {
        EMUXGRP1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp0(&mut self) -> EMUXGRP0_W<EMUXSEL_SPEC> {
        EMUXGRP0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp1(&mut self) -> EMUXGRP1_W<EMUXSEL_SPEC> {
        EMUXGRP1_W::new(self, 4)
    }
}
#[doc = "External Multiplexer Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`emuxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emuxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMUXSEL_SPEC;
impl crate::RegisterSpec for EMUXSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emuxsel::R`](R) reader structure"]
impl crate::Readable for EMUXSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emuxsel::W`](W) writer structure"]
impl crate::Writable for EMUXSEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMUXSEL to value 0"]
impl crate::Resettable for EMUXSEL_SPEC {}
