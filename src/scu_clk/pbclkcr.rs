#[doc = "Register `PBCLKCR` reader"]
pub type R = crate::R<PBCLKCR_SPEC>;
#[doc = "Register `PBCLKCR` writer"]
pub type W = crate::W<PBCLKCR_SPEC>;
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBDIV_A {
    #[doc = "0: fPERIPH = fCPU"]
    VALUE1 = 0,
    #[doc = "1: fPERIPH = fCPU / 2"]
    VALUE2 = 1,
}
impl From<PBDIV_A> for bool {
    #[inline(always)]
    fn from(variant: PBDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBDIV` reader - PB Clock Divider Enable"]
pub type PBDIV_R = crate::BitReader<PBDIV_A>;
impl PBDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PBDIV_A {
        match self.bits {
            false => PBDIV_A::VALUE1,
            true => PBDIV_A::VALUE2,
        }
    }
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PBDIV_A::VALUE1
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PBDIV_A::VALUE2
    }
}
#[doc = "Field `PBDIV` writer - PB Clock Divider Enable"]
pub type PBDIV_W<'a, REG> = crate::BitWriter<'a, REG, PBDIV_A>;
impl<'a, REG> PBDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PBDIV_A::VALUE1)
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PBDIV_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W<PBCLKCR_SPEC> {
        PBDIV_W::new(self, 0)
    }
}
#[doc = "Peripheral Bus Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pbclkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbclkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PBCLKCR_SPEC;
impl crate::RegisterSpec for PBCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbclkcr::R`](R) reader structure"]
impl crate::Readable for PBCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pbclkcr::W`](W) writer structure"]
impl crate::Writable for PBCLKCR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PBCLKCR to value 0"]
impl crate::Resettable for PBCLKCR_SPEC {}
