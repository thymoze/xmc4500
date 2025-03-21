#[doc = "Register `HCDMA_BUFFERMODE` reader"]
pub type R = crate::R<HCDMA_BUFFERMODE_SPEC>;
#[doc = "Register `HCDMA_BUFFERMODE` writer"]
pub type W = crate::W<HCDMA_BUFFERMODE_SPEC>;
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA_BUFFERMODE_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma_buffermode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma_buffermode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct HCDMA_BUFFERMODE_SPEC;
impl crate::RegisterSpec for HCDMA_BUFFERMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma_buffermode::R`](R) reader structure"]
impl crate::Readable for HCDMA_BUFFERMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma_buffermode::W`](W) writer structure"]
impl crate::Writable for HCDMA_BUFFERMODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA_BUFFERMODE to value 0"]
impl crate::Resettable for HCDMA_BUFFERMODE_SPEC {}
