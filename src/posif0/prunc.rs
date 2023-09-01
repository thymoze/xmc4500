#[doc = "Register `PRUNC` writer"]
pub type W = crate::W<PRUNC_SPEC>;
#[doc = "Field `CRB` writer - Clear Run bit"]
pub type CRB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSM` writer - Clear Current internal status"]
pub type CSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear Run bit"]
    #[inline(always)]
    #[must_use]
    pub fn crb(&mut self) -> CRB_W<PRUNC_SPEC, 0> {
        CRB_W::new(self)
    }
    #[doc = "Bit 1 - Clear Current internal status"]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CSM_W<PRUNC_SPEC, 1> {
        CSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "POSIF Run Bit Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prunc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRUNC_SPEC;
impl crate::RegisterSpec for PRUNC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prunc::W`](W) writer structure"]
impl crate::Writable for PRUNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRUNC to value 0"]
impl crate::Resettable for PRUNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
