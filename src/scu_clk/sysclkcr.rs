#[doc = "Register `SYSCLKCR` reader"]
pub type R = crate::R<SYSCLKCR_SPEC>;
#[doc = "Register `SYSCLKCR` writer"]
pub type W = crate::W<SYSCLKCR_SPEC>;
#[doc = "Field `SYSDIV` reader - System Clock Division Value"]
pub type SYSDIV_R = crate::FieldReader;
#[doc = "Field `SYSDIV` writer - System Clock Division Value"]
pub type SYSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    VALUE1 = 0,
    #[doc = "1: fPLL clock"]
    VALUE2 = 1,
}
impl From<SYSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SYSSEL_R = crate::BitReader<SYSSEL_A>;
impl SYSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSSEL_A {
        match self.bits {
            false => SYSSEL_A::VALUE1,
            true => SYSSEL_A::VALUE2,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYSSEL_A::VALUE1
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYSSEL_A::VALUE2
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SYSSEL_W<'a, REG> = crate::BitWriter<'a, REG, SYSSEL_A>;
impl<'a, REG> SYSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::VALUE1)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SYSDIV_R {
        SYSDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&mut self) -> SYSDIV_W<SYSCLKCR_SPEC> {
        SYSDIV_W::new(self, 0)
    }
    #[doc = "Bit 16 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W<SYSCLKCR_SPEC> {
        SYSSEL_W::new(self, 16)
    }
}
#[doc = "System Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCLKCR_SPEC;
impl crate::RegisterSpec for SYSCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclkcr::R`](R) reader structure"]
impl crate::Readable for SYSCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysclkcr::W`](W) writer structure"]
impl crate::Writable for SYSCLKCR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCLKCR to value 0"]
impl crate::Resettable for SYSCLKCR_SPEC {}
