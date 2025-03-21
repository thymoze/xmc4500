#[doc = "Register `MSIMASK` reader"]
pub type R = crate::R<MSIMASK_SPEC>;
#[doc = "Register `MSIMASK` writer"]
pub type W = crate::W<MSIMASK_SPEC>;
#[doc = "Field `IM` reader - Message Index Mask"]
pub type IM_R = crate::FieldReader<u32>;
#[doc = "Field `IM` writer - Message Index Mask"]
pub type IM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Index Mask"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Index Mask"]
    #[inline(always)]
    pub fn im(&mut self) -> IM_W<MSIMASK_SPEC> {
        IM_W::new(self, 0)
    }
}
#[doc = "Message Index Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msimask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msimask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSIMASK_SPEC;
impl crate::RegisterSpec for MSIMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msimask::R`](R) reader structure"]
impl crate::Readable for MSIMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msimask::W`](W) writer structure"]
impl crate::Writable for MSIMASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSIMASK to value 0"]
impl crate::Resettable for MSIMASK_SPEC {}
