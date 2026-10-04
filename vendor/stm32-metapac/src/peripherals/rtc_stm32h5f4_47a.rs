#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn tr(self) -> crate::common::Reg<regs::RtcTr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::RtcDr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::RtcSsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn icsr(self) -> crate::common::Reg<regs::RtcIcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn prer(self) -> crate::common::Reg<regs::RtcPrer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn wutr(self) -> crate::common::Reg<regs::RtcWutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::RtcCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::RtcPrivcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::RtcSeccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn wpr(self) -> crate::common::Reg<regs::RtcWpr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn calr(self) -> crate::common::Reg<regs::RtcCalr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn shiftr(self) -> crate::common::Reg<regs::RtcShiftr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn tstr(self) -> crate::common::Reg<regs::RtcTstr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn tsdr(self) -> crate::common::Reg<regs::RtcTsdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn tsssr(self) -> crate::common::Reg<regs::RtcTsssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn alrmar(self) -> crate::common::Reg<regs::RtcAlrmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn alrmassr(self) -> crate::common::Reg<regs::RtcAlrmassr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn alrmbr(self) -> crate::common::Reg<regs::RtcAlrmbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn alrmbssr(self) -> crate::common::Reg<regs::RtcAlrmbssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::RtcSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::RtcMisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::RtcSmisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::RtcScr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::RtcOr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn alrabinr(self) -> crate::common::Reg<regs::RtcAlrabinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn alrbbinr(self) -> crate::common::Reg<regs::RtcAlrbbinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlrabinr(pub u32);
    impl RtcAlrabinr {
        #[must_use]
        #[inline(always)]
        pub const fn ss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RtcAlrabinr {
        #[inline(always)]
        fn default() -> RtcAlrabinr {
            RtcAlrabinr(0)
        }
    }
    impl core::fmt::Debug for RtcAlrabinr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlrabinr")
                .field("ss", &self.ss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlrabinr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcAlrabinr {{ ss: {=u32:?} }}", self.ss())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlrbbinr(pub u32);
    impl RtcAlrbbinr {
        #[must_use]
        #[inline(always)]
        pub const fn ss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RtcAlrbbinr {
        #[inline(always)]
        fn default() -> RtcAlrbbinr {
            RtcAlrbbinr(0)
        }
    }
    impl core::fmt::Debug for RtcAlrbbinr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlrbbinr")
                .field("ss", &self.ss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlrbbinr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcAlrbbinr {{ ss: {=u32:?} }}", self.ss())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlrmar(pub u32);
    impl RtcAlrmar {
        #[must_use]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msk1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msk1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msk2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msk2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msk3(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msk3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdsel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msk4(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msk4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RtcAlrmar {
        #[inline(always)]
        fn default() -> RtcAlrmar {
            RtcAlrmar(0)
        }
    }
    impl core::fmt::Debug for RtcAlrmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlrmar")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("msk1", &self.msk1())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("msk2", &self.msk2())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .field("msk3", &self.msk3())
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("wdsel", &self.wdsel())
                .field("msk4", &self.msk4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlrmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcAlrmar {{ su: {=u8:?}, st: {=u8:?}, msk1: {=bool:?}, mnu: {=u8:?}, mnt: {=u8:?}, msk2: {=bool:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {=bool:?}, msk3: {=bool:?}, du: {=u8:?}, dt: {=u8:?}, wdsel: {=bool:?}, msk4: {=bool:?} }}",
                self.su(),
                self.st(),
                self.msk1(),
                self.mnu(),
                self.mnt(),
                self.msk2(),
                self.hu(),
                self.ht(),
                self.pm(),
                self.msk3(),
                self.du(),
                self.dt(),
                self.wdsel(),
                self.msk4()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlrmassr(pub u32);
    impl RtcAlrmassr {
        #[must_use]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn maskss(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_maskss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssclr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RtcAlrmassr {
        #[inline(always)]
        fn default() -> RtcAlrmassr {
            RtcAlrmassr(0)
        }
    }
    impl core::fmt::Debug for RtcAlrmassr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlrmassr")
                .field("ss", &self.ss())
                .field("maskss", &self.maskss())
                .field("ssclr", &self.ssclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlrmassr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcAlrmassr {{ ss: {=u16:?}, maskss: {=u8:?}, ssclr: {=bool:?} }}",
                self.ss(),
                self.maskss(),
                self.ssclr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlrmbr(pub u32);
    impl RtcAlrmbr {
        #[must_use]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msk1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msk1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msk2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msk2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msk3(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msk3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdsel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msk4(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msk4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RtcAlrmbr {
        #[inline(always)]
        fn default() -> RtcAlrmbr {
            RtcAlrmbr(0)
        }
    }
    impl core::fmt::Debug for RtcAlrmbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlrmbr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("msk1", &self.msk1())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("msk2", &self.msk2())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .field("msk3", &self.msk3())
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("wdsel", &self.wdsel())
                .field("msk4", &self.msk4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlrmbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcAlrmbr {{ su: {=u8:?}, st: {=u8:?}, msk1: {=bool:?}, mnu: {=u8:?}, mnt: {=u8:?}, msk2: {=bool:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {=bool:?}, msk3: {=bool:?}, du: {=u8:?}, dt: {=u8:?}, wdsel: {=bool:?}, msk4: {=bool:?} }}",
                self.su(),
                self.st(),
                self.msk1(),
                self.mnu(),
                self.mnt(),
                self.msk2(),
                self.hu(),
                self.ht(),
                self.pm(),
                self.msk3(),
                self.du(),
                self.dt(),
                self.wdsel(),
                self.msk4()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlrmbssr(pub u32);
    impl RtcAlrmbssr {
        #[must_use]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn maskss(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_maskss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssclr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RtcAlrmbssr {
        #[inline(always)]
        fn default() -> RtcAlrmbssr {
            RtcAlrmbssr(0)
        }
    }
    impl core::fmt::Debug for RtcAlrmbssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlrmbssr")
                .field("ss", &self.ss())
                .field("maskss", &self.maskss())
                .field("ssclr", &self.ssclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlrmbssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcAlrmbssr {{ ss: {=u16:?}, maskss: {=u8:?}, ssclr: {=bool:?} }}",
                self.ss(),
                self.maskss(),
                self.ssclr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcCalr(pub u32);
    impl RtcCalr {
        #[must_use]
        #[inline(always)]
        pub const fn calm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_calm(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpcal(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calw16(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calw16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calw8(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calw8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for RtcCalr {
        #[inline(always)]
        fn default() -> RtcCalr {
            RtcCalr(0)
        }
    }
    impl core::fmt::Debug for RtcCalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcCalr")
                .field("calm", &self.calm())
                .field("lpcal", &self.lpcal())
                .field("calw16", &self.calw16())
                .field("calw8", &self.calw8())
                .field("calp", &self.calp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcCalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcCalr {{ calm: {=u16:?}, lpcal: {=bool:?}, calw16: {=bool:?}, calw8: {=bool:?}, calp: {=bool:?} }}",
                self.calm(),
                self.lpcal(),
                self.calw16(),
                self.calw8(),
                self.calp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcCr(pub u32);
    impl RtcCr {
        #[must_use]
        #[inline(always)]
        pub const fn wucksel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wucksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsedge(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn refckon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_refckon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bypshad(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bypshad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fmt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fmt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssruie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssruie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wute(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alraie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alraie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wutie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn add1h(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_add1h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sub1h(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sub1h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cosel(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cosel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pol(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn osel(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_osel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn coe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_coe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itse(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_itse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tampts(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tampts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tampoe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tampoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrafclr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrafclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbfclr(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tampalrm_pu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tampalrm_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tampalrm_type(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tampalrm_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn out2en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_out2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RtcCr {
        #[inline(always)]
        fn default() -> RtcCr {
            RtcCr(0)
        }
    }
    impl core::fmt::Debug for RtcCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcCr")
                .field("wucksel", &self.wucksel())
                .field("tsedge", &self.tsedge())
                .field("refckon", &self.refckon())
                .field("bypshad", &self.bypshad())
                .field("fmt", &self.fmt())
                .field("ssruie", &self.ssruie())
                .field("alrae", &self.alrae())
                .field("alrbe", &self.alrbe())
                .field("wute", &self.wute())
                .field("tse", &self.tse())
                .field("alraie", &self.alraie())
                .field("alrbie", &self.alrbie())
                .field("wutie", &self.wutie())
                .field("tsie", &self.tsie())
                .field("add1h", &self.add1h())
                .field("sub1h", &self.sub1h())
                .field("bkp", &self.bkp())
                .field("cosel", &self.cosel())
                .field("pol", &self.pol())
                .field("osel", &self.osel())
                .field("coe", &self.coe())
                .field("itse", &self.itse())
                .field("tampts", &self.tampts())
                .field("tampoe", &self.tampoe())
                .field("alrafclr", &self.alrafclr())
                .field("alrbfclr", &self.alrbfclr())
                .field("tampalrm_pu", &self.tampalrm_pu())
                .field("tampalrm_type", &self.tampalrm_type())
                .field("out2en", &self.out2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcCr {{ wucksel: {=u8:?}, tsedge: {=bool:?}, refckon: {=bool:?}, bypshad: {=bool:?}, fmt: {=bool:?}, ssruie: {=bool:?}, alrae: {=bool:?}, alrbe: {=bool:?}, wute: {=bool:?}, tse: {=bool:?}, alraie: {=bool:?}, alrbie: {=bool:?}, wutie: {=bool:?}, tsie: {=bool:?}, add1h: {=bool:?}, sub1h: {=bool:?}, bkp: {=bool:?}, cosel: {=bool:?}, pol: {=bool:?}, osel: {=u8:?}, coe: {=bool:?}, itse: {=bool:?}, tampts: {=bool:?}, tampoe: {=bool:?}, alrafclr: {=bool:?}, alrbfclr: {=bool:?}, tampalrm_pu: {=bool:?}, tampalrm_type: {=bool:?}, out2en: {=bool:?} }}",
                self.wucksel(),
                self.tsedge(),
                self.refckon(),
                self.bypshad(),
                self.fmt(),
                self.ssruie(),
                self.alrae(),
                self.alrbe(),
                self.wute(),
                self.tse(),
                self.alraie(),
                self.alrbie(),
                self.wutie(),
                self.tsie(),
                self.add1h(),
                self.sub1h(),
                self.bkp(),
                self.cosel(),
                self.pol(),
                self.osel(),
                self.coe(),
                self.itse(),
                self.tampts(),
                self.tampoe(),
                self.alrafclr(),
                self.alrbfclr(),
                self.tampalrm_pu(),
                self.tampalrm_type(),
                self.out2en()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcDr(pub u32);
    impl RtcDr {
        #[must_use]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdu(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wdu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn yu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_yu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn yt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_yt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for RtcDr {
        #[inline(always)]
        fn default() -> RtcDr {
            RtcDr(0)
        }
    }
    impl core::fmt::Debug for RtcDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcDr")
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("mu", &self.mu())
                .field("mt", &self.mt())
                .field("wdu", &self.wdu())
                .field("yu", &self.yu())
                .field("yt", &self.yt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcDr {{ du: {=u8:?}, dt: {=u8:?}, mu: {=u8:?}, mt: {=bool:?}, wdu: {=u8:?}, yu: {=u8:?}, yt: {=u8:?} }}",
                self.du(),
                self.dt(),
                self.mu(),
                self.mt(),
                self.wdu(),
                self.yu(),
                self.yt()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcIcsr(pub u32);
    impl RtcIcsr {
        #[must_use]
        #[inline(always)]
        pub const fn alrawf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrawf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbwf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbwf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wutwf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wutwf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn shpf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_shpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_inits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn initf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_initf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bin(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcdu(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bcdu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn recalpf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_recalpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for RtcIcsr {
        #[inline(always)]
        fn default() -> RtcIcsr {
            RtcIcsr(0)
        }
    }
    impl core::fmt::Debug for RtcIcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcIcsr")
                .field("alrawf", &self.alrawf())
                .field("alrbwf", &self.alrbwf())
                .field("wutwf", &self.wutwf())
                .field("shpf", &self.shpf())
                .field("inits", &self.inits())
                .field("rsf", &self.rsf())
                .field("initf", &self.initf())
                .field("init", &self.init())
                .field("bin", &self.bin())
                .field("bcdu", &self.bcdu())
                .field("recalpf", &self.recalpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcIcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcIcsr {{ alrawf: {=bool:?}, alrbwf: {=bool:?}, wutwf: {=bool:?}, shpf: {=bool:?}, inits: {=bool:?}, rsf: {=bool:?}, initf: {=bool:?}, init: {=bool:?}, bin: {=u8:?}, bcdu: {=u8:?}, recalpf: {=bool:?} }}",
                self.alrawf(),
                self.alrbwf(),
                self.wutwf(),
                self.shpf(),
                self.inits(),
                self.rsf(),
                self.initf(),
                self.init(),
                self.bin(),
                self.bcdu(),
                self.recalpf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcMisr(pub u32);
    impl RtcMisr {
        #[must_use]
        #[inline(always)]
        pub const fn alramf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alramf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbmf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wutmf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wutmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsmf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsovmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsovmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itsmf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_itsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssrumf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssrumf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for RtcMisr {
        #[inline(always)]
        fn default() -> RtcMisr {
            RtcMisr(0)
        }
    }
    impl core::fmt::Debug for RtcMisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcMisr")
                .field("alramf", &self.alramf())
                .field("alrbmf", &self.alrbmf())
                .field("wutmf", &self.wutmf())
                .field("tsmf", &self.tsmf())
                .field("tsovmf", &self.tsovmf())
                .field("itsmf", &self.itsmf())
                .field("ssrumf", &self.ssrumf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcMisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcMisr {{ alramf: {=bool:?}, alrbmf: {=bool:?}, wutmf: {=bool:?}, tsmf: {=bool:?}, tsovmf: {=bool:?}, itsmf: {=bool:?}, ssrumf: {=bool:?} }}",
                self.alramf(),
                self.alrbmf(),
                self.wutmf(),
                self.tsmf(),
                self.tsovmf(),
                self.itsmf(),
                self.ssrumf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcOr(pub u32);
    impl RtcOr {
        #[must_use]
        #[inline(always)]
        pub const fn out2_rmp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_out2_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for RtcOr {
        #[inline(always)]
        fn default() -> RtcOr {
            RtcOr(0)
        }
    }
    impl core::fmt::Debug for RtcOr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcOr")
                .field("out2_rmp", &self.out2_rmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcOr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcOr {{ out2_rmp: {=bool:?} }}", self.out2_rmp())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcPrer(pub u32);
    impl RtcPrer {
        #[must_use]
        #[inline(always)]
        pub const fn prediv_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_prediv_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn prediv_a(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_prediv_a(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for RtcPrer {
        #[inline(always)]
        fn default() -> RtcPrer {
            RtcPrer(0)
        }
    }
    impl core::fmt::Debug for RtcPrer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcPrer")
                .field("prediv_s", &self.prediv_s())
                .field("prediv_a", &self.prediv_a())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcPrer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcPrer {{ prediv_s: {=u16:?}, prediv_a: {=u8:?} }}",
                self.prediv_s(),
                self.prediv_a()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcPrivcfgr(pub u32);
    impl RtcPrivcfgr {
        #[must_use]
        #[inline(always)]
        pub const fn alrapriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrapriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbpriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wutpriv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wutpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tspriv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calpriv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn initpriv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_initpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for RtcPrivcfgr {
        #[inline(always)]
        fn default() -> RtcPrivcfgr {
            RtcPrivcfgr(0)
        }
    }
    impl core::fmt::Debug for RtcPrivcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcPrivcfgr")
                .field("alrapriv", &self.alrapriv())
                .field("alrbpriv", &self.alrbpriv())
                .field("wutpriv", &self.wutpriv())
                .field("tspriv", &self.tspriv())
                .field("calpriv", &self.calpriv())
                .field("initpriv", &self.initpriv())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcPrivcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcPrivcfgr {{ alrapriv: {=bool:?}, alrbpriv: {=bool:?}, wutpriv: {=bool:?}, tspriv: {=bool:?}, calpriv: {=bool:?}, initpriv: {=bool:?}, priv_: {=bool:?} }}",
                self.alrapriv(),
                self.alrbpriv(),
                self.wutpriv(),
                self.tspriv(),
                self.calpriv(),
                self.initpriv(),
                self.priv_()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcScr(pub u32);
    impl RtcScr {
        #[must_use]
        #[inline(always)]
        pub const fn calraf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calraf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calrbf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calrbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cwutf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cwutf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctsf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctsovf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctsovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn citsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_citsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cssruf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cssruf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for RtcScr {
        #[inline(always)]
        fn default() -> RtcScr {
            RtcScr(0)
        }
    }
    impl core::fmt::Debug for RtcScr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcScr")
                .field("calraf", &self.calraf())
                .field("calrbf", &self.calrbf())
                .field("cwutf", &self.cwutf())
                .field("ctsf", &self.ctsf())
                .field("ctsovf", &self.ctsovf())
                .field("citsf", &self.citsf())
                .field("cssruf", &self.cssruf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcScr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcScr {{ calraf: {=bool:?}, calrbf: {=bool:?}, cwutf: {=bool:?}, ctsf: {=bool:?}, ctsovf: {=bool:?}, citsf: {=bool:?}, cssruf: {=bool:?} }}",
                self.calraf(),
                self.calrbf(),
                self.cwutf(),
                self.ctsf(),
                self.ctsovf(),
                self.citsf(),
                self.cssruf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcSeccfgr(pub u32);
    impl RtcSeccfgr {
        #[must_use]
        #[inline(always)]
        pub const fn alrasec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrasec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wutsec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wutsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tssec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tssec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calsec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn initsec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_initsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for RtcSeccfgr {
        #[inline(always)]
        fn default() -> RtcSeccfgr {
            RtcSeccfgr(0)
        }
    }
    impl core::fmt::Debug for RtcSeccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcSeccfgr")
                .field("alrasec", &self.alrasec())
                .field("alrbsec", &self.alrbsec())
                .field("wutsec", &self.wutsec())
                .field("tssec", &self.tssec())
                .field("calsec", &self.calsec())
                .field("initsec", &self.initsec())
                .field("sec", &self.sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcSeccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcSeccfgr {{ alrasec: {=bool:?}, alrbsec: {=bool:?}, wutsec: {=bool:?}, tssec: {=bool:?}, calsec: {=bool:?}, initsec: {=bool:?}, sec: {=bool:?} }}",
                self.alrasec(),
                self.alrbsec(),
                self.wutsec(),
                self.tssec(),
                self.calsec(),
                self.initsec(),
                self.sec()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcShiftr(pub u32);
    impl RtcShiftr {
        #[must_use]
        #[inline(always)]
        pub const fn subfs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_subfs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn add1s(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_add1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RtcShiftr {
        #[inline(always)]
        fn default() -> RtcShiftr {
            RtcShiftr(0)
        }
    }
    impl core::fmt::Debug for RtcShiftr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcShiftr")
                .field("subfs", &self.subfs())
                .field("add1s", &self.add1s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcShiftr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcShiftr {{ subfs: {=u16:?}, add1s: {=bool:?} }}",
                self.subfs(),
                self.add1s()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcSmisr(pub u32);
    impl RtcSmisr {
        #[must_use]
        #[inline(always)]
        pub const fn alramf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alramf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbmf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wutmf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wutmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsmf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsovmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsovmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itsmf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_itsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssrumf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssrumf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for RtcSmisr {
        #[inline(always)]
        fn default() -> RtcSmisr {
            RtcSmisr(0)
        }
    }
    impl core::fmt::Debug for RtcSmisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcSmisr")
                .field("alramf", &self.alramf())
                .field("alrbmf", &self.alrbmf())
                .field("wutmf", &self.wutmf())
                .field("tsmf", &self.tsmf())
                .field("tsovmf", &self.tsovmf())
                .field("itsmf", &self.itsmf())
                .field("ssrumf", &self.ssrumf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcSmisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcSmisr {{ alramf: {=bool:?}, alrbmf: {=bool:?}, wutmf: {=bool:?}, tsmf: {=bool:?}, tsovmf: {=bool:?}, itsmf: {=bool:?}, ssrumf: {=bool:?} }}",
                self.alramf(),
                self.alrbmf(),
                self.wutmf(),
                self.tsmf(),
                self.tsovmf(),
                self.itsmf(),
                self.ssrumf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcSr(pub u32);
    impl RtcSr {
        #[must_use]
        #[inline(always)]
        pub const fn alraf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alraf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alrbf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alrbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wutf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wutf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsovf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_itsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssruf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssruf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for RtcSr {
        #[inline(always)]
        fn default() -> RtcSr {
            RtcSr(0)
        }
    }
    impl core::fmt::Debug for RtcSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcSr")
                .field("alraf", &self.alraf())
                .field("alrbf", &self.alrbf())
                .field("wutf", &self.wutf())
                .field("tsf", &self.tsf())
                .field("tsovf", &self.tsovf())
                .field("itsf", &self.itsf())
                .field("ssruf", &self.ssruf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcSr {{ alraf: {=bool:?}, alrbf: {=bool:?}, wutf: {=bool:?}, tsf: {=bool:?}, tsovf: {=bool:?}, itsf: {=bool:?}, ssruf: {=bool:?} }}",
                self.alraf(),
                self.alrbf(),
                self.wutf(),
                self.tsf(),
                self.tsovf(),
                self.itsf(),
                self.ssruf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcSsr(pub u32);
    impl RtcSsr {
        #[must_use]
        #[inline(always)]
        pub const fn ss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RtcSsr {
        #[inline(always)]
        fn default() -> RtcSsr {
            RtcSsr(0)
        }
    }
    impl core::fmt::Debug for RtcSsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcSsr").field("ss", &self.ss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcSsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcSsr {{ ss: {=u32:?} }}", self.ss())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcTr(pub u32);
    impl RtcTr {
        #[must_use]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for RtcTr {
        #[inline(always)]
        fn default() -> RtcTr {
            RtcTr(0)
        }
    }
    impl core::fmt::Debug for RtcTr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcTr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcTr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcTr {{ su: {=u8:?}, st: {=u8:?}, mnu: {=u8:?}, mnt: {=u8:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {=bool:?} }}",
                self.su(),
                self.st(),
                self.mnu(),
                self.mnt(),
                self.hu(),
                self.ht(),
                self.pm()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcTsdr(pub u32);
    impl RtcTsdr {
        #[must_use]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wdu(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wdu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
    }
    impl Default for RtcTsdr {
        #[inline(always)]
        fn default() -> RtcTsdr {
            RtcTsdr(0)
        }
    }
    impl core::fmt::Debug for RtcTsdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcTsdr")
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("mu", &self.mu())
                .field("mt", &self.mt())
                .field("wdu", &self.wdu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcTsdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcTsdr {{ du: {=u8:?}, dt: {=u8:?}, mu: {=u8:?}, mt: {=bool:?}, wdu: {=u8:?} }}",
                self.du(),
                self.dt(),
                self.mu(),
                self.mt(),
                self.wdu()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcTsssr(pub u32);
    impl RtcTsssr {
        #[must_use]
        #[inline(always)]
        pub const fn ss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RtcTsssr {
        #[inline(always)]
        fn default() -> RtcTsssr {
            RtcTsssr(0)
        }
    }
    impl core::fmt::Debug for RtcTsssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcTsssr").field("ss", &self.ss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcTsssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcTsssr {{ ss: {=u32:?} }}", self.ss())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcTstr(pub u32);
    impl RtcTstr {
        #[must_use]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for RtcTstr {
        #[inline(always)]
        fn default() -> RtcTstr {
            RtcTstr(0)
        }
    }
    impl core::fmt::Debug for RtcTstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcTstr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcTstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcTstr {{ su: {=u8:?}, st: {=u8:?}, mnu: {=u8:?}, mnt: {=u8:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {=bool:?} }}",
                self.su(),
                self.st(),
                self.mnu(),
                self.mnt(),
                self.hu(),
                self.ht(),
                self.pm()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcWpr(pub u32);
    impl RtcWpr {
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for RtcWpr {
        #[inline(always)]
        fn default() -> RtcWpr {
            RtcWpr(0)
        }
    }
    impl core::fmt::Debug for RtcWpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcWpr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcWpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcWpr {{ key: {=u8:?} }}", self.key())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcWutr(pub u32);
    impl RtcWutr {
        #[must_use]
        #[inline(always)]
        pub const fn wut(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_wut(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wutoclr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_wutoclr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for RtcWutr {
        #[inline(always)]
        fn default() -> RtcWutr {
            RtcWutr(0)
        }
    }
    impl core::fmt::Debug for RtcWutr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcWutr")
                .field("wut", &self.wut())
                .field("wutoclr", &self.wutoclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcWutr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcWutr {{ wut: {=u16:?}, wutoclr: {=u16:?} }}",
                self.wut(),
                self.wutoclr()
            )
        }
    }
}
