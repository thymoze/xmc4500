#[doc = "Register `CLEARSRCTRAN` writer"]
pub type W = crate::W<CLEARSRCTRAN_SPEC>;
#[doc = "Clear Interrupt Status and Raw Status for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_A {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Clear Interrupt Status and Raw Status for channel 0"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG, CH0_A>;
impl<'a, REG> CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_A {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Clear Interrupt Status and Raw Status for channel 1"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG, CH1_A>;
impl<'a, REG> CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_A {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Clear Interrupt Status and Raw Status for channel 2"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG, CH2_A>;
impl<'a, REG> CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_A {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Clear Interrupt Status and Raw Status for channel 3"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG, CH3_A>;
impl<'a, REG> CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Interrupt Status and Raw Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<CLEARSRCTRAN_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Interrupt Status and Raw Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<CLEARSRCTRAN_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Interrupt Status and Raw Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<CLEARSRCTRAN_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Interrupt Status and Raw Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<CLEARSRCTRAN_SPEC> {
        CH3_W::new(self, 3)
    }
}
#[doc = "IntSrcTran Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearsrctran::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLEARSRCTRAN_SPEC;
impl crate::RegisterSpec for CLEARSRCTRAN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clearsrctran::W`](W) writer structure"]
impl crate::Writable for CLEARSRCTRAN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARSRCTRAN to value 0"]
impl crate::Resettable for CLEARSRCTRAN_SPEC {}
