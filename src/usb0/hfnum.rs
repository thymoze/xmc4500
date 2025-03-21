#[doc = "Register `HFNUM` reader"]
pub type R = crate::R<HFNUM_SPEC>;
#[doc = "Register `HFNUM` writer"]
pub type W = crate::W<HFNUM_SPEC>;
#[doc = "Field `FrNum` reader - Frame Number"]
pub type FR_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `FrNum` writer - Frame Number"]
pub type FR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FrRem` reader - Frame Time Remaining"]
pub type FR_REM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn fr_num(&self) -> FR_NUM_R {
        FR_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline(always)]
    pub fn fr_rem(&self) -> FR_REM_R {
        FR_REM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn fr_num(&mut self) -> FR_NUM_W<HFNUM_SPEC> {
        FR_NUM_W::new(self, 0)
    }
}
#[doc = "Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFNUM_SPEC;
impl crate::RegisterSpec for HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfnum::R`](R) reader structure"]
impl crate::Readable for HFNUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfnum::W`](W) writer structure"]
impl crate::Writable for HFNUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HFNUM_SPEC {
    const RESET_VALUE: u32 = 0x3fff;
}
