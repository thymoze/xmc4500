#[doc = "Register `TCSET` writer"]
pub type W = crate::W<TCSET_SPEC>;
#[doc = "Field `TRBS` writer - Timer Run Bit set"]
pub type TRBS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Timer Run Bit set"]
    #[inline(always)]
    pub fn trbs(&mut self) -> TRBS_W<TCSET_SPEC> {
        TRBS_W::new(self, 0)
    }
}
#[doc = "Slice Timer Run Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCSET_SPEC;
impl crate::RegisterSpec for TCSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcset::W`](W) writer structure"]
impl crate::Writable for TCSET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCSET to value 0"]
impl crate::Resettable for TCSET_SPEC {}
