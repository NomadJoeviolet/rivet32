#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdf {
    ptr: *mut u8,
}
unsafe impl Send for Mdf {}
unsafe impl Sync for Mdf {}
impl Mdf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::MdfGcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ckgcr(self) -> crate::common::Reg<regs::MdfCkgcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfCkgcr(pub u32);
    impl MdfCkgcr {
        #[must_use]
        #[inline(always)]
        pub const fn ckgden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckgden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cck0en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cck0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cck1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cck1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ckgmod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckgmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cck0dir(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cck0dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cck1dir(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cck1dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trgsens(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_trgsens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trgsrc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trgsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cckdiv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cckdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn procdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_procdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ckgactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckgactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MdfCkgcr {
        #[inline(always)]
        fn default() -> MdfCkgcr {
            MdfCkgcr(0)
        }
    }
    impl core::fmt::Debug for MdfCkgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfCkgcr")
                .field("ckgden", &self.ckgden())
                .field("cck0en", &self.cck0en())
                .field("cck1en", &self.cck1en())
                .field("ckgmod", &self.ckgmod())
                .field("cck0dir", &self.cck0dir())
                .field("cck1dir", &self.cck1dir())
                .field("trgsens", &self.trgsens())
                .field("trgsrc", &self.trgsrc())
                .field("cckdiv", &self.cckdiv())
                .field("procdiv", &self.procdiv())
                .field("ckgactive", &self.ckgactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfCkgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfCkgcr {{ ckgden: {=bool:?}, cck0en: {=bool:?}, cck1en: {=bool:?}, ckgmod: {=bool:?}, cck0dir: {=bool:?}, cck1dir: {=bool:?}, trgsens: {=bool:?}, trgsrc: {=u8:?}, cckdiv: {=u8:?}, procdiv: {=u8:?}, ckgactive: {=bool:?} }}",
                self.ckgden(),
                self.cck0en(),
                self.cck1en(),
                self.ckgmod(),
                self.cck0dir(),
                self.cck1dir(),
                self.trgsens(),
                self.trgsrc(),
                self.cckdiv(),
                self.procdiv(),
                self.ckgactive()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfGcr(pub u32);
    impl MdfGcr {
        #[must_use]
        #[inline(always)]
        pub const fn trgo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ilvnb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ilvnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for MdfGcr {
        #[inline(always)]
        fn default() -> MdfGcr {
            MdfGcr(0)
        }
    }
    impl core::fmt::Debug for MdfGcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfGcr")
                .field("trgo", &self.trgo())
                .field("ilvnb", &self.ilvnb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfGcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfGcr {{ trgo: {=bool:?}, ilvnb: {=u8:?} }}",
                self.trgo(),
                self.ilvnb()
            )
        }
    }
}
