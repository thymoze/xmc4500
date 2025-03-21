#[doc = "Register `TX_BROADCAST_FRAMES_GOOD` reader"]
pub type R = crate::R<TX_BROADCAST_FRAMES_GOOD_SPEC>;
#[doc = "Field `TXBCASTG` reader - This field indicates the number of transmitted good broadcast frames."]
pub type TXBCASTG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good broadcast frames."]
    #[inline(always)]
    pub fn txbcastg(&self) -> TXBCASTG_R {
        TXBCASTG_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good Broadcast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_broadcast_frames_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_BROADCAST_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for TX_BROADCAST_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_broadcast_frames_good::R`](R) reader structure"]
impl crate::Readable for TX_BROADCAST_FRAMES_GOOD_SPEC {}
#[doc = "`reset()` method sets TX_BROADCAST_FRAMES_GOOD to value 0"]
impl crate::Resettable for TX_BROADCAST_FRAMES_GOOD_SPEC {}
