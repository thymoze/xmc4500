#[doc = "Register `BFAR` reader"]
pub type R = crate::R<BFAR_SPEC>;
#[doc = "Register `BFAR` writer"]
pub type W = crate::W<BFAR_SPEC>;
#[doc = "Field `ADDRESS` reader - Address causing the fault"]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - Address causing the fault"]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address causing the fault"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address causing the fault"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W<BFAR_SPEC> {
        ADDRESS_W::new(self, 0)
    }
}
#[doc = "BusFault Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bfar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFAR_SPEC;
impl crate::RegisterSpec for BFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfar::R`](R) reader structure"]
impl crate::Readable for BFAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bfar::W`](W) writer structure"]
impl crate::Writable for BFAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BFAR to value 0"]
impl crate::Resettable for BFAR_SPEC {}
