#[doc = "Register `SHCSR` reader"]
pub type R = crate::R<SHCSR_SPEC>;
#[doc = "Register `SHCSR` writer"]
pub type W = crate::W<SHCSR_SPEC>;
#[doc = "Field `MEMFAULTACT` reader - MemManage exception active bit"]
pub type MEMFAULTACT_R = crate::BitReader;
#[doc = "Field `MEMFAULTACT` writer - MemManage exception active bit"]
pub type MEMFAULTACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTACT` reader - BusFault exception active bit"]
pub type BUSFAULTACT_R = crate::BitReader;
#[doc = "Field `BUSFAULTACT` writer - BusFault exception active bit"]
pub type BUSFAULTACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTACT` reader - UsageFault exception active bit"]
pub type USGFAULTACT_R = crate::BitReader;
#[doc = "Field `USGFAULTACT` writer - UsageFault exception active bit"]
pub type USGFAULTACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVCALLACT` reader - SVCall active bit"]
pub type SVCALLACT_R = crate::BitReader;
#[doc = "Field `SVCALLACT` writer - SVCall active bit"]
pub type SVCALLACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONITORACT` reader - Debug monitor active bit"]
pub type MONITORACT_R = crate::BitReader;
#[doc = "Field `MONITORACT` writer - Debug monitor active bit"]
pub type MONITORACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSVACT` reader - PendSV exception active bit"]
pub type PENDSVACT_R = crate::BitReader;
#[doc = "Field `PENDSVACT` writer - PendSV exception active bit"]
pub type PENDSVACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTICKACT` reader - SysTick exception active bit"]
pub type SYSTICKACT_R = crate::BitReader;
#[doc = "Field `SYSTICKACT` writer - SysTick exception active bit"]
pub type SYSTICKACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTPENDED` reader - UsageFault exception pending bit"]
pub type USGFAULTPENDED_R = crate::BitReader;
#[doc = "Field `USGFAULTPENDED` writer - UsageFault exception pending bit"]
pub type USGFAULTPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMFAULTPENDED` reader - MemManage exception pending bit"]
pub type MEMFAULTPENDED_R = crate::BitReader;
#[doc = "Field `MEMFAULTPENDED` writer - MemManage exception pending bit"]
pub type MEMFAULTPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTPENDED` reader - BusFault exception pending bit"]
pub type BUSFAULTPENDED_R = crate::BitReader;
#[doc = "Field `BUSFAULTPENDED` writer - BusFault exception pending bit"]
pub type BUSFAULTPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVCALLPENDED` reader - SVCall pending bit"]
pub type SVCALLPENDED_R = crate::BitReader;
#[doc = "Field `SVCALLPENDED` writer - SVCall pending bit"]
pub type SVCALLPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMFAULTENA` reader - MemManage enable bit"]
pub type MEMFAULTENA_R = crate::BitReader;
#[doc = "Field `MEMFAULTENA` writer - MemManage enable bit"]
pub type MEMFAULTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTENA` reader - BusFault enable bit"]
pub type BUSFAULTENA_R = crate::BitReader;
#[doc = "Field `BUSFAULTENA` writer - BusFault enable bit"]
pub type BUSFAULTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTENA` reader - UsageFault enable bit"]
pub type USGFAULTENA_R = crate::BitReader;
#[doc = "Field `USGFAULTENA` writer - UsageFault enable bit"]
pub type USGFAULTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debug monitor active bit"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline(always)]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W<SHCSR_SPEC> {
        MEMFAULTACT_W::new(self, 0)
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline(always)]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W<SHCSR_SPEC> {
        BUSFAULTACT_W::new(self, 1)
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline(always)]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W<SHCSR_SPEC> {
        USGFAULTACT_W::new(self, 3)
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline(always)]
    pub fn svcallact(&mut self) -> SVCALLACT_W<SHCSR_SPEC> {
        SVCALLACT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Debug monitor active bit"]
    #[inline(always)]
    pub fn monitoract(&mut self) -> MONITORACT_W<SHCSR_SPEC> {
        MONITORACT_W::new(self, 8)
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline(always)]
    pub fn pendsvact(&mut self) -> PENDSVACT_W<SHCSR_SPEC> {
        PENDSVACT_W::new(self, 10)
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline(always)]
    pub fn systickact(&mut self) -> SYSTICKACT_W<SHCSR_SPEC> {
        SYSTICKACT_W::new(self, 11)
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline(always)]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W<SHCSR_SPEC> {
        USGFAULTPENDED_W::new(self, 12)
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline(always)]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W<SHCSR_SPEC> {
        MEMFAULTPENDED_W::new(self, 13)
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline(always)]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W<SHCSR_SPEC> {
        BUSFAULTPENDED_W::new(self, 14)
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline(always)]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W<SHCSR_SPEC> {
        SVCALLPENDED_W::new(self, 15)
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline(always)]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W<SHCSR_SPEC> {
        MEMFAULTENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline(always)]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W<SHCSR_SPEC> {
        BUSFAULTENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline(always)]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W<SHCSR_SPEC> {
        USGFAULTENA_W::new(self, 18)
    }
}
#[doc = "System Handler Control and State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHCSR_SPEC;
impl crate::RegisterSpec for SHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcsr::R`](R) reader structure"]
impl crate::Readable for SHCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shcsr::W`](W) writer structure"]
impl crate::Writable for SHCSR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {}
