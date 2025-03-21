#[doc = "Register `SYSTEM_TIME_NANOSECONDS_UPDATE` reader"]
pub type R = crate::R<SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC>;
#[doc = "Register `SYSTEM_TIME_NANOSECONDS_UPDATE` writer"]
pub type W = crate::W<SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC>;
#[doc = "Field `TSSS` reader - Timestamp Sub Second"]
pub type TSSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - Timestamp Sub Second"]
pub type TSSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - Add or subtract time"]
pub type ADDSUB_R = crate::BitReader;
#[doc = "Field `ADDSUB` writer - Add or subtract time"]
pub type ADDSUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Second"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub Second"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TSSS_W<SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC> {
        TSSS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W<SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC> {
        ADDSUB_W::new(self, 31)
    }
}
#[doc = "System Time Nanoseconds Update Register\n\nYou can [`read`](crate::Reg::read) this register and get [`system_time_nanoseconds_update::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_time_nanoseconds_update::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC;
impl crate::RegisterSpec for SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_nanoseconds_update::R`](R) reader structure"]
impl crate::Readable for SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`system_time_nanoseconds_update::W`](W) writer structure"]
impl crate::Writable for SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTEM_TIME_NANOSECONDS_UPDATE to value 0"]
impl crate::Resettable for SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC {}
