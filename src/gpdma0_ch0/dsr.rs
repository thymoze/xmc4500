#[doc = "Register `DSR` reader"]
pub type R = crate::R<DSR_SPEC>;
#[doc = "Register `DSR` writer"]
pub type W = crate::W<DSR_SPEC>;
#[doc = "Field `DSI` reader - Destination scatter interval"]
pub type DSI_R = crate::FieldReader<u32>;
#[doc = "Field `DSI` writer - Destination scatter interval"]
pub type DSI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DSC` reader - Destination scatter count"]
pub type DSC_R = crate::FieldReader<u16>;
#[doc = "Field `DSC` writer - Destination scatter count"]
pub type DSC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    pub fn dsi(&self) -> DSI_R {
        DSI_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    pub fn dsc(&self) -> DSC_R {
        DSC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    pub fn dsi(&mut self) -> DSI_W<DSR_SPEC> {
        DSI_W::new(self, 0)
    }
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    pub fn dsc(&mut self) -> DSC_W<DSR_SPEC> {
        DSC_W::new(self, 20)
    }
}
#[doc = "Destination Scatter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSR_SPEC;
impl crate::RegisterSpec for DSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr::R`](R) reader structure"]
impl crate::Readable for DSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsr::W`](W) writer structure"]
impl crate::Writable for DSR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSR to value 0"]
impl crate::Resettable for DSR_SPEC {}
