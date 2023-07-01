#[doc = "Register `RMACR` reader"]
pub struct R(crate::R<RMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMACR` writer"]
pub struct W(crate::W<RMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDWR` reader - Hibernate Retention Memory Register Update Control"]
pub type RDWR_R = crate::BitReader<RDWR_A>;
#[doc = "Hibernate Retention Memory Register Update Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDWR_A {
    #[doc = "0: transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    VALUE1 = 0,
    #[doc = "1: transfer data from RMDATA into Retention Memory in Hibernate domain"]
    VALUE2 = 1,
}
impl From<RDWR_A> for bool {
    #[inline(always)]
    fn from(variant: RDWR_A) -> Self {
        variant as u8 != 0
    }
}
impl RDWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDWR_A {
        match self.bits {
            false => RDWR_A::VALUE1,
            true => RDWR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDWR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDWR_A::VALUE2
    }
}
#[doc = "Field `RDWR` writer - Hibernate Retention Memory Register Update Control"]
pub type RDWR_W<'a, const O: u8> = crate::BitWriter<'a, RMACR_SPEC, O, RDWR_A>;
impl<'a, const O: u8> RDWR_W<'a, O> {
    #[doc = "transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RDWR_A::VALUE1)
    }
    #[doc = "transfer data from RMDATA into Retention Memory in Hibernate domain"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RDWR_A::VALUE2)
    }
}
#[doc = "Field `ADDR` reader - Hibernate Retention Memory Register Address Select"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Hibernate Retention Memory Register Address Select"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, RMACR_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    pub fn rdwr(&self) -> RDWR_R {
        RDWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    #[must_use]
    pub fn rdwr(&mut self) -> RDWR_W<0> {
        RDWR_W::new(self)
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<16> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention Memory Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmacr](index.html) module"]
pub struct RMACR_SPEC;
impl crate::RegisterSpec for RMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmacr::R](R) reader structure"]
impl crate::Readable for RMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmacr::W](W) writer structure"]
impl crate::Writable for RMACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMACR to value 0"]
impl crate::Resettable for RMACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
