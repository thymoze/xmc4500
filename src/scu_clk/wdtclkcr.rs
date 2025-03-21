#[doc = "Register `WDTCLKCR` reader"]
pub type R = crate::R<WDTCLKCR_SPEC>;
#[doc = "Register `WDTCLKCR` writer"]
pub type W = crate::W<WDTCLKCR_SPEC>;
#[doc = "Field `WDTDIV` reader - WDT Clock Divider Value"]
pub type WDTDIV_R = crate::FieldReader;
#[doc = "Field `WDTDIV` writer - WDT Clock Divider Value"]
pub type WDTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTSEL_A {
    #[doc = "0: fOFI clock"]
    VALUE1 = 0,
    #[doc = "1: fSTDBY clock"]
    VALUE2 = 1,
    #[doc = "2: fPLL clock"]
    VALUE3 = 2,
}
impl From<WDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDTSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for WDTSEL_A {}
#[doc = "Field `WDTSEL` reader - WDT Clock Selection Value"]
pub type WDTSEL_R = crate::FieldReader<WDTSEL_A>;
impl WDTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDTSEL_A> {
        match self.bits {
            0 => Some(WDTSEL_A::VALUE1),
            1 => Some(WDTSEL_A::VALUE2),
            2 => Some(WDTSEL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDTSEL_A::VALUE1
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDTSEL_A::VALUE2
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WDTSEL_A::VALUE3
    }
}
#[doc = "Field `WDTSEL` writer - WDT Clock Selection Value"]
pub type WDTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WDTSEL_A>;
impl<'a, REG> WDTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::VALUE1)
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::VALUE2)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&mut self) -> WDTDIV_W<WDTCLKCR_SPEC> {
        WDTDIV_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&mut self) -> WDTSEL_W<WDTCLKCR_SPEC> {
        WDTSEL_W::new(self, 16)
    }
}
#[doc = "WDT Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtclkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtclkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCLKCR_SPEC;
impl crate::RegisterSpec for WDTCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtclkcr::R`](R) reader structure"]
impl crate::Readable for WDTCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtclkcr::W`](W) writer structure"]
impl crate::Writable for WDTCLKCR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCLKCR to value 0"]
impl crate::Resettable for WDTCLKCR_SPEC {}
