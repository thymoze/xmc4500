#[doc = "Register `NIPR` reader"]
pub type R = crate::R<NIPR_SPEC>;
#[doc = "Register `NIPR` writer"]
pub type W = crate::W<NIPR_SPEC>;
#[doc = "Alert Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<ALINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ALINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALINP_A {
    type Ux = u8;
}
impl crate::IsEnum for ALINP_A {}
#[doc = "Field `ALINP` reader - Alert Interrupt Node Pointer"]
pub type ALINP_R = crate::FieldReader<ALINP_A>;
impl ALINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALINP_A> {
        match self.bits {
            0 => Some(ALINP_A::VALUE1),
            1 => Some(ALINP_A::VALUE2),
            7 => Some(ALINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALINP_A::VALUE1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALINP_A::VALUE2
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ALINP_A::VALUE3
    }
}
#[doc = "Field `ALINP` writer - Alert Interrupt Node Pointer"]
pub type ALINP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ALINP_A>;
impl<'a, REG> ALINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ALINP_A::VALUE3)
    }
}
#[doc = "Last Error Code Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LECINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<LECINP_A> for u8 {
    #[inline(always)]
    fn from(variant: LECINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LECINP_A {
    type Ux = u8;
}
impl crate::IsEnum for LECINP_A {}
#[doc = "Field `LECINP` reader - Last Error Code Interrupt Node Pointer"]
pub type LECINP_R = crate::FieldReader<LECINP_A>;
impl LECINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LECINP_A> {
        match self.bits {
            0 => Some(LECINP_A::VALUE1),
            1 => Some(LECINP_A::VALUE2),
            7 => Some(LECINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LECINP_A::VALUE1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LECINP_A::VALUE2
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LECINP_A::VALUE3
    }
}
#[doc = "Field `LECINP` writer - Last Error Code Interrupt Node Pointer"]
pub type LECINP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LECINP_A>;
impl<'a, REG> LECINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LECINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LECINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LECINP_A::VALUE3)
    }
}
#[doc = "Transfer OK Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<TRINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TRINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRINP_A {
    type Ux = u8;
}
impl crate::IsEnum for TRINP_A {}
#[doc = "Field `TRINP` reader - Transfer OK Interrupt Node Pointer"]
pub type TRINP_R = crate::FieldReader<TRINP_A>;
impl TRINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRINP_A> {
        match self.bits {
            0 => Some(TRINP_A::VALUE1),
            1 => Some(TRINP_A::VALUE2),
            7 => Some(TRINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRINP_A::VALUE1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRINP_A::VALUE2
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRINP_A::VALUE3
    }
}
#[doc = "Field `TRINP` writer - Transfer OK Interrupt Node Pointer"]
pub type TRINP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TRINP_A>;
impl<'a, REG> TRINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TRINP_A::VALUE3)
    }
}
#[doc = "Frame Counter Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFCINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<CFCINP_A> for u8 {
    #[inline(always)]
    fn from(variant: CFCINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFCINP_A {
    type Ux = u8;
}
impl crate::IsEnum for CFCINP_A {}
#[doc = "Field `CFCINP` reader - Frame Counter Interrupt Node Pointer"]
pub type CFCINP_R = crate::FieldReader<CFCINP_A>;
impl CFCINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CFCINP_A> {
        match self.bits {
            0 => Some(CFCINP_A::VALUE1),
            1 => Some(CFCINP_A::VALUE2),
            7 => Some(CFCINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCINP_A::VALUE1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCINP_A::VALUE2
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFCINP_A::VALUE3
    }
}
#[doc = "Field `CFCINP` writer - Frame Counter Interrupt Node Pointer"]
pub type CFCINP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CFCINP_A>;
impl<'a, REG> CFCINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CFCINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CFCINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CFCINP_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&self) -> ALINP_R {
        ALINP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&self) -> LECINP_R {
        LECINP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&self) -> TRINP_R {
        TRINP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CFCINP_R {
        CFCINP_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&mut self) -> ALINP_W<NIPR_SPEC> {
        ALINP_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&mut self) -> LECINP_W<NIPR_SPEC> {
        LECINP_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&mut self) -> TRINP_W<NIPR_SPEC> {
        TRINP_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&mut self) -> CFCINP_W<NIPR_SPEC> {
        CFCINP_W::new(self, 12)
    }
}
#[doc = "Node Interrupt Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NIPR_SPEC;
impl crate::RegisterSpec for NIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nipr::R`](R) reader structure"]
impl crate::Readable for NIPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nipr::W`](W) writer structure"]
impl crate::Writable for NIPR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NIPR to value 0"]
impl crate::Resettable for NIPR_SPEC {}
