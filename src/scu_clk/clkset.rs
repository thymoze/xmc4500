#[doc = "Register `CLKSET` writer"]
pub type W = crate::W<CLKSET_SPEC>;
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCEN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<USBCEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCEN` writer - USB Clock Enable"]
pub type USBCEN_W<'a, REG> = crate::BitWriter<'a, REG, USBCEN_A>;
impl<'a, REG> USBCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCEN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBCEN_A::VALUE2)
    }
}
#[doc = "MMC Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCEN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<MMCCEN_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCEN` writer - MMC Clock Enable"]
pub type MMCCEN_W<'a, REG> = crate::BitWriter<'a, REG, MMCCEN_A>;
impl<'a, REG> MMCCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCEN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCEN_A::VALUE2)
    }
}
#[doc = "Ethernet Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CEN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<ETH0CEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CEN` writer - Ethernet Clock Enable"]
pub type ETH0CEN_W<'a, REG> = crate::BitWriter<'a, REG, ETH0CEN_A>;
impl<'a, REG> ETH0CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CEN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CEN_A::VALUE2)
    }
}
#[doc = "EBU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBUCEN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<EBUCEN_A> for bool {
    #[inline(always)]
    fn from(variant: EBUCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBUCEN` writer - EBU Clock Enable"]
pub type EBUCEN_W<'a, REG> = crate::BitWriter<'a, REG, EBUCEN_A>;
impl<'a, REG> EBUCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EBUCEN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EBUCEN_A::VALUE2)
    }
}
#[doc = "CCU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCEN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CCUCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCEN` writer - CCU Clock Enable"]
pub type CCUCEN_W<'a, REG> = crate::BitWriter<'a, REG, CCUCEN_A>;
impl<'a, REG> CCUCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCEN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCEN_A::VALUE2)
    }
}
#[doc = "WDT Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCEN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WDTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCEN` writer - WDT Clock Enable"]
pub type WDTCEN_W<'a, REG> = crate::BitWriter<'a, REG, WDTCEN_A>;
impl<'a, REG> WDTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCEN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCEN_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline(always)]
    pub fn usbcen(&mut self) -> USBCEN_W<CLKSET_SPEC> {
        USBCEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Clock Enable"]
    #[inline(always)]
    pub fn mmccen(&mut self) -> MMCCEN_W<CLKSET_SPEC> {
        MMCCEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Ethernet Clock Enable"]
    #[inline(always)]
    pub fn eth0cen(&mut self) -> ETH0CEN_W<CLKSET_SPEC> {
        ETH0CEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - EBU Clock Enable"]
    #[inline(always)]
    pub fn ebucen(&mut self) -> EBUCEN_W<CLKSET_SPEC> {
        EBUCEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline(always)]
    pub fn ccucen(&mut self) -> CCUCEN_W<CLKSET_SPEC> {
        CCUCEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline(always)]
    pub fn wdtcen(&mut self) -> WDTCEN_W<CLKSET_SPEC> {
        WDTCEN_W::new(self, 5)
    }
}
#[doc = "CLK Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSET_SPEC;
impl crate::RegisterSpec for CLKSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clkset::W`](W) writer structure"]
impl crate::Writable for CLKSET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKSET to value 0"]
impl crate::Resettable for CLKSET_SPEC {}
