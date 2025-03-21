#[doc = "Register `DIEPTSIZ0` reader"]
pub type R = crate::R<DIEPTSIZ0_SPEC>;
#[doc = "Register `DIEPTSIZ0` writer"]
pub type W = crate::W<DIEPTSIZ0_SPEC>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFER_SIZE_R {
        XFER_SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKT_CNT_R {
        PKT_CNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<DIEPTSIZ0_SPEC> {
        XFER_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<DIEPTSIZ0_SPEC> {
        PKT_CNT_W::new(self, 19)
    }
}
#[doc = "Device IN Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ0_SPEC;
impl crate::RegisterSpec for DIEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz0::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz0::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTSIZ0 to value 0"]
impl crate::Resettable for DIEPTSIZ0_SPEC {}
