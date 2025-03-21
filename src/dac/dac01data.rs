#[doc = "Register `DAC01DATA` reader"]
pub type R = crate::R<DAC01DATA_SPEC>;
#[doc = "Register `DAC01DATA` writer"]
pub type W = crate::W<DAC01DATA_SPEC>;
#[doc = "Field `DATA0` reader - DAC0 Data Bits"]
pub type DATA0_R = crate::FieldReader<u16>;
#[doc = "Field `DATA0` writer - DAC0 Data Bits"]
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DATA1` reader - DAC1 Data Bits"]
pub type DATA1_R = crate::FieldReader<u16>;
#[doc = "Field `DATA1` writer - DAC1 Data Bits"]
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC1 Data Bits"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<DAC01DATA_SPEC> {
        DATA0_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC1 Data Bits"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<DAC01DATA_SPEC> {
        DATA1_W::new(self, 16)
    }
}
#[doc = "DAC01 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac01data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac01data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC01DATA_SPEC;
impl crate::RegisterSpec for DAC01DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac01data::R`](R) reader structure"]
impl crate::Readable for DAC01DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac01data::W`](W) writer structure"]
impl crate::Writable for DAC01DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC01DATA to value 0"]
impl crate::Resettable for DAC01DATA_SPEC {}
