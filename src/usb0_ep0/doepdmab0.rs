#[doc = "Register `DOEPDMAB0` reader"]
pub type R = crate::R<DOEPDMAB0_SPEC>;
#[doc = "Field `DMABufferAddr` reader - DMA Buffer Address"]
pub type DMABUFFER_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline(always)]
    pub fn dmabuffer_addr(&self) -> DMABUFFER_ADDR_R {
        DMABUFFER_ADDR_R::new(self.bits)
    }
}
#[doc = "Device Endpoint-0 DMA Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB0_SPEC;
impl crate::RegisterSpec for DOEPDMAB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab0::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB0_SPEC {}
#[doc = "`reset()` method sets DOEPDMAB0 to value 0"]
impl crate::Resettable for DOEPDMAB0_SPEC {}
