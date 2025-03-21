#[doc = "Register `VTOR` reader"]
pub type R = crate::R<VTOR_SPEC>;
#[doc = "Register `VTOR` writer"]
pub type W = crate::W<VTOR_SPEC>;
#[doc = "Field `TBLOFF` reader - Vector table base offset field"]
pub type TBLOFF_R = crate::FieldReader<u32>;
#[doc = "Field `TBLOFF` writer - Vector table base offset field"]
pub type TBLOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - Vector table base offset field"]
    #[inline(always)]
    pub fn tbloff(&self) -> TBLOFF_R {
        TBLOFF_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - Vector table base offset field"]
    #[inline(always)]
    pub fn tbloff(&mut self) -> TBLOFF_W<VTOR_SPEC> {
        TBLOFF_W::new(self, 10)
    }
}
#[doc = "Vector Table Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTOR_SPEC;
impl crate::RegisterSpec for VTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtor::R`](R) reader structure"]
impl crate::Readable for VTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtor::W`](W) writer structure"]
impl crate::Writable for VTOR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VTOR to value 0"]
impl crate::Resettable for VTOR_SPEC {}
