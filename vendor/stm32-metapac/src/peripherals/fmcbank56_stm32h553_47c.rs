#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmcBank56 {
    ptr: *mut u8,
}
unsafe impl Send for FmcBank56 {}
unsafe impl Sync for FmcBank56 {}
impl FmcBank56 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn sdcr(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn sdtr(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn sdcmr(self) -> crate::common::Reg<regs::Sdcmr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn sdrtr(self) -> crate::common::Reg<regs::Sdrtr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn sdsr(self) -> crate::common::Reg<regs::Sdsr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcmr(pub u32);
    impl Sdcmr {
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctb2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctb1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nrfs(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nrfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mrd(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mrd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 9usize)) | (((val as u32) & 0x1fff) << 9usize);
        }
    }
    impl Default for Sdcmr {
        #[inline(always)]
        fn default() -> Sdcmr {
            Sdcmr(0)
        }
    }
    impl core::fmt::Debug for Sdcmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdcmr")
                .field("mode", &self.mode())
                .field("ctb2", &self.ctb2())
                .field("ctb1", &self.ctb1())
                .field("nrfs", &self.nrfs())
                .field("mrd", &self.mrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdcmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdcmr {{ mode: {=u8:?}, ctb2: {=bool:?}, ctb1: {=bool:?}, nrfs: {=u8:?}, mrd: {=u16:?} }}",
                self.mode(),
                self.ctb2(),
                self.ctb1(),
                self.nrfs(),
                self.mrd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdrtr(pub u32);
    impl Sdrtr {
        #[must_use]
        #[inline(always)]
        pub const fn cre(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 1usize)) | (((val as u32) & 0x1fff) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn reie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_reie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sdrtr {
        #[inline(always)]
        fn default() -> Sdrtr {
            Sdrtr(0)
        }
    }
    impl core::fmt::Debug for Sdrtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdrtr")
                .field("cre", &self.cre())
                .field("count", &self.count())
                .field("reie", &self.reie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdrtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdrtr {{ cre: {=bool:?}, count: {=u16:?}, reie: {=bool:?} }}",
                self.cre(),
                self.count(),
                self.reie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdsr(pub u32);
    impl Sdsr {
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn modes1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_modes1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn modes2(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_modes2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
    }
    impl Default for Sdsr {
        #[inline(always)]
        fn default() -> Sdsr {
            Sdsr(0)
        }
    }
    impl core::fmt::Debug for Sdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdsr")
                .field("re", &self.re())
                .field("modes1", &self.modes1())
                .field("modes2", &self.modes2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdsr {{ re: {=bool:?}, modes1: {=u8:?}, modes2: {=u8:?} }}",
                self.re(),
                self.modes1(),
                self.modes2()
            )
        }
    }
}
