#[doc = "Register `TRAPRAW` reader"]
pub type R = crate::R<TRAPRAW_SPEC>;
#[doc = "OSC_HP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSCWDGT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` reader - OSC_HP Oscillator Watchdog Trap Raw Status"]
pub type SOSCWDGT_R = crate::BitReader<SOSCWDGT_A>;
impl SOSCWDGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOSCWDGT_A {
        match self.bits {
            false => SOSCWDGT_A::VALUE1,
            true => SOSCWDGT_A::VALUE2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOSCWDGT_A::VALUE1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOSCWDGT_A::VALUE2
    }
}
#[doc = "System VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` reader - System VCO Lock Trap Raw Status"]
pub type SVCOLCKT_R = crate::BitReader<SVCOLCKT_A>;
impl SVCOLCKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SVCOLCKT_A {
        match self.bits {
            false => SVCOLCKT_A::VALUE1,
            true => SVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SVCOLCKT_A::VALUE1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SVCOLCKT_A::VALUE2
    }
}
#[doc = "USB VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` reader - USB VCO Lock Trap Raw Status"]
pub type UVCOLCKT_R = crate::BitReader<UVCOLCKT_A>;
impl UVCOLCKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UVCOLCKT_A {
        match self.bits {
            false => UVCOLCKT_A::VALUE1,
            true => UVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UVCOLCKT_A::VALUE1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UVCOLCKT_A::VALUE2
    }
}
#[doc = "Parity Error Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PET_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` reader - Parity Error Trap Raw Status"]
pub type PET_R = crate::BitReader<PET_A>;
impl PET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PET_A {
        match self.bits {
            false => PET_A::VALUE1,
            true => PET_A::VALUE2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PET_A::VALUE1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PET_A::VALUE2
    }
}
#[doc = "Brown Out Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRWNT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` reader - Brown Out Trap Raw Status"]
pub type BRWNT_R = crate::BitReader<BRWNT_A>;
impl BRWNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRWNT_A {
        match self.bits {
            false => BRWNT_A::VALUE1,
            true => BRWNT_A::VALUE2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BRWNT_A::VALUE1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRWNT_A::VALUE2
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDGT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` reader - OSC_ULP Oscillator Watchdog Trap Raw Status"]
pub type ULPWDGT_R = crate::BitReader<ULPWDGT_A>;
impl ULPWDGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULPWDGT_A {
        match self.bits {
            false => ULPWDGT_A::VALUE1,
            true => ULPWDGT_A::VALUE2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDGT_A::VALUE1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDGT_A::VALUE2
    }
}
#[doc = "Peripheral Bridge 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR0T_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` reader - Peripheral Bridge 0 Trap Raw Status"]
pub type BWERR0T_R = crate::BitReader<BWERR0T_A>;
impl BWERR0T_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWERR0T_A {
        match self.bits {
            false => BWERR0T_A::VALUE1,
            true => BWERR0T_A::VALUE2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWERR0T_A::VALUE1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWERR0T_A::VALUE2
    }
}
#[doc = "Peripheral Bridge 1 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR1T_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` reader - Peripheral Bridge 1 Trap Raw Status"]
pub type BWERR1T_R = crate::BitReader<BWERR1T_A>;
impl BWERR1T_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWERR1T_A {
        match self.bits {
            false => BWERR1T_A::VALUE1,
            true => BWERR1T_A::VALUE2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWERR1T_A::VALUE1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWERR1T_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Raw Status"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Raw Status"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Trap Raw Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trapraw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRAPRAW_SPEC;
impl crate::RegisterSpec for TRAPRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trapraw::R`](R) reader structure"]
impl crate::Readable for TRAPRAW_SPEC {}
#[doc = "`reset()` method sets TRAPRAW to value 0"]
impl crate::Resettable for TRAPRAW_SPEC {}
