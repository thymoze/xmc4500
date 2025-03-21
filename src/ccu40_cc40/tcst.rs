#[doc = "Register `TCST` reader"]
pub type R = crate::R<TCST_SPEC>;
#[doc = "Timer Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRB_A {
    #[doc = "0: Timer is stopped"]
    VALUE1 = 0,
    #[doc = "1: Timer is running"]
    VALUE2 = 1,
}
impl From<TRB_A> for bool {
    #[inline(always)]
    fn from(variant: TRB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRB` reader - Timer Run Bit"]
pub type TRB_R = crate::BitReader<TRB_A>;
impl TRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRB_A {
        match self.bits {
            false => TRB_A::VALUE1,
            true => TRB_A::VALUE2,
        }
    }
    #[doc = "Timer is stopped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRB_A::VALUE1
    }
    #[doc = "Timer is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRB_A::VALUE2
    }
}
#[doc = "Timer Counting Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDIR_A {
    #[doc = "0: Timer is counting up"]
    VALUE1 = 0,
    #[doc = "1: Timer is counting down"]
    VALUE2 = 1,
}
impl From<CDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDIR` reader - Timer Counting Direction"]
pub type CDIR_R = crate::BitReader<CDIR_A>;
impl CDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CDIR_A {
        match self.bits {
            false => CDIR_A::VALUE1,
            true => CDIR_A::VALUE2,
        }
    }
    #[doc = "Timer is counting up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDIR_A::VALUE1
    }
    #[doc = "Timer is counting down"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDIR_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Timer Run Bit"]
    #[inline(always)]
    pub fn trb(&self) -> TRB_R {
        TRB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Counting Direction"]
    #[inline(always)]
    pub fn cdir(&self) -> CDIR_R {
        CDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Slice Timer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tcst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCST_SPEC;
impl crate::RegisterSpec for TCST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcst::R`](R) reader structure"]
impl crate::Readable for TCST_SPEC {}
#[doc = "`reset()` method sets TCST to value 0"]
impl crate::Resettable for TCST_SPEC {}
