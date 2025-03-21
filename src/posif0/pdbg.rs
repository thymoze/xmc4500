#[doc = "Register `PDBG` reader"]
pub type R = crate::R<PDBG_SPEC>;
#[doc = "Field `QCSV` reader - Quadrature Decoder Current state"]
pub type QCSV_R = crate::FieldReader;
#[doc = "Field `QPSV` reader - Quadrature Decoder Previous state"]
pub type QPSV_R = crate::FieldReader;
#[doc = "Field `IVAL` reader - Current Index Value"]
pub type IVAL_R = crate::BitReader;
#[doc = "Field `HSP` reader - Hall Current Sampled Pattern"]
pub type HSP_R = crate::FieldReader;
#[doc = "Field `LPP0` reader - Actual count of the Low Pass Filter for POSI0"]
pub type LPP0_R = crate::FieldReader;
#[doc = "Field `LPP1` reader - Actual count of the Low Pass Filter for POSI1"]
pub type LPP1_R = crate::FieldReader;
#[doc = "Field `LPP2` reader - Actual count of the Low Pass Filter for POSI2"]
pub type LPP2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Quadrature Decoder Current state"]
    #[inline(always)]
    pub fn qcsv(&self) -> QCSV_R {
        QCSV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Quadrature Decoder Previous state"]
    #[inline(always)]
    pub fn qpsv(&self) -> QPSV_R {
        QPSV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Current Index Value"]
    #[inline(always)]
    pub fn ival(&self) -> IVAL_R {
        IVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Hall Current Sampled Pattern"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:13 - Actual count of the Low Pass Filter for POSI0"]
    #[inline(always)]
    pub fn lpp0(&self) -> LPP0_R {
        LPP0_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Actual count of the Low Pass Filter for POSI1"]
    #[inline(always)]
    pub fn lpp1(&self) -> LPP1_R {
        LPP1_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - Actual count of the Low Pass Filter for POSI2"]
    #[inline(always)]
    pub fn lpp2(&self) -> LPP2_R {
        LPP2_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
#[doc = "POSIF Debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDBG_SPEC;
impl crate::RegisterSpec for PDBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdbg::R`](R) reader structure"]
impl crate::Readable for PDBG_SPEC {}
#[doc = "`reset()` method sets PDBG to value 0"]
impl crate::Resettable for PDBG_SPEC {}
