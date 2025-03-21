#[doc = "Register `MMFAR` reader"]
pub type R = crate::R<MMFAR_SPEC>;
#[doc = "Register `MMFAR` writer"]
pub type W = crate::W<MMFAR_SPEC>;
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
    pub fn address(&mut self) -> ADDRESS_W<MMFAR_SPEC> {
        ADDRESS_W::new(self, 0)
    }
}
#[doc = "MemManage Fault Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmfar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmfar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMFAR_SPEC;
impl crate::RegisterSpec for MMFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmfar::R`](R) reader structure"]
impl crate::Readable for MMFAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmfar::W`](W) writer structure"]
impl crate::Writable for MMFAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMFAR to value 0"]
impl crate::Resettable for MMFAR_SPEC {}
