#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdcan {
    ptr: *mut u8,
}
unsafe impl Send for Fdcan {}
unsafe impl Sync for Fdcan {}
impl Fdcan {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn crel(self) -> crate::common::Reg<regs::FdcanCrel, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn endn(self) -> crate::common::Reg<regs::FdcanEndn, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn dbtp(self) -> crate::common::Reg<regs::FdcanDbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::FdcanTest, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn rwd(self) -> crate::common::Reg<regs::FdcanRwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::FdcanCccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn nbtp(self) -> crate::common::Reg<regs::FdcanNbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn tscc(self) -> crate::common::Reg<regs::FdcanTscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn tscv(self) -> crate::common::Reg<regs::FdcanTscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn tocc(self) -> crate::common::Reg<regs::FdcanTocc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn tocv(self) -> crate::common::Reg<regs::FdcanTocv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::FdcanEcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::FdcanPsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn tdcr(self) -> crate::common::Reg<regs::FdcanTdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::FdcanIr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<regs::FdcanIe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn ils(self) -> crate::common::Reg<regs::FdcanIls, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn ile(self) -> crate::common::Reg<regs::FdcanIle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn rxgfc(self) -> crate::common::Reg<regs::FdcanRxgfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn xidam(self) -> crate::common::Reg<regs::FdcanXidam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn hpms(self) -> crate::common::Reg<regs::FdcanHpms, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn rxf0s(self) -> crate::common::Reg<regs::FdcanRxf0s, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn rxf0a(self) -> crate::common::Reg<regs::FdcanRxf0a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn rxf1s(self) -> crate::common::Reg<regs::FdcanRxf1s, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn rxf1a(self) -> crate::common::Reg<regs::FdcanRxf1a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn txbc(self) -> crate::common::Reg<regs::FdcanTxbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn txfqs(self) -> crate::common::Reg<regs::FdcanTxfqs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn txbrp(self) -> crate::common::Reg<regs::FdcanTxbrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn txbar(self) -> crate::common::Reg<regs::FdcanTxbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn txbcr(self) -> crate::common::Reg<regs::FdcanTxbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn txbto(self) -> crate::common::Reg<regs::FdcanTxbto, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn txbcf(self) -> crate::common::Reg<regs::FdcanTxbcf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn txbtie(self) -> crate::common::Reg<regs::FdcanTxbtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn txbcie(self) -> crate::common::Reg<regs::FdcanTxbcie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn txefs(self) -> crate::common::Reg<regs::FdcanTxefs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn txefa(self) -> crate::common::Reg<regs::FdcanTxefa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanConfig {
    ptr: *mut u8,
}
unsafe impl Send for FdcanConfig {}
unsafe impl Sync for FdcanConfig {}
impl FdcanConfig {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ckdiv(self) -> crate::common::Reg<regs::FdcanConfigCkdiv, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn hwcfg(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[inline(always)]
    pub const fn verr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[inline(always)]
    pub const fn ipidr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[inline(always)]
    pub const fn sidr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanCccr(pub u32);
    impl FdcanCccr {
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_asm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn csr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_csr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn test(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fdoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn brse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_brse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pxhd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pxhd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn efbi(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_efbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn niso(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_niso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for FdcanCccr {
        #[inline(always)]
        fn default() -> FdcanCccr {
            FdcanCccr(0)
        }
    }
    impl core::fmt::Debug for FdcanCccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanCccr")
                .field("init", &self.init())
                .field("cce", &self.cce())
                .field("asm", &self.asm())
                .field("csa", &self.csa())
                .field("csr", &self.csr())
                .field("mon", &self.mon())
                .field("dar", &self.dar())
                .field("test", &self.test())
                .field("fdoe", &self.fdoe())
                .field("brse", &self.brse())
                .field("pxhd", &self.pxhd())
                .field("efbi", &self.efbi())
                .field("txp", &self.txp())
                .field("niso", &self.niso())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanCccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanCccr {{ init: {=bool:?}, cce: {=bool:?}, asm: {=bool:?}, csa: {=bool:?}, csr: {=bool:?}, mon: {=bool:?}, dar: {=bool:?}, test: {=bool:?}, fdoe: {=bool:?}, brse: {=bool:?}, pxhd: {=bool:?}, efbi: {=bool:?}, txp: {=bool:?}, niso: {=bool:?} }}",
                self.init(),
                self.cce(),
                self.asm(),
                self.csa(),
                self.csr(),
                self.mon(),
                self.dar(),
                self.test(),
                self.fdoe(),
                self.brse(),
                self.pxhd(),
                self.efbi(),
                self.txp(),
                self.niso()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanConfigCkdiv(pub u32);
    impl FdcanConfigCkdiv {
        #[must_use]
        #[inline(always)]
        pub const fn pdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for FdcanConfigCkdiv {
        #[inline(always)]
        fn default() -> FdcanConfigCkdiv {
            FdcanConfigCkdiv(0)
        }
    }
    impl core::fmt::Debug for FdcanConfigCkdiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanConfigCkdiv")
                .field("pdiv", &self.pdiv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanConfigCkdiv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanConfigCkdiv {{ pdiv: {=u8:?} }}", self.pdiv())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanCrel(pub u32);
    impl FdcanCrel {
        #[must_use]
        #[inline(always)]
        pub const fn day(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_day(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mon(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mon(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn year(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_year(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn substep(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_substep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn step(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_step(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for FdcanCrel {
        #[inline(always)]
        fn default() -> FdcanCrel {
            FdcanCrel(0)
        }
    }
    impl core::fmt::Debug for FdcanCrel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanCrel")
                .field("day", &self.day())
                .field("mon", &self.mon())
                .field("year", &self.year())
                .field("substep", &self.substep())
                .field("step", &self.step())
                .field("rel", &self.rel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanCrel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanCrel {{ day: {=u8:?}, mon: {=u8:?}, year: {=u8:?}, substep: {=u8:?}, step: {=u8:?}, rel: {=u8:?} }}",
                self.day(),
                self.mon(),
                self.year(),
                self.substep(),
                self.step(),
                self.rel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanDbtp(pub u32);
    impl FdcanDbtp {
        #[must_use]
        #[inline(always)]
        pub const fn dsjw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dtseg2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dtseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dtseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dtseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbrp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dbrp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tdc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for FdcanDbtp {
        #[inline(always)]
        fn default() -> FdcanDbtp {
            FdcanDbtp(0)
        }
    }
    impl core::fmt::Debug for FdcanDbtp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanDbtp")
                .field("dsjw", &self.dsjw())
                .field("dtseg2", &self.dtseg2())
                .field("dtseg1", &self.dtseg1())
                .field("dbrp", &self.dbrp())
                .field("tdc", &self.tdc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanDbtp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanDbtp {{ dsjw: {=u8:?}, dtseg2: {=u8:?}, dtseg1: {=u8:?}, dbrp: {=u8:?}, tdc: {=bool:?} }}",
                self.dsjw(),
                self.dtseg2(),
                self.dtseg1(),
                self.dbrp(),
                self.tdc()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanEcr(pub u32);
    impl FdcanEcr {
        #[must_use]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for FdcanEcr {
        #[inline(always)]
        fn default() -> FdcanEcr {
            FdcanEcr(0)
        }
    }
    impl core::fmt::Debug for FdcanEcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanEcr")
                .field("tec", &self.tec())
                .field("rec", &self.rec())
                .field("rp", &self.rp())
                .field("cel", &self.cel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanEcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanEcr {{ tec: {=u8:?}, rec: {=u8:?}, rp: {=bool:?}, cel: {=u8:?} }}",
                self.tec(),
                self.rec(),
                self.rp(),
                self.cel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanEndn(pub u32);
    impl FdcanEndn {
        #[must_use]
        #[inline(always)]
        pub const fn etv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_etv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FdcanEndn {
        #[inline(always)]
        fn default() -> FdcanEndn {
            FdcanEndn(0)
        }
    }
    impl core::fmt::Debug for FdcanEndn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanEndn")
                .field("etv", &self.etv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanEndn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanEndn {{ etv: {=u32:?} }}", self.etv())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanHpms(pub u32);
    impl FdcanHpms {
        #[must_use]
        #[inline(always)]
        pub const fn bidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msi(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_msi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for FdcanHpms {
        #[inline(always)]
        fn default() -> FdcanHpms {
            FdcanHpms(0)
        }
    }
    impl core::fmt::Debug for FdcanHpms {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanHpms")
                .field("bidx", &self.bidx())
                .field("msi", &self.msi())
                .field("fidx", &self.fidx())
                .field("flst", &self.flst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanHpms {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanHpms {{ bidx: {=u8:?}, msi: {=u8:?}, fidx: {=u8:?}, flst: {=bool:?} }}",
                self.bidx(),
                self.msi(),
                self.fidx(),
                self.flst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanIe(pub u32);
    impl FdcanIe {
        #[must_use]
        #[inline(always)]
        pub const fn rf0ne(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf0ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf0fe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf0fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf0le(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf0le(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf1ne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf1ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf1fe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf1fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf1le(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf1le(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hpme(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hpme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tcfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tcfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfee(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tfee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tefne(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tefne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn teffe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_teffe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tefle(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tefle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tswe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tswe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mrafe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mrafe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tooe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tooe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eloe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eloe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn epe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_epe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ewe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ewe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn boe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_boe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn peae(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_peae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pede(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn arae(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_arae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for FdcanIe {
        #[inline(always)]
        fn default() -> FdcanIe {
            FdcanIe(0)
        }
    }
    impl core::fmt::Debug for FdcanIe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanIe")
                .field("rf0ne", &self.rf0ne())
                .field("rf0fe", &self.rf0fe())
                .field("rf0le", &self.rf0le())
                .field("rf1ne", &self.rf1ne())
                .field("rf1fe", &self.rf1fe())
                .field("rf1le", &self.rf1le())
                .field("hpme", &self.hpme())
                .field("tce", &self.tce())
                .field("tcfe", &self.tcfe())
                .field("tfee", &self.tfee())
                .field("tefne", &self.tefne())
                .field("teffe", &self.teffe())
                .field("tefle", &self.tefle())
                .field("tswe", &self.tswe())
                .field("mrafe", &self.mrafe())
                .field("tooe", &self.tooe())
                .field("eloe", &self.eloe())
                .field("epe", &self.epe())
                .field("ewe", &self.ewe())
                .field("boe", &self.boe())
                .field("wdie", &self.wdie())
                .field("peae", &self.peae())
                .field("pede", &self.pede())
                .field("arae", &self.arae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanIe {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanIe {{ rf0ne: {=bool:?}, rf0fe: {=bool:?}, rf0le: {=bool:?}, rf1ne: {=bool:?}, rf1fe: {=bool:?}, rf1le: {=bool:?}, hpme: {=bool:?}, tce: {=bool:?}, tcfe: {=bool:?}, tfee: {=bool:?}, tefne: {=bool:?}, teffe: {=bool:?}, tefle: {=bool:?}, tswe: {=bool:?}, mrafe: {=bool:?}, tooe: {=bool:?}, eloe: {=bool:?}, epe: {=bool:?}, ewe: {=bool:?}, boe: {=bool:?}, wdie: {=bool:?}, peae: {=bool:?}, pede: {=bool:?}, arae: {=bool:?} }}",
                self.rf0ne(),
                self.rf0fe(),
                self.rf0le(),
                self.rf1ne(),
                self.rf1fe(),
                self.rf1le(),
                self.hpme(),
                self.tce(),
                self.tcfe(),
                self.tfee(),
                self.tefne(),
                self.teffe(),
                self.tefle(),
                self.tswe(),
                self.mrafe(),
                self.tooe(),
                self.eloe(),
                self.epe(),
                self.ewe(),
                self.boe(),
                self.wdie(),
                self.peae(),
                self.pede(),
                self.arae()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanIle(pub u32);
    impl FdcanIle {
        #[must_use]
        #[inline(always)]
        pub const fn eint0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eint0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eint1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eint1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for FdcanIle {
        #[inline(always)]
        fn default() -> FdcanIle {
            FdcanIle(0)
        }
    }
    impl core::fmt::Debug for FdcanIle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanIle")
                .field("eint0", &self.eint0())
                .field("eint1", &self.eint1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanIle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanIle {{ eint0: {=bool:?}, eint1: {=bool:?} }}",
                self.eint0(),
                self.eint1()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanIls(pub u32);
    impl FdcanIls {
        #[must_use]
        #[inline(always)]
        pub const fn rxfifo0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfifo0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfifo1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfifo1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn smsg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_smsg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tferr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tferr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn misc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_misc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for FdcanIls {
        #[inline(always)]
        fn default() -> FdcanIls {
            FdcanIls(0)
        }
    }
    impl core::fmt::Debug for FdcanIls {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanIls")
                .field("rxfifo0", &self.rxfifo0())
                .field("rxfifo1", &self.rxfifo1())
                .field("smsg", &self.smsg())
                .field("tferr", &self.tferr())
                .field("misc", &self.misc())
                .field("berr", &self.berr())
                .field("perr", &self.perr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanIls {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanIls {{ rxfifo0: {=bool:?}, rxfifo1: {=bool:?}, smsg: {=bool:?}, tferr: {=bool:?}, misc: {=bool:?}, berr: {=bool:?}, perr: {=bool:?} }}",
                self.rxfifo0(),
                self.rxfifo1(),
                self.smsg(),
                self.tferr(),
                self.misc(),
                self.berr(),
                self.perr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanIr(pub u32);
    impl FdcanIr {
        #[must_use]
        #[inline(always)]
        pub const fn rf0n(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf0n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf0f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf0l(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf0l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf1n(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf1n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf1f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf1l(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf1l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tefn(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tefn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn teff(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_teff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mraf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mraf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn too(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_too(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn elo(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_elo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdi(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pea(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ped(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ped(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ara(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ara(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for FdcanIr {
        #[inline(always)]
        fn default() -> FdcanIr {
            FdcanIr(0)
        }
    }
    impl core::fmt::Debug for FdcanIr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanIr")
                .field("rf0n", &self.rf0n())
                .field("rf0f", &self.rf0f())
                .field("rf0l", &self.rf0l())
                .field("rf1n", &self.rf1n())
                .field("rf1f", &self.rf1f())
                .field("rf1l", &self.rf1l())
                .field("hpm", &self.hpm())
                .field("tc", &self.tc())
                .field("tcf", &self.tcf())
                .field("tfe", &self.tfe())
                .field("tefn", &self.tefn())
                .field("teff", &self.teff())
                .field("tefl", &self.tefl())
                .field("tsw", &self.tsw())
                .field("mraf", &self.mraf())
                .field("too", &self.too())
                .field("elo", &self.elo())
                .field("ep", &self.ep())
                .field("ew", &self.ew())
                .field("bo", &self.bo())
                .field("wdi", &self.wdi())
                .field("pea", &self.pea())
                .field("ped", &self.ped())
                .field("ara", &self.ara())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanIr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanIr {{ rf0n: {=bool:?}, rf0f: {=bool:?}, rf0l: {=bool:?}, rf1n: {=bool:?}, rf1f: {=bool:?}, rf1l: {=bool:?}, hpm: {=bool:?}, tc: {=bool:?}, tcf: {=bool:?}, tfe: {=bool:?}, tefn: {=bool:?}, teff: {=bool:?}, tefl: {=bool:?}, tsw: {=bool:?}, mraf: {=bool:?}, too: {=bool:?}, elo: {=bool:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, wdi: {=bool:?}, pea: {=bool:?}, ped: {=bool:?}, ara: {=bool:?} }}",
                self.rf0n(),
                self.rf0f(),
                self.rf0l(),
                self.rf1n(),
                self.rf1f(),
                self.rf1l(),
                self.hpm(),
                self.tc(),
                self.tcf(),
                self.tfe(),
                self.tefn(),
                self.teff(),
                self.tefl(),
                self.tsw(),
                self.mraf(),
                self.too(),
                self.elo(),
                self.ep(),
                self.ew(),
                self.bo(),
                self.wdi(),
                self.pea(),
                self.ped(),
                self.ara()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanNbtp(pub u32);
    impl FdcanNbtp {
        #[must_use]
        #[inline(always)]
        pub const fn ntseg2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ntseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ntseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ntseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nbrp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_nbrp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nsjw(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for FdcanNbtp {
        #[inline(always)]
        fn default() -> FdcanNbtp {
            FdcanNbtp(0)
        }
    }
    impl core::fmt::Debug for FdcanNbtp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanNbtp")
                .field("ntseg2", &self.ntseg2())
                .field("ntseg1", &self.ntseg1())
                .field("nbrp", &self.nbrp())
                .field("nsjw", &self.nsjw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanNbtp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanNbtp {{ ntseg2: {=u8:?}, ntseg1: {=u8:?}, nbrp: {=u16:?}, nsjw: {=u8:?} }}",
                self.ntseg2(),
                self.ntseg1(),
                self.nbrp(),
                self.nsjw()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanPsr(pub u32);
    impl FdcanPsr {
        #[must_use]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn act(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_act(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn resi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_resi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbrs(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn redl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_redl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pxe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tdcv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdcv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for FdcanPsr {
        #[inline(always)]
        fn default() -> FdcanPsr {
            FdcanPsr(0)
        }
    }
    impl core::fmt::Debug for FdcanPsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanPsr")
                .field("lec", &self.lec())
                .field("act", &self.act())
                .field("ep", &self.ep())
                .field("ew", &self.ew())
                .field("bo", &self.bo())
                .field("dlec", &self.dlec())
                .field("resi", &self.resi())
                .field("rbrs", &self.rbrs())
                .field("redl", &self.redl())
                .field("pxe", &self.pxe())
                .field("tdcv", &self.tdcv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanPsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanPsr {{ lec: {=u8:?}, act: {=u8:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, dlec: {=u8:?}, resi: {=bool:?}, rbrs: {=bool:?}, redl: {=bool:?}, pxe: {=bool:?}, tdcv: {=u8:?} }}",
                self.lec(),
                self.act(),
                self.ep(),
                self.ew(),
                self.bo(),
                self.dlec(),
                self.resi(),
                self.rbrs(),
                self.redl(),
                self.pxe(),
                self.tdcv()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanRwd(pub u32);
    impl FdcanRwd {
        #[must_use]
        #[inline(always)]
        pub const fn wdc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wdc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wdv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for FdcanRwd {
        #[inline(always)]
        fn default() -> FdcanRwd {
            FdcanRwd(0)
        }
    }
    impl core::fmt::Debug for FdcanRwd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanRwd")
                .field("wdc", &self.wdc())
                .field("wdv", &self.wdv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanRwd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanRwd {{ wdc: {=u8:?}, wdv: {=u8:?} }}",
                self.wdc(),
                self.wdv()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanRxf0a(pub u32);
    impl FdcanRxf0a {
        #[must_use]
        #[inline(always)]
        pub const fn f0ai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_f0ai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanRxf0a {
        #[inline(always)]
        fn default() -> FdcanRxf0a {
            FdcanRxf0a(0)
        }
    }
    impl core::fmt::Debug for FdcanRxf0a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanRxf0a")
                .field("f0ai", &self.f0ai())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanRxf0a {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanRxf0a {{ f0ai: {=u8:?} }}", self.f0ai())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanRxf0s(pub u32);
    impl FdcanRxf0s {
        #[must_use]
        #[inline(always)]
        pub const fn f0fl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_f0fl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn f0gi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_f0gi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn f0pi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_f0pi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn f0f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_f0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf0l(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf0l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for FdcanRxf0s {
        #[inline(always)]
        fn default() -> FdcanRxf0s {
            FdcanRxf0s(0)
        }
    }
    impl core::fmt::Debug for FdcanRxf0s {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanRxf0s")
                .field("f0fl", &self.f0fl())
                .field("f0gi", &self.f0gi())
                .field("f0pi", &self.f0pi())
                .field("f0f", &self.f0f())
                .field("rf0l", &self.rf0l())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanRxf0s {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanRxf0s {{ f0fl: {=u8:?}, f0gi: {=u8:?}, f0pi: {=u8:?}, f0f: {=bool:?}, rf0l: {=bool:?} }}",
                self.f0fl(),
                self.f0gi(),
                self.f0pi(),
                self.f0f(),
                self.rf0l()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanRxf1a(pub u32);
    impl FdcanRxf1a {
        #[must_use]
        #[inline(always)]
        pub const fn f1ai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_f1ai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanRxf1a {
        #[inline(always)]
        fn default() -> FdcanRxf1a {
            FdcanRxf1a(0)
        }
    }
    impl core::fmt::Debug for FdcanRxf1a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanRxf1a")
                .field("f1ai", &self.f1ai())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanRxf1a {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanRxf1a {{ f1ai: {=u8:?} }}", self.f1ai())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanRxf1s(pub u32);
    impl FdcanRxf1s {
        #[must_use]
        #[inline(always)]
        pub const fn f1fl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_f1fl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn f1gi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_f1gi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn f1pi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_f1pi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn f1f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_f1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rf1l(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rf1l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for FdcanRxf1s {
        #[inline(always)]
        fn default() -> FdcanRxf1s {
            FdcanRxf1s(0)
        }
    }
    impl core::fmt::Debug for FdcanRxf1s {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanRxf1s")
                .field("f1fl", &self.f1fl())
                .field("f1gi", &self.f1gi())
                .field("f1pi", &self.f1pi())
                .field("f1f", &self.f1f())
                .field("rf1l", &self.rf1l())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanRxf1s {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanRxf1s {{ f1fl: {=u8:?}, f1gi: {=u8:?}, f1pi: {=u8:?}, f1f: {=bool:?}, rf1l: {=bool:?} }}",
                self.f1fl(),
                self.f1gi(),
                self.f1pi(),
                self.f1f(),
                self.rf1l()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanRxgfc(pub u32);
    impl FdcanRxgfc {
        #[must_use]
        #[inline(always)]
        pub const fn rrfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrfs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rrfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn anfe(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_anfe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn anfs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_anfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn f1om(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_f1om(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn f0om(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_f0om(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lss(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lse(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for FdcanRxgfc {
        #[inline(always)]
        fn default() -> FdcanRxgfc {
            FdcanRxgfc(0)
        }
    }
    impl core::fmt::Debug for FdcanRxgfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanRxgfc")
                .field("rrfe", &self.rrfe())
                .field("rrfs", &self.rrfs())
                .field("anfe", &self.anfe())
                .field("anfs", &self.anfs())
                .field("f1om", &self.f1om())
                .field("f0om", &self.f0om())
                .field("lss", &self.lss())
                .field("lse", &self.lse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanRxgfc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanRxgfc {{ rrfe: {=bool:?}, rrfs: {=bool:?}, anfe: {=u8:?}, anfs: {=u8:?}, f1om: {=bool:?}, f0om: {=bool:?}, lss: {=u8:?}, lse: {=u8:?} }}",
                self.rrfe(),
                self.rrfs(),
                self.anfe(),
                self.anfs(),
                self.f1om(),
                self.f0om(),
                self.lss(),
                self.lse()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTdcr(pub u32);
    impl FdcanTdcr {
        #[must_use]
        #[inline(always)]
        pub const fn tdcf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tdco(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdco(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for FdcanTdcr {
        #[inline(always)]
        fn default() -> FdcanTdcr {
            FdcanTdcr(0)
        }
    }
    impl core::fmt::Debug for FdcanTdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTdcr")
                .field("tdcf", &self.tdcf())
                .field("tdco", &self.tdco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanTdcr {{ tdcf: {=u8:?}, tdco: {=u8:?} }}",
                self.tdcf(),
                self.tdco()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTest(pub u32);
    impl FdcanTest {
        #[must_use]
        #[inline(always)]
        pub const fn lbck(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lbck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tx(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for FdcanTest {
        #[inline(always)]
        fn default() -> FdcanTest {
            FdcanTest(0)
        }
    }
    impl core::fmt::Debug for FdcanTest {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTest")
                .field("lbck", &self.lbck())
                .field("tx", &self.tx())
                .field("rx", &self.rx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTest {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanTest {{ lbck: {=bool:?}, tx: {=u8:?}, rx: {=bool:?} }}",
                self.lbck(),
                self.tx(),
                self.rx()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTocc(pub u32);
    impl FdcanTocc {
        #[must_use]
        #[inline(always)]
        pub const fn etoc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_etoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tos(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn top(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_top(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for FdcanTocc {
        #[inline(always)]
        fn default() -> FdcanTocc {
            FdcanTocc(0)
        }
    }
    impl core::fmt::Debug for FdcanTocc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTocc")
                .field("etoc", &self.etoc())
                .field("tos", &self.tos())
                .field("top", &self.top())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTocc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanTocc {{ etoc: {=bool:?}, tos: {=u8:?}, top: {=u16:?} }}",
                self.etoc(),
                self.tos(),
                self.top()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTocv(pub u32);
    impl FdcanTocv {
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_toc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for FdcanTocv {
        #[inline(always)]
        fn default() -> FdcanTocv {
            FdcanTocv(0)
        }
    }
    impl core::fmt::Debug for FdcanTocv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTocv")
                .field("toc", &self.toc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTocv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTocv {{ toc: {=u16:?} }}", self.toc())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTscc(pub u32);
    impl FdcanTscc {
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tcp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for FdcanTscc {
        #[inline(always)]
        fn default() -> FdcanTscc {
            FdcanTscc(0)
        }
    }
    impl core::fmt::Debug for FdcanTscc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTscc")
                .field("tss", &self.tss())
                .field("tcp", &self.tcp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTscc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanTscc {{ tss: {=u8:?}, tcp: {=u8:?} }}",
                self.tss(),
                self.tcp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTscv(pub u32);
    impl FdcanTscv {
        #[must_use]
        #[inline(always)]
        pub const fn tsc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_tsc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for FdcanTscv {
        #[inline(always)]
        fn default() -> FdcanTscv {
            FdcanTscv(0)
        }
    }
    impl core::fmt::Debug for FdcanTscv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTscv")
                .field("tsc", &self.tsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTscv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTscv {{ tsc: {=u16:?} }}", self.tsc())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxbar(pub u32);
    impl FdcanTxbar {
        #[must_use]
        #[inline(always)]
        pub const fn ar(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanTxbar {
        #[inline(always)]
        fn default() -> FdcanTxbar {
            FdcanTxbar(0)
        }
    }
    impl core::fmt::Debug for FdcanTxbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxbar")
                .field("ar", &self.ar())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxbar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxbar {{ ar: {=u8:?} }}", self.ar())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxbc(pub u32);
    impl FdcanTxbc {
        #[must_use]
        #[inline(always)]
        pub const fn tfqm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tfqm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for FdcanTxbc {
        #[inline(always)]
        fn default() -> FdcanTxbc {
            FdcanTxbc(0)
        }
    }
    impl core::fmt::Debug for FdcanTxbc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxbc")
                .field("tfqm", &self.tfqm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxbc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxbc {{ tfqm: {=bool:?} }}", self.tfqm())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxbcf(pub u32);
    impl FdcanTxbcf {
        #[must_use]
        #[inline(always)]
        pub const fn cf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanTxbcf {
        #[inline(always)]
        fn default() -> FdcanTxbcf {
            FdcanTxbcf(0)
        }
    }
    impl core::fmt::Debug for FdcanTxbcf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxbcf")
                .field("cf", &self.cf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxbcf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxbcf {{ cf: {=u8:?} }}", self.cf())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxbcie(pub u32);
    impl FdcanTxbcie {
        #[must_use]
        #[inline(always)]
        pub const fn cfie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cfie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanTxbcie {
        #[inline(always)]
        fn default() -> FdcanTxbcie {
            FdcanTxbcie(0)
        }
    }
    impl core::fmt::Debug for FdcanTxbcie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxbcie")
                .field("cfie", &self.cfie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxbcie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxbcie {{ cfie: {=u8:?} }}", self.cfie())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxbcr(pub u32);
    impl FdcanTxbcr {
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanTxbcr {
        #[inline(always)]
        fn default() -> FdcanTxbcr {
            FdcanTxbcr(0)
        }
    }
    impl core::fmt::Debug for FdcanTxbcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxbcr")
                .field("cr", &self.cr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxbcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxbcr {{ cr: {=u8:?} }}", self.cr())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxbrp(pub u32);
    impl FdcanTxbrp {
        #[must_use]
        #[inline(always)]
        pub const fn trp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanTxbrp {
        #[inline(always)]
        fn default() -> FdcanTxbrp {
            FdcanTxbrp(0)
        }
    }
    impl core::fmt::Debug for FdcanTxbrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxbrp")
                .field("trp", &self.trp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxbrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxbrp {{ trp: {=u8:?} }}", self.trp())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxbtie(pub u32);
    impl FdcanTxbtie {
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanTxbtie {
        #[inline(always)]
        fn default() -> FdcanTxbtie {
            FdcanTxbtie(0)
        }
    }
    impl core::fmt::Debug for FdcanTxbtie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxbtie")
                .field("tie", &self.tie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxbtie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxbtie {{ tie: {=u8:?} }}", self.tie())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxbto(pub u32);
    impl FdcanTxbto {
        #[must_use]
        #[inline(always)]
        pub const fn to(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_to(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FdcanTxbto {
        #[inline(always)]
        fn default() -> FdcanTxbto {
            FdcanTxbto(0)
        }
    }
    impl core::fmt::Debug for FdcanTxbto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxbto")
                .field("to", &self.to())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxbto {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxbto {{ to: {=u8:?} }}", self.to())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxefa(pub u32);
    impl FdcanTxefa {
        #[must_use]
        #[inline(always)]
        pub const fn efai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_efai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for FdcanTxefa {
        #[inline(always)]
        fn default() -> FdcanTxefa {
            FdcanTxefa(0)
        }
    }
    impl core::fmt::Debug for FdcanTxefa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxefa")
                .field("efai", &self.efai())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxefa {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanTxefa {{ efai: {=u8:?} }}", self.efai())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxefs(pub u32);
    impl FdcanTxefs {
        #[must_use]
        #[inline(always)]
        pub const fn effl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_effl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn efgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_efgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn efpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_efpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for FdcanTxefs {
        #[inline(always)]
        fn default() -> FdcanTxefs {
            FdcanTxefs(0)
        }
    }
    impl core::fmt::Debug for FdcanTxefs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxefs")
                .field("effl", &self.effl())
                .field("efgi", &self.efgi())
                .field("efpi", &self.efpi())
                .field("eff", &self.eff())
                .field("tefl", &self.tefl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxefs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanTxefs {{ effl: {=u8:?}, efgi: {=u8:?}, efpi: {=u8:?}, eff: {=bool:?}, tefl: {=bool:?} }}",
                self.effl(),
                self.efgi(),
                self.efpi(),
                self.eff(),
                self.tefl()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanTxfqs(pub u32);
    impl FdcanTxfqs {
        #[must_use]
        #[inline(always)]
        pub const fn tffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tfgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfqpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tfqpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfqf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tfqf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for FdcanTxfqs {
        #[inline(always)]
        fn default() -> FdcanTxfqs {
            FdcanTxfqs(0)
        }
    }
    impl core::fmt::Debug for FdcanTxfqs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanTxfqs")
                .field("tffl", &self.tffl())
                .field("tfgi", &self.tfgi())
                .field("tfqpi", &self.tfqpi())
                .field("tfqf", &self.tfqf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanTxfqs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FdcanTxfqs {{ tffl: {=u8:?}, tfgi: {=u8:?}, tfqpi: {=u8:?}, tfqf: {=bool:?} }}",
                self.tffl(),
                self.tfgi(),
                self.tfqpi(),
                self.tfqf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdcanXidam(pub u32);
    impl FdcanXidam {
        #[must_use]
        #[inline(always)]
        pub const fn eidm(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_eidm(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for FdcanXidam {
        #[inline(always)]
        fn default() -> FdcanXidam {
            FdcanXidam(0)
        }
    }
    impl core::fmt::Debug for FdcanXidam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FdcanXidam")
                .field("eidm", &self.eidm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FdcanXidam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FdcanXidam {{ eidm: {=u32:?} }}", self.eidm())
        }
    }
}
