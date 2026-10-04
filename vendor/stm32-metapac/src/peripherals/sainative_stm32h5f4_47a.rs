#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai {
    ptr: *mut u8,
}
unsafe impl Send for Sai {}
unsafe impl Sync for Sai {}
impl Sai {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::SaiGcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn pdmcr(self) -> crate::common::Reg<regs::SaiPdmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn pdmdly(self) -> crate::common::Reg<regs::SaiPdmdly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaiBlock {
    ptr: *mut u8,
}
unsafe impl Send for SaiBlock {}
unsafe impl Sync for SaiBlock {}
impl SaiBlock {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn frcr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn slotr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn clrfr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SaiGcr(pub u32);
    impl SaiGcr {
        #[must_use]
        #[inline(always)]
        pub const fn syncin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_syncin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn syncout(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_syncout(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for SaiGcr {
        #[inline(always)]
        fn default() -> SaiGcr {
            SaiGcr(0)
        }
    }
    impl core::fmt::Debug for SaiGcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SaiGcr")
                .field("syncin", &self.syncin())
                .field("syncout", &self.syncout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SaiGcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SaiGcr {{ syncin: {=u8:?}, syncout: {=u8:?} }}",
                self.syncin(),
                self.syncout()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SaiPdmcr(pub u32);
    impl SaiPdmcr {
        #[must_use]
        #[inline(always)]
        pub const fn pdmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn micnbr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_micnbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cken1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cken1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cken2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cken2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cken3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cken3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cken4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cken4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for SaiPdmcr {
        #[inline(always)]
        fn default() -> SaiPdmcr {
            SaiPdmcr(0)
        }
    }
    impl core::fmt::Debug for SaiPdmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SaiPdmcr")
                .field("pdmen", &self.pdmen())
                .field("micnbr", &self.micnbr())
                .field("cken1", &self.cken1())
                .field("cken2", &self.cken2())
                .field("cken3", &self.cken3())
                .field("cken4", &self.cken4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SaiPdmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SaiPdmcr {{ pdmen: {=bool:?}, micnbr: {=u8:?}, cken1: {=bool:?}, cken2: {=bool:?}, cken3: {=bool:?}, cken4: {=bool:?} }}",
                self.pdmen(),
                self.micnbr(),
                self.cken1(),
                self.cken2(),
                self.cken3(),
                self.cken4()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SaiPdmdly(pub u32);
    impl SaiPdmdly {
        #[must_use]
        #[inline(always)]
        pub const fn dlym1l(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlym1l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlym1r(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlym1r(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlym2l(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlym2l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlym2r(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlym2r(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlym3l(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlym3l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlym3r(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlym3r(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlym4l(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlym4l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlym4r(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlym4r(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for SaiPdmdly {
        #[inline(always)]
        fn default() -> SaiPdmdly {
            SaiPdmdly(0)
        }
    }
    impl core::fmt::Debug for SaiPdmdly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SaiPdmdly")
                .field("dlym1l", &self.dlym1l())
                .field("dlym1r", &self.dlym1r())
                .field("dlym2l", &self.dlym2l())
                .field("dlym2r", &self.dlym2r())
                .field("dlym3l", &self.dlym3l())
                .field("dlym3r", &self.dlym3r())
                .field("dlym4l", &self.dlym4l())
                .field("dlym4r", &self.dlym4r())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SaiPdmdly {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SaiPdmdly {{ dlym1l: {=u8:?}, dlym1r: {=u8:?}, dlym2l: {=u8:?}, dlym2r: {=u8:?}, dlym3l: {=u8:?}, dlym3r: {=u8:?}, dlym4l: {=u8:?}, dlym4r: {=u8:?} }}",
                self.dlym1l(),
                self.dlym1r(),
                self.dlym2l(),
                self.dlym2r(),
                self.dlym3l(),
                self.dlym3r(),
                self.dlym4l(),
                self.dlym4r()
            )
        }
    }
}
