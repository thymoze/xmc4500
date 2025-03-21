#[doc = "Register `PDR1` reader"]
pub type R = crate::R<PDR1_SPEC>;
#[doc = "Register `PDR1` writer"]
pub type W = crate::W<PDR1_SPEC>;
#[doc = "Pad Driver Mode for Pn.8\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD8_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD8_A> for u8 {
    #[inline(always)]
    fn from(variant: PD8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD8_A {
    type Ux = u8;
}
impl crate::IsEnum for PD8_A {}
#[doc = "Field `PD8` reader - Pad Driver Mode for Pn.8"]
pub type PD8_R = crate::FieldReader<PD8_A>;
impl PD8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD8_A> {
        match self.bits {
            0 => Some(PD8_A::SD_SHE),
            1 => Some(PD8_A::SD_MEE),
            2 => Some(PD8_A::SD_SOE),
            4 => Some(PD8_A::MD),
            7 => Some(PD8_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD8_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD8_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD8_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD8_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD8_A::WD
    }
}
#[doc = "Field `PD8` writer - Pad Driver Mode for Pn.8"]
pub type PD8_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD8_A>;
impl<'a, REG> PD8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD8_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD8_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD8_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD8_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD8_A::WD)
    }
}
#[doc = "Pad Driver Mode for Pn.9\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD9_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD9_A> for u8 {
    #[inline(always)]
    fn from(variant: PD9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD9_A {
    type Ux = u8;
}
impl crate::IsEnum for PD9_A {}
#[doc = "Field `PD9` reader - Pad Driver Mode for Pn.9"]
pub type PD9_R = crate::FieldReader<PD9_A>;
impl PD9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD9_A> {
        match self.bits {
            0 => Some(PD9_A::SD_SHE),
            1 => Some(PD9_A::SD_MEE),
            2 => Some(PD9_A::SD_SOE),
            4 => Some(PD9_A::MD),
            7 => Some(PD9_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD9_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD9_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD9_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD9_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD9_A::WD
    }
}
#[doc = "Field `PD9` writer - Pad Driver Mode for Pn.9"]
pub type PD9_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD9_A>;
impl<'a, REG> PD9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD9_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD9_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD9_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD9_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD9_A::WD)
    }
}
#[doc = "Pad Driver Mode for Pn.10\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD10_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE = 3,
    #[doc = "4: A1+ medium driver"]
    MD = 4,
    #[doc = "7: A1+ weak driver"]
    WD = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT = 5,
}
impl From<PD10_A> for u8 {
    #[inline(always)]
    fn from(variant: PD10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD10_A {
    type Ux = u8;
}
impl crate::IsEnum for PD10_A {}
#[doc = "Field `PD10` reader - Pad Driver Mode for Pn.10"]
pub type PD10_R = crate::FieldReader<PD10_A>;
impl PD10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD10_A {
        match self.bits {
            2 => PD10_A::SD_SOE,
            3 => PD10_A::SD_SLE,
            4 => PD10_A::MD,
            7 => PD10_A::WD,
            0 => PD10_A::SD_SOE_ALT,
            1 => PD10_A::SD_SLE_ALT,
            6 => PD10_A::MD_ALT,
            5 => PD10_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD10_A::SD_SOE
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD10_A::SD_SLE
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD10_A::MD
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD10_A::WD
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD10_A::SD_SOE_ALT
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD10_A::SD_SLE_ALT
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD10_A::MD_ALT
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD10_A::WD_ALT
    }
}
#[doc = "Field `PD10` writer - Pad Driver Mode for Pn.10"]
pub type PD10_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD10_A, crate::Safe>;
impl<'a, REG> PD10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD10_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(PD10_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD10_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD10_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD10_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD10_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD10_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD10_A::WD_ALT)
    }
}
#[doc = "Pad Driver Mode for Pn.11\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD11_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE = 3,
    #[doc = "4: A1+ medium driver"]
    MD = 4,
    #[doc = "7: A1+ weak driver"]
    WD = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT = 5,
}
impl From<PD11_A> for u8 {
    #[inline(always)]
    fn from(variant: PD11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD11_A {
    type Ux = u8;
}
impl crate::IsEnum for PD11_A {}
#[doc = "Field `PD11` reader - Pad Driver Mode for Pn.11"]
pub type PD11_R = crate::FieldReader<PD11_A>;
impl PD11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD11_A {
        match self.bits {
            2 => PD11_A::SD_SOE,
            3 => PD11_A::SD_SLE,
            4 => PD11_A::MD,
            7 => PD11_A::WD,
            0 => PD11_A::SD_SOE_ALT,
            1 => PD11_A::SD_SLE_ALT,
            6 => PD11_A::MD_ALT,
            5 => PD11_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD11_A::SD_SOE
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD11_A::SD_SLE
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD11_A::MD
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD11_A::WD
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD11_A::SD_SOE_ALT
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD11_A::SD_SLE_ALT
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD11_A::MD_ALT
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD11_A::WD_ALT
    }
}
#[doc = "Field `PD11` writer - Pad Driver Mode for Pn.11"]
pub type PD11_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD11_A, crate::Safe>;
impl<'a, REG> PD11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD11_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(PD11_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD11_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD11_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD11_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD11_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD11_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD11_A::WD_ALT)
    }
}
#[doc = "Pad Driver Mode for Pn.12\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD12_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD12_A> for u8 {
    #[inline(always)]
    fn from(variant: PD12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD12_A {
    type Ux = u8;
}
impl crate::IsEnum for PD12_A {}
#[doc = "Field `PD12` reader - Pad Driver Mode for Pn.12"]
pub type PD12_R = crate::FieldReader<PD12_A>;
impl PD12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD12_A> {
        match self.bits {
            0 => Some(PD12_A::SD_SHE),
            1 => Some(PD12_A::SD_MEE),
            2 => Some(PD12_A::SD_SOE),
            4 => Some(PD12_A::MD),
            7 => Some(PD12_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD12_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD12_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD12_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD12_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD12_A::WD
    }
}
#[doc = "Field `PD12` writer - Pad Driver Mode for Pn.12"]
pub type PD12_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD12_A>;
impl<'a, REG> PD12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD12_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD12_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD12_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD12_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD12_A::WD)
    }
}
#[doc = "Pad Driver Mode for Pn.13\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD13_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD13_A> for u8 {
    #[inline(always)]
    fn from(variant: PD13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD13_A {
    type Ux = u8;
}
impl crate::IsEnum for PD13_A {}
#[doc = "Field `PD13` reader - Pad Driver Mode for Pn.13"]
pub type PD13_R = crate::FieldReader<PD13_A>;
impl PD13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD13_A> {
        match self.bits {
            0 => Some(PD13_A::SD_SHE),
            1 => Some(PD13_A::SD_MEE),
            2 => Some(PD13_A::SD_SOE),
            4 => Some(PD13_A::MD),
            7 => Some(PD13_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD13_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD13_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD13_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD13_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD13_A::WD
    }
}
#[doc = "Field `PD13` writer - Pad Driver Mode for Pn.13"]
pub type PD13_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD13_A>;
impl<'a, REG> PD13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD13_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD13_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD13_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD13_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD13_A::WD)
    }
}
#[doc = "Pad Driver Mode for Pn.14\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD14_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD14_A> for u8 {
    #[inline(always)]
    fn from(variant: PD14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD14_A {
    type Ux = u8;
}
impl crate::IsEnum for PD14_A {}
#[doc = "Field `PD14` reader - Pad Driver Mode for Pn.14"]
pub type PD14_R = crate::FieldReader<PD14_A>;
impl PD14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD14_A> {
        match self.bits {
            0 => Some(PD14_A::SD_SHE),
            1 => Some(PD14_A::SD_MEE),
            2 => Some(PD14_A::SD_SOE),
            4 => Some(PD14_A::MD),
            7 => Some(PD14_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD14_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD14_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD14_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD14_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD14_A::WD
    }
}
#[doc = "Field `PD14` writer - Pad Driver Mode for Pn.14"]
pub type PD14_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD14_A>;
impl<'a, REG> PD14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD14_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD14_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD14_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD14_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD14_A::WD)
    }
}
#[doc = "Pad Driver Mode for Pn.15\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD15_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD15_A> for u8 {
    #[inline(always)]
    fn from(variant: PD15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD15_A {
    type Ux = u8;
}
impl crate::IsEnum for PD15_A {}
#[doc = "Field `PD15` reader - Pad Driver Mode for Pn.15"]
pub type PD15_R = crate::FieldReader<PD15_A>;
impl PD15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD15_A> {
        match self.bits {
            0 => Some(PD15_A::SD_SHE),
            1 => Some(PD15_A::SD_MEE),
            2 => Some(PD15_A::SD_SOE),
            4 => Some(PD15_A::MD),
            7 => Some(PD15_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD15_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD15_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD15_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD15_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD15_A::WD
    }
}
#[doc = "Field `PD15` writer - Pad Driver Mode for Pn.15"]
pub type PD15_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD15_A>;
impl<'a, REG> PD15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD15_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD15_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD15_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD15_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD15_A::WD)
    }
}
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W<PDR1_SPEC> {
        PD8_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W<PDR1_SPEC> {
        PD9_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W<PDR1_SPEC> {
        PD10_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W<PDR1_SPEC> {
        PD11_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W<PDR1_SPEC> {
        PD12_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W<PDR1_SPEC> {
        PD13_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W<PDR1_SPEC> {
        PD14_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W<PDR1_SPEC> {
        PD15_W::new(self, 28)
    }
}
#[doc = "Port 5 Pad Driver Mode 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDR1_SPEC;
impl crate::RegisterSpec for PDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr1::R`](R) reader structure"]
impl crate::Readable for PDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdr1::W`](W) writer structure"]
impl crate::Writable for PDR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDR1 to value 0x2222_2222"]
impl crate::Resettable for PDR1_SPEC {
    const RESET_VALUE: u32 = 0x2222_2222;
}
