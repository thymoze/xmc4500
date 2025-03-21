#[doc = "Register `PSL` reader"]
pub type R = crate::R<PSL_SPEC>;
#[doc = "Register `PSL` writer"]
pub type W = crate::W<PSL_SPEC>;
#[doc = "Output Passive Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL_A> for bool {
    #[inline(always)]
    fn from(variant: PSL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL` reader - Output Passive Level"]
pub type PSL_R = crate::BitReader<PSL_A>;
impl PSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSL_A {
        match self.bits {
            false => PSL_A::VALUE1,
            true => PSL_A::VALUE2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL_A::VALUE1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL_A::VALUE2
    }
}
#[doc = "Field `PSL` writer - Output Passive Level"]
pub type PSL_W<'a, REG> = crate::BitWriter<'a, REG, PSL_A>;
impl<'a, REG> PSL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PSL_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PSL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    pub fn psl(&self) -> PSL_R {
        PSL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    pub fn psl(&mut self) -> PSL_W<PSL_SPEC> {
        PSL_W::new(self, 0)
    }
}
#[doc = "Passive Level Config\n\nYou can [`read`](crate::Reg::read) this register and get [`psl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSL_SPEC;
impl crate::RegisterSpec for PSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psl::R`](R) reader structure"]
impl crate::Readable for PSL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psl::W`](W) writer structure"]
impl crate::Writable for PSL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSL to value 0"]
impl crate::Resettable for PSL_SPEC {}
