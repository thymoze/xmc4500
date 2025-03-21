#[doc = "Register `LSTSRCREG` reader"]
pub type R = crate::R<LSTSRCREG_SPEC>;
#[doc = "Register `LSTSRCREG` writer"]
pub type W = crate::W<LSTSRCREG_SPEC>;
#[doc = "Source last request for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Source last request for channel 0"]
pub type CH0_R = crate::BitReader<CH0_A>;
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::VALUE1,
            true => CH0_A::VALUE2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH0_A::VALUE1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH0_A::VALUE2
    }
}
#[doc = "Field `CH0` writer - Source last request for channel 0"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG, CH0_A>;
impl<'a, REG> CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::VALUE2)
    }
}
#[doc = "Source last request for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Source last request for channel 1"]
pub type CH1_R = crate::BitReader<CH1_A>;
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::VALUE1,
            true => CH1_A::VALUE2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH1_A::VALUE1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH1_A::VALUE2
    }
}
#[doc = "Field `CH1` writer - Source last request for channel 1"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG, CH1_A>;
impl<'a, REG> CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::VALUE2)
    }
}
#[doc = "Source last request for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Source last request for channel 2"]
pub type CH2_R = crate::BitReader<CH2_A>;
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::VALUE1,
            true => CH2_A::VALUE2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH2_A::VALUE1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH2_A::VALUE2
    }
}
#[doc = "Field `CH2` writer - Source last request for channel 2"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG, CH2_A>;
impl<'a, REG> CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::VALUE2)
    }
}
#[doc = "Source last request for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Source last request for channel 3"]
pub type CH3_R = crate::BitReader<CH3_A>;
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::VALUE1,
            true => CH3_A::VALUE2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH3_A::VALUE1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH3_A::VALUE2
    }
}
#[doc = "Field `CH3` writer - Source last request for channel 3"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG, CH3_A>;
impl<'a, REG> CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::VALUE2)
    }
}
#[doc = "Source last request for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH4_A> for bool {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - Source last request for channel 4"]
pub type CH4_R = crate::BitReader<CH4_A>;
impl CH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH4_A {
        match self.bits {
            false => CH4_A::VALUE1,
            true => CH4_A::VALUE2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH4_A::VALUE1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH4_A::VALUE2
    }
}
#[doc = "Field `CH4` writer - Source last request for channel 4"]
pub type CH4_W<'a, REG> = crate::BitWriter<'a, REG, CH4_A>;
impl<'a, REG> CH4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH4_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH4_A::VALUE2)
    }
}
#[doc = "Source last request for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH5_A> for bool {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` reader - Source last request for channel 5"]
pub type CH5_R = crate::BitReader<CH5_A>;
impl CH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH5_A {
        match self.bits {
            false => CH5_A::VALUE1,
            true => CH5_A::VALUE2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH5_A::VALUE1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH5_A::VALUE2
    }
}
#[doc = "Field `CH5` writer - Source last request for channel 5"]
pub type CH5_W<'a, REG> = crate::BitWriter<'a, REG, CH5_A>;
impl<'a, REG> CH5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH5_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH5_A::VALUE2)
    }
}
#[doc = "Source last request for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH6_A> for bool {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` reader - Source last request for channel 6"]
pub type CH6_R = crate::BitReader<CH6_A>;
impl CH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH6_A {
        match self.bits {
            false => CH6_A::VALUE1,
            true => CH6_A::VALUE2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH6_A::VALUE1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH6_A::VALUE2
    }
}
#[doc = "Field `CH6` writer - Source last request for channel 6"]
pub type CH6_W<'a, REG> = crate::BitWriter<'a, REG, CH6_A>;
impl<'a, REG> CH6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH6_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH6_A::VALUE2)
    }
}
#[doc = "Source last request for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH7_A> for bool {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` reader - Source last request for channel 7"]
pub type CH7_R = crate::BitReader<CH7_A>;
impl CH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH7_A {
        match self.bits {
            false => CH7_A::VALUE1,
            true => CH7_A::VALUE2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH7_A::VALUE1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH7_A::VALUE2
    }
}
#[doc = "Field `CH7` writer - Source last request for channel 7"]
pub type CH7_W<'a, REG> = crate::BitWriter<'a, REG, CH7_A>;
impl<'a, REG> CH7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH7_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH7_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH0_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH0_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Source last transaction request write enable for channel 0"]
pub type WE_CH0_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH0_A>;
impl<'a, REG> WE_CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH0_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH0_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH1_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH1_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Source last transaction request write enable for channel 1"]
pub type WE_CH1_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH1_A>;
impl<'a, REG> WE_CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH1_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH1_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH2_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH2_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Source last transaction request write enable for channel 2"]
pub type WE_CH2_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH2_A>;
impl<'a, REG> WE_CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH2_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH2_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH3_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH3_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Source last transaction request write enable for channel 3"]
pub type WE_CH3_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH3_A>;
impl<'a, REG> WE_CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH3_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH3_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH4_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH4_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH4` writer - Source last transaction request write enable for channel 4"]
pub type WE_CH4_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH4_A>;
impl<'a, REG> WE_CH4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH4_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH4_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH5_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH5_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH5` writer - Source last transaction request write enable for channel 5"]
pub type WE_CH5_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH5_A>;
impl<'a, REG> WE_CH5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH5_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH5_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH6_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH6_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH6` writer - Source last transaction request write enable for channel 6"]
pub type WE_CH6_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH6_A>;
impl<'a, REG> WE_CH6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH6_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH6_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH7_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH7_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH7` writer - Source last transaction request write enable for channel 7"]
pub type WE_CH7_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH7_A>;
impl<'a, REG> WE_CH7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH7_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source last request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source last request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source last request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source last request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source last request for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source last request for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source last request for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source last request for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source last request for channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<LSTSRCREG_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Source last request for channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<LSTSRCREG_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Source last request for channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<LSTSRCREG_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Source last request for channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<LSTSRCREG_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Source last request for channel 4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W<LSTSRCREG_SPEC> {
        CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Source last request for channel 5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W<LSTSRCREG_SPEC> {
        CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Source last request for channel 6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W<LSTSRCREG_SPEC> {
        CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Source last request for channel 7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W<LSTSRCREG_SPEC> {
        CH7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Source last transaction request write enable for channel 0"]
    #[inline(always)]
    pub fn we_ch0(&mut self) -> WE_CH0_W<LSTSRCREG_SPEC> {
        WE_CH0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source last transaction request write enable for channel 1"]
    #[inline(always)]
    pub fn we_ch1(&mut self) -> WE_CH1_W<LSTSRCREG_SPEC> {
        WE_CH1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Source last transaction request write enable for channel 2"]
    #[inline(always)]
    pub fn we_ch2(&mut self) -> WE_CH2_W<LSTSRCREG_SPEC> {
        WE_CH2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Source last transaction request write enable for channel 3"]
    #[inline(always)]
    pub fn we_ch3(&mut self) -> WE_CH3_W<LSTSRCREG_SPEC> {
        WE_CH3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Source last transaction request write enable for channel 4"]
    #[inline(always)]
    pub fn we_ch4(&mut self) -> WE_CH4_W<LSTSRCREG_SPEC> {
        WE_CH4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Source last transaction request write enable for channel 5"]
    #[inline(always)]
    pub fn we_ch5(&mut self) -> WE_CH5_W<LSTSRCREG_SPEC> {
        WE_CH5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Source last transaction request write enable for channel 6"]
    #[inline(always)]
    pub fn we_ch6(&mut self) -> WE_CH6_W<LSTSRCREG_SPEC> {
        WE_CH6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Source last transaction request write enable for channel 7"]
    #[inline(always)]
    pub fn we_ch7(&mut self) -> WE_CH7_W<LSTSRCREG_SPEC> {
        WE_CH7_W::new(self, 15)
    }
}
#[doc = "Last Source Transaction Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lstsrcreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lstsrcreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSTSRCREG_SPEC;
impl crate::RegisterSpec for LSTSRCREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lstsrcreg::R`](R) reader structure"]
impl crate::Readable for LSTSRCREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lstsrcreg::W`](W) writer structure"]
impl crate::Writable for LSTSRCREG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LSTSRCREG to value 0"]
impl crate::Resettable for LSTSRCREG_SPEC {}
