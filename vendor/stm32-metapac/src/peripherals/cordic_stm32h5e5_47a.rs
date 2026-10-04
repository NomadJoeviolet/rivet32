#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cordic {
    ptr: *mut u8,
}
unsafe impl Send for Cordic {}
unsafe impl Sync for Cordic {}
impl Cordic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::CordicCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn wdata(self) -> crate::common::Reg<regs::CordicWdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn rdata(self) -> crate::common::Reg<regs::CordicRdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CordicCsr(pub u32);
    impl CordicCsr {
        #[must_use]
        #[inline(always)]
        pub const fn func(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_func(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn precision(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_precision(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scale(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_scale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaren(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmawen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmawen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nres(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nargs(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nargs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ressize(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ressize(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn argsize(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_argsize(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrdy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CordicCsr {
        #[inline(always)]
        fn default() -> CordicCsr {
            CordicCsr(0)
        }
    }
    impl core::fmt::Debug for CordicCsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CordicCsr")
                .field("func", &self.func())
                .field("precision", &self.precision())
                .field("scale", &self.scale())
                .field("ien", &self.ien())
                .field("dmaren", &self.dmaren())
                .field("dmawen", &self.dmawen())
                .field("nres", &self.nres())
                .field("nargs", &self.nargs())
                .field("ressize", &self.ressize())
                .field("argsize", &self.argsize())
                .field("rrdy", &self.rrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CordicCsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CordicCsr {{ func: {=u8:?}, precision: {=u8:?}, scale: {=u8:?}, ien: {=bool:?}, dmaren: {=bool:?}, dmawen: {=bool:?}, nres: {=bool:?}, nargs: {=bool:?}, ressize: {=bool:?}, argsize: {=bool:?}, rrdy: {=bool:?} }}",
                self.func(),
                self.precision(),
                self.scale(),
                self.ien(),
                self.dmaren(),
                self.dmawen(),
                self.nres(),
                self.nargs(),
                self.ressize(),
                self.argsize(),
                self.rrdy()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CordicRdata(pub u32);
    impl CordicRdata {
        #[must_use]
        #[inline(always)]
        pub const fn res(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_res(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CordicRdata {
        #[inline(always)]
        fn default() -> CordicRdata {
            CordicRdata(0)
        }
    }
    impl core::fmt::Debug for CordicRdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CordicRdata")
                .field("res", &self.res())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CordicRdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CordicRdata {{ res: {=u32:?} }}", self.res())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CordicWdata(pub u32);
    impl CordicWdata {
        #[must_use]
        #[inline(always)]
        pub const fn arg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_arg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CordicWdata {
        #[inline(always)]
        fn default() -> CordicWdata {
            CordicWdata(0)
        }
    }
    impl core::fmt::Debug for CordicWdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CordicWdata")
                .field("arg", &self.arg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CordicWdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CordicWdata {{ arg: {=u32:?} }}", self.arg())
        }
    }
}
