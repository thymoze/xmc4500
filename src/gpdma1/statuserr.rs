#[doc = "Register `STATUSERR` reader"]
pub type R = crate::R<STATUSERR_SPEC>;
#[doc = "Field `CH0` reader - Interrupt Status for channel 0"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH1` reader - Interrupt Status for channel 1"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH2` reader - Interrupt Status for channel 2"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH3` reader - Interrupt Status for channel 3"]
pub type CH3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "IntErr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`statuserr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSERR_SPEC;
impl crate::RegisterSpec for STATUSERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statuserr::R`](R) reader structure"]
impl crate::Readable for STATUSERR_SPEC {}
#[doc = "`reset()` method sets STATUSERR to value 0"]
impl crate::Resettable for STATUSERR_SPEC {}
