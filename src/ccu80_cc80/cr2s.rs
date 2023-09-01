#[doc = "Register `CR2S` reader"]
pub type R = crate::R<CR2S_SPEC>;
#[doc = "Register `CR2S` writer"]
pub type W = crate::W<CR2S_SPEC>;
#[doc = "Field `CR2S` reader - Shadow Compare Register for Channel 2"]
pub type CR2S_R = crate::FieldReader<u16>;
#[doc = "Field `CR2S` writer - Shadow Compare Register for Channel 2"]
pub type CR2S_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 2"]
    #[inline(always)]
    pub fn cr2s(&self) -> CR2S_R {
        CR2S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr2s(&mut self) -> CR2S_W<CR2S_SPEC, 0> {
        CR2S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel 2 Compare Shadow Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2S_SPEC;
impl crate::RegisterSpec for CR2S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2s::R`](R) reader structure"]
impl crate::Readable for CR2S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2s::W`](W) writer structure"]
impl crate::Writable for CR2S_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2S to value 0"]
impl crate::Resettable for CR2S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
