#[doc = "Register `PROCON0` reader"]
pub type R = crate::R<PROCON0_SPEC>;
#[doc = "Sector 0 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S0L_A> for bool {
    #[inline(always)]
    fn from(variant: S0L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0L` reader - Sector 0 Locked for Write Protection by User 0"]
pub type S0L_R = crate::BitReader<S0L_A>;
impl S0L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0L_A {
        match self.bits {
            false => S0L_A::VALUE1,
            true => S0L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0L_A::VALUE2
    }
}
#[doc = "Sector 1 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S1L_A> for bool {
    #[inline(always)]
    fn from(variant: S1L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1L` reader - Sector 1 Locked for Write Protection by User 0"]
pub type S1L_R = crate::BitReader<S1L_A>;
impl S1L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1L_A {
        match self.bits {
            false => S1L_A::VALUE1,
            true => S1L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1L_A::VALUE2
    }
}
#[doc = "Sector 2 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S2L_A> for bool {
    #[inline(always)]
    fn from(variant: S2L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2L` reader - Sector 2 Locked for Write Protection by User 0"]
pub type S2L_R = crate::BitReader<S2L_A>;
impl S2L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2L_A {
        match self.bits {
            false => S2L_A::VALUE1,
            true => S2L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2L_A::VALUE2
    }
}
#[doc = "Sector 3 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S3L_A> for bool {
    #[inline(always)]
    fn from(variant: S3L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3L` reader - Sector 3 Locked for Write Protection by User 0"]
pub type S3L_R = crate::BitReader<S3L_A>;
impl S3L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3L_A {
        match self.bits {
            false => S3L_A::VALUE1,
            true => S3L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3L_A::VALUE2
    }
}
#[doc = "Sector 4 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S4L_A> for bool {
    #[inline(always)]
    fn from(variant: S4L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4L` reader - Sector 4 Locked for Write Protection by User 0"]
pub type S4L_R = crate::BitReader<S4L_A>;
impl S4L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S4L_A {
        match self.bits {
            false => S4L_A::VALUE1,
            true => S4L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S4L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S4L_A::VALUE2
    }
}
#[doc = "Sector 5 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S5L_A> for bool {
    #[inline(always)]
    fn from(variant: S5L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5L` reader - Sector 5 Locked for Write Protection by User 0"]
pub type S5L_R = crate::BitReader<S5L_A>;
impl S5L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S5L_A {
        match self.bits {
            false => S5L_A::VALUE1,
            true => S5L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S5L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S5L_A::VALUE2
    }
}
#[doc = "Sector 6 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S6L_A> for bool {
    #[inline(always)]
    fn from(variant: S6L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6L` reader - Sector 6 Locked for Write Protection by User 0"]
pub type S6L_R = crate::BitReader<S6L_A>;
impl S6L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S6L_A {
        match self.bits {
            false => S6L_A::VALUE1,
            true => S6L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S6L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S6L_A::VALUE2
    }
}
#[doc = "Sector 7 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S7L_A> for bool {
    #[inline(always)]
    fn from(variant: S7L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7L` reader - Sector 7 Locked for Write Protection by User 0"]
pub type S7L_R = crate::BitReader<S7L_A>;
impl S7L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S7L_A {
        match self.bits {
            false => S7L_A::VALUE1,
            true => S7L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S7L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S7L_A::VALUE2
    }
}
#[doc = "Sector 8 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S8L_A> for bool {
    #[inline(always)]
    fn from(variant: S8L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8L` reader - Sector 8 Locked for Write Protection by User 0"]
pub type S8L_R = crate::BitReader<S8L_A>;
impl S8L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S8L_A {
        match self.bits {
            false => S8L_A::VALUE1,
            true => S8L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S8L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S8L_A::VALUE2
    }
}
#[doc = "Sector 9 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S9L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S9L_A> for bool {
    #[inline(always)]
    fn from(variant: S9L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9L` reader - Sector 9 Locked for Write Protection by User 0"]
pub type S9L_R = crate::BitReader<S9L_A>;
impl S9L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S9L_A {
        match self.bits {
            false => S9L_A::VALUE1,
            true => S9L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S9L_A::VALUE1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S9L_A::VALUE2
    }
}
#[doc = "Sectors 10 and 11 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S10_S11L_A {
    #[doc = "0: No write protection is configured for sectors 10+11."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sectors 10+11."]
    VALUE2 = 1,
}
impl From<S10_S11L_A> for bool {
    #[inline(always)]
    fn from(variant: S10_S11L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10_S11L` reader - Sectors 10 and 11 Locked for Write Protection by User 0"]
pub type S10_S11L_R = crate::BitReader<S10_S11L_A>;
impl S10_S11L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S10_S11L_A {
        match self.bits {
            false => S10_S11L_A::VALUE1,
            true => S10_S11L_A::VALUE2,
        }
    }
    #[doc = "No write protection is configured for sectors 10+11."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S10_S11L_A::VALUE1
    }
    #[doc = "Write protection is configured for sectors 10+11."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S10_S11L_A::VALUE2
    }
}
#[doc = "Read Protection Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPRO_A {
    #[doc = "0: No read protection configured"]
    VALUE1 = 0,
    #[doc = "1: Read protection and global write protection is configured by user 0 (master user)"]
    VALUE2 = 1,
}
impl From<RPRO_A> for bool {
    #[inline(always)]
    fn from(variant: RPRO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPRO` reader - Read Protection Configuration"]
pub type RPRO_R = crate::BitReader<RPRO_A>;
impl RPRO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPRO_A {
        match self.bits {
            false => RPRO_A::VALUE1,
            true => RPRO_A::VALUE2,
        }
    }
    #[doc = "No read protection configured"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPRO_A::VALUE1
    }
    #[doc = "Read protection and global write protection is configured by user 0 (master user)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPRO_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s0l(&self) -> S0L_R {
        S0L_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s1l(&self) -> S1L_R {
        S1L_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s2l(&self) -> S2L_R {
        S2L_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s3l(&self) -> S3L_R {
        S3L_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s4l(&self) -> S4L_R {
        S4L_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s5l(&self) -> S5L_R {
        S5L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s6l(&self) -> S6L_R {
        S6L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s7l(&self) -> S7L_R {
        S7L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s8l(&self) -> S8L_R {
        S8L_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sector 9 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s9l(&self) -> S9L_R {
        S9L_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Sectors 10 and 11 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s10_s11l(&self) -> S10_S11L_R {
        S10_S11L_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Read Protection Configuration"]
    #[inline(always)]
    pub fn rpro(&self) -> RPRO_R {
        RPRO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Flash Protection Configuration Register User 0\n\nYou can [`read`](crate::Reg::read) this register and get [`procon0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROCON0_SPEC;
impl crate::RegisterSpec for PROCON0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procon0::R`](R) reader structure"]
impl crate::Readable for PROCON0_SPEC {}
#[doc = "`reset()` method sets PROCON0 to value 0"]
impl crate::Resettable for PROCON0_SPEC {}
