#[doc = "Register `TRAPSET` writer"]
pub type W = crate::W<TRAPSET_SPEC>;
#[doc = "OSC_HP Oscillator Watchdog Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSCWDGT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` writer - OSC_HP Oscillator Watchdog Trap Set"]
pub type SOSCWDGT_W<'a, REG> = crate::BitWriter<'a, REG, SOSCWDGT_A>;
impl<'a, REG> SOSCWDGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_A::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_A::VALUE2)
    }
}
#[doc = "System VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCOLCKT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Set"]
pub type SVCOLCKT_W<'a, REG> = crate::BitWriter<'a, REG, SVCOLCKT_A>;
impl<'a, REG> SVCOLCKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_A::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_A::VALUE2)
    }
}
#[doc = "USB VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVCOLCKT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Set"]
pub type UVCOLCKT_W<'a, REG> = crate::BitWriter<'a, REG, UVCOLCKT_A>;
impl<'a, REG> UVCOLCKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_A::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PET_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Set"]
pub type PET_W<'a, REG> = crate::BitWriter<'a, REG, PET_A>;
impl<'a, REG> PET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PET_A::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PET_A::VALUE2)
    }
}
#[doc = "Brown Out Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRWNT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Set"]
pub type BRWNT_W<'a, REG> = crate::BitWriter<'a, REG, BRWNT_A>;
impl<'a, REG> BRWNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_A::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_A::VALUE2)
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<ULPWDT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDT` writer - OSC_ULP Oscillator Watchdog Trap Set"]
pub type ULPWDT_W<'a, REG> = crate::BitWriter<'a, REG, ULPWDT_A>;
impl<'a, REG> ULPWDT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDT_A::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDT_A::VALUE2)
    }
}
#[doc = "Peripheral Bridge 0 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR0T_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Set"]
pub type BWERR0T_W<'a, REG> = crate::BitWriter<'a, REG, BWERR0T_A>;
impl<'a, REG> BWERR0T_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_A::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_A::VALUE2)
    }
}
#[doc = "Peripheral Bridge 1 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR1T_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Set"]
pub type BWERR1T_W<'a, REG> = crate::BitWriter<'a, REG, BWERR1T_A>;
impl<'a, REG> BWERR1T_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_A::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Set"]
    #[inline(always)]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W<TRAPSET_SPEC> {
        SOSCWDGT_W::new(self, 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Set"]
    #[inline(always)]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W<TRAPSET_SPEC> {
        SVCOLCKT_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Set"]
    #[inline(always)]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W<TRAPSET_SPEC> {
        UVCOLCKT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Parity Error Trap Set"]
    #[inline(always)]
    pub fn pet(&mut self) -> PET_W<TRAPSET_SPEC> {
        PET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Brown Out Trap Set"]
    #[inline(always)]
    pub fn brwnt(&mut self) -> BRWNT_W<TRAPSET_SPEC> {
        BRWNT_W::new(self, 5)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Set"]
    #[inline(always)]
    pub fn ulpwdt(&mut self) -> ULPWDT_W<TRAPSET_SPEC> {
        ULPWDT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Set"]
    #[inline(always)]
    pub fn bwerr0t(&mut self) -> BWERR0T_W<TRAPSET_SPEC> {
        BWERR0T_W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Set"]
    #[inline(always)]
    pub fn bwerr1t(&mut self) -> BWERR1T_W<TRAPSET_SPEC> {
        BWERR1T_W::new(self, 8)
    }
}
#[doc = "Trap Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trapset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRAPSET_SPEC;
impl crate::RegisterSpec for TRAPSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trapset::W`](W) writer structure"]
impl crate::Writable for TRAPSET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRAPSET to value 0"]
impl crate::Resettable for TRAPSET_SPEC {}
