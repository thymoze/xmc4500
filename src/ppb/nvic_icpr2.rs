#[doc = "Register `NVIC_ICPR2` reader"]
pub type R = crate::R<NVIC_ICPR2_SPEC>;
#[doc = "Register `NVIC_ICPR2` writer"]
pub type W = crate::W<NVIC_ICPR2_SPEC>;
#[doc = "Interrupt set-pending bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CLRPEND_A {
    #[doc = "0: interrupt is not pending"]
    VALUE3 = 0,
    #[doc = "1: interrupt is pending."]
    VALUE4 = 1,
}
impl From<CLRPEND_A> for u32 {
    #[inline(always)]
    fn from(variant: CLRPEND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLRPEND_A {
    type Ux = u32;
}
impl crate::IsEnum for CLRPEND_A {}
#[doc = "Field `CLRPEND` reader - Interrupt set-pending bits."]
pub type CLRPEND_R = crate::FieldReader<CLRPEND_A>;
impl CLRPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLRPEND_A> {
        match self.bits {
            0 => Some(CLRPEND_A::VALUE3),
            1 => Some(CLRPEND_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLRPEND_A::VALUE3
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLRPEND_A::VALUE4
    }
}
#[doc = "Field `CLRPEND` writer - Interrupt set-pending bits."]
pub type CLRPEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, CLRPEND_A>;
impl<'a, REG> CLRPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CLRPEND_A::VALUE3)
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CLRPEND_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn clrpend(&self) -> CLRPEND_R {
        CLRPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn clrpend(&mut self) -> CLRPEND_W<NVIC_ICPR2_SPEC> {
        CLRPEND_W::new(self, 0)
    }
}
#[doc = "Interrupt Clear-pending Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`nvic_icpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_icpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ICPR2_SPEC;
impl crate::RegisterSpec for NVIC_ICPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icpr2::R`](R) reader structure"]
impl crate::Readable for NVIC_ICPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_icpr2::W`](W) writer structure"]
impl crate::Writable for NVIC_ICPR2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NVIC_ICPR2 to value 0"]
impl crate::Resettable for NVIC_ICPR2_SPEC {}
