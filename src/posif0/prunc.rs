#[doc = "Register `PRUNC` writer"]
pub type W = crate::W<PRUNC_SPEC>;
#[doc = "Field `CRB` writer - Clear Run bit"]
pub type CRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSM` writer - Clear Current internal status"]
pub type CSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Run bit"]
    #[inline(always)]
    pub fn crb(&mut self) -> CRB_W<PRUNC_SPEC> {
        CRB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Current internal status"]
    #[inline(always)]
    pub fn csm(&mut self) -> CSM_W<PRUNC_SPEC> {
        CSM_W::new(self, 1)
    }
}
#[doc = "POSIF Run Bit Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prunc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRUNC_SPEC;
impl crate::RegisterSpec for PRUNC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prunc::W`](W) writer structure"]
impl crate::Writable for PRUNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRUNC to value 0"]
impl crate::Resettable for PRUNC_SPEC {}
