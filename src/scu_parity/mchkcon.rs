#[doc = "Register `MCHKCON` reader"]
pub type R = crate::R<MCHKCON_SPEC>;
#[doc = "Register `MCHKCON` writer"]
pub type W = crate::W<MCHKCON_SPEC>;
#[doc = "Select Memory Check for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELPS_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELPS_A> for bool {
    #[inline(always)]
    fn from(variant: SELPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELPS` reader - Select Memory Check for PSRAM"]
pub type SELPS_R = crate::BitReader<SELPS_A>;
impl SELPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELPS_A {
        match self.bits {
            false => SELPS_A::VALUE1,
            true => SELPS_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELPS_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELPS_A::VALUE2
    }
}
#[doc = "Field `SELPS` writer - Select Memory Check for PSRAM"]
pub type SELPS_W<'a, REG> = crate::BitWriter<'a, REG, SELPS_A>;
impl<'a, REG> SELPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELPS_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELPS_A::VALUE2)
    }
}
#[doc = "Select Memory Check for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELDS1_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELDS1_A> for bool {
    #[inline(always)]
    fn from(variant: SELDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELDS1` reader - Select Memory Check for DSRAM1"]
pub type SELDS1_R = crate::BitReader<SELDS1_A>;
impl SELDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELDS1_A {
        match self.bits {
            false => SELDS1_A::VALUE1,
            true => SELDS1_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELDS1_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELDS1_A::VALUE2
    }
}
#[doc = "Field `SELDS1` writer - Select Memory Check for DSRAM1"]
pub type SELDS1_W<'a, REG> = crate::BitWriter<'a, REG, SELDS1_A>;
impl<'a, REG> SELDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELDS1_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELDS1_A::VALUE2)
    }
}
#[doc = "Select Memory Check for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELDS2_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELDS2_A> for bool {
    #[inline(always)]
    fn from(variant: SELDS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELDS2` reader - Select Memory Check for DSRAM2"]
pub type SELDS2_R = crate::BitReader<SELDS2_A>;
impl SELDS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELDS2_A {
        match self.bits {
            false => SELDS2_A::VALUE1,
            true => SELDS2_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELDS2_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELDS2_A::VALUE2
    }
}
#[doc = "Field `SELDS2` writer - Select Memory Check for DSRAM2"]
pub type SELDS2_W<'a, REG> = crate::BitWriter<'a, REG, SELDS2_A>;
impl<'a, REG> SELDS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELDS2_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELDS2_A::VALUE2)
    }
}
#[doc = "Select Memory Check for USIC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0DRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<USIC0DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0DRA` reader - Select Memory Check for USIC0"]
pub type USIC0DRA_R = crate::BitReader<USIC0DRA_A>;
impl USIC0DRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIC0DRA_A {
        match self.bits {
            false => USIC0DRA_A::VALUE1,
            true => USIC0DRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC0DRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC0DRA_A::VALUE2
    }
}
#[doc = "Field `USIC0DRA` writer - Select Memory Check for USIC0"]
pub type USIC0DRA_W<'a, REG> = crate::BitWriter<'a, REG, USIC0DRA_A>;
impl<'a, REG> USIC0DRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0DRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0DRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for USIC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1DRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<USIC1DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1DRA` reader - Select Memory Check for USIC1"]
pub type USIC1DRA_R = crate::BitReader<USIC1DRA_A>;
impl USIC1DRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIC1DRA_A {
        match self.bits {
            false => USIC1DRA_A::VALUE1,
            true => USIC1DRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC1DRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC1DRA_A::VALUE2
    }
}
#[doc = "Field `USIC1DRA` writer - Select Memory Check for USIC1"]
pub type USIC1DRA_W<'a, REG> = crate::BitWriter<'a, REG, USIC1DRA_A>;
impl<'a, REG> USIC1DRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1DRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1DRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for USIC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC2DRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<USIC2DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC2DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC2DRA` reader - Select Memory Check for USIC2"]
pub type USIC2DRA_R = crate::BitReader<USIC2DRA_A>;
impl USIC2DRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIC2DRA_A {
        match self.bits {
            false => USIC2DRA_A::VALUE1,
            true => USIC2DRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC2DRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC2DRA_A::VALUE2
    }
}
#[doc = "Field `USIC2DRA` writer - Select Memory Check for USIC2"]
pub type USIC2DRA_W<'a, REG> = crate::BitWriter<'a, REG, USIC2DRA_A>;
impl<'a, REG> USIC2DRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2DRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2DRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for MultiCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCANDRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<MCANDRA_A> for bool {
    #[inline(always)]
    fn from(variant: MCANDRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANDRA` reader - Select Memory Check for MultiCAN"]
pub type MCANDRA_R = crate::BitReader<MCANDRA_A>;
impl MCANDRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCANDRA_A {
        match self.bits {
            false => MCANDRA_A::VALUE1,
            true => MCANDRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCANDRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCANDRA_A::VALUE2
    }
}
#[doc = "Field `MCANDRA` writer - Select Memory Check for MultiCAN"]
pub type MCANDRA_W<'a, REG> = crate::BitWriter<'a, REG, MCANDRA_A>;
impl<'a, REG> MCANDRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCANDRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCANDRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for PMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPRFDRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<PPRFDRA_A> for bool {
    #[inline(always)]
    fn from(variant: PPRFDRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPRFDRA` reader - Select Memory Check for PMU"]
pub type PPRFDRA_R = crate::BitReader<PPRFDRA_A>;
impl PPRFDRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPRFDRA_A {
        match self.bits {
            false => PPRFDRA_A::VALUE1,
            true => PPRFDRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPRFDRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPRFDRA_A::VALUE2
    }
}
#[doc = "Field `PPRFDRA` writer - Select Memory Check for PMU"]
pub type PPRFDRA_W<'a, REG> = crate::BitWriter<'a, REG, PPRFDRA_A>;
impl<'a, REG> PPRFDRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRFDRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRFDRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for USB SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELUSB_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELUSB_A> for bool {
    #[inline(always)]
    fn from(variant: SELUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELUSB` reader - Select Memory Check for USB SRAM"]
pub type SELUSB_R = crate::BitReader<SELUSB_A>;
impl SELUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELUSB_A {
        match self.bits {
            false => SELUSB_A::VALUE1,
            true => SELUSB_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELUSB_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELUSB_A::VALUE2
    }
}
#[doc = "Field `SELUSB` writer - Select Memory Check for USB SRAM"]
pub type SELUSB_W<'a, REG> = crate::BitWriter<'a, REG, SELUSB_A>;
impl<'a, REG> SELUSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELUSB_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELUSB_A::VALUE2)
    }
}
#[doc = "Select Memory Check for ETH0 TX SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELETH0TX_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: SELETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELETH0TX` reader - Select Memory Check for ETH0 TX SRAM"]
pub type SELETH0TX_R = crate::BitReader<SELETH0TX_A>;
impl SELETH0TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELETH0TX_A {
        match self.bits {
            false => SELETH0TX_A::VALUE1,
            true => SELETH0TX_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELETH0TX_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELETH0TX_A::VALUE2
    }
}
#[doc = "Field `SELETH0TX` writer - Select Memory Check for ETH0 TX SRAM"]
pub type SELETH0TX_W<'a, REG> = crate::BitWriter<'a, REG, SELETH0TX_A>;
impl<'a, REG> SELETH0TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELETH0TX_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELETH0TX_A::VALUE2)
    }
}
#[doc = "Select Memory Check for ETH0 RX SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELETH0RX_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: SELETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELETH0RX` reader - Select Memory Check for ETH0 RX SRAM"]
pub type SELETH0RX_R = crate::BitReader<SELETH0RX_A>;
impl SELETH0RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELETH0RX_A {
        match self.bits {
            false => SELETH0RX_A::VALUE1,
            true => SELETH0RX_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELETH0RX_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELETH0RX_A::VALUE2
    }
}
#[doc = "Field `SELETH0RX` writer - Select Memory Check for ETH0 RX SRAM"]
pub type SELETH0RX_W<'a, REG> = crate::BitWriter<'a, REG, SELETH0RX_A>;
impl<'a, REG> SELETH0RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELETH0RX_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELETH0RX_A::VALUE2)
    }
}
#[doc = "Select Memory Check for SDMMC SRAM 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELSD0_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELSD0_A> for bool {
    #[inline(always)]
    fn from(variant: SELSD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELSD0` reader - Select Memory Check for SDMMC SRAM 0"]
pub type SELSD0_R = crate::BitReader<SELSD0_A>;
impl SELSD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELSD0_A {
        match self.bits {
            false => SELSD0_A::VALUE1,
            true => SELSD0_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELSD0_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELSD0_A::VALUE2
    }
}
#[doc = "Field `SELSD0` writer - Select Memory Check for SDMMC SRAM 0"]
pub type SELSD0_W<'a, REG> = crate::BitWriter<'a, REG, SELSD0_A>;
impl<'a, REG> SELSD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELSD0_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELSD0_A::VALUE2)
    }
}
#[doc = "Select Memory Check for SDMMC SRAM 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELSD1_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELSD1_A> for bool {
    #[inline(always)]
    fn from(variant: SELSD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELSD1` reader - Select Memory Check for SDMMC SRAM 1"]
pub type SELSD1_R = crate::BitReader<SELSD1_A>;
impl SELSD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELSD1_A {
        match self.bits {
            false => SELSD1_A::VALUE1,
            true => SELSD1_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELSD1_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELSD1_A::VALUE2
    }
}
#[doc = "Field `SELSD1` writer - Select Memory Check for SDMMC SRAM 1"]
pub type SELSD1_W<'a, REG> = crate::BitWriter<'a, REG, SELSD1_A>;
impl<'a, REG> SELSD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELSD1_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELSD1_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&self) -> SELPS_R {
        SELPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&self) -> SELDS1_R {
        SELDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select Memory Check for DSRAM2"]
    #[inline(always)]
    pub fn selds2(&self) -> SELDS2_R {
        SELDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&self) -> USIC0DRA_R {
        USIC0DRA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&self) -> USIC1DRA_R {
        USIC1DRA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select Memory Check for USIC2"]
    #[inline(always)]
    pub fn usic2dra(&self) -> USIC2DRA_R {
        USIC2DRA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&self) -> MCANDRA_R {
        MCANDRA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&self) -> PPRFDRA_R {
        PPRFDRA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&self) -> SELUSB_R {
        SELUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline(always)]
    pub fn seleth0tx(&self) -> SELETH0TX_R {
        SELETH0TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline(always)]
    pub fn seleth0rx(&self) -> SELETH0RX_R {
        SELETH0RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline(always)]
    pub fn selsd0(&self) -> SELSD0_R {
        SELSD0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline(always)]
    pub fn selsd1(&self) -> SELSD1_R {
        SELSD1_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&mut self) -> SELPS_W<MCHKCON_SPEC> {
        SELPS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&mut self) -> SELDS1_W<MCHKCON_SPEC> {
        SELDS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Select Memory Check for DSRAM2"]
    #[inline(always)]
    pub fn selds2(&mut self) -> SELDS2_W<MCHKCON_SPEC> {
        SELDS2_W::new(self, 2)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&mut self) -> USIC0DRA_W<MCHKCON_SPEC> {
        USIC0DRA_W::new(self, 8)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&mut self) -> USIC1DRA_W<MCHKCON_SPEC> {
        USIC1DRA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Select Memory Check for USIC2"]
    #[inline(always)]
    pub fn usic2dra(&mut self) -> USIC2DRA_W<MCHKCON_SPEC> {
        USIC2DRA_W::new(self, 10)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&mut self) -> MCANDRA_W<MCHKCON_SPEC> {
        MCANDRA_W::new(self, 12)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&mut self) -> PPRFDRA_W<MCHKCON_SPEC> {
        PPRFDRA_W::new(self, 13)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&mut self) -> SELUSB_W<MCHKCON_SPEC> {
        SELUSB_W::new(self, 16)
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline(always)]
    pub fn seleth0tx(&mut self) -> SELETH0TX_W<MCHKCON_SPEC> {
        SELETH0TX_W::new(self, 17)
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline(always)]
    pub fn seleth0rx(&mut self) -> SELETH0RX_W<MCHKCON_SPEC> {
        SELETH0RX_W::new(self, 18)
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline(always)]
    pub fn selsd0(&mut self) -> SELSD0_W<MCHKCON_SPEC> {
        SELSD0_W::new(self, 19)
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline(always)]
    pub fn selsd1(&mut self) -> SELSD1_W<MCHKCON_SPEC> {
        SELSD1_W::new(self, 20)
    }
}
#[doc = "Memory Checking Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mchkcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mchkcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCHKCON_SPEC;
impl crate::RegisterSpec for MCHKCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mchkcon::R`](R) reader structure"]
impl crate::Readable for MCHKCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mchkcon::W`](W) writer structure"]
impl crate::Writable for MCHKCON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCHKCON to value 0"]
impl crate::Resettable for MCHKCON_SPEC {}
