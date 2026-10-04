#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac {
    ptr: *mut u8,
}
unsafe impl Send for Dac {}
unsafe impl Sync for Dac {}
impl Dac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::DacCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn swtrigr(self) -> crate::common::Reg<regs::DacSwtrigr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn dhr12r1(self) -> crate::common::Reg<regs::DacDhr12r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn dhr12l1(self) -> crate::common::Reg<regs::DacDhr12l1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn dhr8r1(self) -> crate::common::Reg<regs::DacDhr8r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn dhr12r2(self) -> crate::common::Reg<regs::DacDhr12r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn dhr12l2(self) -> crate::common::Reg<regs::DacDhr12l2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn dhr8r2(self) -> crate::common::Reg<regs::DacDhr8r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn dhr12rd(self) -> crate::common::Reg<regs::DacDhr12rd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn dhr12ld(self) -> crate::common::Reg<regs::DacDhr12ld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn dhr8rd(self) -> crate::common::Reg<regs::DacDhr8rd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn dor1(self) -> crate::common::Reg<regs::DacDor1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn dor2(self) -> crate::common::Reg<regs::DacDor2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::DacSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::DacCcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::DacMcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn shsr1(self) -> crate::common::Reg<regs::DacShsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn shsr2(self) -> crate::common::Reg<regs::DacShsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn shhr(self) -> crate::common::Reg<regs::DacShhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn shrr(self) -> crate::common::Reg<regs::DacShrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacCcr(pub u32);
    impl DacCcr {
        #[must_use]
        #[inline(always)]
        pub const fn otrim1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_otrim1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn otrim2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_otrim2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for DacCcr {
        #[inline(always)]
        fn default() -> DacCcr {
            DacCcr(0)
        }
    }
    impl core::fmt::Debug for DacCcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacCcr")
                .field("otrim1", &self.otrim1())
                .field("otrim2", &self.otrim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacCcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacCcr {{ otrim1: {=u8:?}, otrim2: {=u8:?} }}",
                self.otrim1(),
                self.otrim2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacCr(pub u32);
    impl DacCr {
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ten1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsel1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wave1(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wave1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mamp1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mamp1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaen1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaudrie1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaudrie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cen1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ten2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsel2(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tsel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wave2(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wave2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mamp2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mamp2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaen2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaudrie2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaudrie2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cen2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for DacCr {
        #[inline(always)]
        fn default() -> DacCr {
            DacCr(0)
        }
    }
    impl core::fmt::Debug for DacCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacCr")
                .field("en1", &self.en1())
                .field("ten1", &self.ten1())
                .field("tsel1", &self.tsel1())
                .field("wave1", &self.wave1())
                .field("mamp1", &self.mamp1())
                .field("dmaen1", &self.dmaen1())
                .field("dmaudrie1", &self.dmaudrie1())
                .field("cen1", &self.cen1())
                .field("en2", &self.en2())
                .field("ten2", &self.ten2())
                .field("tsel2", &self.tsel2())
                .field("wave2", &self.wave2())
                .field("mamp2", &self.mamp2())
                .field("dmaen2", &self.dmaen2())
                .field("dmaudrie2", &self.dmaudrie2())
                .field("cen2", &self.cen2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacCr {{ en1: {=bool:?}, ten1: {=bool:?}, tsel1: {=u8:?}, wave1: {=u8:?}, mamp1: {=u8:?}, dmaen1: {=bool:?}, dmaudrie1: {=bool:?}, cen1: {=bool:?}, en2: {=bool:?}, ten2: {=bool:?}, tsel2: {=u8:?}, wave2: {=u8:?}, mamp2: {=u8:?}, dmaen2: {=bool:?}, dmaudrie2: {=bool:?}, cen2: {=bool:?} }}",
                self.en1(),
                self.ten1(),
                self.tsel1(),
                self.wave1(),
                self.mamp1(),
                self.dmaen1(),
                self.dmaudrie1(),
                self.cen1(),
                self.en2(),
                self.ten2(),
                self.tsel2(),
                self.wave2(),
                self.mamp2(),
                self.dmaen2(),
                self.dmaudrie2(),
                self.cen2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr12l1(pub u32);
    impl DacDhr12l1 {
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc1dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhrb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc1dhrb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for DacDhr12l1 {
        #[inline(always)]
        fn default() -> DacDhr12l1 {
            DacDhr12l1(0)
        }
    }
    impl core::fmt::Debug for DacDhr12l1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr12l1")
                .field("dacc1dhr", &self.dacc1dhr())
                .field("dacc1dhrb", &self.dacc1dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr12l1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr12l1 {{ dacc1dhr: {=u16:?}, dacc1dhrb: {=u16:?} }}",
                self.dacc1dhr(),
                self.dacc1dhrb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr12l2(pub u32);
    impl DacDhr12l2 {
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc2dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhrb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc2dhrb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for DacDhr12l2 {
        #[inline(always)]
        fn default() -> DacDhr12l2 {
            DacDhr12l2(0)
        }
    }
    impl core::fmt::Debug for DacDhr12l2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr12l2")
                .field("dacc2dhr", &self.dacc2dhr())
                .field("dacc2dhrb", &self.dacc2dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr12l2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr12l2 {{ dacc2dhr: {=u16:?}, dacc2dhrb: {=u16:?} }}",
                self.dacc2dhr(),
                self.dacc2dhrb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr12ld(pub u32);
    impl DacDhr12ld {
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc1dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc2dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for DacDhr12ld {
        #[inline(always)]
        fn default() -> DacDhr12ld {
            DacDhr12ld(0)
        }
    }
    impl core::fmt::Debug for DacDhr12ld {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr12ld")
                .field("dacc1dhr", &self.dacc1dhr())
                .field("dacc2dhr", &self.dacc2dhr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr12ld {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr12ld {{ dacc1dhr: {=u16:?}, dacc2dhr: {=u16:?} }}",
                self.dacc1dhr(),
                self.dacc2dhr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr12r1(pub u32);
    impl DacDhr12r1 {
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc1dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhrb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc1dhrb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for DacDhr12r1 {
        #[inline(always)]
        fn default() -> DacDhr12r1 {
            DacDhr12r1(0)
        }
    }
    impl core::fmt::Debug for DacDhr12r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr12r1")
                .field("dacc1dhr", &self.dacc1dhr())
                .field("dacc1dhrb", &self.dacc1dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr12r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr12r1 {{ dacc1dhr: {=u16:?}, dacc1dhrb: {=u16:?} }}",
                self.dacc1dhr(),
                self.dacc1dhrb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr12r2(pub u32);
    impl DacDhr12r2 {
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc2dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhrb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc2dhrb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for DacDhr12r2 {
        #[inline(always)]
        fn default() -> DacDhr12r2 {
            DacDhr12r2(0)
        }
    }
    impl core::fmt::Debug for DacDhr12r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr12r2")
                .field("dacc2dhr", &self.dacc2dhr())
                .field("dacc2dhrb", &self.dacc2dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr12r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr12r2 {{ dacc2dhr: {=u16:?}, dacc2dhrb: {=u16:?} }}",
                self.dacc2dhr(),
                self.dacc2dhrb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr12rd(pub u32);
    impl DacDhr12rd {
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc1dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc2dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for DacDhr12rd {
        #[inline(always)]
        fn default() -> DacDhr12rd {
            DacDhr12rd(0)
        }
    }
    impl core::fmt::Debug for DacDhr12rd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr12rd")
                .field("dacc1dhr", &self.dacc1dhr())
                .field("dacc2dhr", &self.dacc2dhr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr12rd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr12rd {{ dacc1dhr: {=u16:?}, dacc2dhr: {=u16:?} }}",
                self.dacc1dhr(),
                self.dacc2dhr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr8r1(pub u32);
    impl DacDhr8r1 {
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dacc1dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhrb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dacc1dhrb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for DacDhr8r1 {
        #[inline(always)]
        fn default() -> DacDhr8r1 {
            DacDhr8r1(0)
        }
    }
    impl core::fmt::Debug for DacDhr8r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr8r1")
                .field("dacc1dhr", &self.dacc1dhr())
                .field("dacc1dhrb", &self.dacc1dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr8r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr8r1 {{ dacc1dhr: {=u8:?}, dacc1dhrb: {=u8:?} }}",
                self.dacc1dhr(),
                self.dacc1dhrb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr8r2(pub u32);
    impl DacDhr8r2 {
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dacc2dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhrb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dacc2dhrb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for DacDhr8r2 {
        #[inline(always)]
        fn default() -> DacDhr8r2 {
            DacDhr8r2(0)
        }
    }
    impl core::fmt::Debug for DacDhr8r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr8r2")
                .field("dacc2dhr", &self.dacc2dhr())
                .field("dacc2dhrb", &self.dacc2dhrb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr8r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr8r2 {{ dacc2dhr: {=u8:?}, dacc2dhrb: {=u8:?} }}",
                self.dacc2dhr(),
                self.dacc2dhrb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDhr8rd(pub u32);
    impl DacDhr8rd {
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dacc1dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dacc2dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for DacDhr8rd {
        #[inline(always)]
        fn default() -> DacDhr8rd {
            DacDhr8rd(0)
        }
    }
    impl core::fmt::Debug for DacDhr8rd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDhr8rd")
                .field("dacc1dhr", &self.dacc1dhr())
                .field("dacc2dhr", &self.dacc2dhr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDhr8rd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDhr8rd {{ dacc1dhr: {=u8:?}, dacc2dhr: {=u8:?} }}",
                self.dacc1dhr(),
                self.dacc2dhr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDor1(pub u32);
    impl DacDor1 {
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dor(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc1dor(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc1dorb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc1dorb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for DacDor1 {
        #[inline(always)]
        fn default() -> DacDor1 {
            DacDor1(0)
        }
    }
    impl core::fmt::Debug for DacDor1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDor1")
                .field("dacc1dor", &self.dacc1dor())
                .field("dacc1dorb", &self.dacc1dorb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDor1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDor1 {{ dacc1dor: {=u16:?}, dacc1dorb: {=u16:?} }}",
                self.dacc1dor(),
                self.dacc1dorb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacDor2(pub u32);
    impl DacDor2 {
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dor(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc2dor(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dacc2dorb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dacc2dorb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for DacDor2 {
        #[inline(always)]
        fn default() -> DacDor2 {
            DacDor2(0)
        }
    }
    impl core::fmt::Debug for DacDor2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacDor2")
                .field("dacc2dor", &self.dacc2dor())
                .field("dacc2dorb", &self.dacc2dorb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacDor2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacDor2 {{ dacc2dor: {=u16:?}, dacc2dorb: {=u16:?} }}",
                self.dacc2dor(),
                self.dacc2dorb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacMcr(pub u32);
    impl DacMcr {
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmadouble1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmadouble1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sinformat1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sinformat1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hfsel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmadouble2(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmadouble2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sinformat2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sinformat2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for DacMcr {
        #[inline(always)]
        fn default() -> DacMcr {
            DacMcr(0)
        }
    }
    impl core::fmt::Debug for DacMcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacMcr")
                .field("mode1", &self.mode1())
                .field("dmadouble1", &self.dmadouble1())
                .field("sinformat1", &self.sinformat1())
                .field("hfsel", &self.hfsel())
                .field("mode2", &self.mode2())
                .field("dmadouble2", &self.dmadouble2())
                .field("sinformat2", &self.sinformat2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacMcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacMcr {{ mode1: {=u8:?}, dmadouble1: {=bool:?}, sinformat1: {=bool:?}, hfsel: {=u8:?}, mode2: {=u8:?}, dmadouble2: {=bool:?}, sinformat2: {=bool:?} }}",
                self.mode1(),
                self.dmadouble1(),
                self.sinformat1(),
                self.hfsel(),
                self.mode2(),
                self.dmadouble2(),
                self.sinformat2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacShhr(pub u32);
    impl DacShhr {
        #[must_use]
        #[inline(always)]
        pub const fn thold1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_thold1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn thold2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_thold2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for DacShhr {
        #[inline(always)]
        fn default() -> DacShhr {
            DacShhr(0)
        }
    }
    impl core::fmt::Debug for DacShhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacShhr")
                .field("thold1", &self.thold1())
                .field("thold2", &self.thold2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacShhr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacShhr {{ thold1: {=u16:?}, thold2: {=u16:?} }}",
                self.thold1(),
                self.thold2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacShrr(pub u32);
    impl DacShrr {
        #[must_use]
        #[inline(always)]
        pub const fn trefresh1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trefresh1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trefresh2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trefresh2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for DacShrr {
        #[inline(always)]
        fn default() -> DacShrr {
            DacShrr(0)
        }
    }
    impl core::fmt::Debug for DacShrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacShrr")
                .field("trefresh1", &self.trefresh1())
                .field("trefresh2", &self.trefresh2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacShrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacShrr {{ trefresh1: {=u8:?}, trefresh2: {=u8:?} }}",
                self.trefresh1(),
                self.trefresh2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacShsr1(pub u32);
    impl DacShsr1 {
        #[must_use]
        #[inline(always)]
        pub const fn tsample1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_tsample1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for DacShsr1 {
        #[inline(always)]
        fn default() -> DacShsr1 {
            DacShsr1(0)
        }
    }
    impl core::fmt::Debug for DacShsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacShsr1")
                .field("tsample1", &self.tsample1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacShsr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DacShsr1 {{ tsample1: {=u16:?} }}", self.tsample1())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacShsr2(pub u32);
    impl DacShsr2 {
        #[must_use]
        #[inline(always)]
        pub const fn tsample2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_tsample2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for DacShsr2 {
        #[inline(always)]
        fn default() -> DacShsr2 {
            DacShsr2(0)
        }
    }
    impl core::fmt::Debug for DacShsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacShsr2")
                .field("tsample2", &self.tsample2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacShsr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DacShsr2 {{ tsample2: {=u16:?} }}", self.tsample2())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacSr(pub u32);
    impl DacSr {
        #[must_use]
        #[inline(always)]
        pub const fn dac1rdy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dac1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dorstat1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dorstat1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaudr1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaudr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cal_flag1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cal_flag1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bwst1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bwst1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dac2rdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dac2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dorstat2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dorstat2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaudr2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaudr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cal_flag2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cal_flag2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bwst2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bwst2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DacSr {
        #[inline(always)]
        fn default() -> DacSr {
            DacSr(0)
        }
    }
    impl core::fmt::Debug for DacSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacSr")
                .field("dac1rdy", &self.dac1rdy())
                .field("dorstat1", &self.dorstat1())
                .field("dmaudr1", &self.dmaudr1())
                .field("cal_flag1", &self.cal_flag1())
                .field("bwst1", &self.bwst1())
                .field("dac2rdy", &self.dac2rdy())
                .field("dorstat2", &self.dorstat2())
                .field("dmaudr2", &self.dmaudr2())
                .field("cal_flag2", &self.cal_flag2())
                .field("bwst2", &self.bwst2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacSr {{ dac1rdy: {=bool:?}, dorstat1: {=bool:?}, dmaudr1: {=bool:?}, cal_flag1: {=bool:?}, bwst1: {=bool:?}, dac2rdy: {=bool:?}, dorstat2: {=bool:?}, dmaudr2: {=bool:?}, cal_flag2: {=bool:?}, bwst2: {=bool:?} }}",
                self.dac1rdy(),
                self.dorstat1(),
                self.dmaudr1(),
                self.cal_flag1(),
                self.bwst1(),
                self.dac2rdy(),
                self.dorstat2(),
                self.dmaudr2(),
                self.cal_flag2(),
                self.bwst2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacSwtrigr(pub u32);
    impl DacSwtrigr {
        #[must_use]
        #[inline(always)]
        pub const fn swtrig1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swtrig1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swtrig2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swtrig2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swtrigb1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swtrigb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swtrigb2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swtrigb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for DacSwtrigr {
        #[inline(always)]
        fn default() -> DacSwtrigr {
            DacSwtrigr(0)
        }
    }
    impl core::fmt::Debug for DacSwtrigr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacSwtrigr")
                .field("swtrig1", &self.swtrig1())
                .field("swtrig2", &self.swtrig2())
                .field("swtrigb1", &self.swtrigb1())
                .field("swtrigb2", &self.swtrigb2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacSwtrigr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DacSwtrigr {{ swtrig1: {=bool:?}, swtrig2: {=bool:?}, swtrigb1: {=bool:?}, swtrigb2: {=bool:?} }}",
                self.swtrig1(),
                self.swtrig2(),
                self.swtrigb1(),
                self.swtrigb2()
            )
        }
    }
}
