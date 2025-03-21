#[doc = "Register `FCON` reader"]
pub type R = crate::R<FCON_SPEC>;
#[doc = "Register `FCON` writer"]
pub type W = crate::W<FCON_SPEC>;
#[doc = "Wait States for read access to PFLASH\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WSPFLASH_A {
    #[doc = "0: PFLASH access in one clock cycle"]
    VALUE1 = 0,
    #[doc = "1: PFLASH access in one clock cycle"]
    VALUE2 = 1,
    #[doc = "2: PFLASH access in two clock cycles"]
    VALUE3 = 2,
    #[doc = "3: PFLASH access in three clock cycles"]
    VALUE4 = 3,
    #[doc = "15: PFLASH access in fifteen clock cycles."]
    VALUE5 = 15,
}
impl From<WSPFLASH_A> for u8 {
    #[inline(always)]
    fn from(variant: WSPFLASH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WSPFLASH_A {
    type Ux = u8;
}
impl crate::IsEnum for WSPFLASH_A {}
#[doc = "Field `WSPFLASH` reader - Wait States for read access to PFLASH"]
pub type WSPFLASH_R = crate::FieldReader<WSPFLASH_A>;
impl WSPFLASH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WSPFLASH_A> {
        match self.bits {
            0 => Some(WSPFLASH_A::VALUE1),
            1 => Some(WSPFLASH_A::VALUE2),
            2 => Some(WSPFLASH_A::VALUE3),
            3 => Some(WSPFLASH_A::VALUE4),
            15 => Some(WSPFLASH_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WSPFLASH_A::VALUE1
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WSPFLASH_A::VALUE2
    }
    #[doc = "PFLASH access in two clock cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WSPFLASH_A::VALUE3
    }
    #[doc = "PFLASH access in three clock cycles"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WSPFLASH_A::VALUE4
    }
    #[doc = "PFLASH access in fifteen clock cycles."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == WSPFLASH_A::VALUE5
    }
}
#[doc = "Field `WSPFLASH` writer - Wait States for read access to PFLASH"]
pub type WSPFLASH_W<'a, REG> = crate::FieldWriter<'a, REG, 4, WSPFLASH_A>;
impl<'a, REG> WSPFLASH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WSPFLASH_A::VALUE1)
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WSPFLASH_A::VALUE2)
    }
    #[doc = "PFLASH access in two clock cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(WSPFLASH_A::VALUE3)
    }
    #[doc = "PFLASH access in three clock cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(WSPFLASH_A::VALUE4)
    }
    #[doc = "PFLASH access in fifteen clock cycles."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(WSPFLASH_A::VALUE5)
    }
}
#[doc = "Wait State for Error Correction of PFLASH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSECPF_A {
    #[doc = "0: No additional wait state for error correction"]
    VALUE1 = 0,
    #[doc = "1: One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    VALUE2 = 1,
}
impl From<WSECPF_A> for bool {
    #[inline(always)]
    fn from(variant: WSECPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSECPF` reader - Wait State for Error Correction of PFLASH"]
pub type WSECPF_R = crate::BitReader<WSECPF_A>;
impl WSECPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WSECPF_A {
        match self.bits {
            false => WSECPF_A::VALUE1,
            true => WSECPF_A::VALUE2,
        }
    }
    #[doc = "No additional wait state for error correction"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WSECPF_A::VALUE1
    }
    #[doc = "One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WSECPF_A::VALUE2
    }
}
#[doc = "Field `WSECPF` writer - Wait State for Error Correction of PFLASH"]
pub type WSECPF_W<'a, REG> = crate::BitWriter<'a, REG, WSECPF_A>;
impl<'a, REG> WSECPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No additional wait state for error correction"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WSECPF_A::VALUE1)
    }
    #[doc = "One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WSECPF_A::VALUE2)
    }
}
#[doc = "Dynamic Flash Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE_A {
    #[doc = "0: Normal/standard Flash read operation"]
    VALUE1 = 0,
    #[doc = "1: Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    VALUE2 = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Dynamic Flash Idle"]
pub type IDLE_R = crate::BitReader<IDLE_A>;
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::VALUE1,
            true => IDLE_A::VALUE2,
        }
    }
    #[doc = "Normal/standard Flash read operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDLE_A::VALUE1
    }
    #[doc = "Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDLE_A::VALUE2
    }
}
#[doc = "Field `IDLE` writer - Dynamic Flash Idle"]
pub type IDLE_W<'a, REG> = crate::BitWriter<'a, REG, IDLE_A>;
impl<'a, REG> IDLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal/standard Flash read operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IDLE_A::VALUE1)
    }
    #[doc = "Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IDLE_A::VALUE2)
    }
}
#[doc = "External Sleep Request Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESLDIS_A {
    #[doc = "0: External sleep request signal input is enabled"]
    VALUE1 = 0,
    #[doc = "1: Externally requested Flash sleep is disabled"]
    VALUE2 = 1,
}
impl From<ESLDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ESLDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESLDIS` reader - External Sleep Request Disable"]
pub type ESLDIS_R = crate::BitReader<ESLDIS_A>;
impl ESLDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESLDIS_A {
        match self.bits {
            false => ESLDIS_A::VALUE1,
            true => ESLDIS_A::VALUE2,
        }
    }
    #[doc = "External sleep request signal input is enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ESLDIS_A::VALUE1
    }
    #[doc = "Externally requested Flash sleep is disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ESLDIS_A::VALUE2
    }
}
#[doc = "Field `ESLDIS` writer - External Sleep Request Disable"]
pub type ESLDIS_W<'a, REG> = crate::BitWriter<'a, REG, ESLDIS_A>;
impl<'a, REG> ESLDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External sleep request signal input is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ESLDIS_A::VALUE1)
    }
    #[doc = "Externally requested Flash sleep is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ESLDIS_A::VALUE2)
    }
}
#[doc = "Flash SLEEP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_A {
    #[doc = "0: Normal state or wake-up"]
    VALUE1 = 0,
    #[doc = "1: Flash sleep mode is requested"]
    VALUE2 = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Flash SLEEP"]
