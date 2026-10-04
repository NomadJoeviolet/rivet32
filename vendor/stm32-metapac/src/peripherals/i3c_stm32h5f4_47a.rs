#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c {
    ptr: *mut u8,
}
unsafe impl Send for I3c {}
unsafe impl Sync for I3c {}
impl I3c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::I3cCr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::I3cCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::I3cRdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn rdwr(self) -> crate::common::Reg<regs::I3cRdwr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::I3cTdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn tdwr(self) -> crate::common::Reg<regs::I3cTdwr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn ibidr(self) -> crate::common::Reg<regs::I3cIbidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn tgttdr(self) -> crate::common::Reg<regs::I3cTgttdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::I3cSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn ser(self) -> crate::common::Reg<regs::I3cSer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn rmr(self) -> crate::common::Reg<regs::I3cRmr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn evr(self) -> crate::common::Reg<regs::I3cEvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::I3cIer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn cevr(self) -> crate::common::Reg<regs::I3cCevr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::I3cMisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn devr0(self) -> crate::common::Reg<regs::I3cDevr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn devrx(self, n: usize) -> crate::common::Reg<regs::I3cDevrx, crate::common::Raw> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn maxrlr(self) -> crate::common::Reg<regs::I3cMaxrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn maxwlr(self) -> crate::common::Reg<regs::I3cMaxwlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn timingr0(self) -> crate::common::Reg<regs::I3cTimingr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn timingr1(self) -> crate::common::Reg<regs::I3cTimingr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn timingr2(self) -> crate::common::Reg<regs::I3cTimingr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn bcr(self) -> crate::common::Reg<regs::I3cBcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::I3cDcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn getcapr(self) -> crate::common::Reg<regs::I3cGetcapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn crcapr(self) -> crate::common::Reg<regs::I3cCrcapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn getmxdsr(self) -> crate::common::Reg<regs::I3cGetmxdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn epidr(self) -> crate::common::Reg<regs::I3cEpidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cBcr(pub u32);
    impl I3cBcr {
        #[must_use]
        #[inline(always)]
        pub const fn bcr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bcr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bcr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bcr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bcr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcr4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bcr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcr5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bcr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bcr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for I3cBcr {
        #[inline(always)]
        fn default() -> I3cBcr {
            I3cBcr(0)
        }
    }
    impl core::fmt::Debug for I3cBcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cBcr")
                .field("bcr", &self.bcr())
                .field("bcr0", &self.bcr0())
                .field("bcr1", &self.bcr1())
                .field("bcr2", &self.bcr2())
                .field("bcr3", &self.bcr3())
                .field("bcr4", &self.bcr4())
                .field("bcr5", &self.bcr5())
                .field("bcr6", &self.bcr6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cBcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cBcr {{ bcr: {=u8:?}, bcr0: {=bool:?}, bcr1: {=bool:?}, bcr2: {=bool:?}, bcr3: {=bool:?}, bcr4: {=bool:?}, bcr5: {=bool:?}, bcr6: {=bool:?} }}",
                self.bcr(),
                self.bcr0(),
                self.bcr1(),
                self.bcr2(),
                self.bcr3(),
                self.bcr4(),
                self.bcr5(),
                self.bcr6()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cCevr(pub u32);
    impl I3cCevr {
        #[must_use]
        #[inline(always)]
        pub const fn cfcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cfcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cerrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cibiendf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cibiendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ccrf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ccrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ccrupdf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ccrupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn chjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_chjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cwkpf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cwkpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cgetf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cgetf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cstaf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cstaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cdaupdf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cdaupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmwlupdf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmwlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmrlupdf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmrlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn casupdf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_casupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cintupdf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cintupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cdeff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cdeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cgrpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cgrpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for I3cCevr {
        #[inline(always)]
        fn default() -> I3cCevr {
            I3cCevr(0)
        }
    }
    impl core::fmt::Debug for I3cCevr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cCevr")
                .field("cfcf", &self.cfcf())
                .field("crxtgtendf", &self.crxtgtendf())
                .field("cerrf", &self.cerrf())
                .field("cibif", &self.cibif())
                .field("cibiendf", &self.cibiendf())
                .field("ccrf", &self.ccrf())
                .field("ccrupdf", &self.ccrupdf())
                .field("chjf", &self.chjf())
                .field("cwkpf", &self.cwkpf())
                .field("cgetf", &self.cgetf())
                .field("cstaf", &self.cstaf())
                .field("cdaupdf", &self.cdaupdf())
                .field("cmwlupdf", &self.cmwlupdf())
                .field("cmrlupdf", &self.cmrlupdf())
                .field("crstf", &self.crstf())
                .field("casupdf", &self.casupdf())
                .field("cintupdf", &self.cintupdf())
                .field("cdeff", &self.cdeff())
                .field("cgrpf", &self.cgrpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cCevr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cCevr {{ cfcf: {=bool:?}, crxtgtendf: {=bool:?}, cerrf: {=bool:?}, cibif: {=bool:?}, cibiendf: {=bool:?}, ccrf: {=bool:?}, ccrupdf: {=bool:?}, chjf: {=bool:?}, cwkpf: {=bool:?}, cgetf: {=bool:?}, cstaf: {=bool:?}, cdaupdf: {=bool:?}, cmwlupdf: {=bool:?}, cmrlupdf: {=bool:?}, crstf: {=bool:?}, casupdf: {=bool:?}, cintupdf: {=bool:?}, cdeff: {=bool:?}, cgrpf: {=bool:?} }}",
                self.cfcf(),
                self.crxtgtendf(),
                self.cerrf(),
                self.cibif(),
                self.cibiendf(),
                self.ccrf(),
                self.ccrupdf(),
                self.chjf(),
                self.cwkpf(),
                self.cgetf(),
                self.cstaf(),
                self.cdaupdf(),
                self.cmwlupdf(),
                self.cmrlupdf(),
                self.crstf(),
                self.casupdf(),
                self.cintupdf(),
                self.cdeff(),
                self.cgrpf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cCfgr(pub u32);
    impl I3cCfgr {
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crinit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn noarbh(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_noarbh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rstptrn(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rstptrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exitptrn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exitptrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hksdaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hksdaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hjack(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hjack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxflush(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxthres(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxthres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txflush(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txthres(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txthres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdmaen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sflush(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn smode(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_smode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tmode(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cdmaen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cflush(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fcfdis(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fcfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsfset(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsfset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for I3cCfgr {
        #[inline(always)]
        fn default() -> I3cCfgr {
            I3cCfgr(0)
        }
    }
    impl core::fmt::Debug for I3cCfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cCfgr")
                .field("en", &self.en())
                .field("crinit", &self.crinit())
                .field("noarbh", &self.noarbh())
                .field("rstptrn", &self.rstptrn())
                .field("exitptrn", &self.exitptrn())
                .field("hksdaen", &self.hksdaen())
                .field("hjack", &self.hjack())
                .field("rxdmaen", &self.rxdmaen())
                .field("rxflush", &self.rxflush())
                .field("rxthres", &self.rxthres())
                .field("txdmaen", &self.txdmaen())
                .field("txflush", &self.txflush())
                .field("txthres", &self.txthres())
                .field("sdmaen", &self.sdmaen())
                .field("sflush", &self.sflush())
                .field("smode", &self.smode())
                .field("tmode", &self.tmode())
                .field("cdmaen", &self.cdmaen())
                .field("cflush", &self.cflush())
                .field("fcfdis", &self.fcfdis())
                .field("tsfset", &self.tsfset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cCfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cCfgr {{ en: {=bool:?}, crinit: {=bool:?}, noarbh: {=bool:?}, rstptrn: {=bool:?}, exitptrn: {=bool:?}, hksdaen: {=bool:?}, hjack: {=bool:?}, rxdmaen: {=bool:?}, rxflush: {=bool:?}, rxthres: {=bool:?}, txdmaen: {=bool:?}, txflush: {=bool:?}, txthres: {=bool:?}, sdmaen: {=bool:?}, sflush: {=bool:?}, smode: {=bool:?}, tmode: {=bool:?}, cdmaen: {=bool:?}, cflush: {=bool:?}, fcfdis: {=bool:?}, tsfset: {=bool:?} }}",
                self.en(),
                self.crinit(),
                self.noarbh(),
                self.rstptrn(),
                self.exitptrn(),
                self.hksdaen(),
                self.hjack(),
                self.rxdmaen(),
                self.rxflush(),
                self.rxthres(),
                self.txdmaen(),
                self.txflush(),
                self.txthres(),
                self.sdmaen(),
                self.sflush(),
                self.smode(),
                self.tmode(),
                self.cdmaen(),
                self.cflush(),
                self.fcfdis(),
                self.tsfset()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cCr(pub u32);
    impl I3cCr {
        #[must_use]
        #[inline(always)]
        pub const fn dcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ccc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ccc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rnw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rnw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mtype(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mend(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for I3cCr {
        #[inline(always)]
        fn default() -> I3cCr {
            I3cCr(0)
        }
    }
    impl core::fmt::Debug for I3cCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cCr")
                .field("dcnt", &self.dcnt())
                .field("ccc", &self.ccc())
                .field("rnw", &self.rnw())
                .field("add", &self.add())
                .field("mtype", &self.mtype())
                .field("mend", &self.mend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cCr {{ dcnt: {=u16:?}, ccc: {=u8:?}, rnw: {=bool:?}, add: {=u8:?}, mtype: {=u8:?}, mend: {=bool:?} }}",
                self.dcnt(),
                self.ccc(),
                self.rnw(),
                self.add(),
                self.mtype(),
                self.mend()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cCrcapr(pub u32);
    impl I3cCrcapr {
        #[must_use]
        #[inline(always)]
        pub const fn capdhoff(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_capdhoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn capgrp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_capgrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for I3cCrcapr {
        #[inline(always)]
        fn default() -> I3cCrcapr {
            I3cCrcapr(0)
        }
    }
    impl core::fmt::Debug for I3cCrcapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cCrcapr")
                .field("capdhoff", &self.capdhoff())
                .field("capgrp", &self.capgrp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cCrcapr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cCrcapr {{ capdhoff: {=bool:?}, capgrp: {=bool:?} }}",
                self.capdhoff(),
                self.capgrp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cDcr(pub u32);
    impl I3cDcr {
        #[must_use]
        #[inline(always)]
        pub const fn dcr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for I3cDcr {
        #[inline(always)]
        fn default() -> I3cDcr {
            I3cDcr(0)
        }
    }
    impl core::fmt::Debug for I3cDcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cDcr").field("dcr", &self.dcr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cDcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "I3cDcr {{ dcr: {=u8:?} }}", self.dcr())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cDevr0(pub u32);
    impl I3cDevr0 {
        #[must_use]
        #[inline(always)]
        pub const fn daval(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_daval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cren(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hjen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hjen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn as_(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_as_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rstact(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rstact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rstval(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rstval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for I3cDevr0 {
        #[inline(always)]
        fn default() -> I3cDevr0 {
            I3cDevr0(0)
        }
    }
    impl core::fmt::Debug for I3cDevr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cDevr0")
                .field("daval", &self.daval())
                .field("da", &self.da())
                .field("ibien", &self.ibien())
                .field("cren", &self.cren())
                .field("hjen", &self.hjen())
                .field("as_", &self.as_())
                .field("rstact", &self.rstact())
                .field("rstval", &self.rstval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cDevr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cDevr0 {{ daval: {=bool:?}, da: {=u8:?}, ibien: {=bool:?}, cren: {=bool:?}, hjen: {=bool:?}, as_: {=u8:?}, rstact: {=u8:?}, rstval: {=bool:?} }}",
                self.daval(),
                self.da(),
                self.ibien(),
                self.cren(),
                self.hjen(),
                self.as_(),
                self.rstact(),
                self.rstval()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cDevrx(pub u32);
    impl I3cDevrx {
        #[must_use]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibiack(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibiack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crack(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibiden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibiden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for I3cDevrx {
        #[inline(always)]
        fn default() -> I3cDevrx {
            I3cDevrx(0)
        }
    }
    impl core::fmt::Debug for I3cDevrx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cDevrx")
                .field("da", &self.da())
                .field("ibiack", &self.ibiack())
                .field("crack", &self.crack())
                .field("ibiden", &self.ibiden())
                .field("susp", &self.susp())
                .field("dis", &self.dis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cDevrx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cDevrx {{ da: {=u8:?}, ibiack: {=bool:?}, crack: {=bool:?}, ibiden: {=bool:?}, susp: {=bool:?}, dis: {=bool:?} }}",
                self.da(),
                self.ibiack(),
                self.crack(),
                self.ibiden(),
                self.susp(),
                self.dis()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cEpidr(pub u32);
    impl I3cEpidr {
        #[must_use]
        #[inline(always)]
        pub const fn mipiid(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mipiid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idtsel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idtsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mipimid(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mipimid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
        }
    }
    impl Default for I3cEpidr {
        #[inline(always)]
        fn default() -> I3cEpidr {
            I3cEpidr(0)
        }
    }
    impl core::fmt::Debug for I3cEpidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cEpidr")
                .field("mipiid", &self.mipiid())
                .field("idtsel", &self.idtsel())
                .field("mipimid", &self.mipimid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cEpidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cEpidr {{ mipiid: {=u8:?}, idtsel: {=bool:?}, mipimid: {=u16:?} }}",
                self.mipiid(),
                self.idtsel(),
                self.mipimid()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cEvr(pub u32);
    impl I3cEvr {
        #[must_use]
        #[inline(always)]
        pub const fn cfef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cfnff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sfnef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfnff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfnef(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txlastf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxlastf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibiendf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibiendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crupdf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wkpf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wkpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn getf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_getf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn staf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_staf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn daupdf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_daupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mwlupdf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mwlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mrlupdf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mrlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asupdf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_asupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn intupdf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_intupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn deff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_deff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn grpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_grpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for I3cEvr {
        #[inline(always)]
        fn default() -> I3cEvr {
            I3cEvr(0)
        }
    }
    impl core::fmt::Debug for I3cEvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cEvr")
                .field("cfef", &self.cfef())
                .field("txfef", &self.txfef())
                .field("cfnff", &self.cfnff())
                .field("sfnef", &self.sfnef())
                .field("txfnff", &self.txfnff())
                .field("rxfnef", &self.rxfnef())
                .field("txlastf", &self.txlastf())
                .field("rxlastf", &self.rxlastf())
                .field("fcf", &self.fcf())
                .field("rxtgtendf", &self.rxtgtendf())
                .field("errf", &self.errf())
                .field("ibif", &self.ibif())
                .field("ibiendf", &self.ibiendf())
                .field("crf", &self.crf())
                .field("crupdf", &self.crupdf())
                .field("hjf", &self.hjf())
                .field("wkpf", &self.wkpf())
                .field("getf", &self.getf())
                .field("staf", &self.staf())
                .field("daupdf", &self.daupdf())
                .field("mwlupdf", &self.mwlupdf())
                .field("mrlupdf", &self.mrlupdf())
                .field("rstf", &self.rstf())
                .field("asupdf", &self.asupdf())
                .field("intupdf", &self.intupdf())
                .field("deff", &self.deff())
                .field("grpf", &self.grpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cEvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cEvr {{ cfef: {=bool:?}, txfef: {=bool:?}, cfnff: {=bool:?}, sfnef: {=bool:?}, txfnff: {=bool:?}, rxfnef: {=bool:?}, txlastf: {=bool:?}, rxlastf: {=bool:?}, fcf: {=bool:?}, rxtgtendf: {=bool:?}, errf: {=bool:?}, ibif: {=bool:?}, ibiendf: {=bool:?}, crf: {=bool:?}, crupdf: {=bool:?}, hjf: {=bool:?}, wkpf: {=bool:?}, getf: {=bool:?}, staf: {=bool:?}, daupdf: {=bool:?}, mwlupdf: {=bool:?}, mrlupdf: {=bool:?}, rstf: {=bool:?}, asupdf: {=bool:?}, intupdf: {=bool:?}, deff: {=bool:?}, grpf: {=bool:?} }}",
                self.cfef(),
                self.txfef(),
                self.cfnff(),
                self.sfnef(),
                self.txfnff(),
                self.rxfnef(),
                self.txlastf(),
                self.rxlastf(),
                self.fcf(),
                self.rxtgtendf(),
                self.errf(),
                self.ibif(),
                self.ibiendf(),
                self.crf(),
                self.crupdf(),
                self.hjf(),
                self.wkpf(),
                self.getf(),
                self.staf(),
                self.daupdf(),
                self.mwlupdf(),
                self.mrlupdf(),
                self.rstf(),
                self.asupdf(),
                self.intupdf(),
                self.deff(),
                self.grpf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cGetcapr(pub u32);
    impl I3cGetcapr {
        #[must_use]
        #[inline(always)]
        pub const fn cappend(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cappend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for I3cGetcapr {
        #[inline(always)]
        fn default() -> I3cGetcapr {
            I3cGetcapr(0)
        }
    }
    impl core::fmt::Debug for I3cGetcapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cGetcapr")
                .field("cappend", &self.cappend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cGetcapr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "I3cGetcapr {{ cappend: {=bool:?} }}", self.cappend())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cGetmxdsr(pub u32);
    impl I3cGetmxdsr {
        #[must_use]
        #[inline(always)]
        pub const fn hoffas(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hoffas(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fmt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rdturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsco(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for I3cGetmxdsr {
        #[inline(always)]
        fn default() -> I3cGetmxdsr {
            I3cGetmxdsr(0)
        }
    }
    impl core::fmt::Debug for I3cGetmxdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cGetmxdsr")
                .field("hoffas", &self.hoffas())
                .field("fmt", &self.fmt())
                .field("rdturn", &self.rdturn())
                .field("tsco", &self.tsco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cGetmxdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cGetmxdsr {{ hoffas: {=u8:?}, fmt: {=u8:?}, rdturn: {=u8:?}, tsco: {=bool:?} }}",
                self.hoffas(),
                self.fmt(),
                self.rdturn(),
                self.tsco()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cIbidr(pub u32);
    impl I3cIbidr {
        #[must_use]
        #[inline(always)]
        pub const fn ibid_bx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ibid_bx(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibidb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ibidb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibidb1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ibidb1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibidb2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ibidb2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibidb3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ibidb3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for I3cIbidr {
        #[inline(always)]
        fn default() -> I3cIbidr {
            I3cIbidr(0)
        }
    }
    impl core::fmt::Debug for I3cIbidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cIbidr")
                .field("ibid_bx", &self.ibid_bx())
                .field("ibidb0", &self.ibidb0())
                .field("ibidb1", &self.ibidb1())
                .field("ibidb2", &self.ibidb2())
                .field("ibidb3", &self.ibidb3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cIbidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cIbidr {{ ibid_bx: {=u32:?}, ibidb0: {=u8:?}, ibidb1: {=u8:?}, ibidb2: {=u8:?}, ibidb3: {=u8:?} }}",
                self.ibid_bx(),
                self.ibidb0(),
                self.ibidb1(),
                self.ibidb2(),
                self.ibidb3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cIer(pub u32);
    impl I3cIer {
        #[must_use]
        #[inline(always)]
        pub const fn cfnfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sfneie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfnfie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fcie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxtgtendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxtgtendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibiie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibiendie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibiendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crupdie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hjie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hjie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wkpie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wkpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn getie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_getie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn staie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_staie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn daupdie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_daupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mwlupdie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mwlupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mrlupdie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mrlupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asupdie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_asupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn intupdie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_intupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn defie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_defie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn grpie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_grpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for I3cIer {
        #[inline(always)]
        fn default() -> I3cIer {
            I3cIer(0)
        }
    }
    impl core::fmt::Debug for I3cIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cIer")
                .field("cfnfie", &self.cfnfie())
                .field("sfneie", &self.sfneie())
                .field("txfnfie", &self.txfnfie())
                .field("rxfneie", &self.rxfneie())
                .field("fcie", &self.fcie())
                .field("rxtgtendie", &self.rxtgtendie())
                .field("errie", &self.errie())
                .field("ibiie", &self.ibiie())
                .field("ibiendie", &self.ibiendie())
                .field("crie", &self.crie())
                .field("crupdie", &self.crupdie())
                .field("hjie", &self.hjie())
                .field("wkpie", &self.wkpie())
                .field("getie", &self.getie())
                .field("staie", &self.staie())
                .field("daupdie", &self.daupdie())
                .field("mwlupdie", &self.mwlupdie())
                .field("mrlupdie", &self.mrlupdie())
                .field("rstie", &self.rstie())
                .field("asupdie", &self.asupdie())
                .field("intupdie", &self.intupdie())
                .field("defie", &self.defie())
                .field("grpie", &self.grpie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cIer {{ cfnfie: {=bool:?}, sfneie: {=bool:?}, txfnfie: {=bool:?}, rxfneie: {=bool:?}, fcie: {=bool:?}, rxtgtendie: {=bool:?}, errie: {=bool:?}, ibiie: {=bool:?}, ibiendie: {=bool:?}, crie: {=bool:?}, crupdie: {=bool:?}, hjie: {=bool:?}, wkpie: {=bool:?}, getie: {=bool:?}, staie: {=bool:?}, daupdie: {=bool:?}, mwlupdie: {=bool:?}, mrlupdie: {=bool:?}, rstie: {=bool:?}, asupdie: {=bool:?}, intupdie: {=bool:?}, defie: {=bool:?}, grpie: {=bool:?} }}",
                self.cfnfie(),
                self.sfneie(),
                self.txfnfie(),
                self.rxfneie(),
                self.fcie(),
                self.rxtgtendie(),
                self.errie(),
                self.ibiie(),
                self.ibiendie(),
                self.crie(),
                self.crupdie(),
                self.hjie(),
                self.wkpie(),
                self.getie(),
                self.staie(),
                self.daupdie(),
                self.mwlupdie(),
                self.mrlupdie(),
                self.rstie(),
                self.asupdie(),
                self.intupdie(),
                self.defie(),
                self.grpie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cMaxrlr(pub u32);
    impl I3cMaxrlr {
        #[must_use]
        #[inline(always)]
        pub const fn mrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibip(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ibip(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for I3cMaxrlr {
        #[inline(always)]
        fn default() -> I3cMaxrlr {
            I3cMaxrlr(0)
        }
    }
    impl core::fmt::Debug for I3cMaxrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cMaxrlr")
                .field("mrl", &self.mrl())
                .field("ibip", &self.ibip())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cMaxrlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cMaxrlr {{ mrl: {=u16:?}, ibip: {=u8:?} }}",
                self.mrl(),
                self.ibip()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cMaxwlr(pub u32);
    impl I3cMaxwlr {
        #[must_use]
        #[inline(always)]
        pub const fn mwl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mwl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for I3cMaxwlr {
        #[inline(always)]
        fn default() -> I3cMaxwlr {
            I3cMaxwlr(0)
        }
    }
    impl core::fmt::Debug for I3cMaxwlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cMaxwlr")
                .field("mwl", &self.mwl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cMaxwlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "I3cMaxwlr {{ mwl: {=u16:?} }}", self.mwl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cMisr(pub u32);
    impl I3cMisr {
        #[must_use]
        #[inline(always)]
        pub const fn cfnfmis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cfnfmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sfnemis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sfnemis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfnfmis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfnfmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfnemis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfnemis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fcmis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fcmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxtgtendmis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxtgtendmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn errmis(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_errmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibimis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibimis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ibiendmis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ibiendmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crmis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crupdmis(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crupdmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hjmis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hjmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wkpmis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wkpmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn getmis(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_getmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stamis(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stamis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn daupdmis(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_daupdmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mwlupdmis(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mwlupdmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mrlupdmis(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mrlupdmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rstmis(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rstmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asupdmis(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_asupdmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn intupdmis(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_intupdmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn defmis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_defmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn grpmis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_grpmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for I3cMisr {
        #[inline(always)]
        fn default() -> I3cMisr {
            I3cMisr(0)
        }
    }
    impl core::fmt::Debug for I3cMisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cMisr")
                .field("cfnfmis", &self.cfnfmis())
                .field("sfnemis", &self.sfnemis())
                .field("txfnfmis", &self.txfnfmis())
                .field("rxfnemis", &self.rxfnemis())
                .field("fcmis", &self.fcmis())
                .field("rxtgtendmis", &self.rxtgtendmis())
                .field("errmis", &self.errmis())
                .field("ibimis", &self.ibimis())
                .field("ibiendmis", &self.ibiendmis())
                .field("crmis", &self.crmis())
                .field("crupdmis", &self.crupdmis())
                .field("hjmis", &self.hjmis())
                .field("wkpmis", &self.wkpmis())
                .field("getmis", &self.getmis())
                .field("stamis", &self.stamis())
                .field("daupdmis", &self.daupdmis())
                .field("mwlupdmis", &self.mwlupdmis())
                .field("mrlupdmis", &self.mrlupdmis())
                .field("rstmis", &self.rstmis())
                .field("asupdmis", &self.asupdmis())
                .field("intupdmis", &self.intupdmis())
                .field("defmis", &self.defmis())
                .field("grpmis", &self.grpmis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cMisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cMisr {{ cfnfmis: {=bool:?}, sfnemis: {=bool:?}, txfnfmis: {=bool:?}, rxfnemis: {=bool:?}, fcmis: {=bool:?}, rxtgtendmis: {=bool:?}, errmis: {=bool:?}, ibimis: {=bool:?}, ibiendmis: {=bool:?}, crmis: {=bool:?}, crupdmis: {=bool:?}, hjmis: {=bool:?}, wkpmis: {=bool:?}, getmis: {=bool:?}, stamis: {=bool:?}, daupdmis: {=bool:?}, mwlupdmis: {=bool:?}, mrlupdmis: {=bool:?}, rstmis: {=bool:?}, asupdmis: {=bool:?}, intupdmis: {=bool:?}, defmis: {=bool:?}, grpmis: {=bool:?} }}",
                self.cfnfmis(),
                self.sfnemis(),
                self.txfnfmis(),
                self.rxfnemis(),
                self.fcmis(),
                self.rxtgtendmis(),
                self.errmis(),
                self.ibimis(),
                self.ibiendmis(),
                self.crmis(),
                self.crupdmis(),
                self.hjmis(),
                self.wkpmis(),
                self.getmis(),
                self.stamis(),
                self.daupdmis(),
                self.mwlupdmis(),
                self.mrlupdmis(),
                self.rstmis(),
                self.asupdmis(),
                self.intupdmis(),
                self.defmis(),
                self.grpmis()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cRdr(pub u32);
    impl I3cRdr {
        #[must_use]
        #[inline(always)]
        pub const fn rdb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rdb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for I3cRdr {
        #[inline(always)]
        fn default() -> I3cRdr {
            I3cRdr(0)
        }
    }
    impl core::fmt::Debug for I3cRdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cRdr")
                .field("rdb0", &self.rdb0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cRdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "I3cRdr {{ rdb0: {=u8:?} }}", self.rdb0())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cRdwr(pub u32);
    impl I3cRdwr {
        #[must_use]
        #[inline(always)]
        pub const fn rd_bx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rd_bx(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rdb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdb1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rdb1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdb2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rdb2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdb3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rdb3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for I3cRdwr {
        #[inline(always)]
        fn default() -> I3cRdwr {
            I3cRdwr(0)
        }
    }
    impl core::fmt::Debug for I3cRdwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cRdwr")
                .field("rd_bx", &self.rd_bx())
                .field("rdb0", &self.rdb0())
                .field("rdb1", &self.rdb1())
                .field("rdb2", &self.rdb2())
                .field("rdb3", &self.rdb3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cRdwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cRdwr {{ rd_bx: {=u32:?}, rdb0: {=u8:?}, rdb1: {=u8:?}, rdb2: {=u8:?}, rdb3: {=u8:?} }}",
                self.rd_bx(),
                self.rdb0(),
                self.rdb1(),
                self.rdb2(),
                self.rdb3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cRmr(pub u32);
    impl I3cRmr {
        #[must_use]
        #[inline(always)]
        pub const fn ibirdcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ibirdcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn radd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_radd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
    }
    impl Default for I3cRmr {
        #[inline(always)]
        fn default() -> I3cRmr {
            I3cRmr(0)
        }
    }
    impl core::fmt::Debug for I3cRmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cRmr")
                .field("ibirdcnt", &self.ibirdcnt())
                .field("rcode", &self.rcode())
                .field("radd", &self.radd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cRmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cRmr {{ ibirdcnt: {=u8:?}, rcode: {=u8:?}, radd: {=u8:?} }}",
                self.ibirdcnt(),
                self.rcode(),
                self.radd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cSer(pub u32);
    impl I3cSer {
        #[must_use]
        #[inline(always)]
        pub const fn coderr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_coderr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dovr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn covr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_covr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn anack(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_anack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dnack(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dnack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn derr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_derr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for I3cSer {
        #[inline(always)]
        fn default() -> I3cSer {
            I3cSer(0)
        }
    }
    impl core::fmt::Debug for I3cSer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cSer")
                .field("coderr", &self.coderr())
                .field("perr", &self.perr())
                .field("stall", &self.stall())
                .field("dovr", &self.dovr())
                .field("covr", &self.covr())
                .field("anack", &self.anack())
                .field("dnack", &self.dnack())
                .field("derr", &self.derr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cSer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cSer {{ coderr: {=u8:?}, perr: {=bool:?}, stall: {=bool:?}, dovr: {=bool:?}, covr: {=bool:?}, anack: {=bool:?}, dnack: {=bool:?}, derr: {=bool:?} }}",
                self.coderr(),
                self.perr(),
                self.stall(),
                self.dovr(),
                self.covr(),
                self.anack(),
                self.dnack(),
                self.derr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cSr(pub u32);
    impl I3cSr {
        #[must_use]
        #[inline(always)]
        pub const fn xdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_xdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_abt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for I3cSr {
        #[inline(always)]
        fn default() -> I3cSr {
            I3cSr(0)
        }
    }
    impl core::fmt::Debug for I3cSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cSr")
                .field("xdcnt", &self.xdcnt())
                .field("abt", &self.abt())
                .field("dir", &self.dir())
                .field("mid", &self.mid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cSr {{ xdcnt: {=u16:?}, abt: {=bool:?}, dir: {=bool:?}, mid: {=u8:?} }}",
                self.xdcnt(),
                self.abt(),
                self.dir(),
                self.mid()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cTdr(pub u32);
    impl I3cTdr {
        #[must_use]
        #[inline(always)]
        pub const fn tdb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for I3cTdr {
        #[inline(always)]
        fn default() -> I3cTdr {
            I3cTdr(0)
        }
    }
    impl core::fmt::Debug for I3cTdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cTdr")
                .field("tdb0", &self.tdb0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cTdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "I3cTdr {{ tdb0: {=u8:?} }}", self.tdb0())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cTdwr(pub u32);
    impl I3cTdwr {
        #[must_use]
        #[inline(always)]
        pub const fn td_bx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_td_bx(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tdb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tdb1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdb1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tdb2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdb2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tdb3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdb3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for I3cTdwr {
        #[inline(always)]
        fn default() -> I3cTdwr {
            I3cTdwr(0)
        }
    }
    impl core::fmt::Debug for I3cTdwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cTdwr")
                .field("td_bx", &self.td_bx())
                .field("tdb0", &self.tdb0())
                .field("tdb1", &self.tdb1())
                .field("tdb2", &self.tdb2())
                .field("tdb3", &self.tdb3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cTdwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cTdwr {{ td_bx: {=u32:?}, tdb0: {=u8:?}, tdb1: {=u8:?}, tdb2: {=u8:?}, tdb3: {=u8:?} }}",
                self.td_bx(),
                self.tdb0(),
                self.tdb1(),
                self.tdb2(),
                self.tdb3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cTgttdr(pub u32);
    impl I3cTgttdr {
        #[must_use]
        #[inline(always)]
        pub const fn tgttdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_tgttdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn preload(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_preload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for I3cTgttdr {
        #[inline(always)]
        fn default() -> I3cTgttdr {
            I3cTgttdr(0)
        }
    }
    impl core::fmt::Debug for I3cTgttdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cTgttdr")
                .field("tgttdcnt", &self.tgttdcnt())
                .field("preload", &self.preload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cTgttdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cTgttdr {{ tgttdcnt: {=u16:?}, preload: {=bool:?} }}",
                self.tgttdcnt(),
                self.preload()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cTimingr0(pub u32);
    impl I3cTimingr0 {
        #[must_use]
        #[inline(always)]
        pub const fn scll_pp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_scll_pp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sclh_i3c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sclh_i3c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scll_od(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_scll_od(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sclh_i2c(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sclh_i2c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for I3cTimingr0 {
        #[inline(always)]
        fn default() -> I3cTimingr0 {
            I3cTimingr0(0)
        }
    }
    impl core::fmt::Debug for I3cTimingr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cTimingr0")
                .field("scll_pp", &self.scll_pp())
                .field("sclh_i3c", &self.sclh_i3c())
                .field("scll_od", &self.scll_od())
                .field("sclh_i2c", &self.sclh_i2c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cTimingr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cTimingr0 {{ scll_pp: {=u8:?}, sclh_i3c: {=u8:?}, scll_od: {=u8:?}, sclh_i2c: {=u8:?} }}",
                self.scll_pp(),
                self.sclh_i3c(),
                self.scll_od(),
                self.sclh_i2c()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cTimingr1(pub u32);
    impl I3cTimingr1 {
        #[must_use]
        #[inline(always)]
        pub const fn aval(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_aval(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asncr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_asncr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn free(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_free(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sda_hd(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sda_hd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for I3cTimingr1 {
        #[inline(always)]
        fn default() -> I3cTimingr1 {
            I3cTimingr1(0)
        }
    }
    impl core::fmt::Debug for I3cTimingr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cTimingr1")
                .field("aval", &self.aval())
                .field("asncr", &self.asncr())
                .field("free", &self.free())
                .field("sda_hd", &self.sda_hd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cTimingr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cTimingr1 {{ aval: {=u8:?}, asncr: {=u8:?}, free: {=u8:?}, sda_hd: {=u8:?} }}",
                self.aval(),
                self.asncr(),
                self.free(),
                self.sda_hd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cTimingr2(pub u32);
    impl I3cTimingr2 {
        #[must_use]
        #[inline(always)]
        pub const fn stallt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stallt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stalld(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stalld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stallc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stallc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stalla(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stalla(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stallr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stallr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stalls(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stalls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stalll(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_stalll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_stall(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for I3cTimingr2 {
        #[inline(always)]
        fn default() -> I3cTimingr2 {
            I3cTimingr2(0)
        }
    }
    impl core::fmt::Debug for I3cTimingr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cTimingr2")
                .field("stallt", &self.stallt())
                .field("stalld", &self.stalld())
                .field("stallc", &self.stallc())
                .field("stalla", &self.stalla())
                .field("stallr", &self.stallr())
                .field("stalls", &self.stalls())
                .field("stalll", &self.stalll())
                .field("stall", &self.stall())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cTimingr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cTimingr2 {{ stallt: {=bool:?}, stalld: {=bool:?}, stallc: {=bool:?}, stalla: {=bool:?}, stallr: {=bool:?}, stalls: {=bool:?}, stalll: {=bool:?}, stall: {=u8:?} }}",
                self.stallt(),
                self.stalld(),
                self.stallc(),
                self.stalla(),
                self.stallr(),
                self.stalls(),
                self.stalll(),
                self.stall()
            )
        }
    }
}
