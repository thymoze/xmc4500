#[doc = "Register `PEEN` reader"]
pub type R = crate::R<PEEN_SPEC>;
#[doc = "Register `PEEN` writer"]
pub type W = crate::W<PEEN_SPEC>;
#[doc = "Parity Error Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENPS_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENPS_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENPS` reader - Parity Error Enable for PSRAM"]
pub type PEENPS_R = crate::BitReader<PEENPS_A>;
impl PEENPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENPS_A {
        match self.bits {
            false => PEENPS_A::VALUE1,
            true => PEENPS_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENPS_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENPS_A::VALUE2
    }
}
#[doc = "Field `PEENPS` writer - Parity Error Enable for PSRAM"]
pub type PEENPS_W<'a, REG> = crate::BitWriter<'a, REG, PEENPS_A>;
impl<'a, REG> PEENPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENPS_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENPS_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENDS1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENDS1` reader - Parity Error Enable for DSRAM1"]
pub type PEENDS1_R = crate::BitReader<PEENDS1_A>;
impl PEENDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENDS1_A {
        match self.bits {
            false => PEENDS1_A::VALUE1,
            true => PEENDS1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENDS1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENDS1_A::VALUE2
    }
}
#[doc = "Field `PEENDS1` writer - Parity Error Enable for DSRAM1"]
pub type PEENDS1_W<'a, REG> = crate::BitWriter<'a, REG, PEENDS1_A>;
impl<'a, REG> PEENDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENDS1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENDS1_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENDS2_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENDS2_A> for bool {
    #[inline(always)]
    fn from(variant: PEENDS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENDS2` reader - Parity Error Enable for DSRAM2"]
pub type PEENDS2_R = crate::BitReader<PEENDS2_A>;
impl PEENDS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENDS2_A {
        match self.bits {
            false => PEENDS2_A::VALUE1,
            true => PEENDS2_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENDS2_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENDS2_A::VALUE2
    }
}
#[doc = "Field `PEENDS2` writer - Parity Error Enable for DSRAM2"]
pub type PEENDS2_W<'a, REG> = crate::BitWriter<'a, REG, PEENDS2_A>;
impl<'a, REG> PEENDS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENDS2_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENDS2_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENU0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENU0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENU0` reader - Parity Error Enable for USIC0 Memory"]
pub type PEENU0_R = crate::BitReader<PEENU0_A>;
impl PEENU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENU0_A {
        match self.bits {
            false => PEENU0_A::VALUE1,
            true => PEENU0_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENU0_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENU0_A::VALUE2
    }
}
#[doc = "Field `PEENU0` writer - Parity Error Enable for USIC0 Memory"]
pub type PEENU0_W<'a, REG> = crate::BitWriter<'a, REG, PEENU0_A>;
impl<'a, REG> PEENU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENU0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENU0_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENU1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENU1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENU1` reader - Parity Error Enable for USIC1 Memory"]
pub type PEENU1_R = crate::BitReader<PEENU1_A>;
impl PEENU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENU1_A {
        match self.bits {
            false => PEENU1_A::VALUE1,
            true => PEENU1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENU1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENU1_A::VALUE2
    }
}
#[doc = "Field `PEENU1` writer - Parity Error Enable for USIC1 Memory"]
pub type PEENU1_W<'a, REG> = crate::BitWriter<'a, REG, PEENU1_A>;
impl<'a, REG> PEENU1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENU1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENU1_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for USIC2 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENU2_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENU2_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENU2` reader - Parity Error Enable for USIC2 Memory"]
pub type PEENU2_R = crate::BitReader<PEENU2_A>;
impl PEENU2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENU2_A {
        match self.bits {
            false => PEENU2_A::VALUE1,
            true => PEENU2_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENU2_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENU2_A::VALUE2
    }
}
#[doc = "Field `PEENU2` writer - Parity Error Enable for USIC2 Memory"]
pub type PEENU2_W<'a, REG> = crate::BitWriter<'a, REG, PEENU2_A>;
impl<'a, REG> PEENU2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENU2_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENU2_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENMC_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEENMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENMC` reader - Parity Error Enable for MultiCAN Memory"]
pub type PEENMC_R = crate::BitReader<PEENMC_A>;
impl PEENMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENMC_A {
        match self.bits {
            false => PEENMC_A::VALUE1,
            true => PEENMC_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENMC_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENMC_A::VALUE2
    }
}
#[doc = "Field `PEENMC` writer - Parity Error Enable for MultiCAN Memory"]
pub type PEENMC_W<'a, REG> = crate::BitWriter<'a, REG, PEENMC_A>;
impl<'a, REG> PEENMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENMC_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENMC_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENPPRF_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENPPRF` reader - Parity Error Enable for PMU Prefetch Memory"]
pub type PEENPPRF_R = crate::BitReader<PEENPPRF_A>;
impl PEENPPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENPPRF_A {
        match self.bits {
            false => PEENPPRF_A::VALUE1,
            true => PEENPPRF_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENPPRF_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENPPRF_A::VALUE2
    }
}
#[doc = "Field `PEENPPRF` writer - Parity Error Enable for PMU Prefetch Memory"]
pub type PEENPPRF_W<'a, REG> = crate::BitWriter<'a, REG, PEENPPRF_A>;
impl<'a, REG> PEENPPRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENPPRF_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENPPRF_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENUSB_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PEENUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENUSB` reader - Parity Error Enable for USB Memory"]
pub type PEENUSB_R = crate::BitReader<PEENUSB_A>;
impl PEENUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENUSB_A {
        match self.bits {
            false => PEENUSB_A::VALUE1,
            true => PEENUSB_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENUSB_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENUSB_A::VALUE2
    }
}
#[doc = "Field `PEENUSB` writer - Parity Error Enable for USB Memory"]
pub type PEENUSB_W<'a, REG> = crate::BitWriter<'a, REG, PEENUSB_A>;
impl<'a, REG> PEENUSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENUSB_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENUSB_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENETH0TX_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: PEENETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENETH0TX` reader - Parity Error Enable for ETH TX Memory"]
pub type PEENETH0TX_R = crate::BitReader<PEENETH0TX_A>;
impl PEENETH0TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENETH0TX_A {
        match self.bits {
            false => PEENETH0TX_A::VALUE1,
            true => PEENETH0TX_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENETH0TX_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENETH0TX_A::VALUE2
    }
}
#[doc = "Field `PEENETH0TX` writer - Parity Error Enable for ETH TX Memory"]
pub type PEENETH0TX_W<'a, REG> = crate::BitWriter<'a, REG, PEENETH0TX_A>;
impl<'a, REG> PEENETH0TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENETH0TX_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENETH0TX_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENETH0RX_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: PEENETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENETH0RX` reader - Parity Error Enable for ETH RX Memory"]
pub type PEENETH0RX_R = crate::BitReader<PEENETH0RX_A>;
impl PEENETH0RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENETH0RX_A {
        match self.bits {
            false => PEENETH0RX_A::VALUE1,
            true => PEENETH0RX_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENETH0RX_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENETH0RX_A::VALUE2
    }
}
#[doc = "Field `PEENETH0RX` writer - Parity Error Enable for ETH RX Memory"]
pub type PEENETH0RX_W<'a, REG> = crate::BitWriter<'a, REG, PEENETH0RX_A>;
impl<'a, REG> PEENETH0RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENETH0RX_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENETH0RX_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENSD0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENSD0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENSD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENSD0` reader - Parity Error Enable for SDMMC Memory 0"]
pub type PEENSD0_R = crate::BitReader<PEENSD0_A>;
impl PEENSD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENSD0_A {
        match self.bits {
            false => PEENSD0_A::VALUE1,
            true => PEENSD0_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENSD0_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENSD0_A::VALUE2
    }
}
#[doc = "Field `PEENSD0` writer - Parity Error Enable for SDMMC Memory 0"]
pub type PEENSD0_W<'a, REG> = crate::BitWriter<'a, REG, PEENSD0_A>;
impl<'a, REG> PEENSD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENSD0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENSD0_A::VALUE2)
    }
}
#[doc = "Parity Error Enable for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENSD1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENSD1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENSD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENSD1` reader - Parity Error Enable for SDMMC Memory 1"]
pub type PEENSD1_R = crate::BitReader<PEENSD1_A>;
impl PEENSD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEENSD1_A {
        match self.bits {
            false => PEENSD1_A::VALUE1,
            true => PEENSD1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENSD1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENSD1_A::VALUE2
    }
}
#[doc = "Field `PEENSD1` writer - Parity Error Enable for SDMMC Memory 1"]
pub type PEENSD1_W<'a, REG> = crate::BitWriter<'a, REG, PEENSD1_A>;
impl<'a, REG> PEENSD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEENSD1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEENSD1_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&self) -> PEENPS_R {
        PEENPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&self) -> PEENDS1_R {
        PEENDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Enable for DSRAM2"]
    #[inline(always)]
    pub fn peends2(&self) -> PEENDS2_R {
        PEENDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&self) -> PEENU0_R {
        PEENU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&self) -> PEENU1_R {
        PEENU1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity Error Enable for USIC2 Memory"]
    #[inline(always)]
    pub fn peenu2(&self) -> PEENU2_R {
        PEENU2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&self) -> PEENMC_R {
        PEENMC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&self) -> PEENPPRF_R {
        PEENPPRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&self) -> PEENUSB_R {
        PEENUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline(always)]
    pub fn peeneth0tx(&self) -> PEENETH0TX_R {
        PEENETH0TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline(always)]
    pub fn peeneth0rx(&self) -> PEENETH0RX_R {
        PEENETH0RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline(always)]
    pub fn peensd0(&self) -> PEENSD0_R {
        PEENSD0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline(always)]
    pub fn peensd1(&self) -> PEENSD1_R {
        PEENSD1_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&mut self) -> PEENPS_W<PEEN_SPEC> {
        PEENPS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&mut self) -> PEENDS1_W<PEEN_SPEC> {
        PEENDS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Parity Error Enable for DSRAM2"]
    #[inline(always)]
    pub fn peends2(&mut self) -> PEENDS2_W<PEEN_SPEC> {
        PEENDS2_W::new(self, 2)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&mut self) -> PEENU0_W<PEEN_SPEC> {
        PEENU0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&mut self) -> PEENU1_W<PEEN_SPEC> {
        PEENU1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity Error Enable for USIC2 Memory"]
    #[inline(always)]
    pub fn peenu2(&mut self) -> PEENU2_W<PEEN_SPEC> {
        PEENU2_W::new(self, 10)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&mut self) -> PEENMC_W<PEEN_SPEC> {
        PEENMC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&mut self) -> PEENPPRF_W<PEEN_SPEC> {
        PEENPPRF_W::new(self, 13)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&mut self) -> PEENUSB_W<PEEN_SPEC> {
        PEENUSB_W::new(self, 16)
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline(always)]
    pub fn peeneth0tx(&mut self) -> PEENETH0TX_W<PEEN_SPEC> {
        PEENETH0TX_W::new(self, 17)
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline(always)]
    pub fn peeneth0rx(&mut self) -> PEENETH0RX_W<PEEN_SPEC> {
        PEENETH0RX_W::new(self, 18)
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline(always)]
    pub fn peensd0(&mut self) -> PEENSD0_W<PEEN_SPEC> {
        PEENSD0_W::new(self, 19)
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline(always)]
    pub fn peensd1(&mut self) -> PEENSD1_W<PEEN_SPEC> {
        PEENSD1_W::new(self, 20)
    }
}
#[doc = "Parity Error Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEEN_SPEC;
impl crate::RegisterSpec for PEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peen::R`](R) reader structure"]
impl crate::Readable for PEEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peen::W`](W) writer structure"]
impl crate::Writable for PEEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEEN to value 0"]
impl crate::Resettable for PEEN_SPEC {}
