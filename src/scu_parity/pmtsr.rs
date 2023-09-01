#[doc = "Register `PMTSR` reader"]
pub type R = crate::R<PMTSR_SPEC>;
#[doc = "Register `PMTSR` writer"]
pub type W = crate::W<PMTSR_SPEC>;
#[doc = "Field `MTENPS` reader - Test Enable Control for PSRAM"]
pub type MTENPS_R = crate::BitReader<MTENPS_A>;
#[doc = "Test Enable Control for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTENPS_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTENPS_A> for bool {
    #[inline(always)]
    fn from(variant: MTENPS_A) -> Self {
        variant as u8 != 0
    }
}
impl MTENPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTENPS_A {
        match self.bits {
            false => MTENPS_A::VALUE1,
            true => MTENPS_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTENPS_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTENPS_A::VALUE2
    }
}
#[doc = "Field `MTENPS` writer - Test Enable Control for PSRAM"]
pub type MTENPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTENPS_A>;
impl<'a, REG, const O: u8> MTENPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTENPS_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTENPS_A::VALUE2)
    }
}
#[doc = "Field `MTENDS1` reader - Test Enable Control for DSRAM1"]
pub type MTENDS1_R = crate::BitReader<MTENDS1_A>;
#[doc = "Test Enable Control for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTENDS1_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: MTENDS1_A) -> Self {
        variant as u8 != 0
    }
}
impl MTENDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTENDS1_A {
        match self.bits {
            false => MTENDS1_A::VALUE1,
            true => MTENDS1_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTENDS1_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTENDS1_A::VALUE2
    }
}
#[doc = "Field `MTENDS1` writer - Test Enable Control for DSRAM1"]
pub type MTENDS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTENDS1_A>;
impl<'a, REG, const O: u8> MTENDS1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTENDS1_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTENDS1_A::VALUE2)
    }
}
#[doc = "Field `MTENDS2` reader - Test Enable Control for DSRAM2"]
pub type MTENDS2_R = crate::BitReader<MTENDS2_A>;
#[doc = "Test Enable Control for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTENDS2_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTENDS2_A> for bool {
    #[inline(always)]
    fn from(variant: MTENDS2_A) -> Self {
        variant as u8 != 0
    }
}
impl MTENDS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTENDS2_A {
        match self.bits {
            false => MTENDS2_A::VALUE1,
            true => MTENDS2_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTENDS2_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTENDS2_A::VALUE2
    }
}
#[doc = "Field `MTENDS2` writer - Test Enable Control for DSRAM2"]
pub type MTENDS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTENDS2_A>;
impl<'a, REG, const O: u8> MTENDS2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTENDS2_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTENDS2_A::VALUE2)
    }
}
#[doc = "Field `MTEU0` reader - Test Enable Control for USIC0 Memory"]
pub type MTEU0_R = crate::BitReader<MTEU0_A>;
#[doc = "Test Enable Control for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTEU0_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEU0_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU0_A) -> Self {
        variant as u8 != 0
    }
}
impl MTEU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEU0_A {
        match self.bits {
            false => MTEU0_A::VALUE1,
            true => MTEU0_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTEU0_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTEU0_A::VALUE2
    }
}
#[doc = "Field `MTEU0` writer - Test Enable Control for USIC0 Memory"]
pub type MTEU0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTEU0_A>;
impl<'a, REG, const O: u8> MTEU0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTEU0_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTEU0_A::VALUE2)
    }
}
#[doc = "Field `MTEU1` reader - Test Enable Control for USIC1 Memory"]
pub type MTEU1_R = crate::BitReader<MTEU1_A>;
#[doc = "Test Enable Control for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTEU1_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEU1_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU1_A) -> Self {
        variant as u8 != 0
    }
}
impl MTEU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEU1_A {
        match self.bits {
            false => MTEU1_A::VALUE1,
            true => MTEU1_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTEU1_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTEU1_A::VALUE2
    }
}
#[doc = "Field `MTEU1` writer - Test Enable Control for USIC1 Memory"]
pub type MTEU1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTEU1_A>;
impl<'a, REG, const O: u8> MTEU1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTEU1_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTEU1_A::VALUE2)
    }
}
#[doc = "Field `MTEU2` reader - Test Enable Control for USIC2 Memory"]
pub type MTEU2_R = crate::BitReader<MTEU2_A>;
#[doc = "Test Enable Control for USIC2 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTEU2_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEU2_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU2_A) -> Self {
        variant as u8 != 0
    }
}
impl MTEU2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEU2_A {
        match self.bits {
            false => MTEU2_A::VALUE1,
            true => MTEU2_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTEU2_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTEU2_A::VALUE2
    }
}
#[doc = "Field `MTEU2` writer - Test Enable Control for USIC2 Memory"]
pub type MTEU2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTEU2_A>;
impl<'a, REG, const O: u8> MTEU2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTEU2_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTEU2_A::VALUE2)
    }
}
#[doc = "Field `MTEMC` reader - Test Enable Control for MultiCAN Memory"]
pub type MTEMC_R = crate::BitReader<MTEMC_A>;
#[doc = "Test Enable Control for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTEMC_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEMC_A> for bool {
    #[inline(always)]
    fn from(variant: MTEMC_A) -> Self {
        variant as u8 != 0
    }
}
impl MTEMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEMC_A {
        match self.bits {
            false => MTEMC_A::VALUE1,
            true => MTEMC_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTEMC_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTEMC_A::VALUE2
    }
}
#[doc = "Field `MTEMC` writer - Test Enable Control for MultiCAN Memory"]
pub type MTEMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTEMC_A>;
impl<'a, REG, const O: u8> MTEMC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTEMC_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTEMC_A::VALUE2)
    }
}
#[doc = "Field `MTEPPRF` reader - Test Enable Control for PMU Prefetch Memory"]
pub type MTEPPRF_R = crate::BitReader<MTEPPRF_A>;
#[doc = "Test Enable Control for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTEPPRF_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: MTEPPRF_A) -> Self {
        variant as u8 != 0
    }
}
impl MTEPPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEPPRF_A {
        match self.bits {
            false => MTEPPRF_A::VALUE1,
            true => MTEPPRF_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTEPPRF_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTEPPRF_A::VALUE2
    }
}
#[doc = "Field `MTEPPRF` writer - Test Enable Control for PMU Prefetch Memory"]
pub type MTEPPRF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTEPPRF_A>;
impl<'a, REG, const O: u8> MTEPPRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTEPPRF_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTEPPRF_A::VALUE2)
    }
}
#[doc = "Field `MTUSB` reader - Test Enable Control for USB Memory"]
pub type MTUSB_R = crate::BitReader<MTUSB_A>;
#[doc = "Test Enable Control for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTUSB_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTUSB_A> for bool {
    #[inline(always)]
    fn from(variant: MTUSB_A) -> Self {
        variant as u8 != 0
    }
}
impl MTUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTUSB_A {
        match self.bits {
            false => MTUSB_A::VALUE1,
            true => MTUSB_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTUSB_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTUSB_A::VALUE2
    }
}
#[doc = "Field `MTUSB` writer - Test Enable Control for USB Memory"]
pub type MTUSB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTUSB_A>;
impl<'a, REG, const O: u8> MTUSB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTUSB_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTUSB_A::VALUE2)
    }
}
#[doc = "Field `MTETH0TX` reader - Test Enable Control for ETH TX Memory"]
pub type MTETH0TX_R = crate::BitReader<MTETH0TX_A>;
#[doc = "Test Enable Control for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTETH0TX_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: MTETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
impl MTETH0TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTETH0TX_A {
        match self.bits {
            false => MTETH0TX_A::VALUE1,
            true => MTETH0TX_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTETH0TX_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTETH0TX_A::VALUE2
    }
}
#[doc = "Field `MTETH0TX` writer - Test Enable Control for ETH TX Memory"]
pub type MTETH0TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTETH0TX_A>;
impl<'a, REG, const O: u8> MTETH0TX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTETH0TX_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTETH0TX_A::VALUE2)
    }
}
#[doc = "Field `MTETH0RX` reader - Test Enable Control for ETH RX Memory"]
pub type MTETH0RX_R = crate::BitReader<MTETH0RX_A>;
#[doc = "Test Enable Control for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTETH0RX_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: MTETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
impl MTETH0RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTETH0RX_A {
        match self.bits {
            false => MTETH0RX_A::VALUE1,
            true => MTETH0RX_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTETH0RX_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTETH0RX_A::VALUE2
    }
}
#[doc = "Field `MTETH0RX` writer - Test Enable Control for ETH RX Memory"]
pub type MTETH0RX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTETH0RX_A>;
impl<'a, REG, const O: u8> MTETH0RX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTETH0RX_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTETH0RX_A::VALUE2)
    }
}
#[doc = "Field `MTSD0` reader - Test Enable Control for SDMMC Memory 0"]
pub type MTSD0_R = crate::BitReader<MTSD0_A>;
#[doc = "Test Enable Control for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTSD0_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTSD0_A> for bool {
    #[inline(always)]
    fn from(variant: MTSD0_A) -> Self {
        variant as u8 != 0
    }
}
impl MTSD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSD0_A {
        match self.bits {
            false => MTSD0_A::VALUE1,
            true => MTSD0_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTSD0_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTSD0_A::VALUE2
    }
}
#[doc = "Field `MTSD0` writer - Test Enable Control for SDMMC Memory 0"]
pub type MTSD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTSD0_A>;
impl<'a, REG, const O: u8> MTSD0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTSD0_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTSD0_A::VALUE2)
    }
}
#[doc = "Field `MTSD1` reader - Test Enable Control for SDMMC Memory 1"]
pub type MTSD1_R = crate::BitReader<MTSD1_A>;
#[doc = "Test Enable Control for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTSD1_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTSD1_A> for bool {
    #[inline(always)]
    fn from(variant: MTSD1_A) -> Self {
        variant as u8 != 0
    }
}
impl MTSD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSD1_A {
        match self.bits {
            false => MTSD1_A::VALUE1,
            true => MTSD1_A::VALUE2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTSD1_A::VALUE1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTSD1_A::VALUE2
    }
}
#[doc = "Field `MTSD1` writer - Test Enable Control for SDMMC Memory 1"]
pub type MTSD1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTSD1_A>;
impl<'a, REG, const O: u8> MTSD1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTSD1_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTSD1_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline(always)]
    pub fn mtenps(&self) -> MTENPS_R {
        MTENPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline(always)]
    pub fn mtends1(&self) -> MTENDS1_R {
        MTENDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test Enable Control for DSRAM2"]
    #[inline(always)]
    pub fn mtends2(&self) -> MTENDS2_R {
        MTENDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline(always)]
    pub fn mteu0(&self) -> MTEU0_R {
        MTEU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline(always)]
    pub fn mteu1(&self) -> MTEU1_R {
        MTEU1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Test Enable Control for USIC2 Memory"]
    #[inline(always)]
    pub fn mteu2(&self) -> MTEU2_R {
        MTEU2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline(always)]
    pub fn mtemc(&self) -> MTEMC_R {
        MTEMC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn mtepprf(&self) -> MTEPPRF_R {
        MTEPPRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline(always)]
    pub fn mtusb(&self) -> MTUSB_R {
        MTUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Test Enable Control for ETH TX Memory"]
    #[inline(always)]
    pub fn mteth0tx(&self) -> MTETH0TX_R {
        MTETH0TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Test Enable Control for ETH RX Memory"]
    #[inline(always)]
    pub fn mteth0rx(&self) -> MTETH0RX_R {
        MTETH0RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Test Enable Control for SDMMC Memory 0"]
    #[inline(always)]
    pub fn mtsd0(&self) -> MTSD0_R {
        MTSD0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Test Enable Control for SDMMC Memory 1"]
    #[inline(always)]
    pub fn mtsd1(&self) -> MTSD1_R {
        MTSD1_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn mtenps(&mut self) -> MTENPS_W<PMTSR_SPEC, 0> {
        MTENPS_W::new(self)
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn mtends1(&mut self) -> MTENDS1_W<PMTSR_SPEC, 1> {
        MTENDS1_W::new(self)
    }
    #[doc = "Bit 2 - Test Enable Control for DSRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn mtends2(&mut self) -> MTENDS2_W<PMTSR_SPEC, 2> {
        MTENDS2_W::new(self)
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteu0(&mut self) -> MTEU0_W<PMTSR_SPEC, 8> {
        MTEU0_W::new(self)
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteu1(&mut self) -> MTEU1_W<PMTSR_SPEC, 9> {
        MTEU1_W::new(self)
    }
    #[doc = "Bit 10 - Test Enable Control for USIC2 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteu2(&mut self) -> MTEU2_W<PMTSR_SPEC, 10> {
        MTEU2_W::new(self)
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mtemc(&mut self) -> MTEMC_W<PMTSR_SPEC, 12> {
        MTEMC_W::new(self)
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mtepprf(&mut self) -> MTEPPRF_W<PMTSR_SPEC, 13> {
        MTEPPRF_W::new(self)
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mtusb(&mut self) -> MTUSB_W<PMTSR_SPEC, 16> {
        MTUSB_W::new(self)
    }
    #[doc = "Bit 17 - Test Enable Control for ETH TX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteth0tx(&mut self) -> MTETH0TX_W<PMTSR_SPEC, 17> {
        MTETH0TX_W::new(self)
    }
    #[doc = "Bit 18 - Test Enable Control for ETH RX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteth0rx(&mut self) -> MTETH0RX_W<PMTSR_SPEC, 18> {
        MTETH0RX_W::new(self)
    }
    #[doc = "Bit 19 - Test Enable Control for SDMMC Memory 0"]
    #[inline(always)]
    #[must_use]
    pub fn mtsd0(&mut self) -> MTSD0_W<PMTSR_SPEC, 19> {
        MTSD0_W::new(self)
    }
    #[doc = "Bit 20 - Test Enable Control for SDMMC Memory 1"]
    #[inline(always)]
    #[must_use]
    pub fn mtsd1(&mut self) -> MTSD1_W<PMTSR_SPEC, 20> {
        MTSD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Parity Memory Test Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMTSR_SPEC;
impl crate::RegisterSpec for PMTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmtsr::R`](R) reader structure"]
impl crate::Readable for PMTSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmtsr::W`](W) writer structure"]
impl crate::Writable for PMTSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMTSR to value 0"]
impl crate::Resettable for PMTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
