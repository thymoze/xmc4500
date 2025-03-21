#[doc = "Register `DTSSTAT` reader"]
pub type R = crate::R<DTSSTAT_SPEC>;
#[doc = "Field `RESULT` reader - Result of the DTS Measurement"]
pub type RESULT_R = crate::FieldReader<u16>;
#[doc = "Sensor Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_A {
    #[doc = "0: The DTS is not ready"]
    VALUE1 = 0,
    #[doc = "1: The DTS is ready"]
    VALUE2 = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Sensor Ready Status"]
pub type RDY_R = crate::BitReader<RDY_A>;
impl RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::VALUE1,
            true => RDY_A::VALUE2,
        }
    }
    #[doc = "The DTS is not ready"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDY_A::VALUE1
    }
    #[doc = "The DTS is ready"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDY_A::VALUE2
    }
}
#[doc = "Sensor Busy Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: not busy"]
    VALUE1 = 0,
    #[doc = "1: busy"]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Sensor Busy Status"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "not busy"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "busy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:9 - Result of the DTS Measurement"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Sensor Ready Status"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sensor Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Die Temperature Sensor Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtsstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTSSTAT_SPEC;
impl crate::RegisterSpec for DTSSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtsstat::R`](R) reader structure"]
impl crate::Readable for DTSSTAT_SPEC {}
#[doc = "`reset()` method sets DTSSTAT to value 0"]
impl crate::Resettable for DTSSTAT_SPEC {}
