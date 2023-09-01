#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `FCM` reader - Force CRC Mismatch"]
pub type FCM_R = crate::BitReader;
#[doc = "Field `FCM` writer - Force CRC Mismatch"]
pub type FCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRM_CFG` reader - Force CFG Register Mismatch"]
pub type FRM_CFG_R = crate::BitReader;
#[doc = "Field `FRM_CFG` writer - Force CFG Register Mismatch"]
pub type FRM_CFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRM_CHECK` reader - Force Check Register Mismatch"]
pub type FRM_CHECK_R = crate::BitReader;
#[doc = "Field `FRM_CHECK` writer - Force Check Register Mismatch"]
pub type FRM_CHECK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Force CRC Mismatch"]
    #[inline(always)]
    pub fn fcm(&self) -> FCM_R {
        FCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CFG Register Mismatch"]
    #[inline(always)]
    pub fn frm_cfg(&self) -> FRM_CFG_R {
        FRM_CFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Check Register Mismatch"]
    #[inline(always)]
    pub fn frm_check(&self) -> FRM_CHECK_R {
        FRM_CHECK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force CRC Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn fcm(&mut self) -> FCM_W<CTR_SPEC, 0> {
        FCM_W::new(self)
    }
    #[doc = "Bit 1 - Force CFG Register Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn frm_cfg(&mut self) -> FRM_CFG_W<CTR_SPEC, 1> {
        FRM_CFG_W::new(self)
    }
    #[doc = "Bit 2 - Force Check Register Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn frm_check(&mut self) -> FRM_CHECK_W<CTR_SPEC, 2> {
        FRM_CHECK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRC Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
