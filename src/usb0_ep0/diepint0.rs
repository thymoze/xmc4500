#[doc = "Register `DIEPINT0` reader"]
pub type R = crate::R<DIEPINT0_SPEC>;
#[doc = "Register `DIEPINT0` writer"]
pub type W = crate::W<DIEPINT0_SPEC>;
#[doc = "Field `XferCompl` reader - Transfer Completed Interrupt"]
pub type XFER_COMPL_R = crate::BitReader;
#[doc = "Field `XferCompl` writer - Transfer Completed Interrupt"]
pub type XFER_COMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDisbld` reader - Endpoint Disabled Interrupt"]
pub type EPDISBLD_R = crate::BitReader;
#[doc = "Field `EPDisbld` writer - Endpoint Disabled Interrupt"]
pub type EPDISBLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErr` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBErr` writer - AHB Error"]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TimeOUT` reader - Timeout Condition"]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TimeOUT` writer - Timeout Condition"]
pub type TIME_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTknTXFEmp` reader - IN Token Received When TxFIFO is Empty"]
pub type INTKN_TXFEMP_R = crate::BitReader;
#[doc = "Field `INTknTXFEmp` writer - IN Token Received When TxFIFO is Empty"]
pub type INTKN_TXFEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNakEff` reader - IN Endpoint NAK Effective"]
pub type INEPNAK_EFF_R = crate::BitReader;
#[doc = "Field `INEPNakEff` writer - IN Endpoint NAK Effective"]
pub type INEPNAK_EFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFEmp` reader - Transmit FIFO Empty"]
pub type TX_FEMP_R = crate::BitReader;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_R = crate::BitReader;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfer_compl(&self) -> XFER_COMPL_R {
        XFER_COMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkn_txfemp(&self) -> INTKN_TXFEMP_R {
        INTKN_TXFEMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnak_eff(&self) -> INEPNAK_EFF_R {
        INEPNAK_EFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn tx_femp(&self) -> TX_FEMP_R {
        TX_FEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfer_compl(&mut self) -> XFER_COMPL_W<DIEPINT0_SPEC> {
        XFER_COMPL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<DIEPINT0_SPEC> {
        EPDISBLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<DIEPINT0_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn time_out(&mut self) -> TIME_OUT_W<DIEPINT0_SPEC> {
        TIME_OUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkn_txfemp(&mut self) -> INTKN_TXFEMP_W<DIEPINT0_SPEC> {
        INTKN_TXFEMP_W::new(self, 4)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnak_eff(&mut self) -> INEPNAK_EFF_W<DIEPINT0_SPEC> {
        INEPNAK_EFF_W::new(self, 6)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&mut self) -> BNAINTR_W<DIEPINT0_SPEC> {
        BNAINTR_W::new(self, 9)
    }
}
#[doc = "Device Endpoint-0 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT0_SPEC;
impl crate::RegisterSpec for DIEPINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint0::R`](R) reader structure"]
impl crate::Readable for DIEPINT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepint0::W`](W) writer structure"]
impl crate::Writable for DIEPINT0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPINT0 to value 0x80"]
impl crate::Resettable for DIEPINT0_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
