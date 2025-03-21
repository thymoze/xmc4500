#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ir: IR,
    res: RES,
    cfg: CFG,
    sts: STS,
    length: LENGTH,
    check: CHECK,
    crc: CRC,
    ctr: CTR,
}
impl RegisterBlock {
    #[doc = "0x00 - Input Register"]
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    #[doc = "0x04 - CRC Result Register"]
    #[inline(always)]
    pub const fn res(&self) -> &RES {
        &self.res
    }
    #[doc = "0x08 - CRC Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x0c - CRC Status Register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x10 - CRC Length Register"]
    #[inline(always)]
    pub const fn length(&self) -> &LENGTH {
        &self.length
    }
    #[doc = "0x14 - CRC Check Register"]
    #[inline(always)]
    pub const fn check(&self) -> &CHECK {
        &self.check
    }
    #[doc = "0x18 - CRC Register"]
    #[inline(always)]
    pub const fn crc(&self) -> &CRC {
        &self.crc
    }
    #[doc = "0x1c - CRC Test Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
}
#[doc = "IR (rw) register accessor: Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`] module"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Input Register"]
pub mod ir;
#[doc = "RES (r) register accessor: CRC Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`res::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res`] module"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "CRC Result Register"]
pub mod res;
#[doc = "CFG (rw) register accessor: CRC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "CRC Configuration Register"]
pub mod cfg;
#[doc = "STS (rw) register accessor: CRC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "CRC Status Register"]
pub mod sts;
#[doc = "LENGTH (rw) register accessor: CRC Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@length`] module"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "CRC Length Register"]
pub mod length;
#[doc = "CHECK (rw) register accessor: CRC Check Register\n\nYou can [`read`](crate::Reg::read) this register and get [`check::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`check::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@check`] module"]
pub type CHECK = crate::Reg<check::CHECK_SPEC>;
#[doc = "CRC Check Register"]
pub mod check;
#[doc = "CRC (rw) register accessor: CRC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc`] module"]
pub type CRC = crate::Reg<crc::CRC_SPEC>;
#[doc = "CRC Register"]
pub mod crc;
#[doc = "CTR (rw) register accessor: CRC Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`] module"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "CRC Test Register"]
pub mod ctr;
