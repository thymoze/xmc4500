#[doc = "Register `TSVAL` reader"]
pub type R = crate::R<TSVAL_SPEC>;
#[doc = "Register `TSVAL` writer"]
pub type W = crate::W<TSVAL_SPEC>;
#[doc = "Field `TSCTRVALR` reader - Shadow TS-Counter (Read)"]
pub type TSCTRVALR_R = crate::FieldReader<u16>;
#[doc = "Field `TSCTRVAL` reader - TS-Counter Value"]
pub type TSCTRVAL_R = crate::FieldReader<u16>;
#[doc = "Field `TSCTRVAL` writer - TS-Counter Value"]
pub type TSCTRVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow TS-Counter (Read)"]
    #[inline(always)]
    pub fn tsctrvalr(&self) -> TSCTRVALR_R {
        TSCTRVALR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    pub fn tsctrval(&self) -> TSCTRVAL_R {
        TSCTRVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    pub fn tsctrval(&mut self) -> TSCTRVAL_W<TSVAL_SPEC> {
        TSCTRVAL_W::new(self, 16)
    }
}
#[doc = "Touch-sense TS-Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tsval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSVAL_SPEC;
impl crate::RegisterSpec for TSVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsval::R`](R) reader structure"]
impl crate::Readable for TSVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsval::W`](W) writer structure"]
impl crate::Writable for TSVAL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSVAL to value 0"]
impl crate::Resettable for TSVAL_SPEC {}
