#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlyb {
    ptr: *mut u8,
}
unsafe impl Send for Dlyb {}
unsafe impl Sync for Dlyb {}
impl Dlyb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::DlybCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::DlybCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DlybCfgr(pub u32);
    impl DlybCfgr {
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn unit(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_unit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lng(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_lng(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lngf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lngf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DlybCfgr {
        #[inline(always)]
        fn default() -> DlybCfgr {
            DlybCfgr(0)
        }
    }
    impl core::fmt::Debug for DlybCfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DlybCfgr")
                .field("sel", &self.sel())
                .field("unit", &self.unit())
                .field("lng", &self.lng())
                .field("lngf", &self.lngf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DlybCfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DlybCfgr {{ sel: {=u8:?}, unit: {=u8:?}, lng: {=u16:?}, lngf: {=bool:?} }}",
                self.sel(),
                self.unit(),
                self.lng(),
                self.lngf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DlybCr(pub u32);
    impl DlybCr {
        #[must_use]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DlybCr {
        #[inline(always)]
        fn default() -> DlybCr {
            DlybCr(0)
        }
    }
    impl core::fmt::Debug for DlybCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DlybCr")
                .field("den", &self.den())
                .field("sen", &self.sen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DlybCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DlybCr {{ den: {=bool:?}, sen: {=bool:?} }}",
                self.den(),
                self.sen()
            )
        }
    }
}
