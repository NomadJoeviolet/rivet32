#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hash {
    ptr: *mut u8,
}
unsafe impl Send for Hash {}
unsafe impl Sync for Hash {}
impl Hash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::HashCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn din(self) -> crate::common::Reg<regs::HashDin, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn str(self) -> crate::common::Reg<regs::HashStr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn hra(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::HashImr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::HashSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn csr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 103usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashDigest {
    ptr: *mut u8,
}
unsafe impl Send for HashDigest {}
unsafe impl Sync for HashDigest {}
impl HashDigest {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn hr(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 42usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HashCr(pub u32);
    impl HashCr {
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmae(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn datatype(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_datatype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nbw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nbw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dinne(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dinne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mdmat(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mdmat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lkey(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lkey(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn algo(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_algo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
        }
    }
    impl Default for HashCr {
        #[inline(always)]
        fn default() -> HashCr {
            HashCr(0)
        }
    }
    impl core::fmt::Debug for HashCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HashCr")
                .field("init", &self.init())
                .field("dmae", &self.dmae())
                .field("datatype", &self.datatype())
                .field("mode", &self.mode())
                .field("nbw", &self.nbw())
                .field("dinne", &self.dinne())
                .field("mdmat", &self.mdmat())
                .field("lkey", &self.lkey())
                .field("algo", &self.algo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HashCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HashCr {{ init: {=bool:?}, dmae: {=bool:?}, datatype: {=u8:?}, mode: {=bool:?}, nbw: {=u8:?}, dinne: {=bool:?}, mdmat: {=bool:?}, lkey: {=bool:?}, algo: {=u8:?} }}",
                self.init(),
                self.dmae(),
                self.datatype(),
                self.mode(),
                self.nbw(),
                self.dinne(),
                self.mdmat(),
                self.lkey(),
                self.algo()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HashDin(pub u32);
    impl HashDin {
        #[must_use]
        #[inline(always)]
        pub const fn datain(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_datain(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HashDin {
        #[inline(always)]
        fn default() -> HashDin {
            HashDin(0)
        }
    }
    impl core::fmt::Debug for HashDin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HashDin")
                .field("datain", &self.datain())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HashDin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HashDin {{ datain: {=u32:?} }}", self.datain())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HashImr(pub u32);
    impl HashImr {
        #[must_use]
        #[inline(always)]
        pub const fn dinie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dinie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for HashImr {
        #[inline(always)]
        fn default() -> HashImr {
            HashImr(0)
        }
    }
    impl core::fmt::Debug for HashImr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HashImr")
                .field("dinie", &self.dinie())
                .field("dcie", &self.dcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HashImr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HashImr {{ dinie: {=bool:?}, dcie: {=bool:?} }}",
                self.dinie(),
                self.dcie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HashSr(pub u32);
    impl HashSr {
        #[must_use]
        #[inline(always)]
        pub const fn dinis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dinis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmas(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nbwp(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nbwp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dinne(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dinne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nbwe(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nbwe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for HashSr {
        #[inline(always)]
        fn default() -> HashSr {
            HashSr(0)
        }
    }
    impl core::fmt::Debug for HashSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HashSr")
                .field("dinis", &self.dinis())
                .field("dcis", &self.dcis())
                .field("dmas", &self.dmas())
                .field("busy", &self.busy())
                .field("nbwp", &self.nbwp())
                .field("dinne", &self.dinne())
                .field("nbwe", &self.nbwe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HashSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HashSr {{ dinis: {=bool:?}, dcis: {=bool:?}, dmas: {=bool:?}, busy: {=bool:?}, nbwp: {=u8:?}, dinne: {=bool:?}, nbwe: {=u8:?} }}",
                self.dinis(),
                self.dcis(),
                self.dmas(),
                self.busy(),
                self.nbwp(),
                self.dinne(),
                self.nbwe()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HashStr(pub u32);
    impl HashStr {
        #[must_use]
        #[inline(always)]
        pub const fn nblw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nblw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcal(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for HashStr {
        #[inline(always)]
        fn default() -> HashStr {
            HashStr(0)
        }
    }
    impl core::fmt::Debug for HashStr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HashStr")
                .field("nblw", &self.nblw())
                .field("dcal", &self.dcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HashStr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HashStr {{ nblw: {=u8:?}, dcal: {=bool:?} }}",
                self.nblw(),
                self.dcal()
            )
        }
    }
}
