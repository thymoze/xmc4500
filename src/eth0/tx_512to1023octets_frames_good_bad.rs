#[doc = "Register `TX_512TO1023OCTETS_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<TX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `TX512_1023OCTGB` reader - This field indicates the number of transmitted good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
pub type TX512_1023OCTGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx512_1023octgb(&self) -> TX512_1023OCTGB_R {
        TX512_1023OCTGB_R::new(self.bits)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_512to1023octets_frames_good_bad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_512to1023octets_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for TX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets TX_512TO1023OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC {}
