#[doc = "Register `MOIPR` reader"]
pub type R = crate::R<MOIPR_SPEC>;
#[doc = "Register `MOIPR` writer"]
pub type W = crate::W<MOIPR_SPEC>;
#[doc = "Receive Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<RXINP_A> for u8 {
    #[inline(always)]
    fn from(variant: RXINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXINP_A {
    type Ux = u8;
}
impl crate::IsEnum for RXINP_A {}
#[doc = "Field `RXINP` reader - Receive Interrupt Node Pointer"]
pub type RXINP_R = crate::FieldReader<RXINP_A>;
impl RXINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXINP_A> {
        match self.bits {
            0 => Some(RXINP_A::VALUE1),
            1 => Some(RXINP_A::VALUE2),
            7 => Some(RXINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXINP_A::VALUE1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXINP_A::VALUE2
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXINP_A::VALUE3
    }
}
#[doc = "Field `RXINP` writer - Receive Interrupt Node Pointer"]
pub type RXINP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RXINP_A>;
impl<'a, REG> RXINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RXINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RXINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RXINP_A::VALUE3)
    }
}
#[doc = "Transmit Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<TXINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TXINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXINP_A {
    type Ux = u8;
}
impl crate::IsEnum for TXINP_A {}
#[doc = "Field `TXINP` reader - Transmit Interrupt Node Pointer"]
pub type TXINP_R = crate::FieldReader<TXINP_A>;
impl TXINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXINP_A> {
        match self.bits {
            0 => Some(TXINP_A::VALUE1),
            1 => Some(TXINP_A::VALUE2),
            7 => Some(TXINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXINP_A::VALUE1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXINP_A::VALUE2
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TXINP_A::VALUE3
    }
}
#[doc = "Field `TXINP` writer - Transmit Interrupt Node Pointer"]
pub type TXINP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TXINP_A>;
impl<'a, REG> TXINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TXINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TXINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TXINP_A::VALUE3)
    }
}
#[doc = "Field `MPN` reader - Message Pending Number"]
pub type MPN_R = crate::FieldReader;
#[doc = "Field `MPN` writer - Message Pending Number"]
pub type MPN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFCVAL` reader - CAN Frame Counter Value"]
pub type CFCVAL_R = crate::FieldReader<u16>;
#[doc = "Field `CFCVAL` writer - CAN Frame Counter Value"]
pub type CFCVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&self) -> RXINP_R {
        RXINP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&self) -> TXINP_R {
        TXINP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&self) -> MPN_R {
        MPN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&self) -> CFCVAL_R {
        CFCVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&mut self) -> RXINP_W<MOIPR_SPEC> {
        RXINP_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&mut self) -> TXINP_W<MOIPR_SPEC> {
        TXINP_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&mut self) -> MPN_W<MOIPR_SPEC> {
        MPN_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&mut self) -> CFCVAL_W<MOIPR_SPEC> {
        CFCVAL_W::new(self, 16)
    }
}
#[doc = "Message Object Interrupt Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`moipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOIPR_SPEC;
impl crate::RegisterSpec for MOIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moipr::R`](R) reader structure"]
impl crate::Readable for MOIPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`moipr::W`](W) writer structure"]
impl crate::Writable for MOIPR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOIPR to value 0"]
impl crate::Resettable for MOIPR_SPEC {}
