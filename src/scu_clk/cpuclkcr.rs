#[doc = "Register `CPUCLKCR` reader"]
pub type R = crate::R<CPUCLKCR_SPEC>;
#[doc = "Register `CPUCLKCR` writer"]
pub type W = crate::W<CPUCLKCR_SPEC>;
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
#[doc = "Field `CPUDIV` reader - CPU Clock Divider Enable"]
pub type CPUDIV_R = crate::BitReader<CPUDIV_A>;
impl CPUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPUDIV_A {
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
pub type CPUDIV_W<'a, REG> = crate::BitWriter<'a, REG, CPUDIV_A>;
impl<'a, REG> CPUDIV_W<'a, REG>
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
    pub fn cpudiv(&mut self) -> CPUDIV_W<CPUCLKCR_SPEC> {
        CPUDIV_W::new(self, 0)
    }
}
#[doc = "CPU Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuclkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuclkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUCLKCR_SPEC;
impl crate::RegisterSpec for CPUCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuclkcr::R`](R) reader structure"]
impl crate::Readable for CPUCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpuclkcr::W`](W) writer structure"]
impl crate::Writable for CPUCLKCR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUCLKCR to value 0"]
impl crate::Resettable for CPUCLKCR_SPEC {}
