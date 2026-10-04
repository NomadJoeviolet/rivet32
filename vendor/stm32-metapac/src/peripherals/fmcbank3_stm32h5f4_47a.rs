#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmcBank3 {
    ptr: *mut u8,
}
unsafe impl Send for FmcBank3 {}
unsafe impl Sync for FmcBank3 {}
impl FmcBank3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::FmcBank3Pcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::FmcBank3Sr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn pmem(self) -> crate::common::Reg<regs::FmcBank3Pmem, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn patt(self) -> crate::common::Reg<regs::FmcBank3Patt, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmcBank3Patt(pub u32);
    impl FmcBank3Patt {
        #[must_use]
        #[inline(always)]
        pub const fn attset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_attset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn attwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_attwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atthold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_atthold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atthiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_atthiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for FmcBank3Patt {
        #[inline(always)]
        fn default() -> FmcBank3Patt {
            FmcBank3Patt(0)
        }
    }
    impl core::fmt::Debug for FmcBank3Patt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmcBank3Patt")
                .field("attset", &self.attset())
                .field("attwait", &self.attwait())
                .field("atthold", &self.atthold())
                .field("atthiz", &self.atthiz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmcBank3Patt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmcBank3Patt {{ attset: {=u8:?}, attwait: {=u8:?}, atthold: {=u8:?}, atthiz: {=u8:?} }}",
                self.attset(),
                self.attwait(),
                self.atthold(),
                self.atthiz()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmcBank3Pcr(pub u32);
    impl FmcBank3Pcr {
        #[must_use]
        #[inline(always)]
        pub const fn pwaiten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pwaiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pbken(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ptyp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ptyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pwid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pwid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tclr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tar(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccps(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eccps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
    }
    impl Default for FmcBank3Pcr {
        #[inline(always)]
        fn default() -> FmcBank3Pcr {
            FmcBank3Pcr(0)
        }
    }
    impl core::fmt::Debug for FmcBank3Pcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmcBank3Pcr")
                .field("pwaiten", &self.pwaiten())
                .field("pbken", &self.pbken())
                .field("ptyp", &self.ptyp())
                .field("pwid", &self.pwid())
                .field("eccen", &self.eccen())
                .field("tclr", &self.tclr())
                .field("tar", &self.tar())
                .field("eccps", &self.eccps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmcBank3Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmcBank3Pcr {{ pwaiten: {=bool:?}, pbken: {=bool:?}, ptyp: {=bool:?}, pwid: {=u8:?}, eccen: {=bool:?}, tclr: {=u8:?}, tar: {=u8:?}, eccps: {=u8:?} }}",
                self.pwaiten(),
                self.pbken(),
                self.ptyp(),
                self.pwid(),
                self.eccen(),
                self.tclr(),
                self.tar(),
                self.eccps()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmcBank3Pmem(pub u32);
    impl FmcBank3Pmem {
        #[must_use]
        #[inline(always)]
        pub const fn memset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_memset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn memwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_memwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn memhold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_memhold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn memhiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_memhiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for FmcBank3Pmem {
        #[inline(always)]
        fn default() -> FmcBank3Pmem {
            FmcBank3Pmem(0)
        }
    }
    impl core::fmt::Debug for FmcBank3Pmem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmcBank3Pmem")
                .field("memset", &self.memset())
                .field("memwait", &self.memwait())
                .field("memhold", &self.memhold())
                .field("memhiz", &self.memhiz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmcBank3Pmem {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmcBank3Pmem {{ memset: {=u8:?}, memwait: {=u8:?}, memhold: {=u8:?}, memhiz: {=u8:?} }}",
                self.memset(),
                self.memwait(),
                self.memhold(),
                self.memhiz()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmcBank3Sr(pub u32);
    impl FmcBank3Sr {
        #[must_use]
        #[inline(always)]
        pub const fn irs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ils(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ils(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ifs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ifs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ifen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fempt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for FmcBank3Sr {
        #[inline(always)]
        fn default() -> FmcBank3Sr {
            FmcBank3Sr(0)
        }
    }
    impl core::fmt::Debug for FmcBank3Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmcBank3Sr")
                .field("irs", &self.irs())
                .field("ils", &self.ils())
                .field("ifs", &self.ifs())
                .field("iren", &self.iren())
                .field("ilen", &self.ilen())
                .field("ifen", &self.ifen())
                .field("fempt", &self.fempt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmcBank3Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmcBank3Sr {{ irs: {=bool:?}, ils: {=bool:?}, ifs: {=bool:?}, iren: {=bool:?}, ilen: {=bool:?}, ifen: {=bool:?}, fempt: {=bool:?} }}",
                self.irs(),
                self.ils(),
                self.ifs(),
                self.iren(),
                self.ilen(),
                self.ifen(),
                self.fempt()
            )
        }
    }
}
