#[doc = "Register `BOUND` reader"]
pub type R = crate::R<BOUND_SPEC>;
#[doc = "Register `BOUND` writer"]
pub type W = crate::W<BOUND_SPEC>;
#[doc = "Field `BOUNDARY0` reader - Boundary Value 0 for Limit Checking"]
pub type BOUNDARY0_R = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARY0` writer - Boundary Value 0 for Limit Checking"]
pub type BOUNDARY0_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BOUNDARY1` reader - Boundary Value 1 for Limit Checking"]
pub type BOUNDARY1_R = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARY1` writer - Boundary Value 1 for Limit Checking"]
pub type BOUNDARY1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Boundary Value 0 for Limit Checking"]
    #[inline(always)]
    pub fn boundary0(&self) -> BOUNDARY0_R {
        BOUNDARY0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Boundary Value 1 for Limit Checking"]
    #[inline(always)]
    pub fn boundary1(&self) -> BOUNDARY1_R {
        BOUNDARY1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Boundary Value 0 for Limit Checking"]
    #[inline(always)]
    pub fn boundary0(&mut self) -> BOUNDARY0_W<BOUND_SPEC> {
        BOUNDARY0_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Boundary Value 1 for Limit Checking"]
    #[inline(always)]
    pub fn boundary1(&mut self) -> BOUNDARY1_W<BOUND_SPEC> {
        BOUNDARY1_W::new(self, 16)
    }
}
#[doc = "Boundary Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bound::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bound::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOUND_SPEC;
impl crate::RegisterSpec for BOUND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bound::R`](R) reader structure"]
impl crate::Readable for BOUND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bound::W`](W) writer structure"]
impl crate::Writable for BOUND_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOUND to value 0"]
impl crate::Resettable for BOUND_SPEC {}
