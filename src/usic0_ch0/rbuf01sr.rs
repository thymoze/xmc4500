#[doc = "Register `RBUF01SR` reader"]
pub type R = crate::R<RBUF01SR_SPEC>;
#[doc = "Field `WLEN0` reader - Received Data Word Length in RBUF0"]
pub type WLEN0_R = crate::FieldReader;
#[doc = "Start of Frame in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF0_A {
    #[doc = "0: The data in RBUF0 has not been the first data word of a data frame."]
    VALUE1 = 0,
    #[doc = "1: The data in RBUF0 has been the first data word of a data frame."]
    VALUE2 = 1,
}
impl From<SOF0_A> for bool {
    #[inline(always)]
    fn from(variant: SOF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF0` reader - Start of Frame in RBUF0"]
pub type SOF0_R = crate::BitReader<SOF0_A>;
impl SOF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOF0_A {
        match self.bits {
            false => SOF0_A::VALUE1,
            true => SOF0_A::VALUE2,
        }
    }
    #[doc = "The data in RBUF0 has not been the first data word of a data frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOF0_A::VALUE1
    }
    #[doc = "The data in RBUF0 has been the first data word of a data frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOF0_A::VALUE2
    }
}
#[doc = "Field `PAR0` reader - Protocol-Related Argument in RBUF0"]
pub type PAR0_R = crate::BitReader;
#[doc = "Protocol-related Error in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERR0_A {
    #[doc = "0: The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    VALUE1 = 0,
    #[doc = "1: The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    VALUE2 = 1,
}
impl From<PERR0_A> for bool {
    #[inline(always)]
    fn from(variant: PERR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR0` reader - Protocol-related Error in RBUF0"]
pub type PERR0_R = crate::BitReader<PERR0_A>;
impl PERR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PERR0_A {
        match self.bits {
            false => PERR0_A::VALUE1,
            true => PERR0_A::VALUE2,
        }
    }
    #[doc = "The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PERR0_A::VALUE1
    }
    #[doc = "The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PERR0_A::VALUE2
    }
}
#[doc = "Receive Data Valid in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDV00_A {
    #[doc = "0: Register RBUF0 does not contain data that has not yet been read out."]
    VALUE1 = 0,
    #[doc = "1: Register RBUF0 contains data that has not yet been read out."]
    VALUE2 = 1,
}
impl From<RDV00_A> for bool {
    #[inline(always)]
    fn from(variant: RDV00_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDV00` reader - Receive Data Valid in RBUF0"]
pub type RDV00_R = crate::BitReader<RDV00_A>;
impl RDV00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDV00_A {
        match self.bits {
            false => RDV00_A::VALUE1,
            true => RDV00_A::VALUE2,
        }
    }
    #[doc = "Register RBUF0 does not contain data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV00_A::VALUE1
    }
    #[doc = "Register RBUF0 contains data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV00_A::VALUE2
    }
}
#[doc = "Receive Data Valid in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDV01_A {
    #[doc = "0: Register RBUF1 does not contain data that has not yet been read out."]
    VALUE1 = 0,
    #[doc = "1: Register RBUF1 contains data that has not yet been read out."]
    VALUE2 = 1,
}
impl From<RDV01_A> for bool {
    #[inline(always)]
    fn from(variant: RDV01_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDV01` reader - Receive Data Valid in RBUF1"]
pub type RDV01_R = crate::BitReader<RDV01_A>;
impl RDV01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDV01_A {
        match self.bits {
            false => RDV01_A::VALUE1,
            true => RDV01_A::VALUE2,
        }
    }
    #[doc = "Register RBUF1 does not contain data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV01_A::VALUE1
    }
    #[doc = "Register RBUF1 contains data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV01_A::VALUE2
    }
}
#[doc = "Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DS0_A {
    #[doc = "0: The register RBUF contains the data of RBUF0 (same for associated status information)."]
    VALUE1 = 0,
    #[doc = "1: The register RBUF contains the data of RBUF1 (same for associated status information)."]
    VALUE2 = 1,
}
impl From<DS0_A> for bool {
    #[inline(always)]
    fn from(variant: DS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS0` reader - Data Source"]
pub type DS0_R = crate::BitReader<DS0_A>;
impl DS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DS0_A {
        match self.bits {
            false => DS0_A::VALUE1,
            true => DS0_A::VALUE2,
        }
    }
    #[doc = "The register RBUF contains the data of RBUF0 (same for associated status information)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DS0_A::VALUE1
    }
    #[doc = "The register RBUF contains the data of RBUF1 (same for associated status information)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DS0_A::VALUE2
    }
}
#[doc = "Field `WLEN1` reader - Received Data Word Length in RBUF1"]
pub type WLEN1_R = crate::FieldReader;
#[doc = "Start of Frame in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF1_A {
    #[doc = "0: The data in RBUF1 has not been the first data word of a data frame."]
    VALUE1 = 0,
    #[doc = "1: The data in RBUF1 has been the first data word of a data frame."]
    VALUE2 = 1,
}
impl From<SOF1_A> for bool {
    #[inline(always)]
    fn from(variant: SOF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF1` reader - Start of Frame in RBUF1"]
pub type SOF1_R = crate::BitReader<SOF1_A>;
impl SOF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOF1_A {
        match self.bits {
            false => SOF1_A::VALUE1,
            true => SOF1_A::VALUE2,
        }
    }
    #[doc = "The data in RBUF1 has not been the first data word of a data frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOF1_A::VALUE1
    }
    #[doc = "The data in RBUF1 has been the first data word of a data frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOF1_A::VALUE2
    }
}
#[doc = "Field `PAR1` reader - Protocol-Related Argument in RBUF1"]
pub type PAR1_R = crate::BitReader;
#[doc = "Protocol-related Error in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERR1_A {
    #[doc = "0: The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    VALUE1 = 0,
    #[doc = "1: The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    VALUE2 = 1,
}
impl From<PERR1_A> for bool {
    #[inline(always)]
    fn from(variant: PERR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR1` reader - Protocol-related Error in RBUF1"]
pub type PERR1_R = crate::BitReader<PERR1_A>;
impl PERR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PERR1_A {
        match self.bits {
            false => PERR1_A::VALUE1,
            true => PERR1_A::VALUE2,
        }
    }
    #[doc = "The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PERR1_A::VALUE1
    }
    #[doc = "The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PERR1_A::VALUE2
    }
}
#[doc = "Receive Data Valid in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDV10_A {
    #[doc = "0: Register RBUF0 does not contain data that has not yet been read out."]
    VALUE1 = 0,
    #[doc = "1: Register RBUF0 contains data that has not yet been read out."]
    VALUE2 = 1,
}
impl From<RDV10_A> for bool {
    #[inline(always)]
    fn from(variant: RDV10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDV10` reader - Receive Data Valid in RBUF0"]
pub type RDV10_R = crate::BitReader<RDV10_A>;
impl RDV10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDV10_A {
        match self.bits {
            false => RDV10_A::VALUE1,
            true => RDV10_A::VALUE2,
        }
    }
    #[doc = "Register RBUF0 does not contain data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV10_A::VALUE1
    }
    #[doc = "Register RBUF0 contains data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV10_A::VALUE2
    }
}
#[doc = "Receive Data Valid in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDV11_A {
    #[doc = "0: Register RBUF1 does not contain data that has not yet been read out."]
    VALUE1 = 0,
    #[doc = "1: Register RBUF1 contains data that has not yet been read out."]
    VALUE2 = 1,
}
impl From<RDV11_A> for bool {
    #[inline(always)]
    fn from(variant: RDV11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDV11` reader - Receive Data Valid in RBUF1"]
pub type RDV11_R = crate::BitReader<RDV11_A>;
impl RDV11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDV11_A {
        match self.bits {
            false => RDV11_A::VALUE1,
            true => RDV11_A::VALUE2,
        }
    }
    #[doc = "Register RBUF1 does not contain data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV11_A::VALUE1
    }
    #[doc = "Register RBUF1 contains data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV11_A::VALUE2
    }
}
#[doc = "Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DS1_A {
    #[doc = "0: The register RBUF contains the data of RBUF0 (same for associated status information)."]
    VALUE1 = 0,
    #[doc = "1: The register RBUF contains the data of RBUF1 (same for associated status information)."]
    VALUE2 = 1,
}
impl From<DS1_A> for bool {
    #[inline(always)]
    fn from(variant: DS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS1` reader - Data Source"]
pub type DS1_R = crate::BitReader<DS1_A>;
impl DS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DS1_A {
        match self.bits {
            false => DS1_A::VALUE1,
            true => DS1_A::VALUE2,
        }
    }
    #[doc = "The register RBUF contains the data of RBUF0 (same for associated status information)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DS1_A::VALUE1
    }
    #[doc = "The register RBUF contains the data of RBUF1 (same for associated status information)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DS1_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:3 - Received Data Word Length in RBUF0"]
    #[inline(always)]
    pub fn wlen0(&self) -> WLEN0_R {
        WLEN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Start of Frame in RBUF0"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol-Related Argument in RBUF0"]
    #[inline(always)]
    pub fn par0(&self) -> PAR0_R {
        PAR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol-related Error in RBUF0"]
    #[inline(always)]
    pub fn perr0(&self) -> PERR0_R {
        PERR0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Data Valid in RBUF0"]
    #[inline(always)]
    pub fn rdv00(&self) -> RDV00_R {
        RDV00_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Data Valid in RBUF1"]
    #[inline(always)]
    pub fn rdv01(&self) -> RDV01_R {
        RDV01_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Source"]
    #[inline(always)]
    pub fn ds0(&self) -> DS0_R {
        DS0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Received Data Word Length in RBUF1"]
    #[inline(always)]
    pub fn wlen1(&self) -> WLEN1_R {
        WLEN1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Start of Frame in RBUF1"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Protocol-Related Argument in RBUF1"]
    #[inline(always)]
    pub fn par1(&self) -> PAR1_R {
        PAR1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protocol-related Error in RBUF1"]
    #[inline(always)]
    pub fn perr1(&self) -> PERR1_R {
        PERR1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive Data Valid in RBUF0"]
    #[inline(always)]
    pub fn rdv10(&self) -> RDV10_R {
        RDV10_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive Data Valid in RBUF1"]
    #[inline(always)]
    pub fn rdv11(&self) -> RDV11_R {
        RDV11_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Source"]
    #[inline(always)]
    pub fn ds1(&self) -> DS1_R {
        DS1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Receiver Buffer 01 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbuf01sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBUF01SR_SPEC;
impl crate::RegisterSpec for RBUF01SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbuf01sr::R`](R) reader structure"]
impl crate::Readable for RBUF01SR_SPEC {}
#[doc = "`reset()` method sets RBUF01SR to value 0"]
impl crate::Resettable for RBUF01SR_SPEC {}
