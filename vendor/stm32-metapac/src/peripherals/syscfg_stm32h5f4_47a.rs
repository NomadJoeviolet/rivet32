#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbs {
    ptr: *mut u8,
}
unsafe impl Send for Sbs {}
unsafe impl Sync for Sbs {}
impl Sbs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn hdplcr(self) -> crate::common::Reg<regs::SbsHdplcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn hdplsr(self) -> crate::common::Reg<regs::SbsHdplsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn nexthdplcr(self) -> crate::common::Reg<regs::SbsNexthdplcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn dbgcr(self) -> crate::common::Reg<regs::SbsDbgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn dbglockr(self) -> crate::common::Reg<regs::SbsDbglockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::SbsRsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn epochselcr(self) -> crate::common::Reg<regs::SbsEpochselcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::SbsSeccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::SbsPmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn fpuimr(self) -> crate::common::Reg<regs::SbsFpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn mesr(self) -> crate::common::Reg<regs::SbsMesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::SbsCccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn ccvalr(self) -> crate::common::Reg<regs::SbsCcvalr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn ccswcr(self) -> crate::common::Reg<regs::SbsCcswcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::SbsCfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn cnslckr(self) -> crate::common::Reg<regs::SbsCnslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn cslckr(self) -> crate::common::Reg<regs::SbsCslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn eccnmir(self) -> crate::common::Reg<regs::SbsEccnmir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCccsr(pub u32);
    impl SbsCccsr {
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cs1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cs2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cs2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdy1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rdy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdy2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rdy2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for SbsCccsr {
        #[inline(always)]
        fn default() -> SbsCccsr {
            SbsCccsr(0)
        }
    }
    impl core::fmt::Debug for SbsCccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCccsr")
                .field("en1", &self.en1())
                .field("cs1", &self.cs1())
                .field("en2", &self.en2())
                .field("cs2", &self.cs2())
                .field("rdy1", &self.rdy1())
                .field("rdy2", &self.rdy2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCccsr {{ en1: {=bool:?}, cs1: {=bool:?}, en2: {=bool:?}, cs2: {=bool:?}, rdy1: {=bool:?}, rdy2: {=bool:?} }}",
                self.en1(),
                self.cs1(),
                self.en2(),
                self.cs2(),
                self.rdy1(),
                self.rdy2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCcswcr(pub u32);
    impl SbsCcswcr {
        #[must_use]
        #[inline(always)]
        pub const fn sw_ansrc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sw_ansrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sw_apsrc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sw_apsrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sw_ansrc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sw_ansrc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sw_apsrc2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sw_apsrc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for SbsCcswcr {
        #[inline(always)]
        fn default() -> SbsCcswcr {
            SbsCcswcr(0)
        }
    }
    impl core::fmt::Debug for SbsCcswcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCcswcr")
                .field("sw_ansrc1", &self.sw_ansrc1())
                .field("sw_apsrc1", &self.sw_apsrc1())
                .field("sw_ansrc2", &self.sw_ansrc2())
                .field("sw_apsrc2", &self.sw_apsrc2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCcswcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCcswcr {{ sw_ansrc1: {=u8:?}, sw_apsrc1: {=u8:?}, sw_ansrc2: {=u8:?}, sw_apsrc2: {=u8:?} }}",
                self.sw_ansrc1(),
                self.sw_apsrc1(),
                self.sw_ansrc2(),
                self.sw_apsrc2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCcvalr(pub u32);
    impl SbsCcvalr {
        #[must_use]
        #[inline(always)]
        pub const fn ansrc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ansrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apsrc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_apsrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ansrc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ansrc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apsrc2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_apsrc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for SbsCcvalr {
        #[inline(always)]
        fn default() -> SbsCcvalr {
            SbsCcvalr(0)
        }
    }
    impl core::fmt::Debug for SbsCcvalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCcvalr")
                .field("ansrc1", &self.ansrc1())
                .field("apsrc1", &self.apsrc1())
                .field("ansrc2", &self.ansrc2())
                .field("apsrc2", &self.apsrc2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCcvalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCcvalr {{ ansrc1: {=u8:?}, apsrc1: {=u8:?}, ansrc2: {=u8:?}, apsrc2: {=u8:?} }}",
                self.ansrc1(),
                self.apsrc1(),
                self.ansrc2(),
                self.apsrc2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCfgr2(pub u32);
    impl SbsCfgr2 {
        #[must_use]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SbsCfgr2 {
        #[inline(always)]
        fn default() -> SbsCfgr2 {
            SbsCfgr2(0)
        }
    }
    impl core::fmt::Debug for SbsCfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCfgr2")
                .field("cll", &self.cll())
                .field("sel", &self.sel())
                .field("pvdl", &self.pvdl())
                .field("eccl", &self.eccl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCfgr2 {{ cll: {=bool:?}, sel: {=bool:?}, pvdl: {=bool:?}, eccl: {=bool:?} }}",
                self.cll(),
                self.sel(),
                self.pvdl(),
                self.eccl()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCnslckr(pub u32);
    impl SbsCnslckr {
        #[must_use]
        #[inline(always)]
        pub const fn locknsvtor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_locknsvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_locknsmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SbsCnslckr {
        #[inline(always)]
        fn default() -> SbsCnslckr {
            SbsCnslckr(0)
        }
    }
    impl core::fmt::Debug for SbsCnslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCnslckr")
                .field("locknsvtor", &self.locknsvtor())
                .field("locknsmpu", &self.locknsmpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCnslckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCnslckr {{ locknsvtor: {=bool:?}, locknsmpu: {=bool:?} }}",
                self.locknsvtor(),
                self.locknsmpu()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCslckr(pub u32);
    impl SbsCslckr {
        #[must_use]
        #[inline(always)]
        pub const fn locksvtaircr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_locksvtaircr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn locksmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_locksmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn locksau(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_locksau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for SbsCslckr {
        #[inline(always)]
        fn default() -> SbsCslckr {
            SbsCslckr(0)
        }
    }
    impl core::fmt::Debug for SbsCslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCslckr")
                .field("locksvtaircr", &self.locksvtaircr())
                .field("locksmpu", &self.locksmpu())
                .field("locksau", &self.locksau())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCslckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCslckr {{ locksvtaircr: {=bool:?}, locksmpu: {=bool:?}, locksau: {=bool:?} }}",
                self.locksvtaircr(),
                self.locksmpu(),
                self.locksau()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsDbgcr(pub u32);
    impl SbsDbgcr {
        #[must_use]
        #[inline(always)]
        pub const fn ap_unlock(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ap_unlock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_unlock(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dbg_unlock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_auth_hdpl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dbg_auth_hdpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_auth_sec(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dbg_auth_sec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SbsDbgcr {
        #[inline(always)]
        fn default() -> SbsDbgcr {
            SbsDbgcr(0)
        }
    }
    impl core::fmt::Debug for SbsDbgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsDbgcr")
                .field("ap_unlock", &self.ap_unlock())
                .field("dbg_unlock", &self.dbg_unlock())
                .field("dbg_auth_hdpl", &self.dbg_auth_hdpl())
                .field("dbg_auth_sec", &self.dbg_auth_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsDbgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsDbgcr {{ ap_unlock: {=u8:?}, dbg_unlock: {=u8:?}, dbg_auth_hdpl: {=u8:?}, dbg_auth_sec: {=u8:?} }}",
                self.ap_unlock(),
                self.dbg_unlock(),
                self.dbg_auth_hdpl(),
                self.dbg_auth_sec()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsDbglockr(pub u32);
    impl SbsDbglockr {
        #[must_use]
        #[inline(always)]
        pub const fn dbgcfg_lock(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dbgcfg_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SbsDbglockr {
        #[inline(always)]
        fn default() -> SbsDbglockr {
            SbsDbglockr(0)
        }
    }
    impl core::fmt::Debug for SbsDbglockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsDbglockr")
                .field("dbgcfg_lock", &self.dbgcfg_lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsDbglockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsDbglockr {{ dbgcfg_lock: {=u8:?} }}",
                self.dbgcfg_lock()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsEccnmir(pub u32);
    impl SbsEccnmir {
        #[must_use]
        #[inline(always)]
        pub const fn eccnmi_mask_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eccnmi_mask_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SbsEccnmir {
        #[inline(always)]
        fn default() -> SbsEccnmir {
            SbsEccnmir(0)
        }
    }
    impl core::fmt::Debug for SbsEccnmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsEccnmir")
                .field("eccnmi_mask_en", &self.eccnmi_mask_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsEccnmir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsEccnmir {{ eccnmi_mask_en: {=bool:?} }}",
                self.eccnmi_mask_en()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsEpochselcr(pub u32);
    impl SbsEpochselcr {
        #[must_use]
        #[inline(always)]
        pub const fn epoch_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_epoch_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for SbsEpochselcr {
        #[inline(always)]
        fn default() -> SbsEpochselcr {
            SbsEpochselcr(0)
        }
    }
    impl core::fmt::Debug for SbsEpochselcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsEpochselcr")
                .field("epoch_sel", &self.epoch_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsEpochselcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsEpochselcr {{ epoch_sel: {=u8:?} }}",
                self.epoch_sel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsFpuimr(pub u32);
    impl SbsFpuimr {
        #[must_use]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_fpu_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for SbsFpuimr {
        #[inline(always)]
        fn default() -> SbsFpuimr {
            SbsFpuimr(0)
        }
    }
    impl core::fmt::Debug for SbsFpuimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsFpuimr")
                .field("fpu_ie", &self.fpu_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsFpuimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsFpuimr {{ fpu_ie: {=u8:?} }}", self.fpu_ie())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsHdplcr(pub u32);
    impl SbsHdplcr {
        #[must_use]
        #[inline(always)]
        pub const fn incr_hdpl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_incr_hdpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SbsHdplcr {
        #[inline(always)]
        fn default() -> SbsHdplcr {
            SbsHdplcr(0)
        }
    }
    impl core::fmt::Debug for SbsHdplcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsHdplcr")
                .field("incr_hdpl", &self.incr_hdpl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsHdplcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsHdplcr {{ incr_hdpl: {=u8:?} }}", self.incr_hdpl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsHdplsr(pub u32);
    impl SbsHdplsr {
        #[must_use]
        #[inline(always)]
        pub const fn hdpl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hdpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SbsHdplsr {
        #[inline(always)]
        fn default() -> SbsHdplsr {
            SbsHdplsr(0)
        }
    }
    impl core::fmt::Debug for SbsHdplsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsHdplsr")
                .field("hdpl", &self.hdpl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsHdplsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsHdplsr {{ hdpl: {=u8:?} }}", self.hdpl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsMesr(pub u32);
    impl SbsMesr {
        #[must_use]
        #[inline(always)]
        pub const fn mclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ipmee(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ipmee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for SbsMesr {
        #[inline(always)]
        fn default() -> SbsMesr {
            SbsMesr(0)
        }
    }
    impl core::fmt::Debug for SbsMesr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsMesr")
                .field("mclr", &self.mclr())
                .field("ipmee", &self.ipmee())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsMesr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsMesr {{ mclr: {=bool:?}, ipmee: {=bool:?} }}",
                self.mclr(),
                self.ipmee()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsNexthdplcr(pub u32);
    impl SbsNexthdplcr {
        #[must_use]
        #[inline(always)]
        pub const fn nexthdpl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nexthdpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for SbsNexthdplcr {
        #[inline(always)]
        fn default() -> SbsNexthdplcr {
            SbsNexthdplcr(0)
        }
    }
    impl core::fmt::Debug for SbsNexthdplcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsNexthdplcr")
                .field("nexthdpl", &self.nexthdpl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsNexthdplcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsNexthdplcr {{ nexthdpl: {=u8:?} }}", self.nexthdpl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsPmcr(pub u32);
    impl SbsPmcr {
        #[must_use]
        #[inline(always)]
        pub const fn pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eth_sel_phy(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eth_sel_phy(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ethintpol(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ethintpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ethpdack(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ethpdack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ethtxlpi(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ethtxlpi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for SbsPmcr {
        #[inline(always)]
        fn default() -> SbsPmcr {
            SbsPmcr(0)
        }
    }
    impl core::fmt::Debug for SbsPmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsPmcr")
                .field("pb6_fmp", &self.pb6_fmp())
                .field("pb7_fmp", &self.pb7_fmp())
                .field("pb8_fmp", &self.pb8_fmp())
                .field("pb9_fmp", &self.pb9_fmp())
                .field("eth_sel_phy", &self.eth_sel_phy())
                .field("ethintpol", &self.ethintpol())
                .field("ethpdack", &self.ethpdack())
                .field("ethtxlpi", &self.ethtxlpi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsPmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsPmcr {{ pb6_fmp: {=bool:?}, pb7_fmp: {=bool:?}, pb8_fmp: {=bool:?}, pb9_fmp: {=bool:?}, eth_sel_phy: {=u8:?}, ethintpol: {=bool:?}, ethpdack: {=bool:?}, ethtxlpi: {=bool:?} }}",
                self.pb6_fmp(),
                self.pb7_fmp(),
                self.pb8_fmp(),
                self.pb9_fmp(),
                self.eth_sel_phy(),
                self.ethintpol(),
                self.ethpdack(),
                self.ethtxlpi()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsRsscmdr(pub u32);
    impl SbsRsscmdr {
        #[must_use]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rsscmd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SbsRsscmdr {
        #[inline(always)]
        fn default() -> SbsRsscmdr {
            SbsRsscmdr(0)
        }
    }
    impl core::fmt::Debug for SbsRsscmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsRsscmdr")
                .field("rsscmd", &self.rsscmd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsRsscmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsRsscmdr {{ rsscmd: {=u16:?} }}", self.rsscmd())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsSeccfgr(pub u32);
    impl SbsSeccfgr {
        #[must_use]
        #[inline(always)]
        pub const fn sbssec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbssec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn classbsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_classbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fpusec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fpusec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdce_sec_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sdce_sec_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SbsSeccfgr {
        #[inline(always)]
        fn default() -> SbsSeccfgr {
            SbsSeccfgr(0)
        }
    }
    impl core::fmt::Debug for SbsSeccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsSeccfgr")
                .field("sbssec", &self.sbssec())
                .field("classbsec", &self.classbsec())
                .field("fpusec", &self.fpusec())
                .field("sdce_sec_en", &self.sdce_sec_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsSeccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsSeccfgr {{ sbssec: {=bool:?}, classbsec: {=bool:?}, fpusec: {=bool:?}, sdce_sec_en: {=bool:?} }}",
                self.sbssec(),
                self.classbsec(),
                self.fpusec(),
                self.sdce_sec_en()
            )
        }
    }
}
