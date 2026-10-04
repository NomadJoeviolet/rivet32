#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmmc {
    ptr: *mut u8,
}
unsafe impl Send for Sdmmc {}
unsafe impl Sync for Sdmmc {}
impl Sdmmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::SdmmcPower, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn clkcr(self) -> crate::common::Reg<regs::SdmmcClkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn arg(self) -> crate::common::Reg<regs::SdmmcArg, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::SdmmcCmd, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn respcmd(self) -> crate::common::Reg<regs::SdmmcRespcmd, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn resp1(self) -> crate::common::Reg<regs::SdmmcResp1, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn resp2(self) -> crate::common::Reg<regs::SdmmcResp2, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn resp3(self) -> crate::common::Reg<regs::SdmmcResp3, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn resp4(self) -> crate::common::Reg<regs::SdmmcResp4, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn dtimer(self) -> crate::common::Reg<regs::SdmmcDtimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn dlen(self) -> crate::common::Reg<regs::SdmmcDlen, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn dctrl(self) -> crate::common::Reg<regs::SdmmcDctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn dcount(self) -> crate::common::Reg<regs::SdmmcDcount, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::SdmmcSta, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::SdmmcIcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<regs::SdmmcMask, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn acktime(self) -> crate::common::Reg<regs::SdmmcAcktime, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn idmactrl(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn idmabsize(self) -> crate::common::Reg<regs::SdmmcIdmabsize, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn idmabaser(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn idmalar(self) -> crate::common::Reg<regs::SdmmcIdmalar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn idmabar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::SdmmcFifo, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcAcktime(pub u32);
    impl SdmmcAcktime {
        #[must_use]
        #[inline(always)]
        pub const fn acktime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_acktime(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcAcktime {
        #[inline(always)]
        fn default() -> SdmmcAcktime {
            SdmmcAcktime(0)
        }
    }
    impl core::fmt::Debug for SdmmcAcktime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcAcktime")
                .field("acktime", &self.acktime())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcAcktime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SdmmcAcktime {{ acktime: {=u32:?} }}", self.acktime())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcArg(pub u32);
    impl SdmmcArg {
        #[must_use]
        #[inline(always)]
        pub const fn cmdarg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_cmdarg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcArg {
        #[inline(always)]
        fn default() -> SdmmcArg {
            SdmmcArg(0)
        }
    }
    impl core::fmt::Debug for SdmmcArg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcArg")
                .field("cmdarg", &self.cmdarg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcArg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SdmmcArg {{ cmdarg: {=u32:?} }}", self.cmdarg())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcClkcr(pub u32);
    impl SdmmcClkcr {
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pwrsav(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pwrsav(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn widbus(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_widbus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn negedge(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_negedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hwfc_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hwfc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ddr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busspeed(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busspeed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn selclkrx(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_selclkrx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for SdmmcClkcr {
        #[inline(always)]
        fn default() -> SdmmcClkcr {
            SdmmcClkcr(0)
        }
    }
    impl core::fmt::Debug for SdmmcClkcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcClkcr")
                .field("clkdiv", &self.clkdiv())
                .field("pwrsav", &self.pwrsav())
                .field("widbus", &self.widbus())
                .field("negedge", &self.negedge())
                .field("hwfc_en", &self.hwfc_en())
                .field("ddr", &self.ddr())
                .field("busspeed", &self.busspeed())
                .field("selclkrx", &self.selclkrx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcClkcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcClkcr {{ clkdiv: {=u16:?}, pwrsav: {=bool:?}, widbus: {=u8:?}, negedge: {=bool:?}, hwfc_en: {=bool:?}, ddr: {=bool:?}, busspeed: {=bool:?}, selclkrx: {=u8:?} }}",
                self.clkdiv(),
                self.pwrsav(),
                self.widbus(),
                self.negedge(),
                self.hwfc_en(),
                self.ddr(),
                self.busspeed(),
                self.selclkrx()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcCmd(pub u32);
    impl SdmmcCmd {
        #[must_use]
        #[inline(always)]
        pub const fn cmdindex(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cmdindex(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdtrans(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdtrans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdstop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn waitresp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_waitresp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn waitint(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_waitint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn waitpend(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_waitpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cpsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cpsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dthold(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dthold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bootmode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bootmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn booten(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_booten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdsuspend(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdsuspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for SdmmcCmd {
        #[inline(always)]
        fn default() -> SdmmcCmd {
            SdmmcCmd(0)
        }
    }
    impl core::fmt::Debug for SdmmcCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcCmd")
                .field("cmdindex", &self.cmdindex())
                .field("cmdtrans", &self.cmdtrans())
                .field("cmdstop", &self.cmdstop())
                .field("waitresp", &self.waitresp())
                .field("waitint", &self.waitint())
                .field("waitpend", &self.waitpend())
                .field("cpsmen", &self.cpsmen())
                .field("dthold", &self.dthold())
                .field("bootmode", &self.bootmode())
                .field("booten", &self.booten())
                .field("cmdsuspend", &self.cmdsuspend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcCmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcCmd {{ cmdindex: {=u8:?}, cmdtrans: {=bool:?}, cmdstop: {=bool:?}, waitresp: {=u8:?}, waitint: {=bool:?}, waitpend: {=bool:?}, cpsmen: {=bool:?}, dthold: {=bool:?}, bootmode: {=bool:?}, booten: {=bool:?}, cmdsuspend: {=bool:?} }}",
                self.cmdindex(),
                self.cmdtrans(),
                self.cmdstop(),
                self.waitresp(),
                self.waitint(),
                self.waitpend(),
                self.cpsmen(),
                self.dthold(),
                self.bootmode(),
                self.booten(),
                self.cmdsuspend()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcDcount(pub u32);
    impl SdmmcDcount {
        #[must_use]
        #[inline(always)]
        pub const fn datacount(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_datacount(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcDcount {
        #[inline(always)]
        fn default() -> SdmmcDcount {
            SdmmcDcount(0)
        }
    }
    impl core::fmt::Debug for SdmmcDcount {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcDcount")
                .field("datacount", &self.datacount())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcDcount {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SdmmcDcount {{ datacount: {=u32:?} }}", self.datacount())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcDctrl(pub u32);
    impl SdmmcDctrl {
        #[must_use]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dtdir(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dtdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dtmode(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dtmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dblocksize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dblocksize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwstart(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwstop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdioen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sdioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bootacken(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bootacken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fiforst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for SdmmcDctrl {
        #[inline(always)]
        fn default() -> SdmmcDctrl {
            SdmmcDctrl(0)
        }
    }
    impl core::fmt::Debug for SdmmcDctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcDctrl")
                .field("dten", &self.dten())
                .field("dtdir", &self.dtdir())
                .field("dtmode", &self.dtmode())
                .field("dblocksize", &self.dblocksize())
                .field("rwstart", &self.rwstart())
                .field("rwstop", &self.rwstop())
                .field("rwmod", &self.rwmod())
                .field("sdioen", &self.sdioen())
                .field("bootacken", &self.bootacken())
                .field("fiforst", &self.fiforst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcDctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcDctrl {{ dten: {=bool:?}, dtdir: {=bool:?}, dtmode: {=u8:?}, dblocksize: {=u8:?}, rwstart: {=bool:?}, rwstop: {=bool:?}, rwmod: {=bool:?}, sdioen: {=bool:?}, bootacken: {=bool:?}, fiforst: {=bool:?} }}",
                self.dten(),
                self.dtdir(),
                self.dtmode(),
                self.dblocksize(),
                self.rwstart(),
                self.rwstop(),
                self.rwmod(),
                self.sdioen(),
                self.bootacken(),
                self.fiforst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcDlen(pub u32);
    impl SdmmcDlen {
        #[must_use]
        #[inline(always)]
        pub const fn datalength(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_datalength(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcDlen {
        #[inline(always)]
        fn default() -> SdmmcDlen {
            SdmmcDlen(0)
        }
    }
    impl core::fmt::Debug for SdmmcDlen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcDlen")
                .field("datalength", &self.datalength())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcDlen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SdmmcDlen {{ datalength: {=u32:?} }}", self.datalength())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcDtimer(pub u32);
    impl SdmmcDtimer {
        #[must_use]
        #[inline(always)]
        pub const fn datatime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_datatime(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcDtimer {
        #[inline(always)]
        fn default() -> SdmmcDtimer {
            SdmmcDtimer(0)
        }
    }
    impl core::fmt::Debug for SdmmcDtimer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcDtimer")
                .field("datatime", &self.datatime())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcDtimer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SdmmcDtimer {{ datatime: {=u32:?} }}", self.datatime())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcFifo(pub u32);
    impl SdmmcFifo {
        #[must_use]
        #[inline(always)]
        pub const fn fifodata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_fifodata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcFifo {
        #[inline(always)]
        fn default() -> SdmmcFifo {
            SdmmcFifo(0)
        }
    }
    impl core::fmt::Debug for SdmmcFifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcFifo")
                .field("fifodata", &self.fifodata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcFifo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SdmmcFifo {{ fifodata: {=u32:?} }}", self.fifodata())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcIcr(pub u32);
    impl SdmmcIcr {
        #[must_use]
        #[inline(always)]
        pub const fn ccrcfailc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ccrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcrcfailc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctimeoutc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dtimeoutc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dtimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txunderrc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txunderrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxoverrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxoverrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdrendc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdrendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdsentc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdsentc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dataendc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dataendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dholdc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dholdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbckendc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbckendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dabortc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dabortc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busyd0endc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busyd0endc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdioitc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sdioitc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ackfailc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ackfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn acktimeoutc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_acktimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vswendc(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vswendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ckstopc(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckstopc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idmatec(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idmatec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idmabtcc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idmabtcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for SdmmcIcr {
        #[inline(always)]
        fn default() -> SdmmcIcr {
            SdmmcIcr(0)
        }
    }
    impl core::fmt::Debug for SdmmcIcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcIcr")
                .field("ccrcfailc", &self.ccrcfailc())
                .field("dcrcfailc", &self.dcrcfailc())
                .field("ctimeoutc", &self.ctimeoutc())
                .field("dtimeoutc", &self.dtimeoutc())
                .field("txunderrc", &self.txunderrc())
                .field("rxoverrc", &self.rxoverrc())
                .field("cmdrendc", &self.cmdrendc())
                .field("cmdsentc", &self.cmdsentc())
                .field("dataendc", &self.dataendc())
                .field("dholdc", &self.dholdc())
                .field("dbckendc", &self.dbckendc())
                .field("dabortc", &self.dabortc())
                .field("busyd0endc", &self.busyd0endc())
                .field("sdioitc", &self.sdioitc())
                .field("ackfailc", &self.ackfailc())
                .field("acktimeoutc", &self.acktimeoutc())
                .field("vswendc", &self.vswendc())
                .field("ckstopc", &self.ckstopc())
                .field("idmatec", &self.idmatec())
                .field("idmabtcc", &self.idmabtcc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcIcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcIcr {{ ccrcfailc: {=bool:?}, dcrcfailc: {=bool:?}, ctimeoutc: {=bool:?}, dtimeoutc: {=bool:?}, txunderrc: {=bool:?}, rxoverrc: {=bool:?}, cmdrendc: {=bool:?}, cmdsentc: {=bool:?}, dataendc: {=bool:?}, dholdc: {=bool:?}, dbckendc: {=bool:?}, dabortc: {=bool:?}, busyd0endc: {=bool:?}, sdioitc: {=bool:?}, ackfailc: {=bool:?}, acktimeoutc: {=bool:?}, vswendc: {=bool:?}, ckstopc: {=bool:?}, idmatec: {=bool:?}, idmabtcc: {=bool:?} }}",
                self.ccrcfailc(),
                self.dcrcfailc(),
                self.ctimeoutc(),
                self.dtimeoutc(),
                self.txunderrc(),
                self.rxoverrc(),
                self.cmdrendc(),
                self.cmdsentc(),
                self.dataendc(),
                self.dholdc(),
                self.dbckendc(),
                self.dabortc(),
                self.busyd0endc(),
                self.sdioitc(),
                self.ackfailc(),
                self.acktimeoutc(),
                self.vswendc(),
                self.ckstopc(),
                self.idmatec(),
                self.idmabtcc()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcIdmabsize(pub u32);
    impl SdmmcIdmabsize {
        #[must_use]
        #[inline(always)]
        pub const fn idmabndt(&self) -> u16 {
            let val = (self.0 >> 5usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_idmabndt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 5usize)) | (((val as u32) & 0x0fff) << 5usize);
        }
    }
    impl Default for SdmmcIdmabsize {
        #[inline(always)]
        fn default() -> SdmmcIdmabsize {
            SdmmcIdmabsize(0)
        }
    }
    impl core::fmt::Debug for SdmmcIdmabsize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcIdmabsize")
                .field("idmabndt", &self.idmabndt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcIdmabsize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcIdmabsize {{ idmabndt: {=u16:?} }}",
                self.idmabndt()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcIdmalar(pub u32);
    impl SdmmcIdmalar {
        #[must_use]
        #[inline(always)]
        pub const fn idmala(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_idmala(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abr(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_abr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn uls(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_uls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ula(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ula(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SdmmcIdmalar {
        #[inline(always)]
        fn default() -> SdmmcIdmalar {
            SdmmcIdmalar(0)
        }
    }
    impl core::fmt::Debug for SdmmcIdmalar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcIdmalar")
                .field("idmala", &self.idmala())
                .field("abr", &self.abr())
                .field("uls", &self.uls())
                .field("ula", &self.ula())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcIdmalar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcIdmalar {{ idmala: {=u16:?}, abr: {=bool:?}, uls: {=bool:?}, ula: {=bool:?} }}",
                self.idmala(),
                self.abr(),
                self.uls(),
                self.ula()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcMask(pub u32);
    impl SdmmcMask {
        #[must_use]
        #[inline(always)]
        pub const fn ccrcfailie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ccrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcrcfailie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctimeoutie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dtimeoutie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dtimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txunderrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txunderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxoverrie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxoverrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdrendie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdrendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdsentie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dataendie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dataendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dholdie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dholdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbckendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbckendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dabortie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dabortie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfifoheie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfifoheie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfifohfie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfifohfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfifofie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfifofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfifoeie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfifoeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busyd0endie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busyd0endie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdioitie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sdioitie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ackfailie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ackfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn acktimeoutie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_acktimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vswendie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vswendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ckstopie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckstopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idmabtcie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idmabtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for SdmmcMask {
        #[inline(always)]
        fn default() -> SdmmcMask {
            SdmmcMask(0)
        }
    }
    impl core::fmt::Debug for SdmmcMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcMask")
                .field("ccrcfailie", &self.ccrcfailie())
                .field("dcrcfailie", &self.dcrcfailie())
                .field("ctimeoutie", &self.ctimeoutie())
                .field("dtimeoutie", &self.dtimeoutie())
                .field("txunderrie", &self.txunderrie())
                .field("rxoverrie", &self.rxoverrie())
                .field("cmdrendie", &self.cmdrendie())
                .field("cmdsentie", &self.cmdsentie())
                .field("dataendie", &self.dataendie())
                .field("dholdie", &self.dholdie())
                .field("dbckendie", &self.dbckendie())
                .field("dabortie", &self.dabortie())
                .field("txfifoheie", &self.txfifoheie())
                .field("rxfifohfie", &self.rxfifohfie())
                .field("rxfifofie", &self.rxfifofie())
                .field("txfifoeie", &self.txfifoeie())
                .field("busyd0endie", &self.busyd0endie())
                .field("sdioitie", &self.sdioitie())
                .field("ackfailie", &self.ackfailie())
                .field("acktimeoutie", &self.acktimeoutie())
                .field("vswendie", &self.vswendie())
                .field("ckstopie", &self.ckstopie())
                .field("idmabtcie", &self.idmabtcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcMask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcMask {{ ccrcfailie: {=bool:?}, dcrcfailie: {=bool:?}, ctimeoutie: {=bool:?}, dtimeoutie: {=bool:?}, txunderrie: {=bool:?}, rxoverrie: {=bool:?}, cmdrendie: {=bool:?}, cmdsentie: {=bool:?}, dataendie: {=bool:?}, dholdie: {=bool:?}, dbckendie: {=bool:?}, dabortie: {=bool:?}, txfifoheie: {=bool:?}, rxfifohfie: {=bool:?}, rxfifofie: {=bool:?}, txfifoeie: {=bool:?}, busyd0endie: {=bool:?}, sdioitie: {=bool:?}, ackfailie: {=bool:?}, acktimeoutie: {=bool:?}, vswendie: {=bool:?}, ckstopie: {=bool:?}, idmabtcie: {=bool:?} }}",
                self.ccrcfailie(),
                self.dcrcfailie(),
                self.ctimeoutie(),
                self.dtimeoutie(),
                self.txunderrie(),
                self.rxoverrie(),
                self.cmdrendie(),
                self.cmdsentie(),
                self.dataendie(),
                self.dholdie(),
                self.dbckendie(),
                self.dabortie(),
                self.txfifoheie(),
                self.rxfifohfie(),
                self.rxfifofie(),
                self.txfifoeie(),
                self.busyd0endie(),
                self.sdioitie(),
                self.ackfailie(),
                self.acktimeoutie(),
                self.vswendie(),
                self.ckstopie(),
                self.idmabtcie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcPower(pub u32);
    impl SdmmcPower {
        #[must_use]
        #[inline(always)]
        pub const fn pwrctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pwrctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vswitch(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vswitch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vswitchen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vswitchen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dirpol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dirpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for SdmmcPower {
        #[inline(always)]
        fn default() -> SdmmcPower {
            SdmmcPower(0)
        }
    }
    impl core::fmt::Debug for SdmmcPower {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcPower")
                .field("pwrctrl", &self.pwrctrl())
                .field("vswitch", &self.vswitch())
                .field("vswitchen", &self.vswitchen())
                .field("dirpol", &self.dirpol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcPower {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcPower {{ pwrctrl: {=u8:?}, vswitch: {=bool:?}, vswitchen: {=bool:?}, dirpol: {=bool:?} }}",
                self.pwrctrl(),
                self.vswitch(),
                self.vswitchen(),
                self.dirpol()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcResp1(pub u32);
    impl SdmmcResp1 {
        #[must_use]
        #[inline(always)]
        pub const fn cardstatus1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_cardstatus1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcResp1 {
        #[inline(always)]
        fn default() -> SdmmcResp1 {
            SdmmcResp1(0)
        }
    }
    impl core::fmt::Debug for SdmmcResp1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcResp1")
                .field("cardstatus1", &self.cardstatus1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcResp1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcResp1 {{ cardstatus1: {=u32:?} }}",
                self.cardstatus1()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcResp2(pub u32);
    impl SdmmcResp2 {
        #[must_use]
        #[inline(always)]
        pub const fn cardstatus2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_cardstatus2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcResp2 {
        #[inline(always)]
        fn default() -> SdmmcResp2 {
            SdmmcResp2(0)
        }
    }
    impl core::fmt::Debug for SdmmcResp2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcResp2")
                .field("cardstatus2", &self.cardstatus2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcResp2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcResp2 {{ cardstatus2: {=u32:?} }}",
                self.cardstatus2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcResp3(pub u32);
    impl SdmmcResp3 {
        #[must_use]
        #[inline(always)]
        pub const fn cardstatus3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_cardstatus3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcResp3 {
        #[inline(always)]
        fn default() -> SdmmcResp3 {
            SdmmcResp3(0)
        }
    }
    impl core::fmt::Debug for SdmmcResp3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcResp3")
                .field("cardstatus3", &self.cardstatus3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcResp3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcResp3 {{ cardstatus3: {=u32:?} }}",
                self.cardstatus3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcResp4(pub u32);
    impl SdmmcResp4 {
        #[must_use]
        #[inline(always)]
        pub const fn cardstatus4(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_cardstatus4(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SdmmcResp4 {
        #[inline(always)]
        fn default() -> SdmmcResp4 {
            SdmmcResp4(0)
        }
    }
    impl core::fmt::Debug for SdmmcResp4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcResp4")
                .field("cardstatus4", &self.cardstatus4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcResp4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcResp4 {{ cardstatus4: {=u32:?} }}",
                self.cardstatus4()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcRespcmd(pub u32);
    impl SdmmcRespcmd {
        #[must_use]
        #[inline(always)]
        pub const fn respcmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_respcmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for SdmmcRespcmd {
        #[inline(always)]
        fn default() -> SdmmcRespcmd {
            SdmmcRespcmd(0)
        }
    }
    impl core::fmt::Debug for SdmmcRespcmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcRespcmd")
                .field("respcmd", &self.respcmd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcRespcmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SdmmcRespcmd {{ respcmd: {=u8:?} }}", self.respcmd())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdmmcSta(pub u32);
    impl SdmmcSta {
        #[must_use]
        #[inline(always)]
        pub const fn ccrcfail(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ccrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcrcfail(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctimeout(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dtimeout(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dtimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txunderr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txunderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxoverr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxoverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdrend(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdrend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cmdsent(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cmdsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dataend(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dataend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dhold(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dhold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbckend(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbckend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dabort(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dabort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dpsmact(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dpsmact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cpsmact(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cpsmact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfifohe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfifohe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfifohf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfifohf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfifof(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfifof(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfifoe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfifoe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busyd0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busyd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busyd0end(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busyd0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdioit(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sdioit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ackfail(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ackfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn acktimeout(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_acktimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vswend(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vswend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ckstop(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idmate(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idmate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn idmabtc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_idmabtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for SdmmcSta {
        #[inline(always)]
        fn default() -> SdmmcSta {
            SdmmcSta(0)
        }
    }
    impl core::fmt::Debug for SdmmcSta {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdmmcSta")
                .field("ccrcfail", &self.ccrcfail())
                .field("dcrcfail", &self.dcrcfail())
                .field("ctimeout", &self.ctimeout())
                .field("dtimeout", &self.dtimeout())
                .field("txunderr", &self.txunderr())
                .field("rxoverr", &self.rxoverr())
                .field("cmdrend", &self.cmdrend())
                .field("cmdsent", &self.cmdsent())
                .field("dataend", &self.dataend())
                .field("dhold", &self.dhold())
                .field("dbckend", &self.dbckend())
                .field("dabort", &self.dabort())
                .field("dpsmact", &self.dpsmact())
                .field("cpsmact", &self.cpsmact())
                .field("txfifohe", &self.txfifohe())
                .field("rxfifohf", &self.rxfifohf())
                .field("txfifof", &self.txfifof())
                .field("rxfifof", &self.rxfifof())
                .field("txfifoe", &self.txfifoe())
                .field("rxfifoe", &self.rxfifoe())
                .field("busyd0", &self.busyd0())
                .field("busyd0end", &self.busyd0end())
                .field("sdioit", &self.sdioit())
                .field("ackfail", &self.ackfail())
                .field("acktimeout", &self.acktimeout())
                .field("vswend", &self.vswend())
                .field("ckstop", &self.ckstop())
                .field("idmate", &self.idmate())
                .field("idmabtc", &self.idmabtc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdmmcSta {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SdmmcSta {{ ccrcfail: {=bool:?}, dcrcfail: {=bool:?}, ctimeout: {=bool:?}, dtimeout: {=bool:?}, txunderr: {=bool:?}, rxoverr: {=bool:?}, cmdrend: {=bool:?}, cmdsent: {=bool:?}, dataend: {=bool:?}, dhold: {=bool:?}, dbckend: {=bool:?}, dabort: {=bool:?}, dpsmact: {=bool:?}, cpsmact: {=bool:?}, txfifohe: {=bool:?}, rxfifohf: {=bool:?}, txfifof: {=bool:?}, rxfifof: {=bool:?}, txfifoe: {=bool:?}, rxfifoe: {=bool:?}, busyd0: {=bool:?}, busyd0end: {=bool:?}, sdioit: {=bool:?}, ackfail: {=bool:?}, acktimeout: {=bool:?}, vswend: {=bool:?}, ckstop: {=bool:?}, idmate: {=bool:?}, idmabtc: {=bool:?} }}",
                self.ccrcfail(),
                self.dcrcfail(),
                self.ctimeout(),
                self.dtimeout(),
                self.txunderr(),
                self.rxoverr(),
                self.cmdrend(),
                self.cmdsent(),
                self.dataend(),
                self.dhold(),
                self.dbckend(),
                self.dabort(),
                self.dpsmact(),
                self.cpsmact(),
                self.txfifohe(),
                self.rxfifohf(),
                self.txfifof(),
                self.rxfifof(),
                self.txfifoe(),
                self.rxfifoe(),
                self.busyd0(),
                self.busyd0end(),
                self.sdioit(),
                self.ackfail(),
                self.acktimeout(),
                self.vswend(),
                self.ckstop(),
                self.idmate(),
                self.idmabtc()
            )
        }
    }
}
