#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jpeg {
    ptr: *mut u8,
}
unsafe impl Send for Jpeg {}
unsafe impl Sync for Jpeg {}
impl Jpeg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn confr0(self) -> crate::common::Reg<regs::JpegConfr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn confr1(self) -> crate::common::Reg<regs::JpegConfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn confr2(self) -> crate::common::Reg<regs::JpegConfr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn confr3(self) -> crate::common::Reg<regs::JpegConfr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn confr4(self) -> crate::common::Reg<regs::JpegConfr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn confr5(self) -> crate::common::Reg<regs::JpegConfr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn confr6(self) -> crate::common::Reg<regs::JpegConfr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn confr7(self) -> crate::common::Reg<regs::JpegConfr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn reserved20(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::JpegCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::JpegSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn cfr(self) -> crate::common::Reg<regs::JpegCfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved3c(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn dir(self) -> crate::common::Reg<regs::JpegDir, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn dor(self) -> crate::common::Reg<regs::JpegDor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved48(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn qmem0(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn qmem1(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn qmem2(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn qmem3(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn huffmin(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn huffbase(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn huffsymb(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 84usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn dhtmem(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 103usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn reserved4fc(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[inline(always)]
    pub const fn huffenc_ac0(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 88usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn huffenc_ac1(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 88usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn huffenc_dc0(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn huffenc_dc1(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07e0usize + n * 4usize) as _)
        }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegCfr(pub u32);
    impl JpegCfr {
        #[must_use]
        #[inline(always)]
        pub const fn ceocf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ceocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn chpdf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_chpdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for JpegCfr {
        #[inline(always)]
        fn default() -> JpegCfr {
            JpegCfr(0)
        }
    }
    impl core::fmt::Debug for JpegCfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegCfr")
                .field("ceocf", &self.ceocf())
                .field("chpdf", &self.chpdf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegCfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegCfr {{ ceocf: {=bool:?}, chpdf: {=bool:?} }}",
                self.ceocf(),
                self.chpdf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr0(pub u32);
    impl JpegConfr0 {
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for JpegConfr0 {
        #[inline(always)]
        fn default() -> JpegConfr0 {
            JpegConfr0(0)
        }
    }
    impl core::fmt::Debug for JpegConfr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr0")
                .field("start", &self.start())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegConfr0 {{ start: {=bool:?} }}", self.start())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr1(pub u32);
    impl JpegConfr1 {
        #[must_use]
        #[inline(always)]
        pub const fn nf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn de(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn colorspace(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_colorspace(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ns(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hdr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ysize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ysize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for JpegConfr1 {
        #[inline(always)]
        fn default() -> JpegConfr1 {
            JpegConfr1(0)
        }
    }
    impl core::fmt::Debug for JpegConfr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr1")
                .field("nf", &self.nf())
                .field("de", &self.de())
                .field("colorspace", &self.colorspace())
                .field("ns", &self.ns())
                .field("hdr", &self.hdr())
                .field("ysize", &self.ysize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr1 {{ nf: {=u8:?}, de: {=bool:?}, colorspace: {=u8:?}, ns: {=u8:?}, hdr: {=bool:?}, ysize: {=u16:?} }}",
                self.nf(),
                self.de(),
                self.colorspace(),
                self.ns(),
                self.hdr(),
                self.ysize()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr2(pub u32);
    impl JpegConfr2 {
        #[must_use]
        #[inline(always)]
        pub const fn nmcu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_nmcu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for JpegConfr2 {
        #[inline(always)]
        fn default() -> JpegConfr2 {
            JpegConfr2(0)
        }
    }
    impl core::fmt::Debug for JpegConfr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr2")
                .field("nmcu", &self.nmcu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegConfr2 {{ nmcu: {=u32:?} }}", self.nmcu())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr3(pub u32);
    impl JpegConfr3 {
        #[must_use]
        #[inline(always)]
        pub const fn xsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_xsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for JpegConfr3 {
        #[inline(always)]
        fn default() -> JpegConfr3 {
            JpegConfr3(0)
        }
    }
    impl core::fmt::Debug for JpegConfr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr3")
                .field("xsize", &self.xsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegConfr3 {{ xsize: {=u16:?} }}", self.xsize())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr4(pub u32);
    impl JpegConfr4 {
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr4 {
        #[inline(always)]
        fn default() -> JpegConfr4 {
            JpegConfr4(0)
        }
    }
    impl core::fmt::Debug for JpegConfr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr4")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nb", &self.nb())
                .field("vsf", &self.vsf())
                .field("hsf", &self.hsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr4 {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nb: {=u8:?}, vsf: {=u8:?}, hsf: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nb(),
                self.vsf(),
                self.hsf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr5(pub u32);
    impl JpegConfr5 {
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr5 {
        #[inline(always)]
        fn default() -> JpegConfr5 {
            JpegConfr5(0)
        }
    }
    impl core::fmt::Debug for JpegConfr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr5")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nb", &self.nb())
                .field("vsf", &self.vsf())
                .field("hsf", &self.hsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr5 {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nb: {=u8:?}, vsf: {=u8:?}, hsf: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nb(),
                self.vsf(),
                self.hsf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr6(pub u32);
    impl JpegConfr6 {
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr6 {
        #[inline(always)]
        fn default() -> JpegConfr6 {
            JpegConfr6(0)
        }
    }
    impl core::fmt::Debug for JpegConfr6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr6")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nb", &self.nb())
                .field("vsf", &self.vsf())
                .field("hsf", &self.hsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr6 {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nb: {=u8:?}, vsf: {=u8:?}, hsf: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nb(),
                self.vsf(),
                self.hsf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr7(pub u32);
    impl JpegConfr7 {
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr7 {
        #[inline(always)]
        fn default() -> JpegConfr7 {
            JpegConfr7(0)
        }
    }
    impl core::fmt::Debug for JpegConfr7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr7")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nb", &self.nb())
                .field("vsf", &self.vsf())
                .field("hsf", &self.hsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr7 {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nb: {=u8:?}, vsf: {=u8:?}, hsf: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nb(),
                self.vsf(),
                self.hsf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegCr(pub u32);
    impl JpegCr {
        #[must_use]
        #[inline(always)]
        pub const fn jcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_jcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iftie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_iftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ifnfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ifnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn oftie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_oftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ofneie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ofneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hpdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hpdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idmaen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn odmaen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_odmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iff(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_iff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn off(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_off(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for JpegCr {
        #[inline(always)]
        fn default() -> JpegCr {
            JpegCr(0)
        }
    }
    impl core::fmt::Debug for JpegCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegCr")
                .field("jcen", &self.jcen())
                .field("iftie", &self.iftie())
                .field("ifnfie", &self.ifnfie())
                .field("oftie", &self.oftie())
                .field("ofneie", &self.ofneie())
                .field("eocie", &self.eocie())
                .field("hpdie", &self.hpdie())
                .field("idmaen", &self.idmaen())
                .field("odmaen", &self.odmaen())
                .field("iff", &self.iff())
                .field("off", &self.off())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegCr {{ jcen: {=bool:?}, iftie: {=bool:?}, ifnfie: {=bool:?}, oftie: {=bool:?}, ofneie: {=bool:?}, eocie: {=bool:?}, hpdie: {=bool:?}, idmaen: {=bool:?}, odmaen: {=bool:?}, iff: {=bool:?}, off: {=bool:?} }}",
                self.jcen(),
                self.iftie(),
                self.ifnfie(),
                self.oftie(),
                self.ofneie(),
                self.eocie(),
                self.hpdie(),
                self.idmaen(),
                self.odmaen(),
                self.iff(),
                self.off()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegDir(pub u32);
    impl JpegDir {
        #[must_use]
        #[inline(always)]
        pub const fn datain(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_datain(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for JpegDir {
        #[inline(always)]
        fn default() -> JpegDir {
            JpegDir(0)
        }
    }
    impl core::fmt::Debug for JpegDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegDir")
                .field("datain", &self.datain())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegDir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegDir {{ datain: {=u32:?} }}", self.datain())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegDor(pub u32);
    impl JpegDor {
        #[must_use]
        #[inline(always)]
        pub const fn dataout(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_dataout(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for JpegDor {
        #[inline(always)]
        fn default() -> JpegDor {
            JpegDor(0)
        }
    }
    impl core::fmt::Debug for JpegDor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegDor")
                .field("dataout", &self.dataout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegDor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegDor {{ dataout: {=u32:?} }}", self.dataout())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegSr(pub u32);
    impl JpegSr {
        #[must_use]
        #[inline(always)]
        pub const fn iftf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_iftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ifnff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ifnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn oftf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_oftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ofnef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ofnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eocf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hpdf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hpdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cof(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for JpegSr {
        #[inline(always)]
        fn default() -> JpegSr {
            JpegSr(0)
        }
    }
    impl core::fmt::Debug for JpegSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegSr")
                .field("iftf", &self.iftf())
                .field("ifnff", &self.ifnff())
                .field("oftf", &self.oftf())
                .field("ofnef", &self.ofnef())
                .field("eocf", &self.eocf())
                .field("hpdf", &self.hpdf())
                .field("cof", &self.cof())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegSr {{ iftf: {=bool:?}, ifnff: {=bool:?}, oftf: {=bool:?}, ofnef: {=bool:?}, eocf: {=bool:?}, hpdf: {=bool:?}, cof: {=bool:?} }}",
                self.iftf(),
                self.ifnff(),
                self.oftf(),
                self.ofnef(),
                self.eocf(),
                self.hpdf(),
                self.cof()
            )
        }
    }
}
