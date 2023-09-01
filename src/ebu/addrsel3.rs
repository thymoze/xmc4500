#[doc = "Register `ADDRSEL3` reader"]
pub type R = crate::R<ADDRSEL3_SPEC>;
#[doc = "Register `ADDRSEL3` writer"]
pub type W = crate::W<ADDRSEL3_SPEC>;
#[doc = "Field `REGENAB` reader - Memory Region Enable"]
pub type REGENAB_R = crate::BitReader<REGENAB_A>;
#[doc = "Memory Region Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGENAB_A {
    #[doc = "0: Memory region is disabled (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: Memory region is enabled."]
    VALUE2 = 1,
}
impl From<REGENAB_A> for bool {
    #[inline(always)]
    fn from(variant: REGENAB_A) -> Self {
        variant as u8 != 0
    }
}
impl REGENAB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGENAB_A {
        match self.bits {
            false => REGENAB_A::VALUE1,
            true => REGENAB_A::VALUE2,
        }
    }
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REGENAB_A::VALUE1
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REGENAB_A::VALUE2
    }
}
#[doc = "Field `REGENAB` writer - Memory Region Enable"]
pub type REGENAB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, REGENAB_A>;
impl<'a, REG, const O: u8> REGENAB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REGENAB_A::VALUE1)
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REGENAB_A::VALUE2)
    }
}
#[doc = "Field `ALTENAB` reader - Alternate Region Enable"]
pub type ALTENAB_R = crate::BitReader<ALTENAB_A>;
#[doc = "Alternate Region Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALTENAB_A {
    #[doc = "0: Memory region is disabled (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: Memory region is enabled."]
    VALUE2 = 1,
}
impl From<ALTENAB_A> for bool {
    #[inline(always)]
    fn from(variant: ALTENAB_A) -> Self {
        variant as u8 != 0
    }
}
impl ALTENAB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALTENAB_A {
        match self.bits {
            false => ALTENAB_A::VALUE1,
            true => ALTENAB_A::VALUE2,
        }
    }
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALTENAB_A::VALUE1
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALTENAB_A::VALUE2
    }
}
#[doc = "Field `ALTENAB` writer - Alternate Region Enable"]
pub type ALTENAB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALTENAB_A>;
impl<'a, REG, const O: u8> ALTENAB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALTENAB_A::VALUE1)
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALTENAB_A::VALUE2)
    }
}
#[doc = "Field `WPROT` reader - Memory Region Write Protect"]
pub type WPROT_R = crate::BitReader<WPROT_A>;
#[doc = "Memory Region Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPROT_A {
    #[doc = "0: Region is enabled for write accesses"]
    VALUE1 = 0,
    #[doc = "1: Region is write protected."]
    VALUE2 = 1,
}
impl From<WPROT_A> for bool {
    #[inline(always)]
    fn from(variant: WPROT_A) -> Self {
        variant as u8 != 0
    }
}
impl WPROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROT_A {
        match self.bits {
            false => WPROT_A::VALUE1,
            true => WPROT_A::VALUE2,
        }
    }
    #[doc = "Region is enabled for write accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPROT_A::VALUE1
    }
    #[doc = "Region is write protected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPROT_A::VALUE2
    }
}
#[doc = "Field `WPROT` writer - Memory Region Write Protect"]
pub type WPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WPROT_A>;
impl<'a, REG, const O: u8> WPROT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Region is enabled for write accesses"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WPROT_A::VALUE1)
    }
    #[doc = "Region is write protected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WPROT_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Region Enable"]
    #[inline(always)]
    pub fn regenab(&self) -> REGENAB_R {
        REGENAB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alternate Region Enable"]
    #[inline(always)]
    pub fn altenab(&self) -> ALTENAB_R {
        ALTENAB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory Region Write Protect"]
    #[inline(always)]
    pub fn wprot(&self) -> WPROT_R {
        WPROT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Region Enable"]
    #[inline(always)]
    #[must_use]
    pub fn regenab(&mut self) -> REGENAB_W<ADDRSEL3_SPEC, 0> {
        REGENAB_W::new(self)
    }
    #[doc = "Bit 1 - Alternate Region Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altenab(&mut self) -> ALTENAB_W<ADDRSEL3_SPEC, 1> {
        ALTENAB_W::new(self)
    }
    #[doc = "Bit 2 - Memory Region Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wprot(&mut self) -> WPROT_W<ADDRSEL3_SPEC, 2> {
        WPROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EBU Address Select Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDRSEL3_SPEC;
impl crate::RegisterSpec for ADDRSEL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrsel3::R`](R) reader structure"]
impl crate::Readable for ADDRSEL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addrsel3::W`](W) writer structure"]
impl crate::Writable for ADDRSEL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDRSEL3 to value 0"]
impl crate::Resettable for ADDRSEL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
