#[doc = "Register `EVFLAGCLR` writer"]
pub type W = crate::W<EVFLAGCLR_SPEC>;
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEC0_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC0_A> for bool {
    #[inline(always)]
    fn from(variant: RESEC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC0` writer - Result Event Clear"]
pub type RESEC0_W<'a, REG> = crate::BitWriter<'a, REG, RESEC0_A>;
impl<'a, REG> RESEC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC0_A::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC0_A::VALUE2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEC1_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC1_A> for bool {
    #[inline(always)]
    fn from(variant: RESEC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC1` writer - Result Event Clear"]
pub type RESEC1_W<'a, REG> = crate::BitWriter<'a, REG, RESEC1_A>;
impl<'a, REG> RESEC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC1_A::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC1_A::VALUE2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEC2_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC2_A> for bool {
    #[inline(always)]
    fn from(variant: RESEC2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC2` writer - Result Event Clear"]
pub type RESEC2_W<'a, REG> = crate::BitWriter<'a, REG, RESEC2_A>;
impl<'a, REG> RESEC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC2_A::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC2_A::VALUE2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEC3_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC3_A> for bool {
    #[inline(always)]
    fn from(variant: RESEC3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC3` writer - Result Event Clear"]
pub type RESEC3_W<'a, REG> = crate::BitWriter<'a, REG, RESEC3_A>;
impl<'a, REG> RESEC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC3_A::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC3_A::VALUE2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEC0_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC0_A> for bool {
    #[inline(always)]
    fn from(variant: ALEC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC0` writer - Alarm Event Clear"]
pub type ALEC0_W<'a, REG> = crate::BitWriter<'a, REG, ALEC0_A>;
impl<'a, REG> ALEC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC0_A::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC0_A::VALUE2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEC1_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC1_A> for bool {
    #[inline(always)]
    fn from(variant: ALEC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC1` writer - Alarm Event Clear"]
pub type ALEC1_W<'a, REG> = crate::BitWriter<'a, REG, ALEC1_A>;
impl<'a, REG> ALEC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC1_A::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC1_A::VALUE2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEC2_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC2_A> for bool {
    #[inline(always)]
    fn from(variant: ALEC2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC2` writer - Alarm Event Clear"]
pub type ALEC2_W<'a, REG> = crate::BitWriter<'a, REG, ALEC2_A>;
impl<'a, REG> ALEC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC2_A::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC2_A::VALUE2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEC3_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC3_A> for bool {
    #[inline(always)]
    fn from(variant: ALEC3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC3` writer - Alarm Event Clear"]
pub type ALEC3_W<'a, REG> = crate::BitWriter<'a, REG, ALEC3_A>;
impl<'a, REG> ALEC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC3_A::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC3_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event Clear"]
    #[inline(always)]
    pub fn resec0(&mut self) -> RESEC0_W<EVFLAGCLR_SPEC> {
        RESEC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Result Event Clear"]
    #[inline(always)]
    pub fn resec1(&mut self) -> RESEC1_W<EVFLAGCLR_SPEC> {
        RESEC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Result Event Clear"]
    #[inline(always)]
    pub fn resec2(&mut self) -> RESEC2_W<EVFLAGCLR_SPEC> {
        RESEC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Result Event Clear"]
    #[inline(always)]
    pub fn resec3(&mut self) -> RESEC3_W<EVFLAGCLR_SPEC> {
        RESEC3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Alarm Event Clear"]
    #[inline(always)]
    pub fn alec0(&mut self) -> ALEC0_W<EVFLAGCLR_SPEC> {
        ALEC0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Alarm Event Clear"]
    #[inline(always)]
    pub fn alec1(&mut self) -> ALEC1_W<EVFLAGCLR_SPEC> {
        ALEC1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Alarm Event Clear"]
    #[inline(always)]
    pub fn alec2(&mut self) -> ALEC2_W<EVFLAGCLR_SPEC> {
        ALEC2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Alarm Event Clear"]
    #[inline(always)]
    pub fn alec3(&mut self) -> ALEC3_W<EVFLAGCLR_SPEC> {
        ALEC3_W::new(self, 19)
    }
}
#[doc = "Event Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evflagclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVFLAGCLR_SPEC;
impl crate::RegisterSpec for EVFLAGCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`evflagclr::W`](W) writer structure"]
impl crate::Writable for EVFLAGCLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVFLAGCLR to value 0"]
impl crate::Resettable for EVFLAGCLR_SPEC {}
