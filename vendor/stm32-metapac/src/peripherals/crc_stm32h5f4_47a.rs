#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
unsafe impl Send for Crc {}
unsafe impl Sync for Crc {}
impl Crc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::CrcDr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<regs::CrcIdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::CrcCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn init(self) -> crate::common::Reg<regs::CrcInit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn pol(self) -> crate::common::Reg<regs::CrcPol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrcCr(pub u32);
    impl CrcCr {
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn polysize(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_polysize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rev_in(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rev_in(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rev_out(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rev_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rtype_in(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rtype_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rtype_out(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rtype_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for CrcCr {
        #[inline(always)]
        fn default() -> CrcCr {
            CrcCr(0)
        }
    }
    impl core::fmt::Debug for CrcCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CrcCr")
                .field("reset", &self.reset())
                .field("polysize", &self.polysize())
                .field("rev_in", &self.rev_in())
                .field("rev_out", &self.rev_out())
                .field("rtype_in", &self.rtype_in())
                .field("rtype_out", &self.rtype_out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CrcCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CrcCr {{ reset: {=bool:?}, polysize: {=u8:?}, rev_in: {=u8:?}, rev_out: {=u8:?}, rtype_in: {=bool:?}, rtype_out: {=bool:?} }}",
                self.reset(),
                self.polysize(),
                self.rev_in(),
                self.rev_out(),
                self.rtype_in(),
                self.rtype_out()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrcDr(pub u32);
    impl CrcDr {
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CrcDr {
        #[inline(always)]
        fn default() -> CrcDr {
            CrcDr(0)
        }
    }
    impl core::fmt::Debug for CrcDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CrcDr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CrcDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CrcDr {{ dr: {=u32:?} }}", self.dr())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrcIdr(pub u32);
    impl CrcIdr {
        #[must_use]
        #[inline(always)]
        pub const fn idr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_idr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CrcIdr {
        #[inline(always)]
        fn default() -> CrcIdr {
            CrcIdr(0)
        }
    }
    impl core::fmt::Debug for CrcIdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CrcIdr").field("idr", &self.idr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CrcIdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CrcIdr {{ idr: {=u32:?} }}", self.idr())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrcInit(pub u32);
    impl CrcInit {
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_init(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CrcInit {
        #[inline(always)]
        fn default() -> CrcInit {
            CrcInit(0)
        }
    }
    impl core::fmt::Debug for CrcInit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CrcInit")
                .field("init", &self.init())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CrcInit {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CrcInit {{ init: {=u32:?} }}", self.init())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrcPol(pub u32);
    impl CrcPol {
        #[must_use]
        #[inline(always)]
        pub const fn pol(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_pol(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CrcPol {
        #[inline(always)]
        fn default() -> CrcPol {
            CrcPol(0)
        }
    }
    impl core::fmt::Debug for CrcPol {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CrcPol").field("pol", &self.pol()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CrcPol {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CrcPol {{ pol: {=u32:?} }}", self.pol())
        }
    }
}
