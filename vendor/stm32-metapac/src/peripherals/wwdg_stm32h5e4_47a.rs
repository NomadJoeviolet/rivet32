#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wwdg {
    ptr: *mut u8,
}
unsafe impl Send for Wwdg {}
unsafe impl Sync for Wwdg {}
impl Wwdg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::WwdgCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cfr(self) -> crate::common::Reg<regs::WwdgCfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::WwdgSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WwdgCfr(pub u32);
    impl WwdgCfr {
        #[must_use]
        #[inline(always)]
        pub const fn w(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_w(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ewi(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ewi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdgtb(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wdgtb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
    }
    impl Default for WwdgCfr {
        #[inline(always)]
        fn default() -> WwdgCfr {
            WwdgCfr(0)
        }
    }
    impl core::fmt::Debug for WwdgCfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WwdgCfr")
                .field("w", &self.w())
                .field("ewi", &self.ewi())
                .field("wdgtb", &self.wdgtb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WwdgCfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "WwdgCfr {{ w: {=u8:?}, ewi: {=bool:?}, wdgtb: {=u8:?} }}",
                self.w(),
                self.ewi(),
                self.wdgtb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WwdgCr(pub u32);
    impl WwdgCr {
        #[must_use]
        #[inline(always)]
        pub const fn t(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdga(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdga(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for WwdgCr {
        #[inline(always)]
        fn default() -> WwdgCr {
            WwdgCr(0)
        }
    }
    impl core::fmt::Debug for WwdgCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WwdgCr")
                .field("t", &self.t())
                .field("wdga", &self.wdga())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WwdgCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "WwdgCr {{ t: {=u8:?}, wdga: {=bool:?} }}",
                self.t(),
                self.wdga()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WwdgSr(pub u32);
    impl WwdgSr {
        #[must_use]
        #[inline(always)]
        pub const fn ewif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ewif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WwdgSr {
        #[inline(always)]
        fn default() -> WwdgSr {
            WwdgSr(0)
        }
    }
    impl core::fmt::Debug for WwdgSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WwdgSr")
                .field("ewif", &self.ewif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WwdgSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WwdgSr {{ ewif: {=bool:?} }}", self.ewif())
        }
    }
}
