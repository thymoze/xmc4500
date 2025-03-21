#[doc = "Register `SPI` reader"]
pub type R = crate::R<SPI_SPEC>;
#[doc = "Register `SPI` writer"]
pub type W = crate::W<SPI_SPEC>;
#[doc = "Field `SPI_INT_SUPPORT` reader - SPI INT SUPPORT"]
pub type SPI_INT_SUPPORT_R = crate::FieldReader;
#[doc = "Field `SPI_INT_SUPPORT` writer - SPI INT SUPPORT"]
pub type SPI_INT_SUPPORT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI INT SUPPORT"]
    #[inline(always)]
    pub fn spi_int_support(&self) -> SPI_INT_SUPPORT_R {
        SPI_INT_SUPPORT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI INT SUPPORT"]
    #[inline(always)]
    pub fn spi_int_support(&mut self) -> SPI_INT_SUPPORT_W<SPI_SPEC> {
        SPI_INT_SUPPORT_W::new(self, 0)
    }
}
#[doc = "SPI Interrupt Support Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SPEC;
impl crate::RegisterSpec for SPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi::R`](R) reader structure"]
impl crate::Readable for SPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi::W`](W) writer structure"]
impl crate::Writable for SPI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI to value 0"]
impl crate::Resettable for SPI_SPEC {}
