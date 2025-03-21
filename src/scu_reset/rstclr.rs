#[doc = "Register `RSTCLR` writer"]
pub type W = crate::W<RSTCLR_SPEC>;
#[doc = "Clear Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSCLR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clears field RSTSTAT.RSTSTAT"]
    VALUE2 = 1,
}
impl From<RSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: RSCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSCLR` writer - Clear Reset Status"]
pub type RSCLR_W<'a, REG> = crate::BitWriter<'a, REG, RSCLR_A>;
impl<'a, REG> RSCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RSCLR_A::VALUE1)
    }
    #[doc = "Clears field RSTSTAT.RSTSTAT"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RSCLR_A::VALUE2)
    }
}
#[doc = "Clear Hibernate Wake-up Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBWK_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset status bit"]
    VALUE2 = 1,
}
impl From<HIBWK_A> for bool {
    #[inline(always)]
    fn from(variant: HIBWK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBWK` writer - Clear Hibernate Wake-up Reset Status"]
pub type HIBWK_W<'a, REG> = crate::BitWriter<'a, REG, HIBWK_A>;
impl<'a, REG> HIBWK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBWK_A::VALUE1)
    }
    #[doc = "De-assert reset status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBWK_A::VALUE2)
    }
}
#[doc = "Clear Hibernate Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBRS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<HIBRS_A> for bool {
    #[inline(always)]
    fn from(variant: HIBRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBRS` writer - Clear Hibernate Reset"]
pub type HIBRS_W<'a, REG> = crate::BitWriter<'a, REG, HIBRS_A>;
impl<'a, REG> HIBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBRS_A::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBRS_A::VALUE2)
    }
}
#[doc = "Enable Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKEN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable reset when Lockup gets asserted"]
    VALUE2 = 1,
}
impl From<LCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKEN` writer - Enable Lockup Reset"]
pub type LCKEN_W<'a, REG> = crate::BitWriter<'a, REG, LCKEN_A>;
impl<'a, REG> LCKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LCKEN_A::VALUE1)
    }
    #[doc = "Disable reset when Lockup gets asserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LCKEN_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Reset Status"]
    #[inline(always)]
    pub fn rsclr(&mut self) -> RSCLR_W<RSTCLR_SPEC> {
        RSCLR_W::new(self, 0)
    }
    #[doc = "Bit 8 - Clear Hibernate Wake-up Reset Status"]
    #[inline(always)]
    pub fn hibwk(&mut self) -> HIBWK_W<RSTCLR_SPEC> {
        HIBWK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Hibernate Reset"]
    #[inline(always)]
    pub fn hibrs(&mut self) -> HIBRS_W<RSTCLR_SPEC> {
        HIBRS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    pub fn lcken(&mut self) -> LCKEN_W<RSTCLR_SPEC> {
        LCKEN_W::new(self, 10)
    }
}
#[doc = "RCU Reset Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTCLR_SPEC;
impl crate::RegisterSpec for RSTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstclr::W`](W) writer structure"]
impl crate::Writable for RSTCLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTCLR to value 0"]
impl crate::Resettable for RSTCLR_SPEC {}
