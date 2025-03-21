#[doc = "Register `EBUCLKCR` reader"]
pub type R = crate::R<EBUCLKCR_SPEC>;
#[doc = "Register `EBUCLKCR` writer"]
pub type W = crate::W<EBUCLKCR_SPEC>;
#[doc = "Field `EBUDIV` reader - EBU Clock Divider Value"]
pub type EBUDIV_R = crate::FieldReader;
#[doc = "Field `EBUDIV` writer - EBU Clock Divider Value"]
pub type EBUDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - EBU Clock Divider Value"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EBUDIV_R {
        EBUDIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - EBU Clock Divider Value"]
    #[inline(always)]
    pub fn ebudiv(&mut self) -> EBUDIV_W<EBUCLKCR_SPEC> {
        EBUDIV_W::new(self, 0)
    }
}
#[doc = "EBU Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ebuclkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebuclkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EBUCLKCR_SPEC;
impl crate::RegisterSpec for EBUCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ebuclkcr::R`](R) reader structure"]
impl crate::Readable for EBUCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ebuclkcr::W`](W) writer structure"]
impl crate::Writable for EBUCLKCR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EBUCLKCR to value 0"]
impl crate::Resettable for EBUCLKCR_SPEC {}
