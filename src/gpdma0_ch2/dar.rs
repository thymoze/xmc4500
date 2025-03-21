#[doc = "Register `DAR` reader"]
pub type R = crate::R<DAR_SPEC>;
#[doc = "Register `DAR` writer"]
pub type W = crate::W<DAR_SPEC>;
#[doc = "Field `DAR` reader - Current Destination address of DMA transfer"]
pub type DAR_R = crate::FieldReader<u32>;
#[doc = "Field `DAR` writer - Current Destination address of DMA transfer"]
pub type DAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Destination address of DMA transfer"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Destination address of DMA transfer"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W<DAR_SPEC> {
        DAR_W::new(self, 0)
    }
}
#[doc = "Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAR_SPEC;
impl crate::RegisterSpec for DAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar::R`](R) reader structure"]
impl crate::Readable for DAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dar::W`](W) writer structure"]
impl crate::Writable for DAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR to value 0"]
impl crate::Resettable for DAR_SPEC {}
