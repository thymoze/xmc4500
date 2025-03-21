#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    oschpstat: OSCHPSTAT,
    oschpctrl: OSCHPCTRL,
    _reserved2: [u8; 0x04],
    clkcalconst: CLKCALCONST,
}
impl RegisterBlock {
    #[doc = "0x00 - OSC_HP Status Register"]
    #[inline(always)]
    pub const fn oschpstat(&self) -> &OSCHPSTAT {
        &self.oschpstat
    }
    #[doc = "0x04 - OSC_HP Control Register"]
    #[inline(always)]
    pub const fn oschpctrl(&self) -> &OSCHPCTRL {
        &self.oschpctrl
    }
    #[doc = "0x0c - Clock Calibration Constant Register"]
    #[inline(always)]
    pub const fn clkcalconst(&self) -> &CLKCALCONST {
        &self.clkcalconst
    }
}
#[doc = "OSCHPSTAT (r) register accessor: OSC_HP Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oschpstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschpstat`] module"]
pub type OSCHPSTAT = crate::Reg<oschpstat::OSCHPSTAT_SPEC>;
#[doc = "OSC_HP Status Register"]
pub mod oschpstat;
#[doc = "OSCHPCTRL (rw) register accessor: OSC_HP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oschpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oschpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschpctrl`] module"]
pub type OSCHPCTRL = crate::Reg<oschpctrl::OSCHPCTRL_SPEC>;
#[doc = "OSC_HP Control Register"]
pub mod oschpctrl;
#[doc = "CLKCALCONST (rw) register accessor: Clock Calibration Constant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcalconst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcalconst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcalconst`] module"]
pub type CLKCALCONST = crate::Reg<clkcalconst::CLKCALCONST_SPEC>;
#[doc = "Clock Calibration Constant Register"]
pub mod clkcalconst;
