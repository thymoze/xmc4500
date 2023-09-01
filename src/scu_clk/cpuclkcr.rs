#[doc = "Register `CPUCLKCR` reader"]
pub type R = crate::R<CPUCLKCR_SPEC>;
#[doc = "Register `CPUCLKCR` writer"]
pub type W = crate::W<CPUCLKCR_SPEC>;
#[doc = "Field `CPUDIV` reader - CPU Clock Divider Enable"]
pub type CPUDIV_R = crate::BitReader<CPUDIV_A>;
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPUDIV_A {
    #[doc = "0: fCPU = fSYS"]
    VALUE1 = 0,
    #[doc = "1: fCPU = fSYS / 2"]
    VALUE2 = 1,
}
impl From<CPUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CPUDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl CPUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUDIV_A {
        match self.bits {
            false => CPUDIV_A::VALUE1,
            true => CPUDIV_A::VALUE2,
        }
    }
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CPUDIV_A::VALUE1
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CPUDIV_A::VALUE2
    }
}
#[doc = "Field `CPUDIV` writer - CPU Clock Divider Enable"]
pub type CPUDIV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPUDIV_A>;
impl<'a, REG, const O: u8> CPUDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIV_A::VALUE1)
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIV_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CPUDIV_W<CPUCLKCR_SPEC, 0> {
        CPUDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CPU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUCLKCR_SPEC;
impl crate::RegisterSpec for CPUCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuclkcr::R`](R) reader structure"]
impl crate::Readable for CPUCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpuclkcr::W`](W) writer structure"]
impl crate::Writable for CPUCLKCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUCLKCR to value 0"]
impl crate::Resettable for CPUCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
