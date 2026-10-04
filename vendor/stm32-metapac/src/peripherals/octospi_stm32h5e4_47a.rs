#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Octospi {
    ptr: *mut u8,
}
unsafe impl Send for Octospi {}
unsafe impl Sync for Octospi {}
impl Octospi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::OctospiCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn dcr1(self) -> crate::common::Reg<regs::OctospiDcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn dcr2(self) -> crate::common::Reg<regs::OctospiDcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn dcr3(self) -> crate::common::Reg<regs::OctospiDcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn dcr4(self) -> crate::common::Reg<regs::OctospiDcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::OctospiSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::OctospiFcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn dlr(self) -> crate::common::Reg<regs::OctospiDlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn ar(self) -> crate::common::Reg<regs::OctospiAr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::OctospiDr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn psmkr(self) -> crate::common::Reg<regs::OctospiPsmkr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn psmar(self) -> crate::common::Reg<regs::OctospiPsmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn pir(self) -> crate::common::Reg<regs::OctospiPir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::OctospiCcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::OctospiTcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::OctospiIr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn abr(self) -> crate::common::Reg<regs::OctospiAbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn lptr(self) -> crate::common::Reg<regs::OctospiLptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn wpccr(self) -> crate::common::Reg<regs::OctospiWpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn wptcr(self) -> crate::common::Reg<regs::OctospiWptcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn wpir(self) -> crate::common::Reg<regs::OctospiWpir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn wpabr(self) -> crate::common::Reg<regs::OctospiWpabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn wccr(self) -> crate::common::Reg<regs::OctospiWccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn wtcr(self) -> crate::common::Reg<regs::OctospiWtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn wir(self) -> crate::common::Reg<regs::OctospiWir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn wabr(self) -> crate::common::Reg<regs::OctospiWabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn hlcr(self) -> crate::common::Reg<regs::OctospiHlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiAbr(pub u32);
    impl OctospiAbr {
        #[must_use]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_alternate(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiAbr {
        #[inline(always)]
        fn default() -> OctospiAbr {
            OctospiAbr(0)
        }
    }
    impl core::fmt::Debug for OctospiAbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiAbr")
                .field("alternate", &self.alternate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiAbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiAbr {{ alternate: {=u32:?} }}", self.alternate())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiAr(pub u32);
    impl OctospiAr {
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiAr {
        #[inline(always)]
        fn default() -> OctospiAr {
            OctospiAr(0)
        }
    }
    impl core::fmt::Debug for OctospiAr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiAr")
                .field("address", &self.address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiAr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiAr {{ address: {=u32:?} }}", self.address())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiCcr(pub u32);
    impl OctospiCcr {
        #[must_use]
        #[inline(always)]
        pub const fn imode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_imode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn isize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_isize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn admode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_admode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn adsize(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_adsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_abmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn absize(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_absize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmode(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dqse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for OctospiCcr {
        #[inline(always)]
        fn default() -> OctospiCcr {
            OctospiCcr(0)
        }
    }
    impl core::fmt::Debug for OctospiCcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiCcr")
                .field("imode", &self.imode())
                .field("idtr", &self.idtr())
                .field("isize", &self.isize())
                .field("admode", &self.admode())
                .field("addtr", &self.addtr())
                .field("adsize", &self.adsize())
                .field("abmode", &self.abmode())
                .field("abdtr", &self.abdtr())
                .field("absize", &self.absize())
                .field("dmode", &self.dmode())
                .field("ddtr", &self.ddtr())
                .field("dqse", &self.dqse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiCcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiCcr {{ imode: {=u8:?}, idtr: {=bool:?}, isize: {=u8:?}, admode: {=u8:?}, addtr: {=bool:?}, adsize: {=u8:?}, abmode: {=u8:?}, abdtr: {=bool:?}, absize: {=u8:?}, dmode: {=u8:?}, ddtr: {=bool:?}, dqse: {=bool:?} }}",
                self.imode(),
                self.idtr(),
                self.isize(),
                self.admode(),
                self.addtr(),
                self.adsize(),
                self.abmode(),
                self.abdtr(),
                self.absize(),
                self.dmode(),
                self.ddtr(),
                self.dqse()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiCr(pub u32);
    impl OctospiCr {
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abort(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_abort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn adoffen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_adoffen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fthres(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fthres(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ftie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn smie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_smie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn toie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_toie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn berrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_berrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apms(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_apms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pmm(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cssel(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cssel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nopref(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nopref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fmode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for OctospiCr {
        #[inline(always)]
        fn default() -> OctospiCr {
            OctospiCr(0)
        }
    }
    impl core::fmt::Debug for OctospiCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiCr")
                .field("en", &self.en())
                .field("abort", &self.abort())
                .field("dmaen", &self.dmaen())
                .field("tcen", &self.tcen())
                .field("adoffen", &self.adoffen())
                .field("dmm", &self.dmm())
                .field("fthres", &self.fthres())
                .field("teie", &self.teie())
                .field("tcie", &self.tcie())
                .field("ftie", &self.ftie())
                .field("smie", &self.smie())
                .field("toie", &self.toie())
                .field("berrie", &self.berrie())
                .field("apms", &self.apms())
                .field("pmm", &self.pmm())
                .field("cssel", &self.cssel())
                .field("nopref", &self.nopref())
                .field("fmode", &self.fmode())
                .field("msel", &self.msel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiCr {{ en: {=bool:?}, abort: {=bool:?}, dmaen: {=bool:?}, tcen: {=bool:?}, adoffen: {=bool:?}, dmm: {=bool:?}, fthres: {=u8:?}, teie: {=bool:?}, tcie: {=bool:?}, ftie: {=bool:?}, smie: {=bool:?}, toie: {=bool:?}, berrie: {=bool:?}, apms: {=bool:?}, pmm: {=bool:?}, cssel: {=bool:?}, nopref: {=bool:?}, fmode: {=u8:?}, msel: {=bool:?} }}",
                self.en(),
                self.abort(),
                self.dmaen(),
                self.tcen(),
                self.adoffen(),
                self.dmm(),
                self.fthres(),
                self.teie(),
                self.tcie(),
                self.ftie(),
                self.smie(),
                self.toie(),
                self.berrie(),
                self.apms(),
                self.pmm(),
                self.cssel(),
                self.nopref(),
                self.fmode(),
                self.msel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiDcr1(pub u32);
    impl OctospiDcr1 {
        #[must_use]
        #[inline(always)]
        pub const fn ckmode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frck(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlybyp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dlybyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn csht(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_csht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn devsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_devsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn extendmem(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_extendmem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mtyp(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mtyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn adoff(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_adoff(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
        }
    }
    impl Default for OctospiDcr1 {
        #[inline(always)]
        fn default() -> OctospiDcr1 {
            OctospiDcr1(0)
        }
    }
    impl core::fmt::Debug for OctospiDcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiDcr1")
                .field("ckmode", &self.ckmode())
                .field("frck", &self.frck())
                .field("dlybyp", &self.dlybyp())
                .field("csht", &self.csht())
                .field("devsize", &self.devsize())
                .field("extendmem", &self.extendmem())
                .field("mtyp", &self.mtyp())
                .field("adoff", &self.adoff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiDcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiDcr1 {{ ckmode: {=bool:?}, frck: {=bool:?}, dlybyp: {=bool:?}, csht: {=u8:?}, devsize: {=u8:?}, extendmem: {=bool:?}, mtyp: {=u8:?}, adoff: {=u8:?} }}",
                self.ckmode(),
                self.frck(),
                self.dlybyp(),
                self.csht(),
                self.devsize(),
                self.extendmem(),
                self.mtyp(),
                self.adoff()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiDcr2(pub u32);
    impl OctospiDcr2 {
        #[must_use]
        #[inline(always)]
        pub const fn prescaler(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_prescaler(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wrapsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wrapsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for OctospiDcr2 {
        #[inline(always)]
        fn default() -> OctospiDcr2 {
            OctospiDcr2(0)
        }
    }
    impl core::fmt::Debug for OctospiDcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiDcr2")
                .field("prescaler", &self.prescaler())
                .field("wrapsize", &self.wrapsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiDcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiDcr2 {{ prescaler: {=u8:?}, wrapsize: {=u8:?} }}",
                self.prescaler(),
                self.wrapsize()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiDcr3(pub u32);
    impl OctospiDcr3 {
        #[must_use]
        #[inline(always)]
        pub const fn csbound(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_csbound(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for OctospiDcr3 {
        #[inline(always)]
        fn default() -> OctospiDcr3 {
            OctospiDcr3(0)
        }
    }
    impl core::fmt::Debug for OctospiDcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiDcr3")
                .field("csbound", &self.csbound())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiDcr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiDcr3 {{ csbound: {=u8:?} }}", self.csbound())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiDcr4(pub u32);
    impl OctospiDcr4 {
        #[must_use]
        #[inline(always)]
        pub const fn refresh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_refresh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiDcr4 {
        #[inline(always)]
        fn default() -> OctospiDcr4 {
            OctospiDcr4(0)
        }
    }
    impl core::fmt::Debug for OctospiDcr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiDcr4")
                .field("refresh", &self.refresh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiDcr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiDcr4 {{ refresh: {=u32:?} }}", self.refresh())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiDlr(pub u32);
    impl OctospiDlr {
        #[must_use]
        #[inline(always)]
        pub const fn dl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_dl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiDlr {
        #[inline(always)]
        fn default() -> OctospiDlr {
            OctospiDlr(0)
        }
    }
    impl core::fmt::Debug for OctospiDlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiDlr")
                .field("dl", &self.dl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiDlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiDlr {{ dl: {=u32:?} }}", self.dl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiDr(pub u32);
    impl OctospiDr {
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiDr {
        #[inline(always)]
        fn default() -> OctospiDr {
            OctospiDr(0)
        }
    }
    impl core::fmt::Debug for OctospiDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiDr")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiDr {{ data: {=u32:?} }}", self.data())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiFcr(pub u32);
    impl OctospiFcr {
        #[must_use]
        #[inline(always)]
        pub const fn ctef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn csmf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_csmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctof(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cberrf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cberrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for OctospiFcr {
        #[inline(always)]
        fn default() -> OctospiFcr {
            OctospiFcr(0)
        }
    }
    impl core::fmt::Debug for OctospiFcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiFcr")
                .field("ctef", &self.ctef())
                .field("ctcf", &self.ctcf())
                .field("csmf", &self.csmf())
                .field("ctof", &self.ctof())
                .field("cberrf", &self.cberrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiFcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiFcr {{ ctef: {=bool:?}, ctcf: {=bool:?}, csmf: {=bool:?}, ctof: {=bool:?}, cberrf: {=bool:?} }}",
                self.ctef(),
                self.ctcf(),
                self.csmf(),
                self.ctof(),
                self.cberrf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiHlcr(pub u32);
    impl OctospiHlcr {
        #[must_use]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wzl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wzl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tacc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tacc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trwr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trwr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for OctospiHlcr {
        #[inline(always)]
        fn default() -> OctospiHlcr {
            OctospiHlcr(0)
        }
    }
    impl core::fmt::Debug for OctospiHlcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiHlcr")
                .field("lm", &self.lm())
                .field("wzl", &self.wzl())
                .field("tacc", &self.tacc())
                .field("trwr", &self.trwr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiHlcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiHlcr {{ lm: {=bool:?}, wzl: {=bool:?}, tacc: {=u8:?}, trwr: {=u8:?} }}",
                self.lm(),
                self.wzl(),
                self.tacc(),
                self.trwr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiIr(pub u32);
    impl OctospiIr {
        #[must_use]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_instruction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiIr {
        #[inline(always)]
        fn default() -> OctospiIr {
            OctospiIr(0)
        }
    }
    impl core::fmt::Debug for OctospiIr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiIr")
                .field("instruction", &self.instruction())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiIr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiIr {{ instruction: {=u32:?} }}",
                self.instruction()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiLptr(pub u32);
    impl OctospiLptr {
        #[must_use]
        #[inline(always)]
        pub const fn timeout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_timeout(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OctospiLptr {
        #[inline(always)]
        fn default() -> OctospiLptr {
            OctospiLptr(0)
        }
    }
    impl core::fmt::Debug for OctospiLptr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiLptr")
                .field("timeout", &self.timeout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiLptr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiLptr {{ timeout: {=u16:?} }}", self.timeout())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiPir(pub u32);
    impl OctospiPir {
        #[must_use]
        #[inline(always)]
        pub const fn interval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_interval(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OctospiPir {
        #[inline(always)]
        fn default() -> OctospiPir {
            OctospiPir(0)
        }
    }
    impl core::fmt::Debug for OctospiPir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiPir")
                .field("interval", &self.interval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiPir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiPir {{ interval: {=u16:?} }}", self.interval())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiPsmar(pub u32);
    impl OctospiPsmar {
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_match_(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiPsmar {
        #[inline(always)]
        fn default() -> OctospiPsmar {
            OctospiPsmar(0)
        }
    }
    impl core::fmt::Debug for OctospiPsmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiPsmar")
                .field("match_", &self.match_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiPsmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiPsmar {{ match_: {=u32:?} }}", self.match_())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiPsmkr(pub u32);
    impl OctospiPsmkr {
        #[must_use]
        #[inline(always)]
        pub const fn mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiPsmkr {
        #[inline(always)]
        fn default() -> OctospiPsmkr {
            OctospiPsmkr(0)
        }
    }
    impl core::fmt::Debug for OctospiPsmkr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiPsmkr")
                .field("mask", &self.mask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiPsmkr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiPsmkr {{ mask: {=u32:?} }}", self.mask())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiSr(pub u32);
    impl OctospiSr {
        #[must_use]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn smf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_smf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tof(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn berrf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_berrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flevel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_flevel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for OctospiSr {
        #[inline(always)]
        fn default() -> OctospiSr {
            OctospiSr(0)
        }
    }
    impl core::fmt::Debug for OctospiSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiSr")
                .field("tef", &self.tef())
                .field("tcf", &self.tcf())
                .field("ftf", &self.ftf())
                .field("smf", &self.smf())
                .field("tof", &self.tof())
                .field("busy", &self.busy())
                .field("berrf", &self.berrf())
                .field("flevel", &self.flevel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiSr {{ tef: {=bool:?}, tcf: {=bool:?}, ftf: {=bool:?}, smf: {=bool:?}, tof: {=bool:?}, busy: {=bool:?}, berrf: {=bool:?}, flevel: {=u8:?} }}",
                self.tef(),
                self.tcf(),
                self.ftf(),
                self.smf(),
                self.tof(),
                self.busy(),
                self.berrf(),
                self.flevel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiTcr(pub u32);
    impl OctospiTcr {
        #[must_use]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dcyc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dhqc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dhqc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sshift(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sshift(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for OctospiTcr {
        #[inline(always)]
        fn default() -> OctospiTcr {
            OctospiTcr(0)
        }
    }
    impl core::fmt::Debug for OctospiTcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiTcr")
                .field("dcyc", &self.dcyc())
                .field("dhqc", &self.dhqc())
                .field("sshift", &self.sshift())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiTcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiTcr {{ dcyc: {=u8:?}, dhqc: {=bool:?}, sshift: {=bool:?} }}",
                self.dcyc(),
                self.dhqc(),
                self.sshift()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiWabr(pub u32);
    impl OctospiWabr {
        #[must_use]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_alternate(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiWabr {
        #[inline(always)]
        fn default() -> OctospiWabr {
            OctospiWabr(0)
        }
    }
    impl core::fmt::Debug for OctospiWabr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiWabr")
                .field("alternate", &self.alternate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiWabr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiWabr {{ alternate: {=u32:?} }}", self.alternate())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiWccr(pub u32);
    impl OctospiWccr {
        #[must_use]
        #[inline(always)]
        pub const fn imode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_imode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn isize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_isize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn admode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_admode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn adsize(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_adsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_abmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn absize(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_absize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmode(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dqse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for OctospiWccr {
        #[inline(always)]
        fn default() -> OctospiWccr {
            OctospiWccr(0)
        }
    }
    impl core::fmt::Debug for OctospiWccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiWccr")
                .field("imode", &self.imode())
                .field("idtr", &self.idtr())
                .field("isize", &self.isize())
                .field("admode", &self.admode())
                .field("addtr", &self.addtr())
                .field("adsize", &self.adsize())
                .field("abmode", &self.abmode())
                .field("abdtr", &self.abdtr())
                .field("absize", &self.absize())
                .field("dmode", &self.dmode())
                .field("ddtr", &self.ddtr())
                .field("dqse", &self.dqse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiWccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiWccr {{ imode: {=u8:?}, idtr: {=bool:?}, isize: {=u8:?}, admode: {=u8:?}, addtr: {=bool:?}, adsize: {=u8:?}, abmode: {=u8:?}, abdtr: {=bool:?}, absize: {=u8:?}, dmode: {=u8:?}, ddtr: {=bool:?}, dqse: {=bool:?} }}",
                self.imode(),
                self.idtr(),
                self.isize(),
                self.admode(),
                self.addtr(),
                self.adsize(),
                self.abmode(),
                self.abdtr(),
                self.absize(),
                self.dmode(),
                self.ddtr(),
                self.dqse()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiWir(pub u32);
    impl OctospiWir {
        #[must_use]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_instruction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiWir {
        #[inline(always)]
        fn default() -> OctospiWir {
            OctospiWir(0)
        }
    }
    impl core::fmt::Debug for OctospiWir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiWir")
                .field("instruction", &self.instruction())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiWir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiWir {{ instruction: {=u32:?} }}",
                self.instruction()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiWpabr(pub u32);
    impl OctospiWpabr {
        #[must_use]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_alternate(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiWpabr {
        #[inline(always)]
        fn default() -> OctospiWpabr {
            OctospiWpabr(0)
        }
    }
    impl core::fmt::Debug for OctospiWpabr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiWpabr")
                .field("alternate", &self.alternate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiWpabr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiWpabr {{ alternate: {=u32:?} }}",
                self.alternate()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiWpccr(pub u32);
    impl OctospiWpccr {
        #[must_use]
        #[inline(always)]
        pub const fn imode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_imode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn isize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_isize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn admode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_admode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn adsize(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_adsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_abmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn absize(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_absize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmode(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dqse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for OctospiWpccr {
        #[inline(always)]
        fn default() -> OctospiWpccr {
            OctospiWpccr(0)
        }
    }
    impl core::fmt::Debug for OctospiWpccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiWpccr")
                .field("imode", &self.imode())
                .field("idtr", &self.idtr())
                .field("isize", &self.isize())
                .field("admode", &self.admode())
                .field("addtr", &self.addtr())
                .field("adsize", &self.adsize())
                .field("abmode", &self.abmode())
                .field("abdtr", &self.abdtr())
                .field("absize", &self.absize())
                .field("dmode", &self.dmode())
                .field("ddtr", &self.ddtr())
                .field("dqse", &self.dqse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiWpccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiWpccr {{ imode: {=u8:?}, idtr: {=bool:?}, isize: {=u8:?}, admode: {=u8:?}, addtr: {=bool:?}, adsize: {=u8:?}, abmode: {=u8:?}, abdtr: {=bool:?}, absize: {=u8:?}, dmode: {=u8:?}, ddtr: {=bool:?}, dqse: {=bool:?} }}",
                self.imode(),
                self.idtr(),
                self.isize(),
                self.admode(),
                self.addtr(),
                self.adsize(),
                self.abmode(),
                self.abdtr(),
                self.absize(),
                self.dmode(),
                self.ddtr(),
                self.dqse()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiWpir(pub u32);
    impl OctospiWpir {
        #[must_use]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_instruction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OctospiWpir {
        #[inline(always)]
        fn default() -> OctospiWpir {
            OctospiWpir(0)
        }
    }
    impl core::fmt::Debug for OctospiWpir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiWpir")
                .field("instruction", &self.instruction())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiWpir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiWpir {{ instruction: {=u32:?} }}",
                self.instruction()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiWptcr(pub u32);
    impl OctospiWptcr {
        #[must_use]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dcyc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dhqc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dhqc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sshift(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sshift(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for OctospiWptcr {
        #[inline(always)]
        fn default() -> OctospiWptcr {
            OctospiWptcr(0)
        }
    }
    impl core::fmt::Debug for OctospiWptcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiWptcr")
                .field("dcyc", &self.dcyc())
                .field("dhqc", &self.dhqc())
                .field("sshift", &self.sshift())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiWptcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OctospiWptcr {{ dcyc: {=u8:?}, dhqc: {=bool:?}, sshift: {=bool:?} }}",
                self.dcyc(),
                self.dhqc(),
                self.sshift()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OctospiWtcr(pub u32);
    impl OctospiWtcr {
        #[must_use]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dcyc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for OctospiWtcr {
        #[inline(always)]
        fn default() -> OctospiWtcr {
            OctospiWtcr(0)
        }
    }
    impl core::fmt::Debug for OctospiWtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OctospiWtcr")
                .field("dcyc", &self.dcyc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OctospiWtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OctospiWtcr {{ dcyc: {=u8:?} }}", self.dcyc())
        }
    }
}
