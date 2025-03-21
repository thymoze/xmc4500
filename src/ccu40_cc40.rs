#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ins: INS,
    cmc: CMC,
    tcst: TCST,
    tcset: TCSET,
    tcclr: TCCLR,
    tc: TC,
    psl: PSL,
    dit: DIT,
    dits: DITS,
    psc: PSC,
    fpc: FPC,
    fpcs: FPCS,
    pr: PR,
    prs: PRS,
    cr: CR,
    crs: CRS,
    _reserved16: [u8; 0x30],
    timer: TIMER,
    c0v: C0V,
    c1v: C1V,
    c2v: C2V,
    c3v: C3V,
    _reserved21: [u8; 0x1c],
    ints: INTS,
    inte: INTE,
    srs: SRS,
    sws: SWS,
    swr: SWR,
}
impl RegisterBlock {
    #[doc = "0x00 - Input Selector Configuration"]
    #[inline(always)]
    pub const fn ins(&self) -> &INS {
        &self.ins
    }
    #[doc = "0x04 - Connection Matrix Control"]
    #[inline(always)]
    pub const fn cmc(&self) -> &CMC {
        &self.cmc
    }
    #[doc = "0x08 - Slice Timer Status"]
    #[inline(always)]
    pub const fn tcst(&self) -> &TCST {
        &self.tcst
    }
    #[doc = "0x0c - Slice Timer Run Set"]
    #[inline(always)]
    pub const fn tcset(&self) -> &TCSET {
        &self.tcset
    }
    #[doc = "0x10 - Slice Timer Clear"]
    #[inline(always)]
    pub const fn tcclr(&self) -> &TCCLR {
        &self.tcclr
    }
    #[doc = "0x14 - Slice Timer Control"]
    #[inline(always)]
    pub const fn tc(&self) -> &TC {
        &self.tc
    }
    #[doc = "0x18 - Passive Level Config"]
    #[inline(always)]
    pub const fn psl(&self) -> &PSL {
        &self.psl
    }
    #[doc = "0x1c - Dither Config"]
    #[inline(always)]
    pub const fn dit(&self) -> &DIT {
        &self.dit
    }
    #[doc = "0x20 - Dither Shadow Register"]
    #[inline(always)]
    pub const fn dits(&self) -> &DITS {
        &self.dits
    }
    #[doc = "0x24 - Prescaler Control"]
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    #[doc = "0x28 - Floating Prescaler Control"]
    #[inline(always)]
    pub const fn fpc(&self) -> &FPC {
        &self.fpc
    }
    #[doc = "0x2c - Floating Prescaler Shadow"]
    #[inline(always)]
    pub const fn fpcs(&self) -> &FPCS {
        &self.fpcs
    }
    #[doc = "0x30 - Timer Period Value"]
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
    #[doc = "0x34 - Timer Shadow Period Value"]
    #[inline(always)]
    pub const fn prs(&self) -> &PRS {
        &self.prs
    }
    #[doc = "0x38 - Timer Compare Value"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x3c - Timer Shadow Compare Value"]
    #[inline(always)]
    pub const fn crs(&self) -> &CRS {
        &self.crs
    }
    #[doc = "0x70 - Timer Value"]
    #[inline(always)]
    pub const fn timer(&self) -> &TIMER {
        &self.timer
    }
    #[doc = "0x74 - Capture Register 0"]
    #[inline(always)]
    pub const fn c0v(&self) -> &C0V {
        &self.c0v
    }
    #[doc = "0x78 - Capture Register 1"]
    #[inline(always)]
    pub const fn c1v(&self) -> &C1V {
        &self.c1v
    }
    #[doc = "0x7c - Capture Register 2"]
    #[inline(always)]
    pub const fn c2v(&self) -> &C2V {
        &self.c2v
    }
    #[doc = "0x80 - Capture Register 3"]
    #[inline(always)]
    pub const fn c3v(&self) -> &C3V {
        &self.c3v
    }
    #[doc = "0xa0 - Interrupt Status"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
    #[doc = "0xa4 - Interrupt Enable Control"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0xa8 - Service Request Selector"]
    #[inline(always)]
    pub const fn srs(&self) -> &SRS {
        &self.srs
    }
    #[doc = "0xac - Interrupt Status Set"]
    #[inline(always)]
    pub const fn sws(&self) -> &SWS {
        &self.sws
    }
    #[doc = "0xb0 - Interrupt Status Clear"]
    #[inline(always)]
    pub const fn swr(&self) -> &SWR {
        &self.swr
    }
}
#[doc = "INS (rw) register accessor: Input Selector Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ins::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ins::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ins`] module"]
pub type INS = crate::Reg<ins::INS_SPEC>;
#[doc = "Input Selector Configuration"]
pub mod ins;
#[doc = "CMC (rw) register accessor: Connection Matrix Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmc`] module"]
pub type CMC = crate::Reg<cmc::CMC_SPEC>;
#[doc = "Connection Matrix Control"]
pub mod cmc;
#[doc = "TCST (r) register accessor: Slice Timer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tcst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcst`] module"]
pub type TCST = crate::Reg<tcst::TCST_SPEC>;
#[doc = "Slice Timer Status"]
pub mod tcst;
#[doc = "TCSET (w) register accessor: Slice Timer Run Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcset`] module"]
pub type TCSET = crate::Reg<tcset::TCSET_SPEC>;
#[doc = "Slice Timer Run Set"]
pub mod tcset;
#[doc = "TCCLR (w) register accessor: Slice Timer Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcclr`] module"]
pub type TCCLR = crate::Reg<tcclr::TCCLR_SPEC>;
#[doc = "Slice Timer Clear"]
pub mod tcclr;
#[doc = "TC (rw) register accessor: Slice Timer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`] module"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Slice Timer Control"]
pub mod tc;
#[doc = "PSL (rw) register accessor: Passive Level Config\n\nYou can [`read`](crate::Reg::read) this register and get [`psl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psl`] module"]
pub type PSL = crate::Reg<psl::PSL_SPEC>;
#[doc = "Passive Level Config"]
pub mod psl;
#[doc = "DIT (r) register accessor: Dither Config\n\nYou can [`read`](crate::Reg::read) this register and get [`dit::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dit`] module"]
pub type DIT = crate::Reg<dit::DIT_SPEC>;
#[doc = "Dither Config"]
pub mod dit;
#[doc = "DITS (rw) register accessor: Dither Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dits`] module"]
pub type DITS = crate::Reg<dits::DITS_SPEC>;
#[doc = "Dither Shadow Register"]
pub mod dits;
#[doc = "PSC (rw) register accessor: Prescaler Control\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "Prescaler Control"]
pub mod psc;
#[doc = "FPC (rw) register accessor: Floating Prescaler Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc`] module"]
pub type FPC = crate::Reg<fpc::FPC_SPEC>;
#[doc = "Floating Prescaler Control"]
pub mod fpc;
#[doc = "FPCS (rw) register accessor: Floating Prescaler Shadow\n\nYou can [`read`](crate::Reg::read) this register and get [`fpcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpcs`] module"]
pub type FPCS = crate::Reg<fpcs::FPCS_SPEC>;
#[doc = "Floating Prescaler Shadow"]
pub mod fpcs;
#[doc = "PR (r) register accessor: Timer Period Value\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Timer Period Value"]
pub mod pr;
#[doc = "PRS (rw) register accessor: Timer Shadow Period Value\n\nYou can [`read`](crate::Reg::read) this register and get [`prs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs`] module"]
pub type PRS = crate::Reg<prs::PRS_SPEC>;
#[doc = "Timer Shadow Period Value"]
pub mod prs;
#[doc = "CR (r) register accessor: Timer Compare Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Timer Compare Value"]
pub mod cr;
#[doc = "CRS (rw) register accessor: Timer Shadow Compare Value\n\nYou can [`read`](crate::Reg::read) this register and get [`crs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crs`] module"]
pub type CRS = crate::Reg<crs::CRS_SPEC>;
#[doc = "Timer Shadow Compare Value"]
pub mod crs;
#[doc = "TIMER (rw) register accessor: Timer Value\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`] module"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "Timer Value"]
pub mod timer;
#[doc = "C0V (r) register accessor: Capture Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`c0v::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@c0v`] module"]
pub type C0V = crate::Reg<c0v::C0V_SPEC>;
#[doc = "Capture Register 0"]
pub mod c0v;
#[doc = "C1V (r) register accessor: Capture Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1v::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@c1v`] module"]
pub type C1V = crate::Reg<c1v::C1V_SPEC>;
#[doc = "Capture Register 1"]
pub mod c1v;
#[doc = "C2V (r) register accessor: Capture Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2v::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@c2v`] module"]
pub type C2V = crate::Reg<c2v::C2V_SPEC>;
#[doc = "Capture Register 2"]
pub mod c2v;
#[doc = "C3V (r) register accessor: Capture Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3v::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@c3v`] module"]
pub type C3V = crate::Reg<c3v::C3V_SPEC>;
#[doc = "Capture Register 3"]
pub mod c3v;
#[doc = "INTS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ints::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ints`] module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt Status"]
pub mod ints;
#[doc = "INTE (rw) register accessor: Interrupt Enable Control\n\nYou can [`read`](crate::Reg::read) this register and get [`inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inte`] module"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable Control"]
pub mod inte;
#[doc = "SRS (rw) register accessor: Service Request Selector\n\nYou can [`read`](crate::Reg::read) this register and get [`srs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srs`] module"]
pub type SRS = crate::Reg<srs::SRS_SPEC>;
#[doc = "Service Request Selector"]
pub mod srs;
#[doc = "SWS (w) register accessor: Interrupt Status Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sws::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sws`] module"]
pub type SWS = crate::Reg<sws::SWS_SPEC>;
#[doc = "Interrupt Status Set"]
pub mod sws;
#[doc = "SWR (w) register accessor: Interrupt Status Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swr`] module"]
pub type SWR = crate::Reg<swr::SWR_SPEC>;
#[doc = "Interrupt Status Clear"]
pub mod swr;
