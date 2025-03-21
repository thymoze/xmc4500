#[doc = "Register `NCR` reader"]
pub type R = crate::R<NCR_SPEC>;
#[doc = "Register `NCR` writer"]
pub type W = crate::W<NCR_SPEC>;
#[doc = "Node Initialization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    #[doc = "0: Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    VALUE1 = 0,
    #[doc = "1: Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    VALUE2 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Node Initialization"]
pub type INIT_R = crate::BitReader<INIT_A>;
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::VALUE1,
            true => INIT_A::VALUE2,
        }
    }
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INIT_A::VALUE1
    }
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INIT_A::VALUE2
    }
}
#[doc = "Field `INIT` writer - Node Initialization"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG, INIT_A>;
impl<'a, REG> INIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(INIT_A::VALUE1)
    }
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(INIT_A::VALUE2)
    }
}
#[doc = "Transfer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIE_A {
    #[doc = "0: Transfer interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Transfer interrupt is enabled."]
    VALUE2 = 1,
}
impl From<TRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIE` reader - Transfer Interrupt Enable"]
pub type TRIE_R = crate::BitReader<TRIE_A>;
impl TRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRIE_A {
        match self.bits {
            false => TRIE_A::VALUE1,
            true => TRIE_A::VALUE2,
        }
    }
    #[doc = "Transfer interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRIE_A::VALUE1
    }
    #[doc = "Transfer interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRIE_A::VALUE2
    }
}
#[doc = "Field `TRIE` writer - Transfer Interrupt Enable"]
pub type TRIE_W<'a, REG> = crate::BitWriter<'a, REG, TRIE_A>;
impl<'a, REG> TRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIE_A::VALUE1)
    }
    #[doc = "Transfer interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIE_A::VALUE2)
    }
}
#[doc = "LEC Indicated Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LECIE_A {
    #[doc = "0: Last error code interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Last error code interrupt is enabled."]
    VALUE2 = 1,
}
impl From<LECIE_A> for bool {
    #[inline(always)]
    fn from(variant: LECIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LECIE` reader - LEC Indicated Error Interrupt Enable"]
