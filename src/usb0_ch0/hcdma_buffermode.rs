#[doc = "Register `HCDMA_BUFFERMODE` reader"]
pub struct R(crate::R<HCDMA_BUFFERMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA_BUFFERMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA_BUFFERMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA_BUFFERMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA_BUFFERMODE` writer"]
pub struct W(crate::W<HCDMA_BUFFERMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA_BUFFERMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HCDMA_BUFFERMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA_BUFFERMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub type DMAADDR_W<'a, const O: u8> = crate::FieldWriter<'a, HCDMA_BUFFERMODE_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<0> {
        DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma_buffermode](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct HCDMA_BUFFERMODE_SPEC;
impl crate::RegisterSpec for HCDMA_BUFFERMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma_buffermode::R](R) reader structure"]
impl crate::Readable for HCDMA_BUFFERMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma_buffermode::W](W) writer structure"]
impl crate::Writable for HCDMA_BUFFERMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDMA_BUFFERMODE to value 0"]
impl crate::Resettable for HCDMA_BUFFERMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
