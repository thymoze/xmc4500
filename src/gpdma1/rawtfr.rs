#[doc = "Register `RAWTFR` reader"]
pub type R = crate::R<RAWTFR_SPEC>;
#[doc = "Register `RAWTFR` writer"]
pub type W = crate::W<RAWTFR_SPEC>;
#[doc = "Field `CH0` reader - Raw Interrupt Status for channel 0"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - Raw Interrupt Status for channel 0"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Raw Interrupt Status for channel 1"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - Raw Interrupt Status for channel 1"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Raw Interrupt Status for channel 2"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - Raw Interrupt Status for channel 2"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Raw Interrupt Status for channel 3"]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - Raw Interrupt Status for channel 3"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<RAWTFR_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<RAWTFR_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<RAWTFR_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<RAWTFR_SPEC> {
        CH3_W::new(self, 3)
    }
}
#[doc = "Raw IntTfr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rawtfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawtfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAWTFR_SPEC;
impl crate::RegisterSpec for RAWTFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawtfr::R`](R) reader structure"]
impl crate::Readable for RAWTFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rawtfr::W`](W) writer structure"]
impl crate::Writable for RAWTFR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAWTFR to value 0"]
impl crate::Resettable for RAWTFR_SPEC {}