pub type LECIE_R = crate::BitReader<LECIE_A>;
impl LECIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LECIE_A {
        match self.bits {
            false => LECIE_A::VALUE1,
            true => LECIE_A::VALUE2,
        }
    }
    #[doc = "Last error code interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LECIE_A::VALUE1
    }
    #[doc = "Last error code interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LECIE_A::VALUE2
    }
}
#[doc = "Field `LECIE` writer - LEC Indicated Error Interrupt Enable"]
pub type LECIE_W<'a, REG> = crate::BitWriter<'a, REG, LECIE_A>;
impl<'a, REG> LECIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Last error code interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LECIE_A::VALUE1)
    }
    #[doc = "Last error code interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LECIE_A::VALUE2)
    }
}
#[doc = "Alert Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIE_A {
    #[doc = "0: Alert interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Alert interrupt is enabled."]
    VALUE2 = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIE` reader - Alert Interrupt Enable"]
pub type ALIE_R = crate::BitReader<ALIE_A>;
impl ALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::VALUE1,
            true => ALIE_A::VALUE2,
        }
    }
    #[doc = "Alert interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALIE_A::VALUE1
    }
    #[doc = "Alert interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALIE_A::VALUE2
    }
}
#[doc = "Field `ALIE` writer - Alert Interrupt Enable"]
pub type ALIE_W<'a, REG> = crate::BitWriter<'a, REG, ALIE_A>;
impl<'a, REG> ALIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alert interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALIE_A::VALUE1)
    }
    #[doc = "Alert interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALIE_A::VALUE2)
    }
}
#[doc = "Field `CANDIS` reader - CAN Disable"]
pub type CANDIS_R = crate::BitReader;
#[doc = "Field `CANDIS` writer - CAN Disable"]
pub type CANDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configuration Change Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCE_A {
    #[doc = "0: The Bit Timing Register, the Port Control Register, and the Error Counter Register may only be read. All attempts to modify them are ignored."]
    VALUE1 = 0,
    #[doc = "1: The Bit Timing Register, the Port Control Register, and the Error Counter Register may be read and written."]
    VALUE2 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CCE_R = crate::BitReader<CCE_A>;
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::VALUE1,
            true => CCE_A::VALUE2,
        }
    }
    #[doc = "The Bit Timing Register, the Port Control Register, and the Error Counter Register may only be read. All attempts to modify them are ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCE_A::VALUE1
    }
    #[doc = "The Bit Timing Register, the Port Control Register, and the Error Counter Register may be read and written."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCE_A::VALUE2
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG, CCE_A>;
impl<'a, REG> CCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Bit Timing Register, the Port Control Register, and the Error Counter Register may only be read. All attempts to modify them are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCE_A::VALUE1)
    }
    #[doc = "The Bit Timing Register, the Port Control Register, and the Error Counter Register may be read and written."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCE_A::VALUE2)
    }
}
#[doc = "Field `CALM` reader - CAN Analyzer Mode"]
pub type CALM_R = crate::BitReader;
#[doc = "Field `CALM` writer - CAN Analyzer Mode"]
pub type CALM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Suspend Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSEN_A {
    #[doc = "0: An OCDS suspend trigger is ignored by the CAN node."]
    VALUE1 = 0,
    #[doc = "1: An OCDS suspend trigger disables the CAN node: As soon as the CAN node becomes bus-idle or bus-off, bit INIT is internally forced to 1 to disable the CAN node. The actual value of bit INIT remains unchanged."]
    VALUE2 = 1,
}
impl From<SUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: SUSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSEN` reader - Suspend Enable"]
pub type SUSEN_R = crate::BitReader<SUSEN_A>;
impl SUSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSEN_A {
        match self.bits {
            false => SUSEN_A::VALUE1,
            true => SUSEN_A::VALUE2,
        }
    }
    #[doc = "An OCDS suspend trigger is ignored by the CAN node."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSEN_A::VALUE1
    }
    #[doc = "An OCDS suspend trigger disables the CAN node: As soon as the CAN node becomes bus-idle or bus-off, bit INIT is internally forced to 1 to disable the CAN node. The actual value of bit INIT remains unchanged."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSEN_A::VALUE2
    }
}
#[doc = "Field `SUSEN` writer - Suspend Enable"]
pub type SUSEN_W<'a, REG> = crate::BitWriter<'a, REG, SUSEN_A>;
impl<'a, REG> SUSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An OCDS suspend trigger is ignored by the CAN node."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSEN_A::VALUE1)
    }
    #[doc = "An OCDS suspend trigger disables the CAN node: As soon as the CAN node becomes bus-idle or bus-off, bit INIT is internally forced to 1 to disable the CAN node. The actual value of bit INIT remains unchanged."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SUSEN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&self) -> TRIE_R {
        TRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    pub fn candis(&self) -> CANDIS_R {
        CANDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Suspend Enable"]
    #[inline(always)]
    pub fn susen(&self) -> SUSEN_R {
        SUSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<NCR_SPEC> {
        INIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&mut self) -> TRIE_W<NCR_SPEC> {
        TRIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    pub fn lecie(&mut self) -> LECIE_W<NCR_SPEC> {
        LECIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&mut self) -> ALIE_W<NCR_SPEC> {
        ALIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    pub fn candis(&mut self) -> CANDIS_W<NCR_SPEC> {
        CANDIS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W<NCR_SPEC> {
        CCE_W::new(self, 6)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W<NCR_SPEC> {
        CALM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Suspend Enable"]
    #[inline(always)]
    pub fn susen(&mut self) -> SUSEN_W<NCR_SPEC> {
        SUSEN_W::new(self, 8)
    }
}
#[doc = "Node Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NCR_SPEC;
impl crate::RegisterSpec for NCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr::R`](R) reader structure"]
impl crate::Readable for NCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ncr::W`](W) writer structure"]
impl crate::Writable for NCR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NCR to value 0x01"]
impl crate::Resettable for NCR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
