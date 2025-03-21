#[doc = "Register `DTXFSTS` reader"]
pub type R = crate::R<DTXFSTS_SPEC>;
#[doc = "IN Endpoint TxFIFO Space Avail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum INEPTX_FSPC_AVAIL_A {
    #[doc = "0: Endpoint TxFIFO is full"]
    VALUE1 = 0,
    #[doc = "1: 1 word available"]
    VALUE2 = 1,
    #[doc = "2: 2 words available"]
    VALUE3 = 2,
}
impl From<INEPTX_FSPC_AVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: INEPTX_FSPC_AVAIL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INEPTX_FSPC_AVAIL_A {
    type Ux = u16;
}
impl crate::IsEnum for INEPTX_FSPC_AVAIL_A {}
#[doc = "Field `INEPTxFSpcAvail` reader - IN Endpoint TxFIFO Space Avail"]
pub type INEPTX_FSPC_AVAIL_R = crate::FieldReader<INEPTX_FSPC_AVAIL_A>;
impl INEPTX_FSPC_AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INEPTX_FSPC_AVAIL_A> {
        match self.bits {
            0 => Some(INEPTX_FSPC_AVAIL_A::VALUE1),
            1 => Some(INEPTX_FSPC_AVAIL_A::VALUE2),
            2 => Some(INEPTX_FSPC_AVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Endpoint TxFIFO is full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INEPTX_FSPC_AVAIL_A::VALUE1
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INEPTX_FSPC_AVAIL_A::VALUE2
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == INEPTX_FSPC_AVAIL_A::VALUE3
    }
}
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline(always)]
    pub fn ineptx_fspc_avail(&self) -> INEPTX_FSPC_AVAIL_R {
        INEPTX_FSPC_AVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS_SPEC;
impl crate::RegisterSpec for DTXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts::R`](R) reader structure"]
impl crate::Readable for DTXFSTS_SPEC {}
#[doc = "`reset()` method sets DTXFSTS to value 0"]
impl crate::Resettable for DTXFSTS_SPEC {}
