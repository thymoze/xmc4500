#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 15 Output Register"]
    pub out: OUT,
    #[doc = "0x04 - Port 15 Output Modification Register"]
    pub omr: OMR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Port 15 Input/Output Control Register 0"]
    pub iocr0: IOCR0,
    #[doc = "0x14 - Port 15 Input/Output Control Register 4"]
    pub iocr4: IOCR4,
    #[doc = "0x18 - Port 15 Input/Output Control Register 8"]
    pub iocr8: IOCR8,
    #[doc = "0x1c - Port 15 Input/Output Control Register 12"]
    pub iocr12: IOCR12,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - Port 15 Input Register"]
    pub in_: IN,
    _reserved7: [u8; 0x38],
    #[doc = "0x60 - Port 15 Pin Function Decision Control Register"]
    pub pdisc: PDISC,
    _reserved8: [u8; 0x0c],
    #[doc = "0x70 - Port 15 Pin Power Save Register"]
    pub pps: PPS,
    #[doc = "0x74 - Port 15 Pin Hardware Select Register"]
    pub hwsel: HWSEL,
}
#[doc = "OUT (rw) register accessor: Port 15 Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out`]
module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Port 15 Output Register"]
pub mod out;
#[doc = "OMR (w) register accessor: Port 15 Output Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`omr`]
module"]
pub type OMR = crate::Reg<omr::OMR_SPEC>;
#[doc = "Port 15 Output Modification Register"]
pub mod omr;
#[doc = "IOCR0 (rw) register accessor: Port 15 Input/Output Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iocr0`]
module"]
pub type IOCR0 = crate::Reg<iocr0::IOCR0_SPEC>;
#[doc = "Port 15 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "IOCR4 (rw) register accessor: Port 15 Input/Output Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iocr4`]
module"]
pub type IOCR4 = crate::Reg<iocr4::IOCR4_SPEC>;
#[doc = "Port 15 Input/Output Control Register 4"]
pub mod iocr4;
#[doc = "IOCR8 (rw) register accessor: Port 15 Input/Output Control Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iocr8`]
module"]
pub type IOCR8 = crate::Reg<iocr8::IOCR8_SPEC>;
#[doc = "Port 15 Input/Output Control Register 8"]
pub mod iocr8;
#[doc = "IOCR12 (rw) register accessor: Port 15 Input/Output Control Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iocr12`]
module"]
pub type IOCR12 = crate::Reg<iocr12::IOCR12_SPEC>;
#[doc = "Port 15 Input/Output Control Register 12"]
pub mod iocr12;
#[doc = "IN (r) register accessor: Port 15 Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Port 15 Input Register"]
pub mod in_;
#[doc = "PDISC (rw) register accessor: Port 15 Pin Function Decision Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdisc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdisc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdisc`]
module"]
pub type PDISC = crate::Reg<pdisc::PDISC_SPEC>;
#[doc = "Port 15 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "PPS (rw) register accessor: Port 15 Pin Power Save Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pps`]
module"]
pub type PPS = crate::Reg<pps::PPS_SPEC>;
#[doc = "Port 15 Pin Power Save Register"]
pub mod pps;
#[doc = "HWSEL (rw) register accessor: Port 15 Pin Hardware Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hwsel`]
module"]
pub type HWSEL = crate::Reg<hwsel::HWSEL_SPEC>;
#[doc = "Port 15 Pin Hardware Select Register"]
pub mod hwsel;
