#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub gctrl: GCTRL,
    #[doc = "0x04 - Global Status Register"]
    pub gstat: GSTAT,
    #[doc = "0x08 - Global Idle Set"]
    pub gidls: GIDLS,
    #[doc = "0x0c - Global Idle Clear"]
    pub gidlc: GIDLC,
    #[doc = "0x10 - Global Channel Set"]
    pub gcss: GCSS,
    #[doc = "0x14 - Global Channel Clear"]
    pub gcsc: GCSC,
    #[doc = "0x18 - Global Channel status"]
    pub gcst: GCST,
    #[doc = "0x1c - Parity Checker Configuration"]
    pub gpchk: GPCHK,
    _reserved8: [u8; 0x30],
    #[doc = "0x50 - Extended Capture Mode Read"]
    pub ecrd: ECRD,
    _reserved9: [u8; 0x2c],
    #[doc = "0x80 - Module Identification"]
    pub midr: MIDR,
}
#[doc = "GCTRL (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gctrl`]
module"]
pub type GCTRL = crate::Reg<gctrl::GCTRL_SPEC>;
#[doc = "Global Control Register"]
pub mod gctrl;
#[doc = "GSTAT (r) register accessor: Global Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gstat`]
module"]
pub type GSTAT = crate::Reg<gstat::GSTAT_SPEC>;
#[doc = "Global Status Register"]
pub mod gstat;
#[doc = "GIDLS (w) register accessor: Global Idle Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidls::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gidls`]
module"]
pub type GIDLS = crate::Reg<gidls::GIDLS_SPEC>;
#[doc = "Global Idle Set"]
pub mod gidls;
#[doc = "GIDLC (w) register accessor: Global Idle Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidlc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gidlc`]
module"]
pub type GIDLC = crate::Reg<gidlc::GIDLC_SPEC>;
#[doc = "Global Idle Clear"]
pub mod gidlc;
#[doc = "GCSS (w) register accessor: Global Channel Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcss::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gcss`]
module"]
pub type GCSS = crate::Reg<gcss::GCSS_SPEC>;
#[doc = "Global Channel Set"]
pub mod gcss;
#[doc = "GCSC (w) register accessor: Global Channel Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcsc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gcsc`]
module"]
pub type GCSC = crate::Reg<gcsc::GCSC_SPEC>;
#[doc = "Global Channel Clear"]
pub mod gcsc;
#[doc = "GCST (r) register accessor: Global Channel status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gcst`]
module"]
pub type GCST = crate::Reg<gcst::GCST_SPEC>;
#[doc = "Global Channel status"]
pub mod gcst;
#[doc = "GPCHK (rw) register accessor: Parity Checker Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpchk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpchk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpchk`]
module"]
pub type GPCHK = crate::Reg<gpchk::GPCHK_SPEC>;
#[doc = "Parity Checker Configuration"]
pub mod gpchk;
#[doc = "ECRD (r) register accessor: Extended Capture Mode Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecrd::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ecrd`]
module"]
pub type ECRD = crate::Reg<ecrd::ECRD_SPEC>;
#[doc = "Extended Capture Mode Read"]
pub mod ecrd;
#[doc = "MIDR (r) register accessor: Module Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`midr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`midr`]
module"]
pub type MIDR = crate::Reg<midr::MIDR_SPEC>;
#[doc = "Module Identification"]
pub mod midr;
