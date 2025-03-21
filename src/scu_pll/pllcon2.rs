#[doc = "Register `PLLCON2` reader"]
pub type R = crate::R<PLLCON2_SPEC>;
#[doc = "Register `PLLCON2` writer"]
pub type W = crate::W<PLLCON2_SPEC>;
#[doc = "P-Divider Input Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINSEL_A {
    #[doc = "0: PLL external oscillator selected"]
    VALUE1 = 0,
    #[doc = "1: Backup clock fofi selected"]
    VALUE2 = 1,
}
impl From<PINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PINSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINSEL` reader - P-Divider Input Selection"]
pub type PINSEL_R = crate::BitReader<PINSEL_A>;
impl PINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINSEL_A {
        match self.bits {
            false => PINSEL_A::VALUE1,
            true => PINSEL_A::VALUE2,
        }
    }
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PINSEL_A::VALUE1
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PINSEL_A::VALUE2
    }
}
#[doc = "Field `PINSEL` writer - P-Divider Input Selection"]
pub type PINSEL_W<'a, REG> = crate::BitWriter<'a, REG, PINSEL_A>;
impl<'a, REG> PINSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PINSEL_A::VALUE1)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PINSEL_A::VALUE2)
    }
}
#[doc = "K1-Divider Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum K1INSEL_A {
    #[doc = "0: PLL external oscillator selected"]
    VALUE1 = 0,
    #[doc = "1: Backup clock fofi selected"]
    VALUE2 = 1,
}
impl From<K1INSEL_A> for bool {
    #[inline(always)]
    fn from(variant: K1INSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `K1INSEL` reader - K1-Divider Input Selection"]
pub type K1INSEL_R = crate::BitReader<K1INSEL_A>;
impl K1INSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> K1INSEL_A {
        match self.bits {
            false => K1INSEL_A::VALUE1,
            true => K1INSEL_A::VALUE2,
        }
    }
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == K1INSEL_A::VALUE1
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == K1INSEL_A::VALUE2
    }
}
#[doc = "Field `K1INSEL` writer - K1-Divider Input Selection"]
pub type K1INSEL_W<'a, REG> = crate::BitWriter<'a, REG, K1INSEL_A>;
impl<'a, REG> K1INSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(K1INSEL_A::VALUE1)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(K1INSEL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline(always)]
    pub fn k1insel(&self) -> K1INSEL_R {
        K1INSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W<PLLCON2_SPEC> {
        PINSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline(always)]
    pub fn k1insel(&mut self) -> K1INSEL_W<PLLCON2_SPEC> {
        K1INSEL_W::new(self, 8)
    }
}
#[doc = "PLL Configuration 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcon2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcon2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCON2_SPEC;
impl crate::RegisterSpec for PLLCON2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcon2::R`](R) reader structure"]
impl crate::Readable for PLLCON2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcon2::W`](W) writer structure"]
impl crate::Writable for PLLCON2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCON2 to value 0x01"]
impl crate::Resettable for PLLCON2_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
