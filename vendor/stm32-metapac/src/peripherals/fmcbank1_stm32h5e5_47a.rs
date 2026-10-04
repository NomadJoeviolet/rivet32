#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmcBank1 {
    ptr: *mut u8,
}
unsafe impl Send for FmcBank1 {}
unsafe impl Sync for FmcBank1 {}
impl FmcBank1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn btcr(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn pcscntr(self) -> crate::common::Reg<regs::FmcBank1Pcscntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmcBank1Pcscntr(pub u32);
    impl FmcBank1Pcscntr {
        #[must_use]
        #[inline(always)]
        pub const fn cscount(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_cscount(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntb1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntb1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntb2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntb2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntb3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntb3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntb4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntb4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for FmcBank1Pcscntr {
        #[inline(always)]
        fn default() -> FmcBank1Pcscntr {
            FmcBank1Pcscntr(0)
        }
    }
    impl core::fmt::Debug for FmcBank1Pcscntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmcBank1Pcscntr")
                .field("cscount", &self.cscount())
                .field("cntb1en", &self.cntb1en())
                .field("cntb2en", &self.cntb2en())
                .field("cntb3en", &self.cntb3en())
                .field("cntb4en", &self.cntb4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmcBank1Pcscntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmcBank1Pcscntr {{ cscount: {=u16:?}, cntb1en: {=bool:?}, cntb2en: {=bool:?}, cntb3en: {=bool:?}, cntb4en: {=bool:?} }}",
                self.cscount(),
                self.cntb1en(),
                self.cntb2en(),
                self.cntb3en(),
                self.cntb4en()
            )
        }
    }
}
