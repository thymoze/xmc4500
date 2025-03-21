#[doc = "Register `PLLCON0` reader"]
pub type R = crate::R<PLLCON0_SPEC>;
#[doc = "Register `PLLCON0` writer"]
pub type W = crate::W<PLLCON0_SPEC>;
#[doc = "VCO Bypass\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOBYP_A {
    #[doc = "0: Normal operation, VCO is not bypassed"]
    VALUE1 = 0,
    #[doc = "1: Prescaler Mode, VCO is bypassed"]
    VALUE2 = 1,
}
impl From<VCOBYP_A> for bool {
    #[inline(always)]
    fn from(variant: VCOBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOBYP` reader - VCO Bypass"]
pub type VCOBYP_R = crate::BitReader<VCOBYP_A>;
impl VCOBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCOBYP_A {
        match self.bits {
            false => VCOBYP_A::VALUE1,
            true => VCOBYP_A::VALUE2,
        }
    }
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VCOBYP_A::VALUE1
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VCOBYP_A::VALUE2
    }
}
#[doc = "Field `VCOBYP` writer - VCO Bypass"]
pub type VCOBYP_W<'a, REG> = crate::BitWriter<'a, REG, VCOBYP_A>;
impl<'a, REG> VCOBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VCOBYP_A::VALUE1)
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VCOBYP_A::VALUE2)
    }
}
#[doc = "VCO Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOPWD_A {
    #[doc = "0: Normal behavior"]
    VALUE1 = 0,
    #[doc = "1: The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    VALUE2 = 1,
}
impl From<VCOPWD_A> for bool {
    #[inline(always)]
    fn from(variant: VCOPWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPWD` reader - VCO Power Saving Mode"]
pub type VCOPWD_R = crate::BitReader<VCOPWD_A>;
impl VCOPWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCOPWD_A {
        match self.bits {
            false => VCOPWD_A::VALUE1,
            true => VCOPWD_A::VALUE2,
        }
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VCOPWD_A::VALUE1
    }
    #[doc = "The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VCOPWD_A::VALUE2
    }
}
#[doc = "Field `VCOPWD` writer - VCO Power Saving Mode"]
pub type VCOPWD_W<'a, REG> = crate::BitWriter<'a, REG, VCOPWD_A>;
impl<'a, REG> VCOPWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VCOPWD_A::VALUE1)
    }
    #[doc = "The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VCOPWD_A::VALUE2)
    }
}
#[doc = "VCO Trim Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOTR_A {
    #[doc = "0: VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    VALUE1 = 0,
    #[doc = "1: VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    VALUE2 = 1,
}
impl From<VCOTR_A> for bool {
    #[inline(always)]
    fn from(variant: VCOTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOTR` reader - VCO Trim Control"]
pub type VCOTR_R = crate::BitReader<VCOTR_A>;
impl VCOTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCOTR_A {
        match self.bits {
            false => VCOTR_A::VALUE1,
            true => VCOTR_A::VALUE2,
        }
    }
    #[doc = "VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VCOTR_A::VALUE1
    }
    #[doc = "VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VCOTR_A::VALUE2
    }
}
#[doc = "Field `VCOTR` writer - VCO Trim Control"]
pub type VCOTR_W<'a, REG> = crate::BitWriter<'a, REG, VCOTR_A>;
impl<'a, REG> VCOTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VCOTR_A::VALUE1)
    }
    #[doc = "VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VCOTR_A::VALUE2)
    }
}
#[doc = "Disconnect Oscillator from VCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FINDIS_A {
    #[doc = "0: connect oscillator to the VCO part"]
    VALUE1 = 0,
    #[doc = "1: disconnect oscillator from the VCO part."]
    VALUE2 = 1,
}
impl From<FINDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FINDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FINDIS` reader - Disconnect Oscillator from VCO"]
pub type FINDIS_R = crate::BitReader<FINDIS_A>;
impl FINDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FINDIS_A {
        match self.bits {
            false => FINDIS_A::VALUE1,
            true => FINDIS_A::VALUE2,
        }
    }
    #[doc = "connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FINDIS_A::VALUE1
    }
    #[doc = "disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FINDIS_A::VALUE2
    }
}
#[doc = "Field `FINDIS` writer - Disconnect Oscillator from VCO"]
pub type FINDIS_W<'a, REG> = crate::BitWriter<'a, REG, FINDIS_A>;
impl<'a, REG> FINDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FINDIS_A::VALUE1)
    }
    #[doc = "disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FINDIS_A::VALUE2)
    }
}
#[doc = "Oscillator Disconnect Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCDISCDIS_A {
    #[doc = "0: In case of a PLL loss-of-lock bit FINDIS is set"]
    VALUE1 = 0,
    #[doc = "1: In case of a PLL loss-of-lock bit FINDIS is cleared"]
    VALUE2 = 1,
}
impl From<OSCDISCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCDISCDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCDISCDIS` reader - Oscillator Disconnect Disable"]
pub type OSCDISCDIS_R = crate::BitReader<OSCDISCDIS_A>;
impl OSCDISCDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCDISCDIS_A {
        match self.bits {
            false => OSCDISCDIS_A::VALUE1,
            true => OSCDISCDIS_A::VALUE2,
        }
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCDISCDIS_A::VALUE1
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCDISCDIS_A::VALUE2
    }
}
#[doc = "Field `OSCDISCDIS` writer - Oscillator Disconnect Disable"]
pub type OSCDISCDIS_W<'a, REG> = crate::BitWriter<'a, REG, OSCDISCDIS_A>;
impl<'a, REG> OSCDISCDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCDISCDIS_A::VALUE1)
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OSCDISCDIS_A::VALUE2)
    }
}
#[doc = "PLL Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPWD_A {
    #[doc = "0: Normal behavior"]
    VALUE1 = 0,
    #[doc = "1: The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    VALUE2 = 1,
}
impl From<PLLPWD_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPWD` reader - PLL Power Saving Mode"]
pub type PLLPWD_R = crate::BitReader<PLLPWD_A>;
impl PLLPWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLPWD_A {
        match self.bits {
            false => PLLPWD_A::VALUE1,
            true => PLLPWD_A::VALUE2,
        }
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLLPWD_A::VALUE1
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLLPWD_A::VALUE2
    }
}
#[doc = "Field `PLLPWD` writer - PLL Power Saving Mode"]
pub type PLLPWD_W<'a, REG> = crate::BitWriter<'a, REG, PLLPWD_A>;
impl<'a, REG> PLLPWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPWD_A::VALUE1)
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPWD_A::VALUE2)
    }
}
#[doc = "Oscillator Watchdog Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCRES_A {
    #[doc = "0: The Oscillator Watchdog of the PLL is not cleared and remains active"]
    VALUE1 = 0,
    #[doc = "1: The Oscillator Watchdog of the PLL is cleared and restarted"]
    VALUE2 = 1,
}
impl From<OSCRES_A> for bool {
    #[inline(always)]
    fn from(variant: OSCRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCRES` reader - Oscillator Watchdog Reset"]
pub type OSCRES_R = crate::BitReader<OSCRES_A>;
impl OSCRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCRES_A {
        match self.bits {
            false => OSCRES_A::VALUE1,
            true => OSCRES_A::VALUE2,
        }
    }
    #[doc = "The Oscillator Watchdog of the PLL is not cleared and remains active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCRES_A::VALUE1
    }
    #[doc = "The Oscillator Watchdog of the PLL is cleared and restarted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCRES_A::VALUE2
    }
}
#[doc = "Field `OSCRES` writer - Oscillator Watchdog Reset"]
pub type OSCRES_W<'a, REG> = crate::BitWriter<'a, REG, OSCRES_A>;
impl<'a, REG> OSCRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Oscillator Watchdog of the PLL is not cleared and remains active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCRES_A::VALUE1)
    }
    #[doc = "The Oscillator Watchdog of the PLL is cleared and restarted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OSCRES_A::VALUE2)
    }
}
#[doc = "Field `RESLD` writer - Restart VCO Lock Detection"]
pub type RESLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Automatic Oscillator Calibration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOTREN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<AOTREN_A> for bool {
    #[inline(always)]
    fn from(variant: AOTREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOTREN` reader - Automatic Oscillator Calibration Enable"]
pub type AOTREN_R = crate::BitReader<AOTREN_A>;
impl AOTREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AOTREN_A {
        match self.bits {
            false => AOTREN_A::VALUE1,
            true => AOTREN_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AOTREN_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AOTREN_A::VALUE2
    }
}
#[doc = "Field `AOTREN` writer - Automatic Oscillator Calibration Enable"]
pub type AOTREN_W<'a, REG> = crate::BitWriter<'a, REG, AOTREN_A>;
impl<'a, REG> AOTREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AOTREN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AOTREN_A::VALUE2)
    }
}
#[doc = "Factory Oscillator Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOTR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Force fixed-value trimming"]
    VALUE2 = 1,
}
impl From<FOTR_A> for bool {
    #[inline(always)]
    fn from(variant: FOTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOTR` reader - Factory Oscillator Calibration"]
pub type FOTR_R = crate::BitReader<FOTR_A>;
impl FOTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FOTR_A {
        match self.bits {
            false => FOTR_A::VALUE1,
            true => FOTR_A::VALUE2,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FOTR_A::VALUE1
    }
    #[doc = "Force fixed-value trimming"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FOTR_A::VALUE2
    }
}
#[doc = "Field `FOTR` writer - Factory Oscillator Calibration"]
pub type FOTR_W<'a, REG> = crate::BitWriter<'a, REG, FOTR_A>;
impl<'a, REG> FOTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FOTR_A::VALUE1)
    }
    #[doc = "Force fixed-value trimming"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FOTR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&self) -> VCOBYP_R {
        VCOBYP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&self) -> VCOPWD_R {
        VCOPWD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&self) -> VCOTR_R {
        VCOTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&self) -> FINDIS_R {
        FINDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&self) -> OSCDISCDIS_R {
        OSCDISCDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&self) -> PLLPWD_R {
        PLLPWD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Oscillator Watchdog Reset"]
    #[inline(always)]
    pub fn oscres(&self) -> OSCRES_R {
        OSCRES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Automatic Oscillator Calibration Enable"]
    #[inline(always)]
    pub fn aotren(&self) -> AOTREN_R {
        AOTREN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Factory Oscillator Calibration"]
    #[inline(always)]
    pub fn fotr(&self) -> FOTR_R {
        FOTR_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&mut self) -> VCOBYP_W<PLLCON0_SPEC> {
        VCOBYP_W::new(self, 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&mut self) -> VCOPWD_W<PLLCON0_SPEC> {
        VCOPWD_W::new(self, 1)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&mut self) -> VCOTR_W<PLLCON0_SPEC> {
        VCOTR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&mut self) -> FINDIS_W<PLLCON0_SPEC> {
        FINDIS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&mut self) -> OSCDISCDIS_W<PLLCON0_SPEC> {
        OSCDISCDIS_W::new(self, 6)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&mut self) -> PLLPWD_W<PLLCON0_SPEC> {
        PLLPWD_W::new(self, 16)
    }
    #[doc = "Bit 17 - Oscillator Watchdog Reset"]
    #[inline(always)]
    pub fn oscres(&mut self) -> OSCRES_W<PLLCON0_SPEC> {
        OSCRES_W::new(self, 17)
    }
    #[doc = "Bit 18 - Restart VCO Lock Detection"]
    #[inline(always)]
    pub fn resld(&mut self) -> RESLD_W<PLLCON0_SPEC> {
        RESLD_W::new(self, 18)
    }
    #[doc = "Bit 19 - Automatic Oscillator Calibration Enable"]
    #[inline(always)]
    pub fn aotren(&mut self) -> AOTREN_W<PLLCON0_SPEC> {
        AOTREN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Factory Oscillator Calibration"]
    #[inline(always)]
    pub fn fotr(&mut self) -> FOTR_W<PLLCON0_SPEC> {
        FOTR_W::new(self, 20)
    }
}
#[doc = "PLL Configuration 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcon0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcon0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCON0_SPEC;
impl crate::RegisterSpec for PLLCON0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcon0::R`](R) reader structure"]
impl crate::Readable for PLLCON0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcon0::W`](W) writer structure"]
impl crate::Writable for PLLCON0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCON0 to value 0x0003_0003"]
impl crate::Resettable for PLLCON0_SPEC {
    const RESET_VALUE: u32 = 0x0003_0003;
}
