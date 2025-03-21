#[doc = "Register `DSTAT` reader"]
pub type R = crate::R<DSTAT_SPEC>;
#[doc = "Register `DSTAT` writer"]
pub type W = crate::W<DSTAT_SPEC>;
#[doc = "Field `DSTAT` reader - Destination Status"]
pub type DSTAT_R = crate::FieldReader<u32>;
#[doc = "Field `DSTAT` writer - Destination Status"]
pub type DSTAT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Status"]
    #[inline(always)]
    pub fn dstat(&self) -> DSTAT_R {
        DSTAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Status"]
    #[inline(always)]
    pub fn dstat(&mut self) -> DSTAT_W<DSTAT_SPEC> {
        DSTAT_W::new(self, 0)
    }
}
#[doc = "Destination Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTAT_SPEC;
impl crate::RegisterSpec for DSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstat::R`](R) reader structure"]
impl crate::Readable for DSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dstat::W`](W) writer structure"]
impl crate::Writable for DSTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSTAT to value 0"]
impl crate::Resettable for DSTAT_SPEC {}
