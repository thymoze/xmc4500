#[doc = "Register `RX_OVERSIZE_FRAMES_GOOD` reader"]
pub type R = crate::R<RX_OVERSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Field `RXOVERSZG` reader - This field indicates the number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled by setting MAC Configuration.2KPE)."]
pub type RXOVERSZG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled by setting MAC Configuration.2KPE)."]
    #[inline(always)]
    pub fn rxoverszg(&self) -> RXOVERSZG_R {
        RXOVERSZG_R::new(self.bits)
    }
}
#[doc = "Rx Oversize Frames Good Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_oversize_frames_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_OVERSIZE_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for RX_OVERSIZE_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_oversize_frames_good::R`](R) reader structure"]
impl crate::Readable for RX_OVERSIZE_FRAMES_GOOD_SPEC {}
#[doc = "`reset()` method sets RX_OVERSIZE_FRAMES_GOOD to value 0"]
impl crate::Resettable for RX_OVERSIZE_FRAMES_GOOD_SPEC {}
