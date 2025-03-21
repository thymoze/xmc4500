#[doc = "Register `WDTCLR` writer"]
pub type W = crate::W<WDTCLR_SPEC>;
#[doc = "Field `ALMC` writer - Pre-warning Alarm"]
pub type ALMC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Pre-warning Alarm"]
    #[inline(always)]
    pub fn almc(&mut self) -> ALMC_W<WDTCLR_SPEC> {
        ALMC_W::new(self, 0)
    }
}
#[doc = "WDT Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCLR_SPEC;
impl crate::RegisterSpec for WDTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdtclr::W`](W) writer structure"]
impl crate::Writable for WDTCLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCLR to value 0"]
impl crate::Resettable for WDTCLR_SPEC {}
