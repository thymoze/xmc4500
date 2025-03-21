#[doc = "Register `NVIC_ISPR2` reader"]
pub type R = crate::R<NVIC_ISPR2_SPEC>;
#[doc = "Register `NVIC_ISPR2` writer"]
pub type W = crate::W<NVIC_ISPR2_SPEC>;
#[doc = "Interrupt set-pending bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SETPEND_A {
    #[doc = "0: interrupt is not pending"]
    VALUE3 = 0,
    #[doc = "1: interrupt is pending."]
    VALUE4 = 1,
}
impl From<SETPEND_A> for u32 {
    #[inline(always)]
    fn from(variant: SETPEND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SETPEND_A {
    type Ux = u32;
}
impl crate::IsEnum for SETPEND_A {}
#[doc = "Field `SETPEND` reader - Interrupt set-pending bits."]
pub type SETPEND_R = crate::FieldReader<SETPEND_A>;
impl SETPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SETPEND_A> {
        match self.bits {
            0 => Some(SETPEND_A::VALUE3),
            1 => Some(SETPEND_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SETPEND_A::VALUE3
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SETPEND_A::VALUE4
    }
}
#[doc = "Field `SETPEND` writer - Interrupt set-pending bits."]
pub type SETPEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, SETPEND_A>;
impl<'a, REG> SETPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SETPEND_A::VALUE3)
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SETPEND_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn setpend(&mut self) -> SETPEND_W<NVIC_ISPR2_SPEC> {
        SETPEND_W::new(self, 0)
    }
}
#[doc = "Interrupt Set-pending Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`nvic_ispr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ispr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ISPR2_SPEC;
impl crate::RegisterSpec for NVIC_ISPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ispr2::R`](R) reader structure"]
impl crate::Readable for NVIC_ISPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ispr2::W`](W) writer structure"]
impl crate::Writable for NVIC_ISPR2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NVIC_ISPR2 to value 0"]
impl crate::Resettable for NVIC_ISPR2_SPEC {}
