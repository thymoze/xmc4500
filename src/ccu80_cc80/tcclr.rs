#[doc = "Register `TCCLR` writer"]
pub type W = crate::W<TCCLR_SPEC>;
#[doc = "Field `TRBC` writer - Timer Run Bit Clear"]
pub type TRBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Timer Clear"]
pub type TCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITC` writer - Dither Counter Clear"]
pub type DITC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTC1C` writer - Dead Time Counter 1 Clear"]
pub type DTC1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTC2C` writer - Dead Time Counter 2 Clear"]
pub type DTC2C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Timer Run Bit Clear"]
    #[inline(always)]
    pub fn trbc(&mut self) -> TRBC_W<TCCLR_SPEC> {
        TRBC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Clear"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W<TCCLR_SPEC> {
        TCC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Dither Counter Clear"]
    #[inline(always)]
    pub fn ditc(&mut self) -> DITC_W<TCCLR_SPEC> {
        DITC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Dead Time Counter 1 Clear"]
    #[inline(always)]
    pub fn dtc1c(&mut self) -> DTC1C_W<TCCLR_SPEC> {
        DTC1C_W::new(self, 3)
    }
    #[doc = "Bit 4 - Dead Time Counter 2 Clear"]
    #[inline(always)]
    pub fn dtc2c(&mut self) -> DTC2C_W<TCCLR_SPEC> {
        DTC2C_W::new(self, 4)
    }
}
#[doc = "Slice Timer Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCLR_SPEC;
impl crate::RegisterSpec for TCCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcclr::W`](W) writer structure"]
impl crate::Writable for TCCLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCCLR to value 0"]
impl crate::Resettable for TCCLR_SPEC {}
