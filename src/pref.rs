#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcon: PCON,
}
impl RegisterBlock {
    #[doc = "0x00 - Prefetch Configuration Register"]
    #[inline(always)]
    pub const fn pcon(&self) -> &PCON {
        &self.pcon
    }
}
#[doc = "PCON (rw) register accessor: Prefetch Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcon`] module"]
pub type PCON = crate::Reg<pcon::PCON_SPEC>;
#[doc = "Prefetch Configuration Register"]
pub mod pcon;
