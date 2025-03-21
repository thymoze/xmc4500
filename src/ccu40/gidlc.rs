#[doc = "Register `GIDLC` writer"]
pub type W = crate::W<GIDLC_SPEC>;
#[doc = "Field `CS0I` writer - CC40 IDLE mode clear"]
pub type CS0I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1I` writer - CC41 IDLE mode clear"]
pub type CS1I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS2I` writer - CC42 IDLE mode clear"]
pub type CS2I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS3I` writer - CC43 IDLE mode clear"]
pub type CS3I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPRB` writer - Prescaler Run Bit Set"]
pub type SPRB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CC40 IDLE mode clear"]
    #[inline(always)]
    pub fn cs0i(&mut self) -> CS0I_W<GIDLC_SPEC> {
        CS0I_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC41 IDLE mode clear"]
    #[inline(always)]
    pub fn cs1i(&mut self) -> CS1I_W<GIDLC_SPEC> {
        CS1I_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC42 IDLE mode clear"]
    #[inline(always)]
    pub fn cs2i(&mut self) -> CS2I_W<GIDLC_SPEC> {
        CS2I_W::new(self, 2)
    }
    #[doc = "Bit 3 - CC43 IDLE mode clear"]
    #[inline(always)]
    pub fn cs3i(&mut self) -> CS3I_W<GIDLC_SPEC> {
        CS3I_W::new(self, 3)
    }
    #[doc = "Bit 8 - Prescaler Run Bit Set"]
    #[inline(always)]
    pub fn sprb(&mut self) -> SPRB_W<GIDLC_SPEC> {
        SPRB_W::new(self, 8)
    }
}
#[doc = "Global Idle Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gidlc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GIDLC_SPEC;
impl crate::RegisterSpec for GIDLC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gidlc::W`](W) writer structure"]
impl crate::Writable for GIDLC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GIDLC to value 0"]
impl crate::Resettable for GIDLC_SPEC {}
