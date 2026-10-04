#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn pmsr(self) -> crate::common::Reg<regs::Pmsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn voscr(self) -> crate::common::Reg<regs::Voscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn vossr(self) -> crate::common::Reg<regs::Vossr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn dbpcr(self) -> crate::common::Reg<regs::Dbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn bdsr(self) -> crate::common::Reg<regs::Bdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn ucpdr(self) -> crate::common::Reg<regs::Ucpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn sccr(self) -> crate::common::Reg<regs::Sccr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn vmcr(self) -> crate::common::Reg<regs::Vmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn usbscr(self) -> crate::common::Reg<regs::Usbscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn vmsr(self) -> crate::common::Reg<regs::Vmsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn wuscr(self) -> crate::common::Reg<regs::Wuscr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn wusr(self) -> crate::common::Reg<regs::Wusr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn wucr(self) -> crate::common::Reg<regs::Wucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn ioretr(self) -> crate::common::Reg<regs::Ioretr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[must_use]
        #[inline(always)]
        pub const fn bren(&self) -> super::vals::Retention {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Retention::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_bren(&mut self, val: super::vals::Retention) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn monen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_monen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vbrs(&self) -> super::vals::Vbrs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vbrs::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_vbrs(&mut self, val: super::vals::Vbrs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    impl core::fmt::Debug for Bdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr")
                .field("bren", &self.bren())
                .field("monen", &self.monen())
                .field("vbe", &self.vbe())
                .field("vbrs", &self.vbrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdcr {{ bren: {:?}, monen: {=bool:?}, vbe: {=bool:?}, vbrs: {:?} }}",
                self.bren(),
                self.monen(),
                self.vbe(),
                self.vbrs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdsr(pub u32);
    impl Bdsr {
        #[must_use]
        #[inline(always)]
        pub const fn brrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_brrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vbatl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vbatl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vbath(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vbath(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn templ(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_templ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn temph(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_temph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Bdsr {
        #[inline(always)]
        fn default() -> Bdsr {
            Bdsr(0)
        }
    }
    impl core::fmt::Debug for Bdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdsr")
                .field("brrdy", &self.brrdy())
                .field("vbatl", &self.vbatl())
                .field("vbath", &self.vbath())
                .field("templ", &self.templ())
                .field("temph", &self.temph())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdsr {{ brrdy: {=bool:?}, vbatl: {=bool:?}, vbath: {=bool:?}, templ: {=bool:?}, temph: {=bool:?} }}",
                self.brrdy(),
                self.vbatl(),
                self.vbath(),
                self.templ(),
                self.temph()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbpcr(pub u32);
    impl Dbpcr {
        #[must_use]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Dbpcr {
        #[inline(always)]
        fn default() -> Dbpcr {
            Dbpcr(0)
        }
    }
    impl core::fmt::Debug for Dbpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbpcr").field("dbp", &self.dbp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dbpcr {{ dbp: {=bool:?} }}", self.dbp())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioretr(pub u32);
    impl Ioretr {
        #[must_use]
        #[inline(always)]
        pub const fn ioreten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ioreten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn jtagioreten(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_jtagioreten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ioretr {
        #[inline(always)]
        fn default() -> Ioretr {
            Ioretr(0)
        }
    }
    impl core::fmt::Debug for Ioretr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ioretr")
                .field("ioreten", &self.ioreten())
                .field("jtagioreten", &self.jtagioreten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ioretr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ioretr {{ ioreten: {=bool:?}, jtagioreten: {=bool:?} }}",
                self.ioreten(),
                self.jtagioreten()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmcr(pub u32);
    impl Pmcr {
        #[must_use]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn svos(&self) -> super::vals::Svos {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Svos::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_svos(&mut self, val: super::vals::Svos) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flps(&self) -> super::vals::PowerModeInStopMode {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::PowerModeInStopMode::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_flps(&mut self, val: super::vals::PowerModeInStopMode) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn booste(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_booste(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn avd_ready(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_avd_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ethernetso(&self) -> super::vals::ShutOff {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::ShutOff::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_ethernetso(&mut self, val: super::vals::ShutOff) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sram3so(&self) -> super::vals::ShutOff {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::ShutOff::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_sram3so(&mut self, val: super::vals::ShutOff) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sram2_16lso(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sram2_16lso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sram2_16hso(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sram2_16hso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sram2_48so(&self) -> super::vals::ShutOff {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::ShutOff::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_sram2_48so(&mut self, val: super::vals::ShutOff) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sram1so(&self) -> super::vals::ShutOff {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::ShutOff::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_sram1so(&mut self, val: super::vals::ShutOff) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Pmcr {
        #[inline(always)]
        fn default() -> Pmcr {
            Pmcr(0)
        }
    }
    impl core::fmt::Debug for Pmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmcr")
                .field("lpms", &self.lpms())
                .field("svos", &self.svos())
                .field("cssf", &self.cssf())
                .field("flps", &self.flps())
                .field("booste", &self.booste())
                .field("avd_ready", &self.avd_ready())
                .field("ethernetso", &self.ethernetso())
                .field("sram3so", &self.sram3so())
                .field("sram2_16lso", &self.sram2_16lso())
                .field("sram2_16hso", &self.sram2_16hso())
                .field("sram2_48so", &self.sram2_48so())
                .field("sram1so", &self.sram1so())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmcr {{ lpms: {:?}, svos: {:?}, cssf: {=bool:?}, flps: {:?}, booste: {=bool:?}, avd_ready: {=bool:?}, ethernetso: {:?}, sram3so: {:?}, sram2_16lso: {=bool:?}, sram2_16hso: {=bool:?}, sram2_48so: {:?}, sram1so: {:?} }}",
                self.lpms(),
                self.svos(),
                self.cssf(),
                self.flps(),
                self.booste(),
                self.avd_ready(),
                self.ethernetso(),
                self.sram3so(),
                self.sram2_16lso(),
                self.sram2_16hso(),
                self.sram2_48so(),
                self.sram1so()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmsr(pub u32);
    impl Pmsr {
        #[must_use]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Pmsr {
        #[inline(always)]
        fn default() -> Pmsr {
            Pmsr(0)
        }
    }
    impl core::fmt::Debug for Pmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmsr")
                .field("stopf", &self.stopf())
                .field("sbf", &self.sbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmsr {{ stopf: {=bool:?}, sbf: {=bool:?} }}",
                self.stopf(),
                self.sbf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[must_use]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("spriv", &self.spriv())
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {=bool:?}, nspriv: {=bool:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sccr(pub u32);
    impl Sccr {
        #[must_use]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ldoen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ldoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Sccr {
        #[inline(always)]
        fn default() -> Sccr {
            Sccr(0)
        }
    }
    impl core::fmt::Debug for Sccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sccr")
                .field("bypass", &self.bypass())
                .field("ldoen", &self.ldoen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sccr {{ bypass: {=bool:?}, ldoen: {=bool:?} }}",
                self.bypass(),
                self.ldoen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[must_use]
        #[inline(always)]
        pub const fn wupsec(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wupsec(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[must_use]
        #[inline(always)]
        pub const fn retsec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_retsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpmsec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scmsec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_scmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vbsec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vusbsec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vusbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("wupsec[0]", &self.wupsec(0usize))
                .field("wupsec[1]", &self.wupsec(1usize))
                .field("wupsec[2]", &self.wupsec(2usize))
                .field("wupsec[3]", &self.wupsec(3usize))
                .field("wupsec[4]", &self.wupsec(4usize))
                .field("wupsec[5]", &self.wupsec(5usize))
                .field("wupsec[6]", &self.wupsec(6usize))
                .field("wupsec[7]", &self.wupsec(7usize))
                .field("retsec", &self.retsec())
                .field("lpmsec", &self.lpmsec())
                .field("scmsec", &self.scmsec())
                .field("vbsec", &self.vbsec())
                .field("vusbsec", &self.vusbsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seccfgr {{ wupsec[0]: {=bool:?}, wupsec[1]: {=bool:?}, wupsec[2]: {=bool:?}, wupsec[3]: {=bool:?}, wupsec[4]: {=bool:?}, wupsec[5]: {=bool:?}, wupsec[6]: {=bool:?}, wupsec[7]: {=bool:?}, retsec: {=bool:?}, lpmsec: {=bool:?}, scmsec: {=bool:?}, vbsec: {=bool:?}, vusbsec: {=bool:?} }}",
                self.wupsec(0usize),
                self.wupsec(1usize),
                self.wupsec(2usize),
                self.wupsec(3usize),
                self.wupsec(4usize),
                self.wupsec(5usize),
                self.wupsec(6usize),
                self.wupsec(7usize),
                self.retsec(),
                self.lpmsec(),
                self.scmsec(),
                self.vbsec(),
                self.vusbsec()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucpdr(pub u32);
    impl Ucpdr {
        #[must_use]
        #[inline(always)]
        pub const fn ucpd_dbdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ucpd_dbdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ucpd_stby(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ucpd_stby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ucpdr {
        #[inline(always)]
        fn default() -> Ucpdr {
            Ucpdr(0)
        }
    }
    impl core::fmt::Debug for Ucpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ucpdr")
                .field("ucpd_dbdis", &self.ucpd_dbdis())
                .field("ucpd_stby", &self.ucpd_stby())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ucpdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ucpdr {{ ucpd_dbdis: {=bool:?}, ucpd_stby: {=bool:?} }}",
                self.ucpd_dbdis(),
                self.ucpd_stby()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usbscr(pub u32);
    impl Usbscr {
        #[must_use]
        #[inline(always)]
        pub const fn usb33den(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usb33den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usb33sv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usb33sv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Usbscr {
        #[inline(always)]
        fn default() -> Usbscr {
            Usbscr(0)
        }
    }
    impl core::fmt::Debug for Usbscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usbscr")
                .field("usb33den", &self.usb33den())
                .field("usb33sv", &self.usb33sv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usbscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usbscr {{ usb33den: {=bool:?}, usb33sv: {=bool:?} }}",
                self.usb33den(),
                self.usb33sv()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmcr(pub u32);
    impl Vmcr {
        #[must_use]
        #[inline(always)]
        pub const fn pvden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pvden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn avden(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_avden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn als(&self) -> super::vals::Als {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Als::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_als(&mut self, val: super::vals::Als) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
    }
    impl Default for Vmcr {
        #[inline(always)]
        fn default() -> Vmcr {
            Vmcr(0)
        }
    }
    impl core::fmt::Debug for Vmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vmcr")
                .field("pvden", &self.pvden())
                .field("pls", &self.pls())
                .field("avden", &self.avden())
                .field("als", &self.als())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vmcr {{ pvden: {=bool:?}, pls: {:?}, avden: {=bool:?}, als: {:?} }}",
                self.pvden(),
                self.pls(),
                self.avden(),
                self.als()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmsr(pub u32);
    impl Vmsr {
        #[must_use]
        #[inline(always)]
        pub const fn avdo(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_avdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vddio2rdy(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vddio2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usb33rdy(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usb33rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Vmsr {
        #[inline(always)]
        fn default() -> Vmsr {
            Vmsr(0)
        }
    }
    impl core::fmt::Debug for Vmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vmsr")
                .field("avdo", &self.avdo())
                .field("vddio2rdy", &self.vddio2rdy())
                .field("pvdo", &self.pvdo())
                .field("usb33rdy", &self.usb33rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vmsr {{ avdo: {=bool:?}, vddio2rdy: {=bool:?}, pvdo: {=bool:?}, usb33rdy: {=bool:?} }}",
                self.avdo(),
                self.vddio2rdy(),
                self.pvdo(),
                self.usb33rdy()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Voscr(pub u32);
    impl Voscr {
        #[must_use]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Voscr {
        #[inline(always)]
        fn default() -> Voscr {
            Voscr(0)
        }
    }
    impl core::fmt::Debug for Voscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Voscr").field("vos", &self.vos()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Voscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Voscr {{ vos: {:?} }}", self.vos())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vossr(pub u32);
    impl Vossr {
        #[must_use]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_actvosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn actvos(&self) -> super::vals::Vos {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_actvos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Vossr {
        #[inline(always)]
        fn default() -> Vossr {
            Vossr(0)
        }
    }
    impl core::fmt::Debug for Vossr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vossr")
                .field("vosrdy", &self.vosrdy())
                .field("actvosrdy", &self.actvosrdy())
                .field("actvos", &self.actvos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vossr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vossr {{ vosrdy: {=bool:?}, actvosrdy: {=bool:?}, actvos: {:?} }}",
                self.vosrdy(),
                self.actvosrdy(),
                self.actvos()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr(pub u32);
    impl Wucr {
        #[must_use]
        #[inline(always)]
        pub const fn wupen(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wupen(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wupen_bits(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wupen_bits(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wupp(&self, n: usize) -> super::vals::Wupp {
            assert!(n < 8usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_wupp(&mut self, n: usize, val: super::vals::Wupp) {
            assert!(n < 8usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd(&self, n: usize) -> super::vals::Wuppupd {
            assert!(n < 8usize);
            let offs = 16usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_wuppupd(&mut self, n: usize, val: super::vals::Wuppupd) {
            assert!(n < 8usize);
            let offs = 16usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Wucr {
        #[inline(always)]
        fn default() -> Wucr {
            Wucr(0)
        }
    }
    impl core::fmt::Debug for Wucr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wucr")
                .field("wupen[0]", &self.wupen(0usize))
                .field("wupen[1]", &self.wupen(1usize))
                .field("wupen[2]", &self.wupen(2usize))
                .field("wupen[3]", &self.wupen(3usize))
                .field("wupen[4]", &self.wupen(4usize))
                .field("wupen[5]", &self.wupen(5usize))
                .field("wupen[6]", &self.wupen(6usize))
                .field("wupen[7]", &self.wupen(7usize))
                .field("wupen_bits", &self.wupen_bits())
                .field("wupp[0]", &self.wupp(0usize))
                .field("wupp[1]", &self.wupp(1usize))
                .field("wupp[2]", &self.wupp(2usize))
                .field("wupp[3]", &self.wupp(3usize))
                .field("wupp[4]", &self.wupp(4usize))
                .field("wupp[5]", &self.wupp(5usize))
                .field("wupp[6]", &self.wupp(6usize))
                .field("wupp[7]", &self.wupp(7usize))
                .field("wuppupd[0]", &self.wuppupd(0usize))
                .field("wuppupd[1]", &self.wuppupd(1usize))
                .field("wuppupd[2]", &self.wuppupd(2usize))
                .field("wuppupd[3]", &self.wuppupd(3usize))
                .field("wuppupd[4]", &self.wuppupd(4usize))
                .field("wuppupd[5]", &self.wuppupd(5usize))
                .field("wuppupd[6]", &self.wuppupd(6usize))
                .field("wuppupd[7]", &self.wuppupd(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wucr {{ wupen[0]: {=bool:?}, wupen[1]: {=bool:?}, wupen[2]: {=bool:?}, wupen[3]: {=bool:?}, wupen[4]: {=bool:?}, wupen[5]: {=bool:?}, wupen[6]: {=bool:?}, wupen[7]: {=bool:?}, wupen_bits: {=u8:?}, wupp[0]: {:?}, wupp[1]: {:?}, wupp[2]: {:?}, wupp[3]: {:?}, wupp[4]: {:?}, wupp[5]: {:?}, wupp[6]: {:?}, wupp[7]: {:?}, wuppupd[0]: {:?}, wuppupd[1]: {:?}, wuppupd[2]: {:?}, wuppupd[3]: {:?}, wuppupd[4]: {:?}, wuppupd[5]: {:?}, wuppupd[6]: {:?}, wuppupd[7]: {:?} }}",
                self.wupen(0usize),
                self.wupen(1usize),
                self.wupen(2usize),
                self.wupen(3usize),
                self.wupen(4usize),
                self.wupen(5usize),
                self.wupen(6usize),
                self.wupen(7usize),
                self.wupen_bits(),
                self.wupp(0usize),
                self.wupp(1usize),
                self.wupp(2usize),
                self.wupp(3usize),
                self.wupp(4usize),
                self.wupp(5usize),
                self.wupp(6usize),
                self.wupp(7usize),
                self.wuppupd(0usize),
                self.wuppupd(1usize),
                self.wuppupd(2usize),
                self.wuppupd(3usize),
                self.wuppupd(4usize),
                self.wuppupd(5usize),
                self.wuppupd(6usize),
                self.wuppupd(7usize)
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wuscr(pub u32);
    impl Wuscr {
        #[must_use]
        #[inline(always)]
        pub const fn cwuf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cwuf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cwuf_bits(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cwuf_bits(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wuscr {
        #[inline(always)]
        fn default() -> Wuscr {
            Wuscr(0)
        }
    }
    impl core::fmt::Debug for Wuscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wuscr")
                .field("cwuf[0]", &self.cwuf(0usize))
                .field("cwuf[1]", &self.cwuf(1usize))
                .field("cwuf[2]", &self.cwuf(2usize))
                .field("cwuf[3]", &self.cwuf(3usize))
                .field("cwuf[4]", &self.cwuf(4usize))
                .field("cwuf[5]", &self.cwuf(5usize))
                .field("cwuf[6]", &self.cwuf(6usize))
                .field("cwuf[7]", &self.cwuf(7usize))
                .field("cwuf_bits", &self.cwuf_bits())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wuscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wuscr {{ cwuf[0]: {=bool:?}, cwuf[1]: {=bool:?}, cwuf[2]: {=bool:?}, cwuf[3]: {=bool:?}, cwuf[4]: {=bool:?}, cwuf[5]: {=bool:?}, cwuf[6]: {=bool:?}, cwuf[7]: {=bool:?}, cwuf_bits: {=u8:?} }}",
                self.cwuf(0usize),
                self.cwuf(1usize),
                self.cwuf(2usize),
                self.cwuf(3usize),
                self.cwuf(4usize),
                self.cwuf(5usize),
                self.cwuf(6usize),
                self.cwuf(7usize),
                self.cwuf_bits()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wusr(pub u32);
    impl Wusr {
        #[must_use]
        #[inline(always)]
        pub const fn wuf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wuf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Wusr {
        #[inline(always)]
        fn default() -> Wusr {
            Wusr(0)
        }
    }
    impl core::fmt::Debug for Wusr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wusr")
                .field("wuf[0]", &self.wuf(0usize))
                .field("wuf[1]", &self.wuf(1usize))
                .field("wuf[2]", &self.wuf(2usize))
                .field("wuf[3]", &self.wuf(3usize))
                .field("wuf[4]", &self.wuf(4usize))
                .field("wuf[5]", &self.wuf(5usize))
                .field("wuf[6]", &self.wuf(6usize))
                .field("wuf[7]", &self.wuf(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wusr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wusr {{ wuf[0]: {=bool:?}, wuf[1]: {=bool:?}, wuf[2]: {=bool:?}, wuf[3]: {=bool:?}, wuf[4]: {=bool:?}, wuf[5]: {=bool:?}, wuf[6]: {=bool:?}, wuf[7]: {=bool:?} }}",
                self.wuf(0usize),
                self.wuf(1usize),
                self.wuf(2usize),
                self.wuf(3usize),
                self.wuf(4usize),
                self.wuf(5usize),
                self.wuf(6usize),
                self.wuf(7usize)
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Als {
        #[doc = "AVD level0 (VAVD0 ~ 1.7 V)"]
        Level0 = 0x0,
        #[doc = "AVD level1 (VAVD1 ~ 2.1 V)"]
        Level1 = 0x01,
        #[doc = "AVD level2 (VAVD2 ~ 2.5 V)"]
        Level2 = 0x02,
        #[doc = "AVD level3 (VAVD3 ~ 2.8 V)"]
        Level3 = 0x03,
    }
    impl Als {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Als {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Als {
        #[inline(always)]
        fn from(val: u8) -> Als {
            Als::from_bits(val)
        }
    }
    impl From<Als> for u8 {
        #[inline(always)]
        fn from(val: Als) -> u8 {
            Als::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpms {
        #[doc = "Keeps Stop mode when entering DeepSleep."]
        Stop = 0x0,
        #[doc = "Allows Standby mode when entering DeepSleep."]
        Standby = 0x01,
    }
    impl Lpms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpms {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpms {
        #[inline(always)]
        fn from(val: u8) -> Lpms {
            Lpms::from_bits(val)
        }
    }
    impl From<Lpms> for u8 {
        #[inline(always)]
        fn from(val: Lpms) -> u8 {
            Lpms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pls {
        #[doc = "PVD level0 (VPVD0 ~ 1.95 V)"]
        Level0 = 0x0,
        #[doc = "PVD level1 (VPVD1 ~ 2.10 V)"]
        Level1 = 0x01,
        #[doc = "PVD level2 (VPVD2 ~ 2.25 V)"]
        Level2 = 0x02,
        #[doc = "PVD level3 (VPVD3 ~ 2.40 V)"]
        Level3 = 0x03,
        #[doc = "PVD level4 (VPVD4 ~ 2.55 V)"]
        Level4 = 0x04,
        #[doc = "PVD level5 (VPVD5 ~ 2.70 V)"]
        Level5 = 0x05,
        #[doc = "PVD level6 (VPVD6 ~ 2.85 V)"]
        Level6 = 0x06,
        #[doc = "PVD_IN pin"]
        PvdInPin = 0x07,
    }
    impl Pls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pls {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pls {
        #[inline(always)]
        fn from(val: u8) -> Pls {
            Pls::from_bits(val)
        }
    }
    impl From<Pls> for u8 {
        #[inline(always)]
        fn from(val: Pls) -> u8 {
            Pls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PowerModeInStopMode {
        #[doc = "Remains in normal mode when the system enters Stop mode (quick restart time)."]
        Normal = 0x0,
        #[doc = "Enters low-power mode when the system enters Stop mode (low-power consumption)."]
        LowPower = 0x01,
    }
    impl PowerModeInStopMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PowerModeInStopMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PowerModeInStopMode {
        #[inline(always)]
        fn from(val: u8) -> PowerModeInStopMode {
            PowerModeInStopMode::from_bits(val)
        }
    }
    impl From<PowerModeInStopMode> for u8 {
        #[inline(always)]
        fn from(val: PowerModeInStopMode) -> u8 {
            PowerModeInStopMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Retention {
        #[doc = "Content is lost."]
        Lost = 0x0,
        #[doc = "Content is preserved."]
        Preserved = 0x01,
    }
    impl Retention {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Retention {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Retention {
        #[inline(always)]
        fn from(val: u8) -> Retention {
            Retention::from_bits(val)
        }
    }
    impl From<Retention> for u8 {
        #[inline(always)]
        fn from(val: Retention) -> u8 {
            Retention::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ShutOff {
        #[doc = "Content is kept."]
        Kept = 0x0,
        #[doc = "Content is lost."]
        Lost = 0x01,
    }
    impl ShutOff {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ShutOff {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ShutOff {
        #[inline(always)]
        fn from(val: u8) -> ShutOff {
            ShutOff::from_bits(val)
        }
    }
    impl From<ShutOff> for u8 {
        #[inline(always)]
        fn from(val: ShutOff) -> u8 {
            ShutOff::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Svos {
        _RESERVED_0 = 0x0,
        #[doc = "SVOS5 scale 5"]
        Scale5 = 0x01,
        #[doc = "SVOS4 scale 4"]
        Scale4 = 0x02,
        #[doc = "SVOS3 scale 3 (default)"]
        Scale3 = 0x03,
    }
    impl Svos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Svos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Svos {
        #[inline(always)]
        fn from(val: u8) -> Svos {
            Svos::from_bits(val)
        }
    }
    impl From<Svos> for u8 {
        #[inline(always)]
        fn from(val: Svos) -> u8 {
            Svos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vbrs {
        #[doc = "Charge VBAT through a 5 kΩ resistor."]
        R5kOhm = 0x0,
        #[doc = "Charge VBAT through a 1.5 kΩ resistor."]
        R15kOhm = 0x01,
    }
    impl Vbrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbrs {
        #[inline(always)]
        fn from(val: u8) -> Vbrs {
            Vbrs::from_bits(val)
        }
    }
    impl From<Vbrs> for u8 {
        #[inline(always)]
        fn from(val: Vbrs) -> u8 {
            Vbrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vos {
        Scale3 = 0x0,
        Scale2 = 0x01,
        Scale1 = 0x02,
        Scale0 = 0x03,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vos {
        #[inline(always)]
        fn from(val: u8) -> Vos {
            Vos::from_bits(val)
        }
    }
    impl From<Vos> for u8 {
        #[inline(always)]
        fn from(val: Vos) -> u8 {
            Vos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wupp {
        #[doc = "detection on high level (rising edge)"]
        High = 0x0,
        #[doc = "detection on low level (falling edge)"]
        Low = 0x01,
    }
    impl Wupp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wupp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wupp {
        #[inline(always)]
        fn from(val: u8) -> Wupp {
            Wupp::from_bits(val)
        }
    }
    impl From<Wupp> for u8 {
        #[inline(always)]
        fn from(val: Wupp) -> u8 {
            Wupp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wuppupd {
        NoPullUp = 0x0,
        PullUp = 0x01,
        PullDown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wuppupd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wuppupd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wuppupd {
        #[inline(always)]
        fn from(val: u8) -> Wuppupd {
            Wuppupd::from_bits(val)
        }
    }
    impl From<Wuppupd> for u8 {
        #[inline(always)]
        fn from(val: Wuppupd) -> u8 {
            Wuppupd::to_bits(val)
        }
    }
}
