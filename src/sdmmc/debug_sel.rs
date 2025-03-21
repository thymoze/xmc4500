#[doc = "Register `DEBUG_SEL` writer"]
pub type W = crate::W<DEBUG_SEL_SPEC>;
#[doc = "Debug_sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUG_SEL_A {
    #[doc = "0: receiver module and fifo_ctrl module signals are probed out"]
    VALUE1 = 0,
    #[doc = "1: cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    VALUE2 = 1,
}
impl From<DEBUG_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUG_SEL` writer - Debug_sel"]
pub type DEBUG_SEL_W<'a, REG> = crate::BitWriter<'a, REG, DEBUG_SEL_A>;
impl<'a, REG> DEBUG_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "receiver module and fifo_ctrl module signals are probed out"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DEBUG_SEL_A::VALUE1)
    }
    #[doc = "cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DEBUG_SEL_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Debug_sel"]
    #[inline(always)]
    pub fn debug_sel(&mut self) -> DEBUG_SEL_W<DEBUG_SEL_SPEC> {
        DEBUG_SEL_W::new(self, 0)
    }
}
#[doc = "Debug Selection Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_sel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SEL_SPEC;
impl crate::RegisterSpec for DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`debug_sel::W`](W) writer structure"]
impl crate::Writable for DEBUG_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_SEL to value 0"]
impl crate::Resettable for DEBUG_SEL_SPEC {}
