#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmac {
    ptr: *mut u8,
}
unsafe impl Send for Fmac {}
unsafe impl Sync for Fmac {}
impl Fmac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn x1bufcfg(self) -> crate::common::Reg<regs::FmacX1bufcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn x2bufcfg(self) -> crate::common::Reg<regs::FmacX2bufcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn ybufcfg(self) -> crate::common::Reg<regs::FmacYbufcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::FmacParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::FmacCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::FmacSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn wdata(self) -> crate::common::Reg<regs::FmacWdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn rdata(self) -> crate::common::Reg<regs::FmacRdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmacCr(pub u32);
    impl FmacCr {
        #[must_use]
        #[inline(always)]
        pub const fn rien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ovflien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ovflien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn unflien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_unflien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn satien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_satien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaren(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmawen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmawen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clipen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for FmacCr {
        #[inline(always)]
        fn default() -> FmacCr {
            FmacCr(0)
        }
    }
    impl core::fmt::Debug for FmacCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmacCr")
                .field("rien", &self.rien())
                .field("wien", &self.wien())
                .field("ovflien", &self.ovflien())
                .field("unflien", &self.unflien())
                .field("satien", &self.satien())
                .field("dmaren", &self.dmaren())
                .field("dmawen", &self.dmawen())
                .field("clipen", &self.clipen())
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmacCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmacCr {{ rien: {=bool:?}, wien: {=bool:?}, ovflien: {=bool:?}, unflien: {=bool:?}, satien: {=bool:?}, dmaren: {=bool:?}, dmawen: {=bool:?}, clipen: {=bool:?}, reset: {=bool:?} }}",
                self.rien(),
                self.wien(),
                self.ovflien(),
                self.unflien(),
                self.satien(),
                self.dmaren(),
                self.dmawen(),
                self.clipen(),
                self.reset()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmacParam(pub u32);
    impl FmacParam {
        #[must_use]
        #[inline(always)]
        pub const fn p(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_p(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn q(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_q(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn r(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_r(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn func(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_func(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FmacParam {
        #[inline(always)]
        fn default() -> FmacParam {
            FmacParam(0)
        }
    }
    impl core::fmt::Debug for FmacParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmacParam")
                .field("p", &self.p())
                .field("q", &self.q())
                .field("r", &self.r())
                .field("func", &self.func())
                .field("start", &self.start())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmacParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmacParam {{ p: {=u8:?}, q: {=u8:?}, r: {=u8:?}, func: {=u8:?}, start: {=bool:?} }}",
                self.p(),
                self.q(),
                self.r(),
                self.func(),
                self.start()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmacRdata(pub u32);
    impl FmacRdata {
        #[must_use]
        #[inline(always)]
        pub const fn rdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for FmacRdata {
        #[inline(always)]
        fn default() -> FmacRdata {
            FmacRdata(0)
        }
    }
    impl core::fmt::Debug for FmacRdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmacRdata")
                .field("rdata", &self.rdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmacRdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FmacRdata {{ rdata: {=u16:?} }}", self.rdata())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmacSr(pub u32);
    impl FmacSr {
        #[must_use]
        #[inline(always)]
        pub const fn yempty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_yempty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn x1full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_x1full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ovfl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ovfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn unfl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_unfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sat(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for FmacSr {
        #[inline(always)]
        fn default() -> FmacSr {
            FmacSr(0)
        }
    }
    impl core::fmt::Debug for FmacSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmacSr")
                .field("yempty", &self.yempty())
                .field("x1full", &self.x1full())
                .field("ovfl", &self.ovfl())
                .field("unfl", &self.unfl())
                .field("sat", &self.sat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmacSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmacSr {{ yempty: {=bool:?}, x1full: {=bool:?}, ovfl: {=bool:?}, unfl: {=bool:?}, sat: {=bool:?} }}",
                self.yempty(),
                self.x1full(),
                self.ovfl(),
                self.unfl(),
                self.sat()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmacWdata(pub u32);
    impl FmacWdata {
        #[must_use]
        #[inline(always)]
        pub const fn wdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_wdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for FmacWdata {
        #[inline(always)]
        fn default() -> FmacWdata {
            FmacWdata(0)
        }
    }
    impl core::fmt::Debug for FmacWdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmacWdata")
                .field("wdata", &self.wdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmacWdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FmacWdata {{ wdata: {=u16:?} }}", self.wdata())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmacX1bufcfg(pub u32);
    impl FmacX1bufcfg {
        #[must_use]
        #[inline(always)]
        pub const fn x1_base(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_x1_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn x1_buf_size(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_x1_buf_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn full_wm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_full_wm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for FmacX1bufcfg {
        #[inline(always)]
        fn default() -> FmacX1bufcfg {
            FmacX1bufcfg(0)
        }
    }
    impl core::fmt::Debug for FmacX1bufcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmacX1bufcfg")
                .field("x1_base", &self.x1_base())
                .field("x1_buf_size", &self.x1_buf_size())
                .field("full_wm", &self.full_wm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmacX1bufcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmacX1bufcfg {{ x1_base: {=u8:?}, x1_buf_size: {=u8:?}, full_wm: {=u8:?} }}",
                self.x1_base(),
                self.x1_buf_size(),
                self.full_wm()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmacX2bufcfg(pub u32);
    impl FmacX2bufcfg {
        #[must_use]
        #[inline(always)]
        pub const fn x2_base(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_x2_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn x2_buf_size(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_x2_buf_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for FmacX2bufcfg {
        #[inline(always)]
        fn default() -> FmacX2bufcfg {
            FmacX2bufcfg(0)
        }
    }
    impl core::fmt::Debug for FmacX2bufcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmacX2bufcfg")
                .field("x2_base", &self.x2_base())
                .field("x2_buf_size", &self.x2_buf_size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmacX2bufcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmacX2bufcfg {{ x2_base: {=u8:?}, x2_buf_size: {=u8:?} }}",
                self.x2_base(),
                self.x2_buf_size()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmacYbufcfg(pub u32);
    impl FmacYbufcfg {
        #[must_use]
        #[inline(always)]
        pub const fn y_base(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_y_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn y_buf_size(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_y_buf_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn empty_wm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_empty_wm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for FmacYbufcfg {
        #[inline(always)]
        fn default() -> FmacYbufcfg {
            FmacYbufcfg(0)
        }
    }
    impl core::fmt::Debug for FmacYbufcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmacYbufcfg")
                .field("y_base", &self.y_base())
                .field("y_buf_size", &self.y_buf_size())
                .field("empty_wm", &self.empty_wm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmacYbufcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmacYbufcfg {{ y_base: {=u8:?}, y_buf_size: {=u8:?}, empty_wm: {=u8:?} }}",
                self.y_base(),
                self.y_buf_size(),
                self.empty_wm()
            )
        }
    }
}
