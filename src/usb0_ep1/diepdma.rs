#[doc = "Register `DIEPDMA` reader"]
pub type R = crate::R<DIEPDMA_SPEC>;
#[doc = "Register `DIEPDMA` writer"]
pub type W = crate::W<DIEPDMA_SPEC>;
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
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DIEPDMA_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Device Endpoint DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct DIEPDMA_SPEC;
impl crate::RegisterSpec for DIEPDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma::R`](R) reader structure"]
impl crate::Readable for DIEPDMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepdma::W`](W) writer structure"]
impl crate::Writable for DIEPDMA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPDMA to value 0"]
impl crate::Resettable for DIEPDMA_SPEC {}
