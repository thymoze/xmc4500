#[doc = "Register `PCON` reader"]
pub type R = crate::R<PCON_SPEC>;
#[doc = "Register `PCON` writer"]
pub type W = crate::W<PCON_SPEC>;
#[doc = "Instruction Prefetch Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBYP_A {
    #[doc = "0: Instruction prefetch buffer not bypassed."]
    VALUE1 = 0,
    #[doc = "1: Instruction prefetch buffer bypassed."]
    VALUE2 = 1,
}
impl From<IBYP_A> for bool {
    #[inline(always)]
    fn from(variant: IBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBYP` reader - Instruction Prefetch Buffer Bypass"]
pub type IBYP_R = crate::BitReader<IBYP_A>;
impl IBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBYP_A {
        match self.bits {
            false => IBYP_A::VALUE1,
            true => IBYP_A::VALUE2,
        }
    }
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IBYP_A::VALUE1
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IBYP_A::VALUE2
    }
}
#[doc = "Field `IBYP` writer - Instruction Prefetch Buffer Bypass"]
pub type IBYP_W<'a, REG> = crate::BitWriter<'a, REG, IBYP_A>;
impl<'a, REG> IBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IBYP_A::VALUE1)
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IBYP_A::VALUE2)
    }
}
#[doc = "Instruction Prefetch Buffer Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IINV_A {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Initiate invalidation of entire instruction cache."]
    VALUE2 = 1,
}
impl From<IINV_A> for bool {
    #[inline(always)]
    fn from(variant: IINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IINV` writer - Instruction Prefetch Buffer Invalidate"]
pub type IINV_W<'a, REG> = crate::BitWriter<'a, REG, IINV_A>;
impl<'a, REG> IINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IINV_A::VALUE1)
    }
    #[doc = "Initiate invalidation of entire instruction cache."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IINV_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&self) -> IBYP_R {
        IBYP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&mut self) -> IBYP_W<PCON_SPEC> {
        IBYP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Instruction Prefetch Buffer Invalidate"]
    #[inline(always)]
    pub fn iinv(&mut self) -> IINV_W<PCON_SPEC> {
        IINV_W::new(self, 1)
    }
}
#[doc = "Prefetch Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcon::R`](R) reader structure"]
impl crate::Readable for PCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcon::W`](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {}
