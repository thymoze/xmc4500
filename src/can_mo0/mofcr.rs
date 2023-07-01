#[doc = "Register `MOFCR` reader"]
pub struct R(crate::R<MOFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOFCR` writer"]
pub struct W(crate::W<MOFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MOFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMC` reader - Message Mode Control"]
pub type MMC_R = crate::FieldReader<MMC_A>;
#[doc = "Message Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMC_A {
    #[doc = "0: Standard Message Object"]
    VALUE1 = 0,
    #[doc = "1: Receive FIFO Base Object"]
    VALUE2 = 1,
    #[doc = "2: Transmit FIFO Base Object"]
    VALUE3 = 2,
    #[doc = "3: Transmit FIFO Slave Object"]
    VALUE4 = 3,
    #[doc = "4: Gateway Source Object"]
    VALUE5 = 4,
}
impl From<MMC_A> for u8 {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMC_A {
    type Ux = u8;
}
impl MMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MMC_A> {
        match self.bits {
            0 => Some(MMC_A::VALUE1),
            1 => Some(MMC_A::VALUE2),
            2 => Some(MMC_A::VALUE3),
            3 => Some(MMC_A::VALUE4),
            4 => Some(MMC_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MMC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MMC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MMC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == MMC_A::VALUE5
    }
}
#[doc = "Field `MMC` writer - Message Mode Control"]
pub type MMC_W<'a, const O: u8> = crate::FieldWriter<'a, MOFCR_SPEC, 4, O, MMC_A>;
impl<'a, const O: u8> MMC_W<'a, O> {
    #[doc = "Standard Message Object"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMC_A::VALUE1)
    }
    #[doc = "Receive FIFO Base Object"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMC_A::VALUE2)
    }
    #[doc = "Transmit FIFO Base Object"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MMC_A::VALUE3)
    }
    #[doc = "Transmit FIFO Slave Object"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MMC_A::VALUE4)
    }
    #[doc = "Gateway Source Object"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(MMC_A::VALUE5)
    }
}
#[doc = "Field `GDFS` reader - Gateway Data Frame Send"]
pub type GDFS_R = crate::BitReader<GDFS_A>;
#[doc = "Gateway Data Frame Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GDFS_A {
    #[doc = "0: TXRQ is unchanged in the destination object."]
    VALUE1 = 0,
    #[doc = "1: TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    VALUE2 = 1,
}
impl From<GDFS_A> for bool {
    #[inline(always)]
    fn from(variant: GDFS_A) -> Self {
        variant as u8 != 0
    }
}
impl GDFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GDFS_A {
        match self.bits {
            false => GDFS_A::VALUE1,
            true => GDFS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GDFS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GDFS_A::VALUE2
    }
}
#[doc = "Field `GDFS` writer - Gateway Data Frame Send"]
pub type GDFS_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, GDFS_A>;
impl<'a, const O: u8> GDFS_W<'a, O> {
    #[doc = "TXRQ is unchanged in the destination object."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GDFS_A::VALUE1)
    }
    #[doc = "TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GDFS_A::VALUE2)
    }
}
#[doc = "Field `IDC` reader - Identifier Copy"]
pub type IDC_R = crate::BitReader<IDC_A>;
#[doc = "Identifier Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDC_A {
    #[doc = "0: The identifier of the gateway source object is not copied."]
    VALUE1 = 0,
    #[doc = "1: The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2 = 1,
}
impl From<IDC_A> for bool {
    #[inline(always)]
    fn from(variant: IDC_A) -> Self {
        variant as u8 != 0
    }
}
impl IDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDC_A {
        match self.bits {
            false => IDC_A::VALUE1,
            true => IDC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDC_A::VALUE2
    }
}
#[doc = "Field `IDC` writer - Identifier Copy"]
pub type IDC_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, IDC_A>;
impl<'a, const O: u8> IDC_W<'a, O> {
    #[doc = "The identifier of the gateway source object is not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDC_A::VALUE1)
    }
    #[doc = "The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDC_A::VALUE2)
    }
}
#[doc = "Field `DLCC` reader - Data Length Code Copy"]
pub type DLCC_R = crate::BitReader<DLCC_A>;
#[doc = "Data Length Code Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLCC_A {
    #[doc = "0: Data length code is not copied."]
    VALUE1 = 0,
    #[doc = "1: Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2 = 1,
}
impl From<DLCC_A> for bool {
    #[inline(always)]
    fn from(variant: DLCC_A) -> Self {
        variant as u8 != 0
    }
}
impl DLCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLCC_A {
        match self.bits {
            false => DLCC_A::VALUE1,
            true => DLCC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLCC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLCC_A::VALUE2
    }
}
#[doc = "Field `DLCC` writer - Data Length Code Copy"]
pub type DLCC_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, DLCC_A>;
impl<'a, const O: u8> DLCC_W<'a, O> {
    #[doc = "Data length code is not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLCC_A::VALUE1)
    }
    #[doc = "Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLCC_A::VALUE2)
    }
}
#[doc = "Field `DATC` reader - Data Copy"]
pub type DATC_R = crate::BitReader<DATC_A>;
#[doc = "Data Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATC_A {
    #[doc = "0: Data fields are not copied."]
    VALUE1 = 0,
    #[doc = "1: Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    VALUE2 = 1,
}
impl From<DATC_A> for bool {
    #[inline(always)]
    fn from(variant: DATC_A) -> Self {
        variant as u8 != 0
    }
}
impl DATC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATC_A {
        match self.bits {
            false => DATC_A::VALUE1,
            true => DATC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATC_A::VALUE2
    }
}
#[doc = "Field `DATC` writer - Data Copy"]
pub type DATC_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, DATC_A>;
impl<'a, const O: u8> DATC_W<'a, O> {
    #[doc = "Data fields are not copied."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATC_A::VALUE1)
    }
    #[doc = "Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATC_A::VALUE2)
    }
}
#[doc = "Field `RXIE` reader - Receive Interrupt Enable"]
pub type RXIE_R = crate::BitReader<RXIE_A>;
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIE_A {
    #[doc = "0: Message receive interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Message receive interrupt is enabled."]
    VALUE2 = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::VALUE1,
            true => RXIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXIE_A::VALUE2
    }
}
#[doc = "Field `RXIE` writer - Receive Interrupt Enable"]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, RXIE_A>;
impl<'a, const O: u8> RXIE_W<'a, O> {
    #[doc = "Message receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXIE_A::VALUE1)
    }
    #[doc = "Message receive interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXIE_A::VALUE2)
    }
}
#[doc = "Field `TXIE` reader - Transmit Interrupt Enable"]
pub type TXIE_R = crate::BitReader<TXIE_A>;
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIE_A {
    #[doc = "0: Message transmit interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Message transmit interrupt is enabled."]
    VALUE2 = 1,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::VALUE1,
            true => TXIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXIE_A::VALUE2
    }
}
#[doc = "Field `TXIE` writer - Transmit Interrupt Enable"]
pub type TXIE_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, TXIE_A>;
impl<'a, const O: u8> TXIE_W<'a, O> {
    #[doc = "Message transmit interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXIE_A::VALUE1)
    }
    #[doc = "Message transmit interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXIE_A::VALUE2)
    }
}
#[doc = "Field `OVIE` reader - Overflow Interrupt Enable"]
pub type OVIE_R = crate::BitReader<OVIE_A>;
#[doc = "Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVIE_A {
    #[doc = "0: FIFO full interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: FIFO full interrupt is enabled."]
    VALUE2 = 1,
}
impl From<OVIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVIE_A {
        match self.bits {
            false => OVIE_A::VALUE1,
            true => OVIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OVIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OVIE_A::VALUE2
    }
}
#[doc = "Field `OVIE` writer - Overflow Interrupt Enable"]
pub type OVIE_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, OVIE_A>;
impl<'a, const O: u8> OVIE_W<'a, O> {
    #[doc = "FIFO full interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OVIE_A::VALUE1)
    }
    #[doc = "FIFO full interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OVIE_A::VALUE2)
    }
}
#[doc = "Field `FRREN` reader - Foreign Remote Request Enable"]
pub type FRREN_R = crate::BitReader<FRREN_A>;
#[doc = "Foreign Remote Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRREN_A {
    #[doc = "0: TXRQ of message object n is set on reception of a matching Remote Frame."]
    VALUE1 = 0,
    #[doc = "1: TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    VALUE2 = 1,
}
impl From<FRREN_A> for bool {
    #[inline(always)]
    fn from(variant: FRREN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRREN_A {
        match self.bits {
            false => FRREN_A::VALUE1,
            true => FRREN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FRREN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FRREN_A::VALUE2
    }
}
#[doc = "Field `FRREN` writer - Foreign Remote Request Enable"]
pub type FRREN_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, FRREN_A>;
impl<'a, const O: u8> FRREN_W<'a, O> {
    #[doc = "TXRQ of message object n is set on reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FRREN_A::VALUE1)
    }
    #[doc = "TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FRREN_A::VALUE2)
    }
}
#[doc = "Field `RMM` reader - Transmit Object Remote Monitoring"]
pub type RMM_R = crate::BitReader<RMM_A>;
#[doc = "Transmit Object Remote Monitoring\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMM_A {
    #[doc = "0: Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    VALUE1 = 0,
    #[doc = "1: Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    VALUE2 = 1,
}
impl From<RMM_A> for bool {
    #[inline(always)]
    fn from(variant: RMM_A) -> Self {
        variant as u8 != 0
    }
}
impl RMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMM_A {
        match self.bits {
            false => RMM_A::VALUE1,
            true => RMM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RMM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RMM_A::VALUE2
    }
}
#[doc = "Field `RMM` writer - Transmit Object Remote Monitoring"]
pub type RMM_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O, RMM_A>;
impl<'a, const O: u8> RMM_W<'a, O> {
    #[doc = "Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RMM_A::VALUE1)
    }
    #[doc = "Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RMM_A::VALUE2)
    }
}
#[doc = "Field `SDT` reader - Single Data Transfer"]
pub type SDT_R = crate::BitReader;
#[doc = "Field `SDT` writer - Single Data Transfer"]
pub type SDT_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O>;
#[doc = "Field `STT` reader - Single Transmit Trial"]
pub type STT_R = crate::BitReader;
#[doc = "Field `STT` writer - Single Transmit Trial"]
pub type STT_W<'a, const O: u8> = crate::BitWriter<'a, MOFCR_SPEC, O>;
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DLC_W<'a, const O: u8> = crate::FieldWriter<'a, MOFCR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Gateway Data Frame Send"]
    #[inline(always)]
    pub fn gdfs(&self) -> GDFS_R {
        GDFS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Length Code Copy"]
    #[inline(always)]
    pub fn dlcc(&self) -> DLCC_R {
        DLCC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline(always)]
    pub fn datc(&self) -> DATC_R {
        DATC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovie(&self) -> OVIE_R {
        OVIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline(always)]
    pub fn frren(&self) -> FRREN_R {
        FRREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline(always)]
    pub fn rmm(&self) -> RMM_R {
        RMM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline(always)]
    pub fn sdt(&self) -> SDT_R {
        SDT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline(always)]
    pub fn stt(&self) -> STT_R {
        STT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn mmc(&mut self) -> MMC_W<0> {
        MMC_W::new(self)
    }
    #[doc = "Bit 8 - Gateway Data Frame Send"]
    #[inline(always)]
    #[must_use]
    pub fn gdfs(&mut self) -> GDFS_W<8> {
        GDFS_W::new(self)
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline(always)]
    #[must_use]
    pub fn idc(&mut self) -> IDC_W<9> {
        IDC_W::new(self)
    }
    #[doc = "Bit 10 - Data Length Code Copy"]
    #[inline(always)]
    #[must_use]
    pub fn dlcc(&mut self) -> DLCC_W<10> {
        DLCC_W::new(self)
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline(always)]
    #[must_use]
    pub fn datc(&mut self) -> DATC_W<11> {
        DATC_W::new(self)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<16> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<17> {
        TXIE_W::new(self)
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovie(&mut self) -> OVIE_W<18> {
        OVIE_W::new(self)
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frren(&mut self) -> FRREN_W<20> {
        FRREN_W::new(self)
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn rmm(&mut self) -> RMM_W<21> {
        RMM_W::new(self)
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sdt(&mut self) -> SDT_W<22> {
        SDT_W::new(self)
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline(always)]
    #[must_use]
    pub fn stt(&mut self) -> STT_W<23> {
        STT_W::new(self)
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<24> {
        DLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mofcr](index.html) module"]
pub struct MOFCR_SPEC;
impl crate::RegisterSpec for MOFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mofcr::R](R) reader structure"]
impl crate::Readable for MOFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mofcr::W](W) writer structure"]
impl crate::Writable for MOFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOFCR to value 0"]
impl crate::Resettable for MOFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
