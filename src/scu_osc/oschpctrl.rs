#[doc = "Register `OSCHPCTRL` reader"]
pub type R = crate::R<OSCHPCTRL_SPEC>;
#[doc = "Register `OSCHPCTRL` writer"]
pub type W = crate::W<OSCHPCTRL_SPEC>;
#[doc = "XTAL1 Data Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X1DEN_A {
    #[doc = "0: Bit X1D is not updated"]
    VALUE1 = 0,
    #[doc = "1: Bit X1D can be updated"]
    VALUE2 = 1,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `X1DEN` reader - XTAL1 Data Enable"]
pub type X1DEN_R = crate::BitReader<X1DEN_A>;
impl X1DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> X1DEN_A {
        match self.bits {
            false => X1DEN_A::VALUE1,
            true => X1DEN_A::VALUE2,
        }
    }
    #[doc = "Bit X1D is not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == X1DEN_A::VALUE1
    }
    #[doc = "Bit X1D can be updated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == X1DEN_A::VALUE2
    }
}
#[doc = "Field `X1DEN` writer - XTAL1 Data Enable"]
pub type X1DEN_W<'a, REG> = crate::BitWriter<'a, REG, X1DEN_A>;
impl<'a, REG> X1DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit X1D is not updated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(X1DEN_A::VALUE1)
    }
    #[doc = "Bit X1D can be updated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(X1DEN_A::VALUE2)
    }
}
#[doc = "Shaper Bypass\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHBY_A {
    #[doc = "0: The shaper is not bypassed"]
    VALUE1 = 0,
    #[doc = "1: The shaper is bypassed"]
    VALUE2 = 1,
}
impl From<SHBY_A> for bool {
    #[inline(always)]
    fn from(variant: SHBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHBY` reader - Shaper Bypass"]
pub type SHBY_R = crate::BitReader<SHBY_A>;
impl SHBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SHBY_A {
        match self.bits {
            false => SHBY_A::VALUE1,
            true => SHBY_A::VALUE2,
        }
    }
    #[doc = "The shaper is not bypassed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SHBY_A::VALUE1
    }
    #[doc = "The shaper is bypassed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SHBY_A::VALUE2
    }
}
#[doc = "Field `SHBY` writer - Shaper Bypass"]
pub type SHBY_W<'a, REG> = crate::BitWriter<'a, REG, SHBY_A>;
impl<'a, REG> SHBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The shaper is not bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SHBY_A::VALUE1)
    }
    #[doc = "The shaper is bypassed"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SHBY_A::VALUE2)
    }
}
#[doc = "Oscillator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    VALUE1 = 0,
    #[doc = "1: OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    VALUE2 = 1,
    #[doc = "2: External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    VALUE3 = 2,
    #[doc = "3: OSC is disabled. The oscillator Power-Saving Mode is entered."]
    VALUE4 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE_A {}
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::VALUE1,
            1 => MODE_A::VALUE2,
            2 => MODE_A::VALUE3,
            3 => MODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODE_A::VALUE1
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODE_A::VALUE2
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MODE_A::VALUE3
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MODE_A::VALUE4
    }
}
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A, crate::Safe>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE3)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE4)
    }
}
#[doc = "Field `OSCVAL` reader - OSC Frequency Value"]
pub type OSCVAL_R = crate::FieldReader;
#[doc = "Field `OSCVAL` writer - OSC Frequency Value"]
pub type OSCVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1DEN_R {
        X1DEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&self) -> SHBY_R {
        SHBY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&self) -> OSCVAL_R {
        OSCVAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&mut self) -> X1DEN_W<OSCHPCTRL_SPEC> {
        X1DEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&mut self) -> SHBY_W<OSCHPCTRL_SPEC> {
        SHBY_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<OSCHPCTRL_SPEC> {
        MODE_W::new(self, 4)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&mut self) -> OSCVAL_W<OSCHPCTRL_SPEC> {
        OSCVAL_W::new(self, 16)
    }
}
#[doc = "OSC_HP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oschpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oschpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCHPCTRL_SPEC;
impl crate::RegisterSpec for OSCHPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oschpctrl::R`](R) reader structure"]
impl crate::Readable for OSCHPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oschpctrl::W`](W) writer structure"]
impl crate::Writable for OSCHPCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSCHPCTRL to value 0x33"]
impl crate::Resettable for OSCHPCTRL_SPEC {
    const RESET_VALUE: u32 = 0x33;
}
