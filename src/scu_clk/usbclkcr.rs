#[doc = "Register `USBCLKCR` reader"]
pub type R = crate::R<USBCLKCR_SPEC>;
#[doc = "Register `USBCLKCR` writer"]
pub type W = crate::W<USBCLKCR_SPEC>;
#[doc = "Field `USBDIV` reader - USB Clock Divider Value"]
pub type USBDIV_R = crate::FieldReader;
#[doc = "Field `USBDIV` writer - USB Clock Divider Value"]
pub type USBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "USB Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSEL_A {
    #[doc = "0: USB PLL Clock"]
    VALUE1 = 0,
    #[doc = "1: PLL Clock"]
    VALUE2 = 1,
}
impl From<USBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: USBSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSEL` reader - USB Clock Selection Value"]
pub type USBSEL_R = crate::BitReader<USBSEL_A>;
impl USBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBSEL_A {
        match self.bits {
            false => USBSEL_A::VALUE1,
            true => USBSEL_A::VALUE2,
        }
    }
    #[doc = "USB PLL Clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBSEL_A::VALUE1
    }
    #[doc = "PLL Clock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBSEL_A::VALUE2
    }
}
#[doc = "Field `USBSEL` writer - USB Clock Selection Value"]
pub type USBSEL_W<'a, REG> = crate::BitWriter<'a, REG, USBSEL_A>;
impl<'a, REG> USBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB PLL Clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL_A::VALUE1)
    }
    #[doc = "PLL Clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:2 - USB Clock Divider Value"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - USB Clock Selection Value"]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Clock Divider Value"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W<USBCLKCR_SPEC> {
        USBDIV_W::new(self, 0)
    }
    #[doc = "Bit 16 - USB Clock Selection Value"]
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W<USBCLKCR_SPEC> {
        USBSEL_W::new(self, 16)
    }
}
#[doc = "USB Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCLKCR_SPEC;
impl crate::RegisterSpec for USBCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkcr::R`](R) reader structure"]
impl crate::Readable for USBCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbclkcr::W`](W) writer structure"]
impl crate::Writable for USBCLKCR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCLKCR to value 0"]
impl crate::Resettable for USBCLKCR_SPEC {}
