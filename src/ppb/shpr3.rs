#[doc = "Register `SHPR3` reader"]
pub type R = crate::R<SHPR3_SPEC>;
#[doc = "Register `SHPR3` writer"]
pub type W = crate::W<SHPR3_SPEC>;
#[doc = "Field `PRI_14` reader - Priority of system handler 14"]
pub type PRI_14_R = crate::FieldReader;
#[doc = "Field `PRI_14` writer - Priority of system handler 14"]
pub type PRI_14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_15` reader - Priority of system handler 15"]
pub type PRI_15_R = crate::FieldReader;
#[doc = "Field `PRI_15` writer - Priority of system handler 15"]
pub type PRI_15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - Priority of system handler 14"]
    #[inline(always)]
    pub fn pri_14(&self) -> PRI_14_R {
        PRI_14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priority of system handler 15"]
    #[inline(always)]
    pub fn pri_15(&self) -> PRI_15_R {
        PRI_15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Priority of system handler 14"]
    #[inline(always)]
    pub fn pri_14(&mut self) -> PRI_14_W<SHPR3_SPEC> {
        PRI_14_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Priority of system handler 15"]
    #[inline(always)]
    pub fn pri_15(&mut self) -> PRI_15_W<SHPR3_SPEC> {
        PRI_15_W::new(self, 24)
    }
}
#[doc = "System Handler Priority Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`shpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHPR3_SPEC;
impl crate::RegisterSpec for SHPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr3::R`](R) reader structure"]
impl crate::Readable for SHPR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shpr3::W`](W) writer structure"]
impl crate::Writable for SHPR3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHPR3 to value 0"]
impl crate::Resettable for SHPR3_SPEC {}
