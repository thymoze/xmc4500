#[doc = "Register `NVIC_ICER1` reader"]
pub type R = crate::R<NVIC_ICER1_SPEC>;
#[doc = "Register `NVIC_ICER1` writer"]
pub type W = crate::W<NVIC_ICER1_SPEC>;
#[doc = "Interrupt clear-enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CLRENA_A {
    #[doc = "0: interrupt disabled"]
    VALUE3 = 0,
    #[doc = "1: interrupt enabled."]
    VALUE4 = 1,
}
impl From<CLRENA_A> for u32 {
    #[inline(always)]
    fn from(variant: CLRENA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLRENA_A {
    type Ux = u32;
}
impl crate::IsEnum for CLRENA_A {}
#[doc = "Field `CLRENA` reader - Interrupt clear-enable bits."]
pub type CLRENA_R = crate::FieldReader<CLRENA_A>;
impl CLRENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLRENA_A> {
        match self.bits {
            0 => Some(CLRENA_A::VALUE3),
            1 => Some(CLRENA_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLRENA_A::VALUE3
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLRENA_A::VALUE4
    }
}
#[doc = "Field `CLRENA` writer - Interrupt clear-enable bits."]
pub type CLRENA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, CLRENA_A>;
impl<'a, REG> CLRENA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CLRENA_A::VALUE3)
    }
    #[doc = "interrupt enabled."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CLRENA_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits."]
    #[inline(always)]
    pub fn clrena(&self) -> CLRENA_R {
        CLRENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits."]
    #[inline(always)]
    pub fn clrena(&mut self) -> CLRENA_W<NVIC_ICER1_SPEC> {
        CLRENA_W::new(self, 0)
    }
}
#[doc = "Interrupt Clear-enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`nvic_icer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_icer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ICER1_SPEC;
impl crate::RegisterSpec for NVIC_ICER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icer1::R`](R) reader structure"]
impl crate::Readable for NVIC_ICER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_icer1::W`](W) writer structure"]
impl crate::Writable for NVIC_ICER1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NVIC_ICER1 to value 0"]
impl crate::Resettable for NVIC_ICER1_SPEC {}
