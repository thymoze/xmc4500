#[doc = "Register `PETE` reader"]
pub type R = crate::R<PETE_SPEC>;
#[doc = "Register `PETE` writer"]
pub type W = crate::W<PETE_SPEC>;
#[doc = "Parity Error Trap Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEPS_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEPS_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPS` reader - Parity Error Trap Enable for PSRAM"]
pub type PETEPS_R = crate::BitReader<PETEPS_A>;
impl PETEPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEPS_A {
        match self.bits {
            false => PETEPS_A::VALUE1,
            true => PETEPS_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEPS_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEPS_A::VALUE2
    }
}
#[doc = "Field `PETEPS` writer - Parity Error Trap Enable for PSRAM"]
pub type PETEPS_W<'a, REG> = crate::BitWriter<'a, REG, PETEPS_A>;
impl<'a, REG> PETEPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEPS_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEPS_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEDS1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEDS1` reader - Parity Error Trap Enable for DSRAM1"]
pub type PETEDS1_R = crate::BitReader<PETEDS1_A>;
impl PETEDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEDS1_A {
        match self.bits {
            false => PETEDS1_A::VALUE1,
            true => PETEDS1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEDS1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEDS1_A::VALUE2
    }
}
#[doc = "Field `PETEDS1` writer - Parity Error Trap Enable for DSRAM1"]
pub type PETEDS1_W<'a, REG> = crate::BitWriter<'a, REG, PETEDS1_A>;
impl<'a, REG> PETEDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEDS1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEDS1_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEDS2_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEDS2_A> for bool {
    #[inline(always)]
    fn from(variant: PETEDS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEDS2` reader - Parity Error Trap Enable for DSRAM2"]
pub type PETEDS2_R = crate::BitReader<PETEDS2_A>;
impl PETEDS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEDS2_A {
        match self.bits {
            false => PETEDS2_A::VALUE1,
            true => PETEDS2_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEDS2_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEDS2_A::VALUE2
    }
}
#[doc = "Field `PETEDS2` writer - Parity Error Trap Enable for DSRAM2"]
pub type PETEDS2_W<'a, REG> = crate::BitWriter<'a, REG, PETEDS2_A>;
impl<'a, REG> PETEDS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEDS2_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEDS2_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEU0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEU0_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU0` reader - Parity Error Trap Enable for USIC0 Memory"]
pub type PETEU0_R = crate::BitReader<PETEU0_A>;
impl PETEU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEU0_A {
        match self.bits {
            false => PETEU0_A::VALUE1,
            true => PETEU0_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEU0_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEU0_A::VALUE2
    }
}
#[doc = "Field `PETEU0` writer - Parity Error Trap Enable for USIC0 Memory"]
pub type PETEU0_W<'a, REG> = crate::BitWriter<'a, REG, PETEU0_A>;
impl<'a, REG> PETEU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU0_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEU1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEU1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU1` reader - Parity Error Trap Enable for USIC1 Memory"]
pub type PETEU1_R = crate::BitReader<PETEU1_A>;
impl PETEU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEU1_A {
        match self.bits {
            false => PETEU1_A::VALUE1,
            true => PETEU1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEU1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEU1_A::VALUE2
    }
}
#[doc = "Field `PETEU1` writer - Parity Error Trap Enable for USIC1 Memory"]
pub type PETEU1_W<'a, REG> = crate::BitWriter<'a, REG, PETEU1_A>;
impl<'a, REG> PETEU1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU1_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for USIC2 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEU2_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEU2_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU2` reader - Parity Error Trap Enable for USIC2 Memory"]
pub type PETEU2_R = crate::BitReader<PETEU2_A>;
impl PETEU2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEU2_A {
        match self.bits {
            false => PETEU2_A::VALUE1,
            true => PETEU2_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEU2_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEU2_A::VALUE2
    }
}
#[doc = "Field `PETEU2` writer - Parity Error Trap Enable for USIC2 Memory"]
pub type PETEU2_W<'a, REG> = crate::BitWriter<'a, REG, PETEU2_A>;
impl<'a, REG> PETEU2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU2_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU2_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEMC_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEMC_A> for bool {
    #[inline(always)]
    fn from(variant: PETEMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEMC` reader - Parity Error Trap Enable for MultiCAN Memory"]
pub type PETEMC_R = crate::BitReader<PETEMC_A>;
impl PETEMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEMC_A {
        match self.bits {
            false => PETEMC_A::VALUE1,
            true => PETEMC_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEMC_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEMC_A::VALUE2
    }
}
#[doc = "Field `PETEMC` writer - Parity Error Trap Enable for MultiCAN Memory"]
pub type PETEMC_W<'a, REG> = crate::BitWriter<'a, REG, PETEMC_A>;
impl<'a, REG> PETEMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEMC_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEMC_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEPPRF_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPPRF` reader - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PETEPPRF_R = crate::BitReader<PETEPPRF_A>;
impl PETEPPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEPPRF_A {
        match self.bits {
            false => PETEPPRF_A::VALUE1,
            true => PETEPPRF_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEPPRF_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEPPRF_A::VALUE2
    }
}
#[doc = "Field `PETEPPRF` writer - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PETEPPRF_W<'a, REG> = crate::BitWriter<'a, REG, PETEPPRF_A>;
impl<'a, REG> PETEPPRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEPPRF_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEPPRF_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEUSB_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PETEUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEUSB` reader - Parity Error Trap Enable for USB Memory"]
pub type PETEUSB_R = crate::BitReader<PETEUSB_A>;
impl PETEUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEUSB_A {
        match self.bits {
            false => PETEUSB_A::VALUE1,
            true => PETEUSB_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEUSB_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEUSB_A::VALUE2
    }
}
#[doc = "Field `PETEUSB` writer - Parity Error Trap Enable for USB Memory"]
pub type PETEUSB_W<'a, REG> = crate::BitWriter<'a, REG, PETEUSB_A>;
impl<'a, REG> PETEUSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEUSB_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEUSB_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for ETH 0TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEETH0TX_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: PETEETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEETH0TX` reader - Parity Error Trap Enable for ETH 0TX Memory"]
pub type PETEETH0TX_R = crate::BitReader<PETEETH0TX_A>;
impl PETEETH0TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEETH0TX_A {
        match self.bits {
            false => PETEETH0TX_A::VALUE1,
            true => PETEETH0TX_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEETH0TX_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEETH0TX_A::VALUE2
    }
}
#[doc = "Field `PETEETH0TX` writer - Parity Error Trap Enable for ETH 0TX Memory"]
pub type PETEETH0TX_W<'a, REG> = crate::BitWriter<'a, REG, PETEETH0TX_A>;
impl<'a, REG> PETEETH0TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEETH0TX_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEETH0TX_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for ETH0 RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEETH0RX_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: PETEETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEETH0RX` reader - Parity Error Trap Enable for ETH0 RX Memory"]
pub type PETEETH0RX_R = crate::BitReader<PETEETH0RX_A>;
impl PETEETH0RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEETH0RX_A {
        match self.bits {
            false => PETEETH0RX_A::VALUE1,
            true => PETEETH0RX_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEETH0RX_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEETH0RX_A::VALUE2
    }
}
#[doc = "Field `PETEETH0RX` writer - Parity Error Trap Enable for ETH0 RX Memory"]
pub type PETEETH0RX_W<'a, REG> = crate::BitWriter<'a, REG, PETEETH0RX_A>;
impl<'a, REG> PETEETH0RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEETH0RX_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEETH0RX_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for SDMMC SRAM 0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETESD0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETESD0_A> for bool {
    #[inline(always)]
    fn from(variant: PETESD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETESD0` reader - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
pub type PETESD0_R = crate::BitReader<PETESD0_A>;
impl PETESD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETESD0_A {
        match self.bits {
            false => PETESD0_A::VALUE1,
            true => PETESD0_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETESD0_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETESD0_A::VALUE2
    }
}
#[doc = "Field `PETESD0` writer - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
pub type PETESD0_W<'a, REG> = crate::BitWriter<'a, REG, PETESD0_A>;
impl<'a, REG> PETESD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETESD0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETESD0_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for SDMMC SRAM 1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETESD1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETESD1_A> for bool {
    #[inline(always)]
    fn from(variant: PETESD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETESD1` reader - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
pub type PETESD1_R = crate::BitReader<PETESD1_A>;
impl PETESD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETESD1_A {
        match self.bits {
            false => PETESD1_A::VALUE1,
            true => PETESD1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETESD1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETESD1_A::VALUE2
    }
}
#[doc = "Field `PETESD1` writer - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
pub type PETESD1_W<'a, REG> = crate::BitWriter<'a, REG, PETESD1_A>;
impl<'a, REG> PETESD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETESD1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETESD1_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&self) -> PETEPS_R {
        PETEPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&self) -> PETEDS1_R {
        PETEDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Trap Enable for DSRAM2"]
    #[inline(always)]
    pub fn peteds2(&self) -> PETEDS2_R {
        PETEDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&self) -> PETEU0_R {
        PETEU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&self) -> PETEU1_R {
        PETEU1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity Error Trap Enable for USIC2 Memory"]
    #[inline(always)]
    pub fn peteu2(&self) -> PETEU2_R {
        PETEU2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&self) -> PETEMC_R {
        PETEMC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&self) -> PETEPPRF_R {
        PETEPPRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&self) -> PETEUSB_R {
        PETEUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline(always)]
    pub fn peteeth0tx(&self) -> PETEETH0TX_R {
        PETEETH0TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline(always)]
    pub fn peteeth0rx(&self) -> PETEETH0RX_R {
        PETEETH0RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline(always)]
    pub fn petesd0(&self) -> PETESD0_R {
        PETESD0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline(always)]
    pub fn petesd1(&self) -> PETESD1_R {
        PETESD1_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&mut self) -> PETEPS_W<PETE_SPEC> {
        PETEPS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&mut self) -> PETEDS1_W<PETE_SPEC> {
        PETEDS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Parity Error Trap Enable for DSRAM2"]
    #[inline(always)]
    pub fn peteds2(&mut self) -> PETEDS2_W<PETE_SPEC> {
        PETEDS2_W::new(self, 2)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&mut self) -> PETEU0_W<PETE_SPEC> {
        PETEU0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&mut self) -> PETEU1_W<PETE_SPEC> {
        PETEU1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity Error Trap Enable for USIC2 Memory"]
    #[inline(always)]
    pub fn peteu2(&mut self) -> PETEU2_W<PETE_SPEC> {
        PETEU2_W::new(self, 10)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&mut self) -> PETEMC_W<PETE_SPEC> {
        PETEMC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&mut self) -> PETEPPRF_W<PETE_SPEC> {
        PETEPPRF_W::new(self, 13)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&mut self) -> PETEUSB_W<PETE_SPEC> {
        PETEUSB_W::new(self, 16)
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline(always)]
    pub fn peteeth0tx(&mut self) -> PETEETH0TX_W<PETE_SPEC> {
        PETEETH0TX_W::new(self, 17)
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline(always)]
    pub fn peteeth0rx(&mut self) -> PETEETH0RX_W<PETE_SPEC> {
        PETEETH0RX_W::new(self, 18)
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline(always)]
    pub fn petesd0(&mut self) -> PETESD0_W<PETE_SPEC> {
        PETESD0_W::new(self, 19)
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline(always)]
    pub fn petesd1(&mut self) -> PETESD1_W<PETE_SPEC> {
        PETESD1_W::new(self, 20)
    }
}
#[doc = "Parity Error Trap Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pete::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pete::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PETE_SPEC;
impl crate::RegisterSpec for PETE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pete::R`](R) reader structure"]
impl crate::Readable for PETE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pete::W`](W) writer structure"]
impl crate::Writable for PETE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PETE to value 0"]
impl crate::Resettable for PETE_SPEC {}
