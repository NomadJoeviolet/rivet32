#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdg {
    ptr: *mut u8,
}
unsafe impl Send for Iwdg {}
unsafe impl Sync for Iwdg {}
impl Iwdg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn kr(self) -> crate::common::Reg<regs::IwdgKr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn pr(self) -> crate::common::Reg<regs::IwdgPr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn rlr(self) -> crate::common::Reg<regs::IwdgRlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::IwdgSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn winr(self) -> crate::common::Reg<regs::IwdgWinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn ewcr(self) -> crate::common::Reg<regs::IwdgEwcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwdgEwcr(pub u32);
    impl IwdgEwcr {
        #[must_use]
        #[inline(always)]
        pub const fn ewit(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ewit(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ewic(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ewic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ewie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ewie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IwdgEwcr {
        #[inline(always)]
        fn default() -> IwdgEwcr {
            IwdgEwcr(0)
        }
    }
    impl core::fmt::Debug for IwdgEwcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IwdgEwcr")
                .field("ewit", &self.ewit())
                .field("ewic", &self.ewic())
                .field("ewie", &self.ewie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IwdgEwcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IwdgEwcr {{ ewit: {=u16:?}, ewic: {=bool:?}, ewie: {=bool:?} }}",
                self.ewit(),
                self.ewic(),
                self.ewie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwdgKr(pub u32);
    impl IwdgKr {
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for IwdgKr {
        #[inline(always)]
        fn default() -> IwdgKr {
            IwdgKr(0)
        }
    }
    impl core::fmt::Debug for IwdgKr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IwdgKr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IwdgKr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IwdgKr {{ key: {=u16:?} }}", self.key())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwdgPr(pub u32);
    impl IwdgPr {
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for IwdgPr {
        #[inline(always)]
        fn default() -> IwdgPr {
            IwdgPr(0)
        }
    }
    impl core::fmt::Debug for IwdgPr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IwdgPr").field("pr", &self.pr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IwdgPr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IwdgPr {{ pr: {=u8:?} }}", self.pr())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwdgRlr(pub u32);
    impl IwdgRlr {
        #[must_use]
        #[inline(always)]
        pub const fn rl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for IwdgRlr {
        #[inline(always)]
        fn default() -> IwdgRlr {
            IwdgRlr(0)
        }
    }
    impl core::fmt::Debug for IwdgRlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IwdgRlr").field("rl", &self.rl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IwdgRlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IwdgRlr {{ rl: {=u16:?} }}", self.rl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwdgSr(pub u32);
    impl IwdgSr {
        #[must_use]
        #[inline(always)]
        pub const fn pvu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pvu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rvu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rvu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wvu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wvu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ewu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ewu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn onf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_onf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ewif(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ewif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for IwdgSr {
        #[inline(always)]
        fn default() -> IwdgSr {
            IwdgSr(0)
        }
    }
    impl core::fmt::Debug for IwdgSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IwdgSr")
                .field("pvu", &self.pvu())
                .field("rvu", &self.rvu())
                .field("wvu", &self.wvu())
                .field("ewu", &self.ewu())
                .field("onf", &self.onf())
                .field("ewif", &self.ewif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IwdgSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IwdgSr {{ pvu: {=bool:?}, rvu: {=bool:?}, wvu: {=bool:?}, ewu: {=bool:?}, onf: {=bool:?}, ewif: {=bool:?} }}",
                self.pvu(),
                self.rvu(),
                self.wvu(),
                self.ewu(),
                self.onf(),
                self.ewif()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwdgWinr(pub u32);
    impl IwdgWinr {
        #[must_use]
        #[inline(always)]
        pub const fn win(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_win(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for IwdgWinr {
        #[inline(always)]
        fn default() -> IwdgWinr {
            IwdgWinr(0)
        }
    }
    impl core::fmt::Debug for IwdgWinr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IwdgWinr")
                .field("win", &self.win())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IwdgWinr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IwdgWinr {{ win: {=u16:?} }}", self.win())
        }
    }
}
