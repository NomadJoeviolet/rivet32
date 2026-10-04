#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn nskeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn seckeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn nsobkkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn secobkkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn opsr(self) -> crate::common::Reg<regs::Opsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn optcr(self) -> crate::common::Reg<regs::Optcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn nssr(self) -> crate::common::Reg<regs::Nssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn secsr(self) -> crate::common::Reg<regs::Secsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::Nscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn seccr(self) -> crate::common::Reg<regs::Seccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn nsccr(self) -> crate::common::Reg<regs::Nsccr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn secccr(self) -> crate::common::Reg<regs::Secccr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn nsobkcfgr(self) -> crate::common::Reg<regs::Nsobkcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn secobkcfgr(self) -> crate::common::Reg<regs::Secobkcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn hdpextr(self) -> crate::common::Reg<regs::Hdpextr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn optsr_cur(self) -> crate::common::Reg<regs::Optsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn optsr_prg(self) -> crate::common::Reg<regs::Optsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn nsepochr_cur(self) -> crate::common::Reg<regs::Nsepochr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn nsepochr_prg(self) -> crate::common::Reg<regs::NsepochrPrg, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn secepochr_cur(self) -> crate::common::Reg<regs::Secepochr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn secepochr_prg(self) -> crate::common::Reg<regs::SecepochrPrg, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn optsr2_cur(self) -> crate::common::Reg<regs::Optsr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn optsr2_prg(self) -> crate::common::Reg<regs::Optsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn nsbootr_cur(self) -> crate::common::Reg<regs::Nsbootr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn nsbootr_prg(self) -> crate::common::Reg<regs::Nsbootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn secbootr_cur(self) -> crate::common::Reg<regs::Secbootr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn secbootr_prg(self) -> crate::common::Reg<regs::Bootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn otpblr_cur(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn otpblr_prg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn secbb1r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn secbb1r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn privbb1r1(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn privbb1r2(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn secwm1r_cur(self) -> crate::common::Reg<regs::Secwm, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn secwm1r_prg(self) -> crate::common::Reg<regs::Secwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn wrp1r_cur(self) -> crate::common::Reg<regs::Wrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn wrp1r_prg(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn edata1r_cur(self) -> crate::common::Reg<regs::Edata1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn edata1r_prg(self) -> crate::common::Reg<regs::Edata1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn hdp1r_cur(self) -> crate::common::Reg<regs::Hdp1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn hdp1r_prg(self) -> crate::common::Reg<regs::Hdp1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn ecccorr(self) -> crate::common::Reg<regs::Ecccorr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn eccdetr(self) -> crate::common::Reg<regs::Eccdetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn eccdr(self) -> crate::common::Reg<regs::Eccdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn secbb2r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn secbb2r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[inline(always)]
    pub const fn privbb2r1(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[inline(always)]
    pub const fn privbb2r2(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[inline(always)]
    pub const fn secwm2r_cur(self) -> crate::common::Reg<regs::Secwm, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[inline(always)]
    pub const fn secwm2r_prg(self) -> crate::common::Reg<regs::Secwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[inline(always)]
    pub const fn wrp2r_cur(self) -> crate::common::Reg<regs::Wrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[inline(always)]
    pub const fn wrp2r_prg(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[inline(always)]
    pub const fn edata2r_cur(self) -> crate::common::Reg<regs::Edata2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[inline(always)]
    pub const fn edata2r_prg(self) -> crate::common::Reg<regs::Edata2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[inline(always)]
    pub const fn hdp2r_cur(self) -> crate::common::Reg<regs::Hdp2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[inline(always)]
    pub const fn hdp2r_prg(self) -> crate::common::Reg<regs::Hdp2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[must_use]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wrhighfreq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wrhighfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    impl core::fmt::Debug for Acr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acr")
                .field("latency", &self.latency())
                .field("wrhighfreq", &self.wrhighfreq())
                .field("prften", &self.prften())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acr {{ latency: {=u8:?}, wrhighfreq: {=u8:?}, prften: {=bool:?} }}",
                self.latency(),
                self.wrhighfreq(),
                self.prften()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bootr(pub u32);
    impl Bootr {
        #[must_use]
        #[inline(always)]
        pub const fn boot_lock(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_boot_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_bootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Bootr {
        #[inline(always)]
        fn default() -> Bootr {
            Bootr(0)
        }
    }
    impl core::fmt::Debug for Bootr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bootr")
                .field("boot_lock", &self.boot_lock())
                .field("bootadd", &self.bootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bootr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bootr {{ boot_lock: {=u8:?}, bootadd: {=u32:?} }}",
                self.boot_lock(),
                self.bootadd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecccorr(pub u32);
    impl Ecccorr {
        #[must_use]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obk_ecc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn data_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_data_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn otp_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_otp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ecccorr {
        #[inline(always)]
        fn default() -> Ecccorr {
            Ecccorr(0)
        }
    }
    impl core::fmt::Debug for Ecccorr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecccorr")
                .field("addr_ecc", &self.addr_ecc())
                .field("obk_ecc", &self.obk_ecc())
                .field("data_ecc", &self.data_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("otp_ecc", &self.otp_ecc())
                .field("eccie", &self.eccie())
                .field("eccc", &self.eccc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecccorr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ecccorr {{ addr_ecc: {=u16:?}, obk_ecc: {=bool:?}, data_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, eccie: {=bool:?}, eccc: {=bool:?}, eccd: {=bool:?} }}",
                self.addr_ecc(),
                self.obk_ecc(),
                self.data_ecc(),
                self.bk_ecc(),
                self.sysf_ecc(),
                self.otp_ecc(),
                self.eccie(),
                self.eccc(),
                self.eccd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdetr(pub u32);
    impl Eccdetr {
        #[must_use]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obk_ecc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn data_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_data_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn otp_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_otp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccdetr {
        #[inline(always)]
        fn default() -> Eccdetr {
            Eccdetr(0)
        }
    }
    impl core::fmt::Debug for Eccdetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdetr")
                .field("addr_ecc", &self.addr_ecc())
                .field("obk_ecc", &self.obk_ecc())
                .field("data_ecc", &self.data_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("otp_ecc", &self.otp_ecc())
                .field("eccie", &self.eccie())
                .field("eccc", &self.eccc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eccdetr {{ addr_ecc: {=u16:?}, obk_ecc: {=bool:?}, data_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, eccie: {=bool:?}, eccc: {=bool:?}, eccd: {=bool:?} }}",
                self.addr_ecc(),
                self.obk_ecc(),
                self.data_ecc(),
                self.bk_ecc(),
                self.sysf_ecc(),
                self.otp_ecc(),
                self.eccie(),
                self.eccc(),
                self.eccd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdr(pub u32);
    impl Eccdr {
        #[must_use]
        #[inline(always)]
        pub const fn fail_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_fail_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Eccdr {
        #[inline(always)]
        fn default() -> Eccdr {
            Eccdr(0)
        }
    }
    impl core::fmt::Debug for Eccdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdr")
                .field("fail_data", &self.fail_data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccdr {{ fail_data: {=u16:?} }}", self.fail_data())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Edata1r(pub u32);
    impl Edata1r {
        #[must_use]
        #[inline(always)]
        pub const fn edata_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_edata_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn edata_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_edata_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Edata1r {
        #[inline(always)]
        fn default() -> Edata1r {
            Edata1r(0)
        }
    }
    impl core::fmt::Debug for Edata1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Edata1r")
                .field("edata_strt", &self.edata_strt())
                .field("edata_en", &self.edata_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Edata1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Edata1r {{ edata_strt: {=u8:?}, edata_en: {=bool:?} }}",
                self.edata_strt(),
                self.edata_en()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Edata2r(pub u32);
    impl Edata2r {
        #[must_use]
        #[inline(always)]
        pub const fn edata_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_edata_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn edata_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_edata_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Edata2r {
        #[inline(always)]
        fn default() -> Edata2r {
            Edata2r(0)
        }
    }
    impl core::fmt::Debug for Edata2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Edata2r")
                .field("edata_strt", &self.edata_strt())
                .field("edata_en", &self.edata_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Edata2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Edata2r {{ edata_strt: {=u8:?}, edata_en: {=bool:?} }}",
                self.edata_strt(),
                self.edata_en()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp1r(pub u32);
    impl Hdp1r {
        #[must_use]
        #[inline(always)]
        pub const fn hdp_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hdp_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hdp_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hdp_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Hdp1r {
        #[inline(always)]
        fn default() -> Hdp1r {
            Hdp1r(0)
        }
    }
    impl core::fmt::Debug for Hdp1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp1r")
                .field("hdp_strt", &self.hdp_strt())
                .field("hdp_end", &self.hdp_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp1r {{ hdp_strt: {=u8:?}, hdp_end: {=u8:?} }}",
                self.hdp_strt(),
                self.hdp_end()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp2r(pub u32);
    impl Hdp2r {
        #[must_use]
        #[inline(always)]
        pub const fn hdp_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hdp_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hdp_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hdp_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Hdp2r {
        #[inline(always)]
        fn default() -> Hdp2r {
            Hdp2r(0)
        }
    }
    impl core::fmt::Debug for Hdp2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp2r")
                .field("hdp_strt", &self.hdp_strt())
                .field("hdp_end", &self.hdp_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp2r {{ hdp_strt: {=u8:?}, hdp_end: {=u8:?} }}",
                self.hdp_strt(),
                self.hdp_end()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdpextr(pub u32);
    impl Hdpextr {
        #[must_use]
        #[inline(always)]
        pub const fn hdp1_ext(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hdp1_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hdp2_ext(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hdp2_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Hdpextr {
        #[inline(always)]
        fn default() -> Hdpextr {
            Hdpextr(0)
        }
    }
    impl core::fmt::Debug for Hdpextr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdpextr")
                .field("hdp1_ext", &self.hdp1_ext())
                .field("hdp2_ext", &self.hdp2_ext())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpextr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdpextr {{ hdp1_ext: {=u8:?}, hdp2_ext: {=u8:?} }}",
                self.hdp1_ext(),
                self.hdp2_ext()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootr(pub u32);
    impl Nsbootr {
        #[must_use]
        #[inline(always)]
        pub const fn boot_lock(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_boot_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_bootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Nsbootr {
        #[inline(always)]
        fn default() -> Nsbootr {
            Nsbootr(0)
        }
    }
    impl core::fmt::Debug for Nsbootr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsbootr")
                .field("boot_lock", &self.boot_lock())
                .field("bootadd", &self.bootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsbootr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nsbootr {{ boot_lock: {=u8:?}, bootadd: {=u32:?} }}",
                self.boot_lock(),
                self.bootadd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsccr(pub u32);
    impl Nsccr {
        #[must_use]
        #[inline(always)]
        pub const fn clr_eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Nsccr {
        #[inline(always)]
        fn default() -> Nsccr {
            Nsccr(0)
        }
    }
    impl core::fmt::Debug for Nsccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsccr")
                .field("clr_eop", &self.clr_eop())
                .field("clr_wrperr", &self.clr_wrperr())
                .field("clr_pgserr", &self.clr_pgserr())
                .field("clr_strberr", &self.clr_strberr())
                .field("clr_incerr", &self.clr_incerr())
                .field("clr_obkerr", &self.clr_obkerr())
                .field("clr_obkwerr", &self.clr_obkwerr())
                .field("clr_optchangeerr", &self.clr_optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nsccr {{ clr_eop: {=bool:?}, clr_wrperr: {=bool:?}, clr_pgserr: {=bool:?}, clr_strberr: {=bool:?}, clr_incerr: {=bool:?}, clr_obkerr: {=bool:?}, clr_obkwerr: {=bool:?}, clr_optchangeerr: {=bool:?} }}",
                self.clr_eop(),
                self.clr_wrperr(),
                self.clr_pgserr(),
                self.clr_strberr(),
                self.clr_incerr(),
                self.clr_obkerr(),
                self.clr_obkwerr(),
                self.clr_optchangeerr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr(pub u32);
    impl Nscr {
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ser(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ser(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn snb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_snb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obkerrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obkerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obkwerrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obkwerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_optchangeerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bksel(&self) -> super::vals::NscrBksel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::NscrBksel::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_bksel(&mut self, val: super::vals::NscrBksel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Nscr {
        #[inline(always)]
        fn default() -> Nscr {
            Nscr(0)
        }
    }
    impl core::fmt::Debug for Nscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nscr")
                .field("lock", &self.lock())
                .field("pg", &self.pg())
                .field("ser", &self.ser())
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("start", &self.start())
                .field("snb", &self.snb())
                .field("mer", &self.mer())
                .field("eopie", &self.eopie())
                .field("wrperrie", &self.wrperrie())
                .field("pgserrie", &self.pgserrie())
                .field("strberrie", &self.strberrie())
                .field("incerrie", &self.incerrie())
                .field("obkerrie", &self.obkerrie())
                .field("obkwerrie", &self.obkwerrie())
                .field("optchangeerrie", &self.optchangeerrie())
                .field("inv", &self.inv())
                .field("bksel", &self.bksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nscr {{ lock: {=bool:?}, pg: {=bool:?}, ser: {=bool:?}, ber: {=bool:?}, fw: {=bool:?}, start: {=bool:?}, snb: {=u8:?}, mer: {=bool:?}, eopie: {=bool:?}, wrperrie: {=bool:?}, pgserrie: {=bool:?}, strberrie: {=bool:?}, incerrie: {=bool:?}, obkerrie: {=bool:?}, obkwerrie: {=bool:?}, optchangeerrie: {=bool:?}, inv: {=bool:?}, bksel: {:?} }}",
                self.lock(),
                self.pg(),
                self.ser(),
                self.ber(),
                self.fw(),
                self.start(),
                self.snb(),
                self.mer(),
                self.eopie(),
                self.wrperrie(),
                self.pgserrie(),
                self.strberrie(),
                self.incerrie(),
                self.obkerrie(),
                self.obkwerrie(),
                self.optchangeerrie(),
                self.inv(),
                self.bksel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsepochr(pub u32);
    impl Nsepochr {
        #[must_use]
        #[inline(always)]
        pub const fn epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Nsepochr {
        #[inline(always)]
        fn default() -> Nsepochr {
            Nsepochr(0)
        }
    }
    impl core::fmt::Debug for Nsepochr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsepochr")
                .field("epoch", &self.epoch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsepochr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nsepochr {{ epoch: {=u32:?} }}", self.epoch())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NsepochrPrg(pub u32);
    impl NsepochrPrg {
        #[must_use]
        #[inline(always)]
        pub const fn epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for NsepochrPrg {
        #[inline(always)]
        fn default() -> NsepochrPrg {
            NsepochrPrg(0)
        }
    }
    impl core::fmt::Debug for NsepochrPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NsepochrPrg")
                .field("epoch", &self.epoch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NsepochrPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "NsepochrPrg {{ epoch: {=u32:?} }}", self.epoch())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsobkcfgr(pub u32);
    impl Nsobkcfgr {
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swap_sect_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swap_sect_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alt_sect(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alt_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alt_sect_erase(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alt_sect_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swap_offset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_swap_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Nsobkcfgr {
        #[inline(always)]
        fn default() -> Nsobkcfgr {
            Nsobkcfgr(0)
        }
    }
    impl core::fmt::Debug for Nsobkcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsobkcfgr")
                .field("lock", &self.lock())
                .field("swap_sect_req", &self.swap_sect_req())
                .field("alt_sect", &self.alt_sect())
                .field("alt_sect_erase", &self.alt_sect_erase())
                .field("swap_offset", &self.swap_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsobkcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nsobkcfgr {{ lock: {=bool:?}, swap_sect_req: {=bool:?}, alt_sect: {=bool:?}, alt_sect_erase: {=bool:?}, swap_offset: {=u16:?} }}",
                self.lock(),
                self.swap_sect_req(),
                self.alt_sect(),
                self.alt_sect_erase(),
                self.swap_offset()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nssr(pub u32);
    impl Nssr {
        #[must_use]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Nssr {
        #[inline(always)]
        fn default() -> Nssr {
            Nssr(0)
        }
    }
    impl core::fmt::Debug for Nssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nssr")
                .field("bsy", &self.bsy())
                .field("wbne", &self.wbne())
                .field("dbne", &self.dbne())
                .field("eop", &self.eop())
                .field("wrperr", &self.wrperr())
                .field("pgserr", &self.pgserr())
                .field("strberr", &self.strberr())
                .field("incerr", &self.incerr())
                .field("obkerr", &self.obkerr())
                .field("obkwerr", &self.obkwerr())
                .field("optchangeerr", &self.optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nssr {{ bsy: {=bool:?}, wbne: {=bool:?}, dbne: {=bool:?}, eop: {=bool:?}, wrperr: {=bool:?}, pgserr: {=bool:?}, strberr: {=bool:?}, incerr: {=bool:?}, obkerr: {=bool:?}, obkwerr: {=bool:?}, optchangeerr: {=bool:?} }}",
                self.bsy(),
                self.wbne(),
                self.dbne(),
                self.eop(),
                self.wrperr(),
                self.pgserr(),
                self.strberr(),
                self.incerr(),
                self.obkerr(),
                self.obkwerr(),
                self.optchangeerr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opsr(pub u32);
    impl Opsr {
        #[must_use]
        #[inline(always)]
        pub const fn addr_op(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_addr_op(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn data_op(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_data_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bk_op(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bk_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sysf_op(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sysf_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn otp_op(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_otp_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn code_op(&self) -> super::vals::CodeOp {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::CodeOp::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_code_op(&mut self, val: super::vals::CodeOp) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Opsr {
        #[inline(always)]
        fn default() -> Opsr {
            Opsr(0)
        }
    }
    impl core::fmt::Debug for Opsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opsr")
                .field("addr_op", &self.addr_op())
                .field("data_op", &self.data_op())
                .field("bk_op", &self.bk_op())
                .field("sysf_op", &self.sysf_op())
                .field("otp_op", &self.otp_op())
                .field("code_op", &self.code_op())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Opsr {{ addr_op: {=u32:?}, data_op: {=bool:?}, bk_op: {=bool:?}, sysf_op: {=bool:?}, otp_op: {=bool:?}, code_op: {:?} }}",
                self.addr_op(),
                self.data_op(),
                self.bk_op(),
                self.sysf_op(),
                self.otp_op(),
                self.code_op()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr(pub u32);
    impl Optcr {
        #[must_use]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn optstart(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_optstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::OptcrSwapBank {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OptcrSwapBank::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_swap_bank(&mut self, val: super::vals::OptcrSwapBank) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optcr {
        #[inline(always)]
        fn default() -> Optcr {
            Optcr(0)
        }
    }
    impl core::fmt::Debug for Optcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optcr")
                .field("optlock", &self.optlock())
                .field("optstart", &self.optstart())
                .field("swap_bank", &self.swap_bank())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optcr {{ optlock: {=bool:?}, optstart: {=bool:?}, swap_bank: {:?} }}",
                self.optlock(),
                self.optstart(),
                self.swap_bank()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr(pub u32);
    impl Optsr {
        #[must_use]
        #[inline(always)]
        pub const fn bor_lev(&self) -> super::vals::OptsrBorLev {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::OptsrBorLev::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_bor_lev(&mut self, val: super::vals::OptsrBorLev) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn borh_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_borh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::OptsrIwdgSw {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::OptsrIwdgSw::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_iwdg_sw(&mut self, val: super::vals::OptsrIwdgSw) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> super::vals::OptsrWwdgSw {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrWwdgSw::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_wwdg_sw(&mut self, val: super::vals::OptsrWwdgSw) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> super::vals::OptsrNrstStop {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrNrstStop::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_nrst_stop(&mut self, val: super::vals::OptsrNrstStop) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> super::vals::OptsrNrstStdby {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::OptsrNrstStdby::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_nrst_stdby(&mut self, val: super::vals::OptsrNrstStdby) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn product_state(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_product_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn io_vdd_hslv(&self) -> super::vals::OptsrIoVddHslv {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::OptsrIoVddHslv::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_io_vdd_hslv(&mut self, val: super::vals::OptsrIoVddHslv) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn io_vddio2_hslv(&self) -> super::vals::OptsrIoVddioHslv {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::OptsrIoVddioHslv::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_io_vddio2_hslv(&mut self, val: super::vals::OptsrIoVddioHslv) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> super::vals::OptsrIwdgStop {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::OptsrIwdgStop::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_iwdg_stop(&mut self, val: super::vals::OptsrIwdgStop) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> super::vals::OptsrIwdgStdby {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::OptsrIwdgStdby::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_iwdg_stdby(&mut self, val: super::vals::OptsrIwdgStdby) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn boot_ube(&self) -> super::vals::OptsrBootUbe {
            let val = (self.0 >> 22usize) & 0xff;
            super::vals::OptsrBootUbe::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_boot_ube(&mut self, val: super::vals::OptsrBootUbe) {
            self.0 = (self.0 & !(0xff << 22usize)) | (((val.to_bits() as u32) & 0xff) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::OptsrSwapBank {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OptsrSwapBank::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_swap_bank(&mut self, val: super::vals::OptsrSwapBank) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optsr {
        #[inline(always)]
        fn default() -> Optsr {
            Optsr(0)
        }
    }
    impl core::fmt::Debug for Optsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr")
                .field("bor_lev", &self.bor_lev())
                .field("borh_en", &self.borh_en())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stdby", &self.nrst_stdby())
                .field("product_state", &self.product_state())
                .field("io_vdd_hslv", &self.io_vdd_hslv())
                .field("io_vddio2_hslv", &self.io_vddio2_hslv())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("boot_ube", &self.boot_ube())
                .field("swap_bank", &self.swap_bank())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr {{ bor_lev: {:?}, borh_en: {=bool:?}, iwdg_sw: {:?}, wwdg_sw: {:?}, nrst_stop: {:?}, nrst_stdby: {:?}, product_state: {=u8:?}, io_vdd_hslv: {:?}, io_vddio2_hslv: {:?}, iwdg_stop: {:?}, iwdg_stdby: {:?}, boot_ube: {:?}, swap_bank: {:?} }}",
                self.bor_lev(),
                self.borh_en(),
                self.iwdg_sw(),
                self.wwdg_sw(),
                self.nrst_stop(),
                self.nrst_stdby(),
                self.product_state(),
                self.io_vdd_hslv(),
                self.io_vddio2_hslv(),
                self.iwdg_stop(),
                self.iwdg_stdby(),
                self.boot_ube(),
                self.swap_bank()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr2(pub u32);
    impl Optsr2 {
        #[must_use]
        #[inline(always)]
        pub const fn sram1_3_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sram1_3_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bkpram_ecc(&self) -> super::vals::OptsrBkpramEcc {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrBkpramEcc::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_bkpram_ecc(&mut self, val: super::vals::OptsrBkpramEcc) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sram3_ecc(&self) -> super::vals::OptsrSramEcc {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::OptsrSramEcc::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_sram3_ecc(&mut self, val: super::vals::OptsrSramEcc) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> super::vals::OptsrSramEcc {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrSramEcc::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_sram2_ecc(&mut self, val: super::vals::OptsrSramEcc) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usbpd_dis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usbpd_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tzen(&self) -> super::vals::OptsrTzen {
            let val = (self.0 >> 24usize) & 0xff;
            super::vals::OptsrTzen::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_tzen(&mut self, val: super::vals::OptsrTzen) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Optsr2 {
        #[inline(always)]
        fn default() -> Optsr2 {
            Optsr2(0)
        }
    }
    impl core::fmt::Debug for Optsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr2")
                .field("sram1_3_rst", &self.sram1_3_rst())
                .field("sram2_rst", &self.sram2_rst())
                .field("bkpram_ecc", &self.bkpram_ecc())
                .field("sram3_ecc", &self.sram3_ecc())
                .field("sram2_ecc", &self.sram2_ecc())
                .field("usbpd_dis", &self.usbpd_dis())
                .field("tzen", &self.tzen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr2 {{ sram1_3_rst: {=bool:?}, sram2_rst: {=bool:?}, bkpram_ecc: {:?}, sram3_ecc: {:?}, sram2_ecc: {:?}, usbpd_dis: {=bool:?}, tzen: {:?} }}",
                self.sram1_3_rst(),
                self.sram2_rst(),
                self.bkpram_ecc(),
                self.sram3_ecc(),
                self.sram2_ecc(),
                self.usbpd_dis(),
                self.tzen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb(pub u32);
    impl Privbb {
        #[must_use]
        #[inline(always)]
        pub const fn privbb(&self) -> super::vals::PrivbbrPrivbb {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::PrivbbrPrivbb::from_bits(val as u32)
        }
        #[inline(always)]
        pub const fn set_privbb(&mut self, val: super::vals::PrivbbrPrivbb) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize))
                | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Privbb {
        #[inline(always)]
        fn default() -> Privbb {
            Privbb(0)
        }
    }
    impl core::fmt::Debug for Privbb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb")
                .field("privbb", &self.privbb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Privbb {{ privbb: {:?} }}", self.privbb())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[must_use]
        #[inline(always)]
        pub const fn spriv(&self) -> super::vals::Spriv {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Spriv::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_spriv(&mut self, val: super::vals::Spriv) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nspriv(&self) -> super::vals::Nspriv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Nspriv::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_nspriv(&mut self, val: super::vals::Nspriv) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("spriv", &self.spriv())
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {:?}, nspriv: {:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbootr(pub u32);
    impl Secbootr {
        #[must_use]
        #[inline(always)]
        pub const fn boot_lock(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_boot_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_bootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Secbootr {
        #[inline(always)]
        fn default() -> Secbootr {
            Secbootr(0)
        }
    }
    impl core::fmt::Debug for Secbootr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbootr")
                .field("boot_lock", &self.boot_lock())
                .field("bootadd", &self.bootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbootr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secbootr {{ boot_lock: {=u8:?}, bootadd: {=u32:?} }}",
                self.boot_lock(),
                self.bootadd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secccr(pub u32);
    impl Secccr {
        #[must_use]
        #[inline(always)]
        pub const fn clr_eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clr_optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clr_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Secccr {
        #[inline(always)]
        fn default() -> Secccr {
            Secccr(0)
        }
    }
    impl core::fmt::Debug for Secccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secccr")
                .field("clr_eop", &self.clr_eop())
                .field("clr_wrperr", &self.clr_wrperr())
                .field("clr_pgserr", &self.clr_pgserr())
                .field("clr_strberr", &self.clr_strberr())
                .field("clr_incerr", &self.clr_incerr())
                .field("clr_obkerr", &self.clr_obkerr())
                .field("clr_obkwerr", &self.clr_obkwerr())
                .field("clr_optchangeerr", &self.clr_optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secccr {{ clr_eop: {=bool:?}, clr_wrperr: {=bool:?}, clr_pgserr: {=bool:?}, clr_strberr: {=bool:?}, clr_incerr: {=bool:?}, clr_obkerr: {=bool:?}, clr_obkwerr: {=bool:?}, clr_optchangeerr: {=bool:?} }}",
                self.clr_eop(),
                self.clr_wrperr(),
                self.clr_pgserr(),
                self.clr_strberr(),
                self.clr_incerr(),
                self.clr_obkerr(),
                self.clr_obkwerr(),
                self.clr_optchangeerr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccr(pub u32);
    impl Seccr {
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ser(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ser(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn snb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_snb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obkerrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obkerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obkwerrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obkwerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_optchangeerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bksel(&self) -> super::vals::SeccrBksel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::SeccrBksel::from_bits(val as u8)
        }
        #[inline(always)]
        pub const fn set_bksel(&mut self, val: super::vals::SeccrBksel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccr {
        #[inline(always)]
        fn default() -> Seccr {
            Seccr(0)
        }
    }
    impl core::fmt::Debug for Seccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccr")
                .field("lock", &self.lock())
                .field("pg", &self.pg())
                .field("ser", &self.ser())
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("start", &self.start())
                .field("snb", &self.snb())
                .field("mer", &self.mer())
                .field("eopie", &self.eopie())
                .field("wrperrie", &self.wrperrie())
                .field("pgserrie", &self.pgserrie())
                .field("strberrie", &self.strberrie())
                .field("incerrie", &self.incerrie())
                .field("obkerrie", &self.obkerrie())
                .field("obkwerrie", &self.obkwerrie())
                .field("optchangeerrie", &self.optchangeerrie())
                .field("inv", &self.inv())
                .field("bksel", &self.bksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seccr {{ lock: {=bool:?}, pg: {=bool:?}, ser: {=bool:?}, ber: {=bool:?}, fw: {=bool:?}, start: {=bool:?}, snb: {=u8:?}, mer: {=bool:?}, eopie: {=bool:?}, wrperrie: {=bool:?}, pgserrie: {=bool:?}, strberrie: {=bool:?}, incerrie: {=bool:?}, obkerrie: {=bool:?}, obkwerrie: {=bool:?}, optchangeerrie: {=bool:?}, inv: {=bool:?}, bksel: {:?} }}",
                self.lock(),
                self.pg(),
                self.ser(),
                self.ber(),
                self.fw(),
                self.start(),
                self.snb(),
                self.mer(),
                self.eopie(),
                self.wrperrie(),
                self.pgserrie(),
                self.strberrie(),
                self.incerrie(),
                self.obkerrie(),
                self.obkwerrie(),
                self.optchangeerrie(),
                self.inv(),
                self.bksel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secepochr(pub u32);
    impl Secepochr {
        #[must_use]
        #[inline(always)]
        pub const fn epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Secepochr {
        #[inline(always)]
        fn default() -> Secepochr {
            Secepochr(0)
        }
    }
    impl core::fmt::Debug for Secepochr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secepochr")
                .field("epoch", &self.epoch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secepochr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secepochr {{ epoch: {=u32:?} }}", self.epoch())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecepochrPrg(pub u32);
    impl SecepochrPrg {
        #[must_use]
        #[inline(always)]
        pub const fn epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for SecepochrPrg {
        #[inline(always)]
        fn default() -> SecepochrPrg {
            SecepochrPrg(0)
        }
    }
    impl core::fmt::Debug for SecepochrPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SecepochrPrg")
                .field("epoch", &self.epoch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SecepochrPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SecepochrPrg {{ epoch: {=u32:?} }}", self.epoch())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secobkcfgr(pub u32);
    impl Secobkcfgr {
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swap_sect_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swap_sect_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alt_sect(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alt_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alt_sect_erase(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alt_sect_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swap_offset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_swap_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Secobkcfgr {
        #[inline(always)]
        fn default() -> Secobkcfgr {
            Secobkcfgr(0)
        }
    }
    impl core::fmt::Debug for Secobkcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secobkcfgr")
                .field("lock", &self.lock())
                .field("swap_sect_req", &self.swap_sect_req())
                .field("alt_sect", &self.alt_sect())
                .field("alt_sect_erase", &self.alt_sect_erase())
                .field("swap_offset", &self.swap_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secobkcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secobkcfgr {{ lock: {=bool:?}, swap_sect_req: {=bool:?}, alt_sect: {=bool:?}, alt_sect_erase: {=bool:?}, swap_offset: {=u16:?} }}",
                self.lock(),
                self.swap_sect_req(),
                self.alt_sect(),
                self.alt_sect_erase(),
                self.swap_offset()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secsr(pub u32);
    impl Secsr {
        #[must_use]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Secsr {
        #[inline(always)]
        fn default() -> Secsr {
            Secsr(0)
        }
    }
    impl core::fmt::Debug for Secsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secsr")
                .field("bsy", &self.bsy())
                .field("wbne", &self.wbne())
                .field("dbne", &self.dbne())
                .field("eop", &self.eop())
                .field("wrperr", &self.wrperr())
                .field("pgserr", &self.pgserr())
                .field("strberr", &self.strberr())
                .field("incerr", &self.incerr())
                .field("obkerr", &self.obkerr())
                .field("obkwerr", &self.obkwerr())
                .field("optchangeerr", &self.optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secsr {{ bsy: {=bool:?}, wbne: {=bool:?}, dbne: {=bool:?}, eop: {=bool:?}, wrperr: {=bool:?}, pgserr: {=bool:?}, strberr: {=bool:?}, incerr: {=bool:?}, obkerr: {=bool:?}, obkwerr: {=bool:?}, optchangeerr: {=bool:?} }}",
                self.bsy(),
                self.wbne(),
                self.dbne(),
                self.eop(),
                self.wrperr(),
                self.pgserr(),
                self.strberr(),
                self.incerr(),
                self.obkerr(),
                self.obkwerr(),
                self.optchangeerr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm(pub u32);
    impl Secwm {
        #[must_use]
        #[inline(always)]
        pub const fn secwm_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_secwm_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn secwm_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_secwm_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Secwm {
        #[inline(always)]
        fn default() -> Secwm {
            Secwm(0)
        }
    }
    impl core::fmt::Debug for Secwm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm")
                .field("secwm_strt", &self.secwm_strt())
                .field("secwm_end", &self.secwm_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secwm {{ secwm_strt: {=u8:?}, secwm_end: {=u8:?} }}",
                self.secwm_strt(),
                self.secwm_end()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp(pub u32);
    impl Wrp {
        #[must_use]
        #[inline(always)]
        pub const fn wrpsg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_wrpsg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wrp {
        #[inline(always)]
        fn default() -> Wrp {
            Wrp(0)
        }
    }
    impl core::fmt::Debug for Wrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp").field("wrpsg", &self.wrpsg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp {{ wrpsg: {=u16:?} }}", self.wrpsg())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CodeOp {
        #[doc = "No flash operation on going during previous reset"]
        B0x0 = 0x0,
        #[doc = "Single write operation interrupted"]
        B0x1 = 0x01,
        #[doc = "OBK alternate sector erase"]
        B0x2 = 0x02,
        #[doc = "Sector erase operation interrupted"]
        B0x3 = 0x03,
        #[doc = "Bank erase operation interrupted"]
        B0x4 = 0x04,
        #[doc = "Mass erase operation interrupted"]
        B0x5 = 0x05,
        #[doc = "Option change operation interrupted"]
        B0x6 = 0x06,
        #[doc = "OBK swap sector request"]
        B0x7 = 0x07,
    }
    impl CodeOp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CodeOp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CodeOp {
        #[inline(always)]
        fn from(val: u8) -> CodeOp {
            CodeOp::from_bits(val)
        }
    }
    impl From<CodeOp> for u8 {
        #[inline(always)]
        fn from(val: CodeOp) -> u8 {
            CodeOp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum NscrBksel {
        #[doc = "Bank1 is selected for Bank erase / sector erase / interrupt enable"]
        B0x0 = 0x0,
        #[doc = "Bank2 is selected for BER / SER"]
        B0x1 = 0x01,
    }
    impl NscrBksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NscrBksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NscrBksel {
        #[inline(always)]
        fn from(val: u8) -> NscrBksel {
            NscrBksel::from_bits(val)
        }
    }
    impl From<NscrBksel> for u8 {
        #[inline(always)]
        fn from(val: NscrBksel) -> u8 {
            NscrBksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nspriv {
        #[doc = "access to non secure registers is always granted"]
        B0x0 = 0x0,
        #[doc = "access to non secure registers is denied in case of unprivileged access."]
        B0x1 = 0x01,
    }
    impl Nspriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nspriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nspriv {
        #[inline(always)]
        fn from(val: u8) -> Nspriv {
            Nspriv::from_bits(val)
        }
    }
    impl From<Nspriv> for u8 {
        #[inline(always)]
        fn from(val: Nspriv) -> u8 {
            Nspriv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptcrSwapBank {
        #[doc = "Bank1 and Bank2 not swapped"]
        B0x0 = 0x0,
        #[doc = "Bank1 and Bank2 swapped"]
        B0x1 = 0x01,
    }
    impl OptcrSwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptcrSwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptcrSwapBank {
        #[inline(always)]
        fn from(val: u8) -> OptcrSwapBank {
            OptcrSwapBank::from_bits(val)
        }
    }
    impl From<OptcrSwapBank> for u8 {
        #[inline(always)]
        fn from(val: OptcrSwapBank) -> u8 {
            OptcrSwapBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrBkpramEcc {
        #[doc = "BKPRAM ECC check enabled"]
        B0x0 = 0x0,
        #[doc = "BKPRAM ECC check disabled"]
        B0x1 = 0x01,
    }
    impl OptsrBkpramEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrBkpramEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrBkpramEcc {
        #[inline(always)]
        fn from(val: u8) -> OptsrBkpramEcc {
            OptsrBkpramEcc::from_bits(val)
        }
    }
    impl From<OptsrBkpramEcc> for u8 {
        #[inline(always)]
        fn from(val: OptsrBkpramEcc) -> u8 {
            OptsrBkpramEcc::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OptsrBootUbe(u8);
    impl OptsrBootUbe {
        #[doc = "OEM-iRoT (user flash) selected. In Open PRODUCT_STATE this value selects bootloader. Defaut value."]
        pub const B0xB4: Self = Self(0xb4);
        #[doc = "ST-iRoT (system flash) selected"]
        pub const B0xC3: Self = Self(0xc3);
    }
    impl OptsrBootUbe {
        pub const fn from_bits(val: u8) -> OptsrBootUbe {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for OptsrBootUbe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B0xB4"),
                0xc3 => f.write_str("B0xC3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OptsrBootUbe {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B0xB4"),
                0xc3 => defmt::write!(f, "B0xC3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for OptsrBootUbe {
        #[inline(always)]
        fn from(val: u8) -> OptsrBootUbe {
            OptsrBootUbe::from_bits(val)
        }
    }
    impl From<OptsrBootUbe> for u8 {
        #[inline(always)]
        fn from(val: OptsrBootUbe) -> u8 {
            OptsrBootUbe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrBorLev {
        _RESERVED_0 = 0x0,
        #[doc = "BOR Level 2, the threshold level is medium (around 2.4 V)"]
        B0x1 = 0x01,
        #[doc = "BOR Level 3, the threshold level is high (around 2.7 V)"]
        B0x2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl OptsrBorLev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrBorLev {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrBorLev {
        #[inline(always)]
        fn from(val: u8) -> OptsrBorLev {
            OptsrBorLev::from_bits(val)
        }
    }
    impl From<OptsrBorLev> for u8 {
        #[inline(always)]
        fn from(val: OptsrBorLev) -> u8 {
            OptsrBorLev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIoVddHslv {
        #[doc = "High-speed IO at low V<sub>DD</sub> voltage feature disabled (V<sub>DD</sub> can exceed 2.7�V)"]
        B0x0 = 0x0,
        #[doc = "High-speed IO at low V<sub>DD</sub> voltage feature enabled (V<sub>DD</sub> remains below 2.7�V)"]
        B0x1 = 0x01,
    }
    impl OptsrIoVddHslv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIoVddHslv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIoVddHslv {
        #[inline(always)]
        fn from(val: u8) -> OptsrIoVddHslv {
            OptsrIoVddHslv::from_bits(val)
        }
    }
    impl From<OptsrIoVddHslv> for u8 {
        #[inline(always)]
        fn from(val: OptsrIoVddHslv) -> u8 {
            OptsrIoVddHslv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIoVddioHslv {
        #[doc = "High-speed IO at low V<sub>DDIO2</sub> voltage feature disabled (V<sub>DDIO2</sub> can exceed 2.7�V)"]
        B0x0 = 0x0,
        #[doc = "High-speed IO at low V<sub>DDIO2</sub> voltage feature enabled (V<sub>DDIO2</sub> remains below 2.7�V)"]
        B0x1 = 0x01,
    }
    impl OptsrIoVddioHslv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIoVddioHslv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIoVddioHslv {
        #[inline(always)]
        fn from(val: u8) -> OptsrIoVddioHslv {
            OptsrIoVddioHslv::from_bits(val)
        }
    }
    impl From<OptsrIoVddioHslv> for u8 {
        #[inline(always)]
        fn from(val: OptsrIoVddioHslv) -> u8 {
            OptsrIoVddioHslv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgStdby {
        #[doc = "Independent watchdog frozen in Standby mode"]
        B0x0 = 0x0,
        #[doc = "Independent watchdog keep running in Standby mode."]
        B0x1 = 0x01,
    }
    impl OptsrIwdgStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgStdby {
            OptsrIwdgStdby::from_bits(val)
        }
    }
    impl From<OptsrIwdgStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgStdby) -> u8 {
            OptsrIwdgStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgStop {
        #[doc = "Independent watchdog frozen in system Stop mode"]
        B0x0 = 0x0,
        #[doc = "Independent watchdog keep running in system Stop mode."]
        B0x1 = 0x01,
    }
    impl OptsrIwdgStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgStop {
            OptsrIwdgStop::from_bits(val)
        }
    }
    impl From<OptsrIwdgStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgStop) -> u8 {
            OptsrIwdgStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgSw {
        #[doc = "IWDG watchdog is controlled by hardware"]
        B0x0 = 0x0,
        #[doc = "IWDG watchdog is controlled by software"]
        B0x1 = 0x01,
    }
    impl OptsrIwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgSw {
            OptsrIwdgSw::from_bits(val)
        }
    }
    impl From<OptsrIwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgSw) -> u8 {
            OptsrIwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrNrstStdby {
        #[doc = "a reset is generated when entering Standby mode on core domain"]
        B0x0 = 0x0,
        #[doc = "no reset generated when entering Standby mode on core domain."]
        B0x1 = 0x01,
    }
    impl OptsrNrstStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstStdby {
            OptsrNrstStdby::from_bits(val)
        }
    }
    impl From<OptsrNrstStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstStdby) -> u8 {
            OptsrNrstStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrNrstStop {
        #[doc = "a reset is generated when entering Stop mode on core domain"]
        B0x0 = 0x0,
        #[doc = "no reset generated when entering Stop mode on core domain."]
        B0x1 = 0x01,
    }
    impl OptsrNrstStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstStop {
            OptsrNrstStop::from_bits(val)
        }
    }
    impl From<OptsrNrstStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstStop) -> u8 {
            OptsrNrstStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrSramEcc {
        #[doc = "SRAM2 ECC check enabled"]
        B0x0 = 0x0,
        #[doc = "SRAM2 ECC check disabled"]
        B0x1 = 0x01,
    }
    impl OptsrSramEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrSramEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrSramEcc {
        #[inline(always)]
        fn from(val: u8) -> OptsrSramEcc {
            OptsrSramEcc::from_bits(val)
        }
    }
    impl From<OptsrSramEcc> for u8 {
        #[inline(always)]
        fn from(val: OptsrSramEcc) -> u8 {
            OptsrSramEcc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrSwapBank {
        #[doc = "Bank1 and Bank2 not swapped"]
        B0x0 = 0x0,
        #[doc = "Bank1 and Bank2 swapped"]
        B0x1 = 0x01,
    }
    impl OptsrSwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrSwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrSwapBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrSwapBank {
            OptsrSwapBank::from_bits(val)
        }
    }
    impl From<OptsrSwapBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrSwapBank) -> u8 {
            OptsrSwapBank::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OptsrTzen(u8);
    impl OptsrTzen {
        #[doc = "TrustZone enabled."]
        pub const B0xB4: Self = Self(0xb4);
        #[doc = "TrustZone disabled"]
        pub const B0xC3: Self = Self(0xc3);
    }
    impl OptsrTzen {
        pub const fn from_bits(val: u8) -> OptsrTzen {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for OptsrTzen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B0xB4"),
                0xc3 => f.write_str("B0xC3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OptsrTzen {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B0xB4"),
                0xc3 => defmt::write!(f, "B0xC3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for OptsrTzen {
        #[inline(always)]
        fn from(val: u8) -> OptsrTzen {
            OptsrTzen::from_bits(val)
        }
    }
    impl From<OptsrTzen> for u8 {
        #[inline(always)]
        fn from(val: OptsrTzen) -> u8 {
            OptsrTzen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrWwdgSw {
        #[doc = "WWDG watchdog is controlled by hardware"]
        B0x0 = 0x0,
        #[doc = "WWDG watchdog is controlled by software"]
        B0x1 = 0x01,
    }
    impl OptsrWwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrWwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrWwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrWwdgSw {
            OptsrWwdgSw::from_bits(val)
        }
    }
    impl From<OptsrWwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrWwdgSw) -> u8 {
            OptsrWwdgSw::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct PrivbbrPrivbb(u32);
    impl PrivbbrPrivbb {
        #[doc = "sectors (32 * (x-1) + y) in bank 2 is unprivileged"]
        pub const B0x0: Self = Self(0x0);
        #[doc = "sector (32 * (x-1) + y) in bank 2 is privileged"]
        pub const B0x1: Self = Self(0x01);
    }
    impl PrivbbrPrivbb {
        pub const fn from_bits(val: u32) -> PrivbbrPrivbb {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for PrivbbrPrivbb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PrivbbrPrivbb {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for PrivbbrPrivbb {
        #[inline(always)]
        fn from(val: u32) -> PrivbbrPrivbb {
            PrivbbrPrivbb::from_bits(val)
        }
    }
    impl From<PrivbbrPrivbb> for u32 {
        #[inline(always)]
        fn from(val: PrivbbrPrivbb) -> u32 {
            PrivbbrPrivbb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SeccrBksel {
        #[doc = "Bank1 is selected for Bank erase / sector erase / interrupt enable"]
        B0x0 = 0x0,
        #[doc = "Bank2 is selected for BER / SER"]
        B0x1 = 0x01,
    }
    impl SeccrBksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SeccrBksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SeccrBksel {
        #[inline(always)]
        fn from(val: u8) -> SeccrBksel {
            SeccrBksel::from_bits(val)
        }
    }
    impl From<SeccrBksel> for u8 {
        #[inline(always)]
        fn from(val: SeccrBksel) -> u8 {
            SeccrBksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spriv {
        #[doc = "access to secure registers is always granted"]
        B0x0 = 0x0,
        #[doc = "access to secure registers is denied in case of unprivileged access."]
        B0x1 = 0x01,
    }
    impl Spriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spriv {
        #[inline(always)]
        fn from(val: u8) -> Spriv {
            Spriv::from_bits(val)
        }
    }
    impl From<Spriv> for u8 {
        #[inline(always)]
        fn from(val: Spriv) -> u8 {
            Spriv::to_bits(val)
        }
    }
}
