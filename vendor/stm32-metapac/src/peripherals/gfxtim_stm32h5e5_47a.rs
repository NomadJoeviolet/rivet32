#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gfxtim {
    ptr: *mut u8,
}
unsafe impl Send for Gfxtim {}
unsafe impl Sync for Gfxtim {}
impl Gfxtim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::GfxtimCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cgcr(self) -> crate::common::Reg<regs::GfxtimCgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::GfxtimTcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::GfxtimTdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn evcr(self) -> crate::common::Reg<regs::GfxtimEvcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn evsr(self) -> crate::common::Reg<regs::GfxtimEvsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn wdgtcr(self) -> crate::common::Reg<regs::GfxtimWdgtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::GfxtimIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::GfxtimIcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::GfxtimIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn tsr(self) -> crate::common::Reg<regs::GfxtimTsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn lccrr(self) -> crate::common::Reg<regs::GfxtimLccrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn fccrr(self) -> crate::common::Reg<regs::GfxtimFccrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn atr(self) -> crate::common::Reg<regs::GfxtimAtr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn afcr(self) -> crate::common::Reg<regs::GfxtimAfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn alcr(self) -> crate::common::Reg<regs::GfxtimAlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn afcc1r(self) -> crate::common::Reg<regs::GfxtimAfcc1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn alcc1r(self) -> crate::common::Reg<regs::GfxtimAlcc1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn alcc2r(self) -> crate::common::Reg<regs::GfxtimAlcc2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn rfc1r(self) -> crate::common::Reg<regs::GfxtimRfc1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn rfc1rr(self) -> crate::common::Reg<regs::GfxtimRfc1rr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn rfc2r(self) -> crate::common::Reg<regs::GfxtimRfc2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn rfc2rr(self) -> crate::common::Reg<regs::GfxtimRfc2rr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn wdgcr(self) -> crate::common::Reg<regs::GfxtimWdgcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn wdgrr(self) -> crate::common::Reg<regs::GfxtimWdgrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn wdgpar(self) -> crate::common::Reg<regs::GfxtimWdgpar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn hwcfgr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[inline(always)]
    pub const fn verr(self) -> crate::common::Reg<regs::GfxtimVerr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[inline(always)]
    pub const fn ipidr(self) -> crate::common::Reg<regs::GfxtimIpidr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[inline(always)]
    pub const fn sidr(self) -> crate::common::Reg<regs::GfxtimSidr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimAfcc1r(pub u32);
    impl GfxtimAfcc1r {
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for GfxtimAfcc1r {
        #[inline(always)]
        fn default() -> GfxtimAfcc1r {
            GfxtimAfcc1r(0)
        }
    }
    impl core::fmt::Debug for GfxtimAfcc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimAfcc1r")
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimAfcc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimAfcc1r {{ frame: {=u32:?} }}", self.frame())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimAfcr(pub u32);
    impl GfxtimAfcr {
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for GfxtimAfcr {
        #[inline(always)]
        fn default() -> GfxtimAfcr {
            GfxtimAfcr(0)
        }
    }
    impl core::fmt::Debug for GfxtimAfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimAfcr")
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimAfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimAfcr {{ frame: {=u32:?} }}", self.frame())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimAlcc1r(pub u32);
    impl GfxtimAlcc1r {
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_line(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for GfxtimAlcc1r {
        #[inline(always)]
        fn default() -> GfxtimAlcc1r {
            GfxtimAlcc1r(0)
        }
    }
    impl core::fmt::Debug for GfxtimAlcc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimAlcc1r")
                .field("line", &self.line())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimAlcc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimAlcc1r {{ line: {=u16:?} }}", self.line())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimAlcc2r(pub u32);
    impl GfxtimAlcc2r {
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_line(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for GfxtimAlcc2r {
        #[inline(always)]
        fn default() -> GfxtimAlcc2r {
            GfxtimAlcc2r(0)
        }
    }
    impl core::fmt::Debug for GfxtimAlcc2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimAlcc2r")
                .field("line", &self.line())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimAlcc2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimAlcc2r {{ line: {=u16:?} }}", self.line())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimAlcr(pub u32);
    impl GfxtimAlcr {
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_line(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for GfxtimAlcr {
        #[inline(always)]
        fn default() -> GfxtimAlcr {
            GfxtimAlcr(0)
        }
    }
    impl core::fmt::Debug for GfxtimAlcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimAlcr")
                .field("line", &self.line())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimAlcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimAlcr {{ line: {=u16:?} }}", self.line())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimAtr(pub u32);
    impl GfxtimAtr {
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_line(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for GfxtimAtr {
        #[inline(always)]
        fn default() -> GfxtimAtr {
            GfxtimAtr(0)
        }
    }
    impl core::fmt::Debug for GfxtimAtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimAtr")
                .field("line", &self.line())
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimAtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimAtr {{ line: {=u16:?}, frame: {=u32:?} }}",
                self.line(),
                self.frame()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimCgcr(pub u32);
    impl GfxtimCgcr {
        #[must_use]
        #[inline(always)]
        pub const fn lcs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lcccs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lcccs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lccfr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lccfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lcchrs(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lcchrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fcs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fcccs(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fcccs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fccfr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fccfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fcchrs(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fcchrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for GfxtimCgcr {
        #[inline(always)]
        fn default() -> GfxtimCgcr {
            GfxtimCgcr(0)
        }
    }
    impl core::fmt::Debug for GfxtimCgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimCgcr")
                .field("lcs", &self.lcs())
                .field("lcccs", &self.lcccs())
                .field("lccfr", &self.lccfr())
                .field("lcchrs", &self.lcchrs())
                .field("fcs", &self.fcs())
                .field("fcccs", &self.fcccs())
                .field("fccfr", &self.fccfr())
                .field("fcchrs", &self.fcchrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimCgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimCgcr {{ lcs: {=u8:?}, lcccs: {=bool:?}, lccfr: {=bool:?}, lcchrs: {=u8:?}, fcs: {=u8:?}, fcccs: {=u8:?}, fccfr: {=bool:?}, fcchrs: {=u8:?} }}",
                self.lcs(),
                self.lcccs(),
                self.lccfr(),
                self.lcchrs(),
                self.fcs(),
                self.fcccs(),
                self.fccfr(),
                self.fcchrs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimCr(pub u32);
    impl GfxtimCr {
        #[must_use]
        #[inline(always)]
        pub const fn tes(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tepol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tepol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn syncs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_syncs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fccoe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fccoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lccoe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lccoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for GfxtimCr {
        #[inline(always)]
        fn default() -> GfxtimCr {
            GfxtimCr(0)
        }
    }
    impl core::fmt::Debug for GfxtimCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimCr")
                .field("tes", &self.tes())
                .field("tepol", &self.tepol())
                .field("syncs", &self.syncs())
                .field("fccoe", &self.fccoe())
                .field("lccoe", &self.lccoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimCr {{ tes: {=u8:?}, tepol: {=bool:?}, syncs: {=u8:?}, fccoe: {=bool:?}, lccoe: {=bool:?} }}",
                self.tes(),
                self.tepol(),
                self.syncs(),
                self.fccoe(),
                self.lccoe()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimEvcr(pub u32);
    impl GfxtimEvcr {
        #[must_use]
        #[inline(always)]
        pub const fn ev1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev3en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev4en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for GfxtimEvcr {
        #[inline(always)]
        fn default() -> GfxtimEvcr {
            GfxtimEvcr(0)
        }
    }
    impl core::fmt::Debug for GfxtimEvcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimEvcr")
                .field("ev1en", &self.ev1en())
                .field("ev2en", &self.ev2en())
                .field("ev3en", &self.ev3en())
                .field("ev4en", &self.ev4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimEvcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimEvcr {{ ev1en: {=bool:?}, ev2en: {=bool:?}, ev3en: {=bool:?}, ev4en: {=bool:?} }}",
                self.ev1en(),
                self.ev2en(),
                self.ev3en(),
                self.ev4en()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimEvsr(pub u32);
    impl GfxtimEvsr {
        #[must_use]
        #[inline(always)]
        pub const fn les1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_les1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fes1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fes1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn les2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_les2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fes2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fes2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn les3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_les3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fes3(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fes3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn les4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_les4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fes4(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fes4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for GfxtimEvsr {
        #[inline(always)]
        fn default() -> GfxtimEvsr {
            GfxtimEvsr(0)
        }
    }
    impl core::fmt::Debug for GfxtimEvsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimEvsr")
                .field("les1", &self.les1())
                .field("fes1", &self.fes1())
                .field("les2", &self.les2())
                .field("fes2", &self.fes2())
                .field("les3", &self.les3())
                .field("fes3", &self.fes3())
                .field("les4", &self.les4())
                .field("fes4", &self.fes4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimEvsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimEvsr {{ les1: {=u8:?}, fes1: {=u8:?}, les2: {=u8:?}, fes2: {=u8:?}, les3: {=u8:?}, fes3: {=u8:?}, les4: {=u8:?}, fes4: {=u8:?} }}",
                self.les1(),
                self.fes1(),
                self.les2(),
                self.fes2(),
                self.les3(),
                self.fes3(),
                self.les4(),
                self.fes4()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimFccrr(pub u32);
    impl GfxtimFccrr {
        #[must_use]
        #[inline(always)]
        pub const fn reload(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_reload(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for GfxtimFccrr {
        #[inline(always)]
        fn default() -> GfxtimFccrr {
            GfxtimFccrr(0)
        }
    }
    impl core::fmt::Debug for GfxtimFccrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimFccrr")
                .field("reload", &self.reload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimFccrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimFccrr {{ reload: {=u16:?} }}", self.reload())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimIcr(pub u32);
    impl GfxtimIcr {
        #[must_use]
        #[inline(always)]
        pub const fn cafcof(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cafcof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calcof(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calcof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cafcc1f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cafcc1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calcc1f(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calcc1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calcc2f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calcc2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crfc1rf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crfc1rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crfc2rf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crfc2rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cev1f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cev1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cev2f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cev2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cev3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cev3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cev4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cev4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cwdgaf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cwdgaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cwdgpf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cwdgpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for GfxtimIcr {
        #[inline(always)]
        fn default() -> GfxtimIcr {
            GfxtimIcr(0)
        }
    }
    impl core::fmt::Debug for GfxtimIcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimIcr")
                .field("cafcof", &self.cafcof())
                .field("calcof", &self.calcof())
                .field("ctef", &self.ctef())
                .field("cafcc1f", &self.cafcc1f())
                .field("calcc1f", &self.calcc1f())
                .field("calcc2f", &self.calcc2f())
                .field("crfc1rf", &self.crfc1rf())
                .field("crfc2rf", &self.crfc2rf())
                .field("cev1f", &self.cev1f())
                .field("cev2f", &self.cev2f())
                .field("cev3f", &self.cev3f())
                .field("cev4f", &self.cev4f())
                .field("cwdgaf", &self.cwdgaf())
                .field("cwdgpf", &self.cwdgpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimIcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimIcr {{ cafcof: {=bool:?}, calcof: {=bool:?}, ctef: {=bool:?}, cafcc1f: {=bool:?}, calcc1f: {=bool:?}, calcc2f: {=bool:?}, crfc1rf: {=bool:?}, crfc2rf: {=bool:?}, cev1f: {=bool:?}, cev2f: {=bool:?}, cev3f: {=bool:?}, cev4f: {=bool:?}, cwdgaf: {=bool:?}, cwdgpf: {=bool:?} }}",
                self.cafcof(),
                self.calcof(),
                self.ctef(),
                self.cafcc1f(),
                self.calcc1f(),
                self.calcc2f(),
                self.crfc1rf(),
                self.crfc2rf(),
                self.cev1f(),
                self.cev2f(),
                self.cev3f(),
                self.cev4f(),
                self.cwdgaf(),
                self.cwdgpf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimIer(pub u32);
    impl GfxtimIer {
        #[must_use]
        #[inline(always)]
        pub const fn afcoie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afcoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcoie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afcc1ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afcc1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcc1ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcc1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcc2ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcc2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc1rie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc1rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc2rie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc2rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev1ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev2ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev3ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev4ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdgaie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdgaie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdgpie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdgpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for GfxtimIer {
        #[inline(always)]
        fn default() -> GfxtimIer {
            GfxtimIer(0)
        }
    }
    impl core::fmt::Debug for GfxtimIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimIer")
                .field("afcoie", &self.afcoie())
                .field("alcoie", &self.alcoie())
                .field("teie", &self.teie())
                .field("afcc1ie", &self.afcc1ie())
                .field("alcc1ie", &self.alcc1ie())
                .field("alcc2ie", &self.alcc2ie())
                .field("rfc1rie", &self.rfc1rie())
                .field("rfc2rie", &self.rfc2rie())
                .field("ev1ie", &self.ev1ie())
                .field("ev2ie", &self.ev2ie())
                .field("ev3ie", &self.ev3ie())
                .field("ev4ie", &self.ev4ie())
                .field("wdgaie", &self.wdgaie())
                .field("wdgpie", &self.wdgpie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimIer {{ afcoie: {=bool:?}, alcoie: {=bool:?}, teie: {=bool:?}, afcc1ie: {=bool:?}, alcc1ie: {=bool:?}, alcc2ie: {=bool:?}, rfc1rie: {=bool:?}, rfc2rie: {=bool:?}, ev1ie: {=bool:?}, ev2ie: {=bool:?}, ev3ie: {=bool:?}, ev4ie: {=bool:?}, wdgaie: {=bool:?}, wdgpie: {=bool:?} }}",
                self.afcoie(),
                self.alcoie(),
                self.teie(),
                self.afcc1ie(),
                self.alcc1ie(),
                self.alcc2ie(),
                self.rfc1rie(),
                self.rfc2rie(),
                self.ev1ie(),
                self.ev2ie(),
                self.ev3ie(),
                self.ev4ie(),
                self.wdgaie(),
                self.wdgpie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimIpidr(pub u32);
    impl GfxtimIpidr {
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GfxtimIpidr {
        #[inline(always)]
        fn default() -> GfxtimIpidr {
            GfxtimIpidr(0)
        }
    }
    impl core::fmt::Debug for GfxtimIpidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimIpidr")
                .field("id", &self.id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimIpidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimIpidr {{ id: {=u32:?} }}", self.id())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimIsr(pub u32);
    impl GfxtimIsr {
        #[must_use]
        #[inline(always)]
        pub const fn afcof(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afcof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcof(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afcc1f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afcc1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcc1f(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcc1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcc2f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcc2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc1rf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc1rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc2rf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc2rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev1f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev2f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ev4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ev4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdgaf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdgaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdgpf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdgpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for GfxtimIsr {
        #[inline(always)]
        fn default() -> GfxtimIsr {
            GfxtimIsr(0)
        }
    }
    impl core::fmt::Debug for GfxtimIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimIsr")
                .field("afcof", &self.afcof())
                .field("alcof", &self.alcof())
                .field("tef", &self.tef())
                .field("afcc1f", &self.afcc1f())
                .field("alcc1f", &self.alcc1f())
                .field("alcc2f", &self.alcc2f())
                .field("rfc1rf", &self.rfc1rf())
                .field("rfc2rf", &self.rfc2rf())
                .field("ev1f", &self.ev1f())
                .field("ev2f", &self.ev2f())
                .field("ev3f", &self.ev3f())
                .field("ev4f", &self.ev4f())
                .field("wdgaf", &self.wdgaf())
                .field("wdgpf", &self.wdgpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimIsr {{ afcof: {=bool:?}, alcof: {=bool:?}, tef: {=bool:?}, afcc1f: {=bool:?}, alcc1f: {=bool:?}, alcc2f: {=bool:?}, rfc1rf: {=bool:?}, rfc2rf: {=bool:?}, ev1f: {=bool:?}, ev2f: {=bool:?}, ev3f: {=bool:?}, ev4f: {=bool:?}, wdgaf: {=bool:?}, wdgpf: {=bool:?} }}",
                self.afcof(),
                self.alcof(),
                self.tef(),
                self.afcc1f(),
                self.alcc1f(),
                self.alcc2f(),
                self.rfc1rf(),
                self.rfc2rf(),
                self.ev1f(),
                self.ev2f(),
                self.ev3f(),
                self.ev4f(),
                self.wdgaf(),
                self.wdgpf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimLccrr(pub u32);
    impl GfxtimLccrr {
        #[must_use]
        #[inline(always)]
        pub const fn reload(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_reload(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for GfxtimLccrr {
        #[inline(always)]
        fn default() -> GfxtimLccrr {
            GfxtimLccrr(0)
        }
    }
    impl core::fmt::Debug for GfxtimLccrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimLccrr")
                .field("reload", &self.reload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimLccrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimLccrr {{ reload: {=u32:?} }}", self.reload())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimRfc1r(pub u32);
    impl GfxtimRfc1r {
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for GfxtimRfc1r {
        #[inline(always)]
        fn default() -> GfxtimRfc1r {
            GfxtimRfc1r(0)
        }
    }
    impl core::fmt::Debug for GfxtimRfc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimRfc1r")
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimRfc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimRfc1r {{ frame: {=u16:?} }}", self.frame())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimRfc1rr(pub u32);
    impl GfxtimRfc1rr {
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for GfxtimRfc1rr {
        #[inline(always)]
        fn default() -> GfxtimRfc1rr {
            GfxtimRfc1rr(0)
        }
    }
    impl core::fmt::Debug for GfxtimRfc1rr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimRfc1rr")
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimRfc1rr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimRfc1rr {{ frame: {=u16:?} }}", self.frame())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimRfc2r(pub u32);
    impl GfxtimRfc2r {
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for GfxtimRfc2r {
        #[inline(always)]
        fn default() -> GfxtimRfc2r {
            GfxtimRfc2r(0)
        }
    }
    impl core::fmt::Debug for GfxtimRfc2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimRfc2r")
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimRfc2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimRfc2r {{ frame: {=u16:?} }}", self.frame())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimRfc2rr(pub u32);
    impl GfxtimRfc2rr {
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for GfxtimRfc2rr {
        #[inline(always)]
        fn default() -> GfxtimRfc2rr {
            GfxtimRfc2rr(0)
        }
    }
    impl core::fmt::Debug for GfxtimRfc2rr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimRfc2rr")
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimRfc2rr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimRfc2rr {{ frame: {=u16:?} }}", self.frame())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimSidr(pub u32);
    impl GfxtimSidr {
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GfxtimSidr {
        #[inline(always)]
        fn default() -> GfxtimSidr {
            GfxtimSidr(0)
        }
    }
    impl core::fmt::Debug for GfxtimSidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimSidr")
                .field("sid", &self.sid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimSidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimSidr {{ sid: {=u32:?} }}", self.sid())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimTcr(pub u32);
    impl GfxtimTcr {
        #[must_use]
        #[inline(always)]
        pub const fn afcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fafcr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fafcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn falcr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_falcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc1cm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc1cm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frfc1r(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frfc1r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc2en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc2cm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc2cm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frfc2r(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frfc2r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for GfxtimTcr {
        #[inline(always)]
        fn default() -> GfxtimTcr {
            GfxtimTcr(0)
        }
    }
    impl core::fmt::Debug for GfxtimTcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimTcr")
                .field("afcen", &self.afcen())
                .field("fafcr", &self.fafcr())
                .field("alcen", &self.alcen())
                .field("falcr", &self.falcr())
                .field("rfc1en", &self.rfc1en())
                .field("rfc1cm", &self.rfc1cm())
                .field("frfc1r", &self.frfc1r())
                .field("rfc2en", &self.rfc2en())
                .field("rfc2cm", &self.rfc2cm())
                .field("frfc2r", &self.frfc2r())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimTcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimTcr {{ afcen: {=bool:?}, fafcr: {=bool:?}, alcen: {=bool:?}, falcr: {=bool:?}, rfc1en: {=bool:?}, rfc1cm: {=bool:?}, frfc1r: {=bool:?}, rfc2en: {=bool:?}, rfc2cm: {=bool:?}, frfc2r: {=bool:?} }}",
                self.afcen(),
                self.fafcr(),
                self.alcen(),
                self.falcr(),
                self.rfc1en(),
                self.rfc1cm(),
                self.frfc1r(),
                self.rfc2en(),
                self.rfc2cm(),
                self.frfc2r()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimTdr(pub u32);
    impl GfxtimTdr {
        #[must_use]
        #[inline(always)]
        pub const fn afcdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afcdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc1dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc2dis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for GfxtimTdr {
        #[inline(always)]
        fn default() -> GfxtimTdr {
            GfxtimTdr(0)
        }
    }
    impl core::fmt::Debug for GfxtimTdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimTdr")
                .field("afcdis", &self.afcdis())
                .field("alcdis", &self.alcdis())
                .field("rfc1dis", &self.rfc1dis())
                .field("rfc2dis", &self.rfc2dis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimTdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimTdr {{ afcdis: {=bool:?}, alcdis: {=bool:?}, rfc1dis: {=bool:?}, rfc2dis: {=bool:?} }}",
                self.afcdis(),
                self.alcdis(),
                self.rfc1dis(),
                self.rfc2dis()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimTsr(pub u32);
    impl GfxtimTsr {
        #[must_use]
        #[inline(always)]
        pub const fn afcs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alcs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc1s(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfc2s(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfc2s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for GfxtimTsr {
        #[inline(always)]
        fn default() -> GfxtimTsr {
            GfxtimTsr(0)
        }
    }
    impl core::fmt::Debug for GfxtimTsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimTsr")
                .field("afcs", &self.afcs())
                .field("alcs", &self.alcs())
                .field("rfc1s", &self.rfc1s())
                .field("rfc2s", &self.rfc2s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimTsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimTsr {{ afcs: {=bool:?}, alcs: {=bool:?}, rfc1s: {=bool:?}, rfc2s: {=bool:?} }}",
                self.afcs(),
                self.alcs(),
                self.rfc1s(),
                self.rfc2s()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimVerr(pub u32);
    impl GfxtimVerr {
        #[must_use]
        #[inline(always)]
        pub const fn minrev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_minrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn majrev(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_majrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for GfxtimVerr {
        #[inline(always)]
        fn default() -> GfxtimVerr {
            GfxtimVerr(0)
        }
    }
    impl core::fmt::Debug for GfxtimVerr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimVerr")
                .field("minrev", &self.minrev())
                .field("majrev", &self.majrev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimVerr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimVerr {{ minrev: {=u8:?}, majrev: {=u8:?} }}",
                self.minrev(),
                self.majrev()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimWdgcr(pub u32);
    impl GfxtimWdgcr {
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_value(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for GfxtimWdgcr {
        #[inline(always)]
        fn default() -> GfxtimWdgcr {
            GfxtimWdgcr(0)
        }
    }
    impl core::fmt::Debug for GfxtimWdgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimWdgcr")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimWdgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimWdgcr {{ value: {=u16:?} }}", self.value())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimWdgpar(pub u32);
    impl GfxtimWdgpar {
        #[must_use]
        #[inline(always)]
        pub const fn prealarm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_prealarm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for GfxtimWdgpar {
        #[inline(always)]
        fn default() -> GfxtimWdgpar {
            GfxtimWdgpar(0)
        }
    }
    impl core::fmt::Debug for GfxtimWdgpar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimWdgpar")
                .field("prealarm", &self.prealarm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimWdgpar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimWdgpar {{ prealarm: {=u16:?} }}", self.prealarm())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimWdgrr(pub u32);
    impl GfxtimWdgrr {
        #[must_use]
        #[inline(always)]
        pub const fn reload(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_reload(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for GfxtimWdgrr {
        #[inline(always)]
        fn default() -> GfxtimWdgrr {
            GfxtimWdgrr(0)
        }
    }
    impl core::fmt::Debug for GfxtimWdgrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimWdgrr")
                .field("reload", &self.reload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimWdgrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GfxtimWdgrr {{ reload: {=u16:?} }}", self.reload())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfxtimWdgtcr(pub u32);
    impl GfxtimWdgtcr {
        #[must_use]
        #[inline(always)]
        pub const fn wdgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdgdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdgdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdgs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdgs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdghrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wdghrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdgcs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wdgcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fwdgr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fwdgr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for GfxtimWdgtcr {
        #[inline(always)]
        fn default() -> GfxtimWdgtcr {
            GfxtimWdgtcr(0)
        }
    }
    impl core::fmt::Debug for GfxtimWdgtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GfxtimWdgtcr")
                .field("wdgen", &self.wdgen())
                .field("wdgdis", &self.wdgdis())
                .field("wdgs", &self.wdgs())
                .field("wdghrc", &self.wdghrc())
                .field("wdgcs", &self.wdgcs())
                .field("fwdgr", &self.fwdgr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GfxtimWdgtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GfxtimWdgtcr {{ wdgen: {=bool:?}, wdgdis: {=bool:?}, wdgs: {=bool:?}, wdghrc: {=u8:?}, wdgcs: {=u8:?}, fwdgr: {=bool:?} }}",
                self.wdgen(),
                self.wdgdis(),
                self.wdgs(),
                self.wdghrc(),
                self.wdgcs(),
                self.fwdgr()
            )
        }
    }
}
