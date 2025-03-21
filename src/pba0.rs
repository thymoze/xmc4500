#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: STS,
    waddr: WADDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Peripheral Bridge Status Register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x04 - PBA Write Error Address Register"]
    #[inline(always)]
    pub const fn waddr(&self) -> &WADDR {
        &self.waddr
    }
}
#[doc = "STS (rw) register accessor: Peripheral Bridge Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Peripheral Bridge Status Register"]
pub mod sts;
#[doc = "WADDR (r) register accessor: PBA Write Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`waddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waddr`] module"]
pub type WADDR = crate::Reg<waddr::WADDR_SPEC>;
#[doc = "PBA Write Error Address Register"]
pub mod waddr;
