#[doc = "Register `RSTSET` writer"]
pub type W = crate::W<RSTSET_SPEC>;
#[doc = "Set Hibernate Wake-up Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBWK_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset status bit"]
    VALUE2 = 1,
}
impl From<HIBWK_A> for bool {
    #[inline(always)]
    fn from(variant: HIBWK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBWK` writer - Set Hibernate Wake-up Reset Status"]
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
    #[doc = "Assert reset status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBWK_A::VALUE2)
    }
}
#[doc = "Set Hibernate Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBRS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<HIBRS_A> for bool {
    #[inline(always)]
    fn from(variant: HIBRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBRS` writer - Set Hibernate Reset"]
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
    #[doc = "Assert reset"]
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
    #[doc = "1: Enable reset when Lockup gets asserted"]
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
    #[doc = "Enable reset when Lockup gets asserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LCKEN_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 8 - Set Hibernate Wake-up Reset Status"]
    #[inline(always)]
    pub fn hibwk(&mut self) -> HIBWK_W<RSTSET_SPEC> {
        HIBWK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set Hibernate Reset"]
    #[inline(always)]
    pub fn hibrs(&mut self) -> HIBRS_W<RSTSET_SPEC> {
        HIBRS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    pub fn lcken(&mut self) -> LCKEN_W<RSTSET_SPEC> {
        LCKEN_W::new(self, 10)
    }
}
#[doc = "RCU Reset Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTSET_SPEC;
impl crate::RegisterSpec for RSTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstset::W`](W) writer structure"]
impl crate::Writable for RSTSET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTSET to value 0"]
impl crate::Resettable for RSTSET_SPEC {}
