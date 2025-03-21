#[doc = "Register `CRC` reader"]
pub type R = crate::R<CRC_SPEC>;
#[doc = "Register `CRC` writer"]
pub type W = crate::W<CRC_SPEC>;
#[doc = "Field `CRC` reader - CRC Register"]
pub type CRC_R = crate::FieldReader<u32>;
#[doc = "Field `CRC` writer - CRC Register"]
pub type CRC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Register"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Register"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<CRC_SPEC> {
        CRC_W::new(self, 0)
    }
}
#[doc = "CRC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_SPEC;
impl crate::RegisterSpec for CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc::R`](R) reader structure"]
impl crate::Readable for CRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crc::W`](W) writer structure"]
impl crate::Writable for CRC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC to value 0"]
impl crate::Resettable for CRC_SPEC {}
