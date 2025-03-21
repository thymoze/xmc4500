#[doc = "Register `PWRCLR` writer"]
pub type W = crate::W<PWRCLR_SPEC>;
#[doc = "Clear Disable Hibernate Domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIB_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable Hibernate domain"]
    VALUE2 = 1,
}
impl From<HIB_A> for bool {
    #[inline(always)]
    fn from(variant: HIB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` writer - Clear Disable Hibernate Domain"]
pub type HIB_W<'a, REG> = crate::BitWriter<'a, REG, HIB_A>;
impl<'a, REG> HIB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_A::VALUE1)
    }
    #[doc = "Disable Hibernate domain"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIB_A::VALUE2)
    }
}
#[doc = "Clear USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPHYPDQ_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Power-down"]
    VALUE2 = 1,
}
impl From<USBPHYPDQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPHYPDQ` writer - Clear USB PHY Transceiver Disable"]
pub type USBPHYPDQ_W<'a, REG> = crate::BitWriter<'a, REG, USBPHYPDQ_A>;
impl<'a, REG> USBPHYPDQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYPDQ_A::VALUE1)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBPHYPDQ_A::VALUE2)
    }
}
#[doc = "Clear USB On-The-Go Comparators Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTGEN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Power-down"]
    VALUE2 = 1,
}
impl From<USBOTGEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTGEN` writer - Clear USB On-The-Go Comparators Enable"]
pub type USBOTGEN_W<'a, REG> = crate::BitWriter<'a, REG, USBOTGEN_A>;
impl<'a, REG> USBOTGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTGEN_A::VALUE1)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBOTGEN_A::VALUE2)
    }
}
#[doc = "Clear USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPUWQ_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up active"]
    VALUE2 = 1,
}
impl From<USBPUWQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPUWQ` writer - Clear USB Weak Pull-Up at PADN Enable"]
pub type USBPUWQ_W<'a, REG> = crate::BitWriter<'a, REG, USBPUWQ_A>;
impl<'a, REG> USBPUWQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBPUWQ_A::VALUE1)
    }
    #[doc = "Pull-up active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBPUWQ_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Disable Hibernate Domain"]
    #[inline(always)]
    pub fn hib(&mut self) -> HIB_W<PWRCLR_SPEC> {
        HIB_W::new(self, 0)
    }
    #[doc = "Bit 16 - Clear USB PHY Transceiver Disable"]
    #[inline(always)]
    pub fn usbphypdq(&mut self) -> USBPHYPDQ_W<PWRCLR_SPEC> {
        USBPHYPDQ_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear USB On-The-Go Comparators Enable"]
    #[inline(always)]
    pub fn usbotgen(&mut self) -> USBOTGEN_W<PWRCLR_SPEC> {
        USBOTGEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    pub fn usbpuwq(&mut self) -> USBPUWQ_W<PWRCLR_SPEC> {
        USBPUWQ_W::new(self, 18)
    }
}
#[doc = "PCU Clear Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCLR_SPEC;
impl crate::RegisterSpec for PWRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pwrclr::W`](W) writer structure"]
impl crate::Writable for PWRCLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCLR to value 0"]
impl crate::Resettable for PWRCLR_SPEC {}
