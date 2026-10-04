#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccb {
    ptr: *mut u8,
}
unsafe impl Send for Ccb {}
unsafe impl Sync for Ccb {}
impl Ccb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::CcbCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::CcbSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn reftagr(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcbCr(pub u32);
    impl CcbCr {
        #[must_use]
        #[inline(always)]
        pub const fn ccop(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ccop(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iprst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_iprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CcbCr {
        #[inline(always)]
        fn default() -> CcbCr {
            CcbCr(0)
        }
    }
    impl core::fmt::Debug for CcbCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcbCr")
                .field("ccop", &self.ccop())
                .field("iprst", &self.iprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcbCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcbCr {{ ccop: {=u8:?}, iprst: {=bool:?} }}",
                self.ccop(),
                self.iprst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcbSr(pub u32);
    impl CcbSr {
        #[must_use]
        #[inline(always)]
        pub const fn opstep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_opstep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn operr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_operr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tamp_evt0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tamp_evt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tamp_evt1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tamp_evt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tamp_evt2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tamp_evt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tamp_evt3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tamp_evt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tamp_evt4(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tamp_evt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tamp_evt5(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tamp_evt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for CcbSr {
        #[inline(always)]
        fn default() -> CcbSr {
            CcbSr(0)
        }
    }
    impl core::fmt::Debug for CcbSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcbSr")
                .field("opstep", &self.opstep())
                .field("operr", &self.operr())
                .field("busy", &self.busy())
                .field("tamp_evt0", &self.tamp_evt0())
                .field("tamp_evt1", &self.tamp_evt1())
                .field("tamp_evt2", &self.tamp_evt2())
                .field("tamp_evt3", &self.tamp_evt3())
                .field("tamp_evt4", &self.tamp_evt4())
                .field("tamp_evt5", &self.tamp_evt5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcbSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcbSr {{ opstep: {=u8:?}, operr: {=u8:?}, busy: {=bool:?}, tamp_evt0: {=bool:?}, tamp_evt1: {=bool:?}, tamp_evt2: {=bool:?}, tamp_evt3: {=bool:?}, tamp_evt4: {=bool:?}, tamp_evt5: {=bool:?} }}",
                self.opstep(),
                self.operr(),
                self.busy(),
                self.tamp_evt0(),
                self.tamp_evt1(),
                self.tamp_evt2(),
                self.tamp_evt3(),
                self.tamp_evt4(),
                self.tamp_evt5()
            )
        }
    }
}