pub type SLEEP_R = crate::BitReader<SLEEP_A>;
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::VALUE1,
            true => SLEEP_A::VALUE2,
        }
    }
    #[doc = "Normal state or wake-up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLEEP_A::VALUE1
    }
    #[doc = "Flash sleep mode is requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLEEP_A::VALUE2
    }
}
#[doc = "Field `SLEEP` writer - Flash SLEEP"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG, SLEEP_A>;
impl<'a, REG> SLEEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal state or wake-up"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::VALUE1)
    }
    #[doc = "Flash sleep mode is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::VALUE2)
    }
}
#[doc = "Read Protection Activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPA_A {
    #[doc = "0: The Flash-internal read protection is not activated. Bits DCF, DDF are not taken into account. Bits DCF, DDFx can be cleared"]
    VALUE1 = 0,
    #[doc = "1: The Flash-internal read protection is activated. Bits DCF, DDF are enabled and evaluated."]
    VALUE2 = 1,
}
impl From<RPA_A> for bool {
    #[inline(always)]
    fn from(variant: RPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPA` reader - Read Protection Activated"]
pub type RPA_R = crate::BitReader<RPA_A>;
impl RPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPA_A {
        match self.bits {
            false => RPA_A::VALUE1,
            true => RPA_A::VALUE2,
        }
    }
    #[doc = "The Flash-internal read protection is not activated. Bits DCF, DDF are not taken into account. Bits DCF, DDFx can be cleared"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPA_A::VALUE1
    }
    #[doc = "The Flash-internal read protection is activated. Bits DCF, DDF are enabled and evaluated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPA_A::VALUE2
    }
}
#[doc = "Disable Code Fetch from Flash Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCF_A {
    #[doc = "0: Code fetching from the Flash memory area is allowed."]
    VALUE1 = 0,
    #[doc = "1: Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    VALUE2 = 1,
}
impl From<DCF_A> for bool {
    #[inline(always)]
    fn from(variant: DCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCF` reader - Disable Code Fetch from Flash Memory"]
pub type DCF_R = crate::BitReader<DCF_A>;
impl DCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCF_A {
        match self.bits {
            false => DCF_A::VALUE1,
            true => DCF_A::VALUE2,
        }
    }
    #[doc = "Code fetching from the Flash memory area is allowed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCF_A::VALUE1
    }
    #[doc = "Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCF_A::VALUE2
    }
}
#[doc = "Field `DCF` writer - Disable Code Fetch from Flash Memory"]
pub type DCF_W<'a, REG> = crate::BitWriter<'a, REG, DCF_A>;
impl<'a, REG> DCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Code fetching from the Flash memory area is allowed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DCF_A::VALUE1)
    }
    #[doc = "Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DCF_A::VALUE2)
    }
}
#[doc = "Disable Any Data Fetch from Flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDF_A {
    #[doc = "0: Data read access to the Flash memory area is allowed."]
    VALUE1 = 0,
    #[doc = "1: Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    VALUE2 = 1,
}
impl From<DDF_A> for bool {
    #[inline(always)]
    fn from(variant: DDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDF` reader - Disable Any Data Fetch from Flash"]
pub type DDF_R = crate::BitReader<DDF_A>;
impl DDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDF_A {
        match self.bits {
            false => DDF_A::VALUE1,
            true => DDF_A::VALUE2,
        }
    }
    #[doc = "Data read access to the Flash memory area is allowed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DDF_A::VALUE1
    }
    #[doc = "Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DDF_A::VALUE2
    }
}
#[doc = "Field `DDF` writer - Disable Any Data Fetch from Flash"]
pub type DDF_W<'a, REG> = crate::BitWriter<'a, REG, DDF_A>;
impl<'a, REG> DDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data read access to the Flash memory area is allowed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DDF_A::VALUE1)
    }
    #[doc = "Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DDF_A::VALUE2)
    }
}
#[doc = "Verify and Operation Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOPERM_A {
    #[doc = "0: Interrupt not enabled"]
    VALUE1 = 0,
    #[doc = "1: Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    VALUE2 = 1,
}
impl From<VOPERM_A> for bool {
    #[inline(always)]
    fn from(variant: VOPERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPERM` reader - Verify and Operation Error Interrupt Mask"]
pub type VOPERM_R = crate::BitReader<VOPERM_A>;
impl VOPERM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOPERM_A {
        match self.bits {
            false => VOPERM_A::VALUE1,
            true => VOPERM_A::VALUE2,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VOPERM_A::VALUE1
    }
    #[doc = "Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VOPERM_A::VALUE2
    }
}
#[doc = "Field `VOPERM` writer - Verify and Operation Error Interrupt Mask"]
pub type VOPERM_W<'a, REG> = crate::BitWriter<'a, REG, VOPERM_A>;
impl<'a, REG> VOPERM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VOPERM_A::VALUE1)
    }
    #[doc = "Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VOPERM_A::VALUE2)
    }
}
#[doc = "Command Sequence Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQERM_A {
    #[doc = "0: Interrupt not enabled"]
    VALUE1 = 0,
    #[doc = "1: Flash interrupt because of Sequence Error is enabled"]
    VALUE2 = 1,
}
impl From<SQERM_A> for bool {
    #[inline(always)]
    fn from(variant: SQERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQERM` reader - Command Sequence Error Interrupt Mask"]
pub type SQERM_R = crate::BitReader<SQERM_A>;
impl SQERM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SQERM_A {
        match self.bits {
            false => SQERM_A::VALUE1,
            true => SQERM_A::VALUE2,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SQERM_A::VALUE1
    }
    #[doc = "Flash interrupt because of Sequence Error is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SQERM_A::VALUE2
    }
}
#[doc = "Field `SQERM` writer - Command Sequence Error Interrupt Mask"]
pub type SQERM_W<'a, REG> = crate::BitWriter<'a, REG, SQERM_A>;
impl<'a, REG> SQERM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SQERM_A::VALUE1)
    }
    #[doc = "Flash interrupt because of Sequence Error is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SQERM_A::VALUE2)
    }
}
#[doc = "Protection Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROERM_A {
    #[doc = "0: Interrupt not enabled"]
    VALUE1 = 0,
    #[doc = "1: Flash interrupt because of Protection Error is enabled"]
    VALUE2 = 1,
}
impl From<PROERM_A> for bool {
    #[inline(always)]
    fn from(variant: PROERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROERM` reader - Protection Error Interrupt Mask"]
pub type PROERM_R = crate::BitReader<PROERM_A>;
impl PROERM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PROERM_A {
        match self.bits {
            false => PROERM_A::VALUE1,
            true => PROERM_A::VALUE2,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PROERM_A::VALUE1
    }
    #[doc = "Flash interrupt because of Protection Error is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PROERM_A::VALUE2
    }
}
#[doc = "Field `PROERM` writer - Protection Error Interrupt Mask"]
pub type PROERM_W<'a, REG> = crate::BitWriter<'a, REG, PROERM_A>;
impl<'a, REG> PROERM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PROERM_A::VALUE1)
    }
    #[doc = "Flash interrupt because of Protection Error is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PROERM_A::VALUE2)
    }
}
#[doc = "PFLASH Single-Bit Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFSBERM_A {
    #[doc = "0: No Single-Bit Error interrupt enabled"]
    VALUE1 = 0,
    #[doc = "1: Single-Bit Error interrupt enabled for PFLASH"]
    VALUE2 = 1,
}
impl From<PFSBERM_A> for bool {
    #[inline(always)]
    fn from(variant: PFSBERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFSBERM` reader - PFLASH Single-Bit Error Interrupt Mask"]
pub type PFSBERM_R = crate::BitReader<PFSBERM_A>;
impl PFSBERM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PFSBERM_A {
        match self.bits {
            false => PFSBERM_A::VALUE1,
            true => PFSBERM_A::VALUE2,
        }
    }
    #[doc = "No Single-Bit Error interrupt enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PFSBERM_A::VALUE1
    }
    #[doc = "Single-Bit Error interrupt enabled for PFLASH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PFSBERM_A::VALUE2
    }
}
#[doc = "Field `PFSBERM` writer - PFLASH Single-Bit Error Interrupt Mask"]
pub type PFSBERM_W<'a, REG> = crate::BitWriter<'a, REG, PFSBERM_A>;
impl<'a, REG> PFSBERM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Single-Bit Error interrupt enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PFSBERM_A::VALUE1)
    }
    #[doc = "Single-Bit Error interrupt enabled for PFLASH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PFSBERM_A::VALUE2)
    }
}
#[doc = "PFLASH Double-Bit Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFDBERM_A {
    #[doc = "0: Double-Bit Error interrupt for PFLASH not enabled"]
    VALUE1 = 0,
    #[doc = "1: Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    VALUE2 = 1,
}
impl From<PFDBERM_A> for bool {
    #[inline(always)]
    fn from(variant: PFDBERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFDBERM` reader - PFLASH Double-Bit Error Interrupt Mask"]
pub type PFDBERM_R = crate::BitReader<PFDBERM_A>;
impl PFDBERM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PFDBERM_A {
        match self.bits {
            false => PFDBERM_A::VALUE1,
            true => PFDBERM_A::VALUE2,
        }
    }
    #[doc = "Double-Bit Error interrupt for PFLASH not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PFDBERM_A::VALUE1
    }
    #[doc = "Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PFDBERM_A::VALUE2
    }
}
#[doc = "Field `PFDBERM` writer - PFLASH Double-Bit Error Interrupt Mask"]
pub type PFDBERM_W<'a, REG> = crate::BitWriter<'a, REG, PFDBERM_A>;
impl<'a, REG> PFDBERM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Double-Bit Error interrupt for PFLASH not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PFDBERM_A::VALUE1)
    }
    #[doc = "Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PFDBERM_A::VALUE2)
    }
}
#[doc = "End of Busy Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBM_A {
    #[doc = "0: Interrupt not enabled"]
    VALUE1 = 0,
    #[doc = "1: EOB interrupt is enabled"]
    VALUE2 = 1,
}
impl From<EOBM_A> for bool {
    #[inline(always)]
    fn from(variant: EOBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOBM` reader - End of Busy Interrupt Mask"]
pub type EOBM_R = crate::BitReader<EOBM_A>;
impl EOBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOBM_A {
        match self.bits {
            false => EOBM_A::VALUE1,
            true => EOBM_A::VALUE2,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EOBM_A::VALUE1
    }
    #[doc = "EOB interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EOBM_A::VALUE2
    }
}
#[doc = "Field `EOBM` writer - End of Busy Interrupt Mask"]
pub type EOBM_W<'a, REG> = crate::BitWriter<'a, REG, EOBM_A>;
impl<'a, REG> EOBM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EOBM_A::VALUE1)
    }
    #[doc = "EOB interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EOBM_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait States for read access to PFLASH"]
    #[inline(always)]
    pub fn wspflash(&self) -> WSPFLASH_R {
        WSPFLASH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Wait State for Error Correction of PFLASH"]
    #[inline(always)]
    pub fn wsecpf(&self) -> WSECPF_R {
        WSECPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Dynamic Flash Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Sleep Request Disable"]
    #[inline(always)]
    pub fn esldis(&self) -> ESLDIS_R {
        ESLDIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read Protection Activated"]
    #[inline(always)]
    pub fn rpa(&self) -> RPA_R {
        RPA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Code Fetch from Flash Memory"]
    #[inline(always)]
    pub fn dcf(&self) -> DCF_R {
        DCF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable Any Data Fetch from Flash"]
    #[inline(always)]
    pub fn ddf(&self) -> DDF_R {
        DDF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Verify and Operation Error Interrupt Mask"]
    #[inline(always)]
    pub fn voperm(&self) -> VOPERM_R {
        VOPERM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Command Sequence Error Interrupt Mask"]
    #[inline(always)]
    pub fn sqerm(&self) -> SQERM_R {
        SQERM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection Error Interrupt Mask"]
    #[inline(always)]
    pub fn proerm(&self) -> PROERM_R {
        PROERM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PFLASH Single-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfsberm(&self) -> PFSBERM_R {
        PFSBERM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - PFLASH Double-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfdberm(&self) -> PFDBERM_R {
        PFDBERM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - End of Busy Interrupt Mask"]
    #[inline(always)]
    pub fn eobm(&self) -> EOBM_R {
        EOBM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait States for read access to PFLASH"]
    #[inline(always)]
    pub fn wspflash(&mut self) -> WSPFLASH_W<FCON_SPEC> {
        WSPFLASH_W::new(self, 0)
    }
    #[doc = "Bit 4 - Wait State for Error Correction of PFLASH"]
    #[inline(always)]
    pub fn wsecpf(&mut self) -> WSECPF_W<FCON_SPEC> {
        WSECPF_W::new(self, 4)
    }
    #[doc = "Bit 13 - Dynamic Flash Idle"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W<FCON_SPEC> {
        IDLE_W::new(self, 13)
    }
    #[doc = "Bit 14 - External Sleep Request Disable"]
    #[inline(always)]
    pub fn esldis(&mut self) -> ESLDIS_W<FCON_SPEC> {
        ESLDIS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Flash SLEEP"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<FCON_SPEC> {
        SLEEP_W::new(self, 15)
    }
    #[doc = "Bit 17 - Disable Code Fetch from Flash Memory"]
    #[inline(always)]
    pub fn dcf(&mut self) -> DCF_W<FCON_SPEC> {
        DCF_W::new(self, 17)
    }
    #[doc = "Bit 18 - Disable Any Data Fetch from Flash"]
    #[inline(always)]
    pub fn ddf(&mut self) -> DDF_W<FCON_SPEC> {
        DDF_W::new(self, 18)
    }
    #[doc = "Bit 24 - Verify and Operation Error Interrupt Mask"]
    #[inline(always)]
    pub fn voperm(&mut self) -> VOPERM_W<FCON_SPEC> {
        VOPERM_W::new(self, 24)
    }
    #[doc = "Bit 25 - Command Sequence Error Interrupt Mask"]
    #[inline(always)]
    pub fn sqerm(&mut self) -> SQERM_W<FCON_SPEC> {
        SQERM_W::new(self, 25)
    }
    #[doc = "Bit 26 - Protection Error Interrupt Mask"]
    #[inline(always)]
    pub fn proerm(&mut self) -> PROERM_W<FCON_SPEC> {
        PROERM_W::new(self, 26)
    }
    #[doc = "Bit 27 - PFLASH Single-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfsberm(&mut self) -> PFSBERM_W<FCON_SPEC> {
        PFSBERM_W::new(self, 27)
    }
    #[doc = "Bit 29 - PFLASH Double-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfdberm(&mut self) -> PFDBERM_W<FCON_SPEC> {
        PFDBERM_W::new(self, 29)
    }
    #[doc = "Bit 31 - End of Busy Interrupt Mask"]
    #[inline(always)]
    pub fn eobm(&mut self) -> EOBM_W<FCON_SPEC> {
        EOBM_W::new(self, 31)
    }
}
#[doc = "Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCON_SPEC;
impl crate::RegisterSpec for FCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcon::R`](R) reader structure"]
impl crate::Readable for FCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcon::W`](W) writer structure"]
impl crate::Writable for FCON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCON to value 0x06"]
impl crate::Resettable for FCON_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
