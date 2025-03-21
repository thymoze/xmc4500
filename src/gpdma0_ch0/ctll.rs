#[doc = "Register `CTLL` reader"]
pub type R = crate::R<CTLL_SPEC>;
#[doc = "Register `CTLL` writer"]
pub type W = crate::W<CTLL_SPEC>;
#[doc = "Field `INT_EN` reader - Interrupt Enable Bit"]
pub type INT_EN_R = crate::BitReader;
#[doc = "Field `INT_EN` writer - Interrupt Enable Bit"]
pub type INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_TR_WIDTH` reader - Destination Transfer Width"]
pub type DST_TR_WIDTH_R = crate::FieldReader;
#[doc = "Field `DST_TR_WIDTH` writer - Destination Transfer Width"]
pub type DST_TR_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_TR_WIDTH` reader - Source Transfer Width"]
pub type SRC_TR_WIDTH_R = crate::FieldReader;
#[doc = "Field `SRC_TR_WIDTH` writer - Source Transfer Width"]
pub type SRC_TR_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Destination Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DINC_A {
    #[doc = "0: Increment"]
    VALUE1 = 0,
    #[doc = "1: Decrement"]
    VALUE2 = 1,
    #[doc = "2: No change"]
    VALUE3 = 2,
}
impl From<DINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DINC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DINC_A {
    type Ux = u8;
}
impl crate::IsEnum for DINC_A {}
#[doc = "Field `DINC` reader - Destination Address Increment"]
pub type DINC_R = crate::FieldReader<DINC_A>;
impl DINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DINC_A> {
        match self.bits {
            0 => Some(DINC_A::VALUE1),
            1 => Some(DINC_A::VALUE2),
            2 => Some(DINC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Increment"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DINC_A::VALUE1
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DINC_A::VALUE2
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DINC_A::VALUE3
    }
}
#[doc = "Field `DINC` writer - Destination Address Increment"]
pub type DINC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DINC_A>;
impl<'a, REG> DINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DINC_A::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DINC_A::VALUE2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DINC_A::VALUE3)
    }
}
#[doc = "Source Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SINC_A {
    #[doc = "0: Increment"]
    VALUE1 = 0,
    #[doc = "1: Decrement"]
    VALUE2 = 1,
    #[doc = "2: No change"]
    VALUE3 = 2,
}
impl From<SINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SINC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SINC_A {
    type Ux = u8;
}
impl crate::IsEnum for SINC_A {}
#[doc = "Field `SINC` reader - Source Address Increment"]
pub type SINC_R = crate::FieldReader<SINC_A>;
impl SINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SINC_A> {
        match self.bits {
            0 => Some(SINC_A::VALUE1),
            1 => Some(SINC_A::VALUE2),
            2 => Some(SINC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Increment"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SINC_A::VALUE1
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SINC_A::VALUE2
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SINC_A::VALUE3
    }
}
#[doc = "Field `SINC` writer - Source Address Increment"]
pub type SINC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SINC_A>;
impl<'a, REG> SINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SINC_A::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SINC_A::VALUE2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SINC_A::VALUE3)
    }
}
#[doc = "Field `DEST_MSIZE` reader - Destination Burst Transaction Length"]
pub type DEST_MSIZE_R = crate::FieldReader;
#[doc = "Field `DEST_MSIZE` writer - Destination Burst Transaction Length"]
pub type DEST_MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_MSIZE` reader - Source Burst Transaction Length"]
pub type SRC_MSIZE_R = crate::FieldReader;
#[doc = "Field `SRC_MSIZE` writer - Source Burst Transaction Length"]
pub type SRC_MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Source gather enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRC_GATHER_EN_A {
    #[doc = "0: Gather disabled"]
    VALUE1 = 0,
    #[doc = "1: Gather enabled"]
    VALUE2 = 1,
}
impl From<SRC_GATHER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_GATHER_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_GATHER_EN` reader - Source gather enable"]
pub type SRC_GATHER_EN_R = crate::BitReader<SRC_GATHER_EN_A>;
impl SRC_GATHER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRC_GATHER_EN_A {
        match self.bits {
            false => SRC_GATHER_EN_A::VALUE1,
            true => SRC_GATHER_EN_A::VALUE2,
        }
    }
    #[doc = "Gather disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRC_GATHER_EN_A::VALUE1
    }
    #[doc = "Gather enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRC_GATHER_EN_A::VALUE2
    }
}
#[doc = "Field `SRC_GATHER_EN` writer - Source gather enable"]
pub type SRC_GATHER_EN_W<'a, REG> = crate::BitWriter<'a, REG, SRC_GATHER_EN_A>;
impl<'a, REG> SRC_GATHER_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gather disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_GATHER_EN_A::VALUE1)
    }
    #[doc = "Gather enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_GATHER_EN_A::VALUE2)
    }
}
#[doc = "Destination scatter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DST_SCATTER_EN_A {
    #[doc = "0: Scatter disabled"]
    VALUE1 = 0,
    #[doc = "1: Scatter enabled"]
    VALUE2 = 1,
}
impl From<DST_SCATTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DST_SCATTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_SCATTER_EN` reader - Destination scatter enable"]
pub type DST_SCATTER_EN_R = crate::BitReader<DST_SCATTER_EN_A>;
impl DST_SCATTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DST_SCATTER_EN_A {
        match self.bits {
            false => DST_SCATTER_EN_A::VALUE1,
            true => DST_SCATTER_EN_A::VALUE2,
        }
    }
    #[doc = "Scatter disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DST_SCATTER_EN_A::VALUE1
    }
    #[doc = "Scatter enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DST_SCATTER_EN_A::VALUE2
    }
}
#[doc = "Field `DST_SCATTER_EN` writer - Destination scatter enable"]
pub type DST_SCATTER_EN_W<'a, REG> = crate::BitWriter<'a, REG, DST_SCATTER_EN_A>;
impl<'a, REG> DST_SCATTER_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scatter disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DST_SCATTER_EN_A::VALUE1)
    }
    #[doc = "Scatter enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DST_SCATTER_EN_A::VALUE2)
    }
}
#[doc = "Field `TT_FC` reader - Transfer Type and Flow Control"]
pub type TT_FC_R = crate::FieldReader;
#[doc = "Field `TT_FC` writer - Transfer Type and Flow Control"]
pub type TT_FC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LLP_DST_EN` reader - Linked List Pointer for Destination Enable"]
pub type LLP_DST_EN_R = crate::BitReader;
#[doc = "Field `LLP_DST_EN` writer - Linked List Pointer for Destination Enable"]
pub type LLP_DST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLP_SRC_EN` reader - Linked List Pointer for Source Enable"]
pub type LLP_SRC_EN_R = crate::BitReader;
#[doc = "Field `LLP_SRC_EN` writer - Linked List Pointer for Source Enable"]
pub type LLP_SRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DST_TR_WIDTH_R {
        DST_TR_WIDTH_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SRC_TR_WIDTH_R {
        SRC_TR_WIDTH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dest_msize(&self) -> DEST_MSIZE_R {
        DEST_MSIZE_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SRC_MSIZE_R {
        SRC_MSIZE_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - Source gather enable"]
    #[inline(always)]
    pub fn src_gather_en(&self) -> SRC_GATHER_EN_R {
        SRC_GATHER_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination scatter enable"]
    #[inline(always)]
    pub fn dst_scatter_en(&self) -> DST_SCATTER_EN_R {
        DST_SCATTER_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TT_FC_R {
        TT_FC_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 27 - Linked List Pointer for Destination Enable"]
    #[inline(always)]
    pub fn llp_dst_en(&self) -> LLP_DST_EN_R {
        LLP_DST_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Linked List Pointer for Source Enable"]
    #[inline(always)]
    pub fn llp_src_en(&self) -> LLP_SRC_EN_R {
        LLP_SRC_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W<CTLL_SPEC> {
        INT_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&mut self) -> DST_TR_WIDTH_W<CTLL_SPEC> {
        DST_TR_WIDTH_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&mut self) -> SRC_TR_WIDTH_W<CTLL_SPEC> {
        SRC_TR_WIDTH_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W<CTLL_SPEC> {
        DINC_W::new(self, 7)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W<CTLL_SPEC> {
        SINC_W::new(self, 9)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dest_msize(&mut self) -> DEST_MSIZE_W<CTLL_SPEC> {
        DEST_MSIZE_W::new(self, 11)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&mut self) -> SRC_MSIZE_W<CTLL_SPEC> {
        SRC_MSIZE_W::new(self, 14)
    }
    #[doc = "Bit 17 - Source gather enable"]
    #[inline(always)]
    pub fn src_gather_en(&mut self) -> SRC_GATHER_EN_W<CTLL_SPEC> {
        SRC_GATHER_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Destination scatter enable"]
    #[inline(always)]
    pub fn dst_scatter_en(&mut self) -> DST_SCATTER_EN_W<CTLL_SPEC> {
        DST_SCATTER_EN_W::new(self, 18)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    pub fn tt_fc(&mut self) -> TT_FC_W<CTLL_SPEC> {
        TT_FC_W::new(self, 20)
    }
    #[doc = "Bit 27 - Linked List Pointer for Destination Enable"]
    #[inline(always)]
    pub fn llp_dst_en(&mut self) -> LLP_DST_EN_W<CTLL_SPEC> {
        LLP_DST_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Linked List Pointer for Source Enable"]
    #[inline(always)]
    pub fn llp_src_en(&mut self) -> LLP_SRC_EN_W<CTLL_SPEC> {
        LLP_SRC_EN_W::new(self, 28)
    }
}
#[doc = "Control Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`ctll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLL_SPEC;
impl crate::RegisterSpec for CTLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctll::R`](R) reader structure"]
impl crate::Readable for CTLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctll::W`](W) writer structure"]
impl crate::Writable for CTLL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTLL to value 0x0030_4801"]
impl crate::Resettable for CTLL_SPEC {
    const RESET_VALUE: u32 = 0x0030_4801;
}
