#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MdfFilter {
    ptr: *mut u8,
}
unsafe impl Send for MdfFilter {}
unsafe impl Sync for MdfFilter {}
impl MdfFilter {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn sitfcr(self) -> crate::common::Reg<regs::MdfFilterSitfcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn bsmxcr(self) -> crate::common::Reg<regs::MdfFilterBsmxcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn dfltcr(self) -> crate::common::Reg<regs::MdfFilterDfltcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn dfltcicr(self) -> crate::common::Reg<regs::MdfFilterDfltcicr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn dfltrsfr(self) -> crate::common::Reg<regs::MdfFilterDfltrsfr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn dfltintr(self) -> crate::common::Reg<regs::MdfFilterDfltintr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn oldcr(self) -> crate::common::Reg<regs::MdfFilterOldcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn oldthlr(self) -> crate::common::Reg<regs::MdfFilterOldthlr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn oldthhr(self) -> crate::common::Reg<regs::MdfFilterOldthhr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn dlycr(self) -> crate::common::Reg<regs::MdfFilterDlycr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn scdcr(self) -> crate::common::Reg<regs::MdfFilterScdcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn dfltier(self) -> crate::common::Reg<regs::MdfFilterDfltier, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn dfltisr(self) -> crate::common::Reg<regs::MdfFilterDfltisr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn oeccr(self) -> crate::common::Reg<regs::MdfFilterOeccr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn sadcr(self) -> crate::common::Reg<regs::MdfFilterSadcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn sadcfgr(self) -> crate::common::Reg<regs::MdfFilterSadcfgr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn sadsdlvr(self) -> crate::common::Reg<regs::MdfFilterSadsdlvr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn sadanlvr(self) -> crate::common::Reg<regs::MdfFilterSadanlvr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn snpsdr(self) -> crate::common::Reg<regs::MdfFilterSnpsdr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn dfltdr(self) -> crate::common::Reg<regs::MdfFilterDfltdr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterBsmxcr(pub u32);
    impl MdfFilterBsmxcr {
        #[must_use]
        #[inline(always)]
        pub const fn bssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bsmxactivate(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bsmxactivate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MdfFilterBsmxcr {
        #[inline(always)]
        fn default() -> MdfFilterBsmxcr {
            MdfFilterBsmxcr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterBsmxcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterBsmxcr")
                .field("bssel", &self.bssel())
                .field("bsmxactivate", &self.bsmxactivate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterBsmxcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterBsmxcr {{ bssel: {=u8:?}, bsmxactivate: {=bool:?} }}",
                self.bssel(),
                self.bsmxactivate()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterDfltcicr(pub u32);
    impl MdfFilterDfltcicr {
        #[must_use]
        #[inline(always)]
        pub const fn datsrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_datsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cicmod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cicmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mcicd(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mcicd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scale(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_scale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
    }
    impl Default for MdfFilterDfltcicr {
        #[inline(always)]
        fn default() -> MdfFilterDfltcicr {
            MdfFilterDfltcicr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterDfltcicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterDfltcicr")
                .field("datsrc", &self.datsrc())
                .field("cicmod", &self.cicmod())
                .field("mcicd", &self.mcicd())
                .field("scale", &self.scale())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterDfltcicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterDfltcicr {{ datsrc: {=u8:?}, cicmod: {=u8:?}, mcicd: {=u16:?}, scale: {=u8:?} }}",
                self.datsrc(),
                self.cicmod(),
                self.mcicd(),
                self.scale()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterDfltcr(pub u32);
    impl MdfFilterDfltcr {
        #[must_use]
        #[inline(always)]
        pub const fn dflten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dflten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fth(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn acqmod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_acqmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
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
        pub const fn snpsfmt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_snpsfmt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nbdis(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nbdis(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dfltrun(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dfltrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dfltactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dfltactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MdfFilterDfltcr {
        #[inline(always)]
        fn default() -> MdfFilterDfltcr {
            MdfFilterDfltcr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterDfltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterDfltcr")
                .field("dflten", &self.dflten())
                .field("dmaen", &self.dmaen())
                .field("fth", &self.fth())
                .field("acqmod", &self.acqmod())
                .field("trgsens", &self.trgsens())
                .field("trgsrc", &self.trgsrc())
                .field("snpsfmt", &self.snpsfmt())
                .field("nbdis", &self.nbdis())
                .field("dfltrun", &self.dfltrun())
                .field("dfltactive", &self.dfltactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterDfltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterDfltcr {{ dflten: {=bool:?}, dmaen: {=bool:?}, fth: {=bool:?}, acqmod: {=u8:?}, trgsens: {=bool:?}, trgsrc: {=u8:?}, snpsfmt: {=bool:?}, nbdis: {=u8:?}, dfltrun: {=bool:?}, dfltactive: {=bool:?} }}",
                self.dflten(),
                self.dmaen(),
                self.fth(),
                self.acqmod(),
                self.trgsens(),
                self.trgsrc(),
                self.snpsfmt(),
                self.nbdis(),
                self.dfltrun(),
                self.dfltactive()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterDfltdr(pub u32);
    impl MdfFilterDfltdr {
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MdfFilterDfltdr {
        #[inline(always)]
        fn default() -> MdfFilterDfltdr {
            MdfFilterDfltdr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterDfltdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterDfltdr")
                .field("dr", &self.dr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterDfltdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MdfFilterDfltdr {{ dr: {=u32:?} }}", self.dr())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterDfltier(pub u32);
    impl MdfFilterDfltier {
        #[must_use]
        #[inline(always)]
        pub const fn fthie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fthie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dovrie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssdrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssdrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn oldie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_oldie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssovrie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scdie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_scdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn satie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_satie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ckabie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckabie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfovrie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sddetie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sddetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdlvlie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sdlvlie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for MdfFilterDfltier {
        #[inline(always)]
        fn default() -> MdfFilterDfltier {
            MdfFilterDfltier(0)
        }
    }
    impl core::fmt::Debug for MdfFilterDfltier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterDfltier")
                .field("fthie", &self.fthie())
                .field("dovrie", &self.dovrie())
                .field("ssdrie", &self.ssdrie())
                .field("oldie", &self.oldie())
                .field("ssovrie", &self.ssovrie())
                .field("scdie", &self.scdie())
                .field("satie", &self.satie())
                .field("ckabie", &self.ckabie())
                .field("rfovrie", &self.rfovrie())
                .field("sddetie", &self.sddetie())
                .field("sdlvlie", &self.sdlvlie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterDfltier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterDfltier {{ fthie: {=bool:?}, dovrie: {=bool:?}, ssdrie: {=bool:?}, oldie: {=bool:?}, ssovrie: {=bool:?}, scdie: {=bool:?}, satie: {=bool:?}, ckabie: {=bool:?}, rfovrie: {=bool:?}, sddetie: {=bool:?}, sdlvlie: {=bool:?} }}",
                self.fthie(),
                self.dovrie(),
                self.ssdrie(),
                self.oldie(),
                self.ssovrie(),
                self.scdie(),
                self.satie(),
                self.ckabie(),
                self.rfovrie(),
                self.sddetie(),
                self.sdlvlie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterDfltintr(pub u32);
    impl MdfFilterDfltintr {
        #[must_use]
        #[inline(always)]
        pub const fn intdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_intdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn intval(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_intval(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 4usize)) | (((val as u32) & 0x7f) << 4usize);
        }
    }
    impl Default for MdfFilterDfltintr {
        #[inline(always)]
        fn default() -> MdfFilterDfltintr {
            MdfFilterDfltintr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterDfltintr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterDfltintr")
                .field("intdiv", &self.intdiv())
                .field("intval", &self.intval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterDfltintr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterDfltintr {{ intdiv: {=u8:?}, intval: {=u8:?} }}",
                self.intdiv(),
                self.intval()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterDfltisr(pub u32);
    impl MdfFilterDfltisr {
        #[must_use]
        #[inline(always)]
        pub const fn fthf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fthf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dovrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssdrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssdrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxnef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn oldf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_oldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn thlf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_thlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn thhf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_thhf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ssovrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ssovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scdf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_scdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn satf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_satf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ckabf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckabf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfovrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sddetf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sddetf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdlvlf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sdlvlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for MdfFilterDfltisr {
        #[inline(always)]
        fn default() -> MdfFilterDfltisr {
            MdfFilterDfltisr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterDfltisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterDfltisr")
                .field("fthf", &self.fthf())
                .field("dovrf", &self.dovrf())
                .field("ssdrf", &self.ssdrf())
                .field("rxnef", &self.rxnef())
                .field("oldf", &self.oldf())
                .field("thlf", &self.thlf())
                .field("thhf", &self.thhf())
                .field("ssovrf", &self.ssovrf())
                .field("scdf", &self.scdf())
                .field("satf", &self.satf())
                .field("ckabf", &self.ckabf())
                .field("rfovrf", &self.rfovrf())
                .field("sddetf", &self.sddetf())
                .field("sdlvlf", &self.sdlvlf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterDfltisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterDfltisr {{ fthf: {=bool:?}, dovrf: {=bool:?}, ssdrf: {=bool:?}, rxnef: {=bool:?}, oldf: {=bool:?}, thlf: {=bool:?}, thhf: {=bool:?}, ssovrf: {=bool:?}, scdf: {=bool:?}, satf: {=bool:?}, ckabf: {=bool:?}, rfovrf: {=bool:?}, sddetf: {=bool:?}, sdlvlf: {=bool:?} }}",
                self.fthf(),
                self.dovrf(),
                self.ssdrf(),
                self.rxnef(),
                self.oldf(),
                self.thlf(),
                self.thhf(),
                self.ssovrf(),
                self.scdf(),
                self.satf(),
                self.ckabf(),
                self.rfovrf(),
                self.sddetf(),
                self.sdlvlf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterDfltrsfr(pub u32);
    impl MdfFilterDfltrsfr {
        #[must_use]
        #[inline(always)]
        pub const fn rsfltbyp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rsfltbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rsfltd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rsfltd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hpfbyp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hpfbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hpfc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hpfc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for MdfFilterDfltrsfr {
        #[inline(always)]
        fn default() -> MdfFilterDfltrsfr {
            MdfFilterDfltrsfr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterDfltrsfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterDfltrsfr")
                .field("rsfltbyp", &self.rsfltbyp())
                .field("rsfltd", &self.rsfltd())
                .field("hpfbyp", &self.hpfbyp())
                .field("hpfc", &self.hpfc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterDfltrsfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterDfltrsfr {{ rsfltbyp: {=bool:?}, rsfltd: {=bool:?}, hpfbyp: {=bool:?}, hpfc: {=u8:?} }}",
                self.rsfltbyp(),
                self.rsfltd(),
                self.hpfbyp(),
                self.hpfc()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterDlycr(pub u32);
    impl MdfFilterDlycr {
        #[must_use]
        #[inline(always)]
        pub const fn skpdly(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_skpdly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn skpbf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_skpbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MdfFilterDlycr {
        #[inline(always)]
        fn default() -> MdfFilterDlycr {
            MdfFilterDlycr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterDlycr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterDlycr")
                .field("skpdly", &self.skpdly())
                .field("skpbf", &self.skpbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterDlycr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterDlycr {{ skpdly: {=u8:?}, skpbf: {=bool:?} }}",
                self.skpdly(),
                self.skpbf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterOeccr(pub u32);
    impl MdfFilterOeccr {
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for MdfFilterOeccr {
        #[inline(always)]
        fn default() -> MdfFilterOeccr {
            MdfFilterOeccr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterOeccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterOeccr")
                .field("offset", &self.offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterOeccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MdfFilterOeccr {{ offset: {=u32:?} }}", self.offset())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterOldcr(pub u32);
    impl MdfFilterOldcr {
        #[must_use]
        #[inline(always)]
        pub const fn olden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_olden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn thinb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_thinb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bkold(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bkold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn acicn(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_acicn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn acicd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_acicd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn oldactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_oldactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MdfFilterOldcr {
        #[inline(always)]
        fn default() -> MdfFilterOldcr {
            MdfFilterOldcr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterOldcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterOldcr")
                .field("olden", &self.olden())
                .field("thinb", &self.thinb())
                .field("bkold", &self.bkold())
                .field("acicn", &self.acicn())
                .field("acicd", &self.acicd())
                .field("oldactive", &self.oldactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterOldcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterOldcr {{ olden: {=bool:?}, thinb: {=bool:?}, bkold: {=u8:?}, acicn: {=u8:?}, acicd: {=u8:?}, oldactive: {=bool:?} }}",
                self.olden(),
                self.thinb(),
                self.bkold(),
                self.acicn(),
                self.acicd(),
                self.oldactive()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterOldthhr(pub u32);
    impl MdfFilterOldthhr {
        #[must_use]
        #[inline(always)]
        pub const fn oldthh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_oldthh(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for MdfFilterOldthhr {
        #[inline(always)]
        fn default() -> MdfFilterOldthhr {
            MdfFilterOldthhr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterOldthhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterOldthhr")
                .field("oldthh", &self.oldthh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterOldthhr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MdfFilterOldthhr {{ oldthh: {=u32:?} }}", self.oldthh())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterOldthlr(pub u32);
    impl MdfFilterOldthlr {
        #[must_use]
        #[inline(always)]
        pub const fn oldthl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_oldthl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for MdfFilterOldthlr {
        #[inline(always)]
        fn default() -> MdfFilterOldthlr {
            MdfFilterOldthlr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterOldthlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterOldthlr")
                .field("oldthl", &self.oldthl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterOldthlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MdfFilterOldthlr {{ oldthl: {=u32:?} }}", self.oldthl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterSadanlvr(pub u32);
    impl MdfFilterSadanlvr {
        #[must_use]
        #[inline(always)]
        pub const fn anlvl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_anlvl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for MdfFilterSadanlvr {
        #[inline(always)]
        fn default() -> MdfFilterSadanlvr {
            MdfFilterSadanlvr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterSadanlvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterSadanlvr")
                .field("anlvl", &self.anlvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterSadanlvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MdfFilterSadanlvr {{ anlvl: {=u16:?} }}", self.anlvl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterSadcfgr(pub u32);
    impl MdfFilterSadcfgr {
        #[must_use]
        #[inline(always)]
        pub const fn snthr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_snthr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn anslp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_anslp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lfrnb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lfrnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hgovr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hgovr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn anmin(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_anmin(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for MdfFilterSadcfgr {
        #[inline(always)]
        fn default() -> MdfFilterSadcfgr {
            MdfFilterSadcfgr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterSadcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterSadcfgr")
                .field("snthr", &self.snthr())
                .field("anslp", &self.anslp())
                .field("lfrnb", &self.lfrnb())
                .field("hgovr", &self.hgovr())
                .field("anmin", &self.anmin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterSadcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterSadcfgr {{ snthr: {=u8:?}, anslp: {=u8:?}, lfrnb: {=u8:?}, hgovr: {=u8:?}, anmin: {=u16:?} }}",
                self.snthr(),
                self.anslp(),
                self.lfrnb(),
                self.hgovr(),
                self.anmin()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterSadcr(pub u32);
    impl MdfFilterSadcr {
        #[must_use]
        #[inline(always)]
        pub const fn saden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_saden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn datcap(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_datcap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn detcfg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_detcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sadst(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sadst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hysten(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hysten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frsize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_frsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sadmod(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sadmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sadactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sadactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MdfFilterSadcr {
        #[inline(always)]
        fn default() -> MdfFilterSadcr {
            MdfFilterSadcr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterSadcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterSadcr")
                .field("saden", &self.saden())
                .field("datcap", &self.datcap())
                .field("detcfg", &self.detcfg())
                .field("sadst", &self.sadst())
                .field("hysten", &self.hysten())
                .field("frsize", &self.frsize())
                .field("sadmod", &self.sadmod())
                .field("sadactive", &self.sadactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterSadcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterSadcr {{ saden: {=bool:?}, datcap: {=u8:?}, detcfg: {=bool:?}, sadst: {=u8:?}, hysten: {=bool:?}, frsize: {=u8:?}, sadmod: {=u8:?}, sadactive: {=bool:?} }}",
                self.saden(),
                self.datcap(),
                self.detcfg(),
                self.sadst(),
                self.hysten(),
                self.frsize(),
                self.sadmod(),
                self.sadactive()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterSadsdlvr(pub u32);
    impl MdfFilterSadsdlvr {
        #[must_use]
        #[inline(always)]
        pub const fn sdlvl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_sdlvl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for MdfFilterSadsdlvr {
        #[inline(always)]
        fn default() -> MdfFilterSadsdlvr {
            MdfFilterSadsdlvr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterSadsdlvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterSadsdlvr")
                .field("sdlvl", &self.sdlvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterSadsdlvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MdfFilterSadsdlvr {{ sdlvl: {=u16:?} }}", self.sdlvl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterScdcr(pub u32);
    impl MdfFilterScdcr {
        #[must_use]
        #[inline(always)]
        pub const fn scden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_scden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bkscd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bkscd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scdt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_scdt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scdactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_scdactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MdfFilterScdcr {
        #[inline(always)]
        fn default() -> MdfFilterScdcr {
            MdfFilterScdcr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterScdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterScdcr")
                .field("scden", &self.scden())
                .field("bkscd", &self.bkscd())
                .field("scdt", &self.scdt())
                .field("scdactive", &self.scdactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterScdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterScdcr {{ scden: {=bool:?}, bkscd: {=u8:?}, scdt: {=u8:?}, scdactive: {=bool:?} }}",
                self.scden(),
                self.bkscd(),
                self.scdt(),
                self.scdactive()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterSitfcr(pub u32);
    impl MdfFilterSitfcr {
        #[must_use]
        #[inline(always)]
        pub const fn sitfen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sitfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn scksrc(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_scksrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sitfmod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sitfmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sitfactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sitfactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MdfFilterSitfcr {
        #[inline(always)]
        fn default() -> MdfFilterSitfcr {
            MdfFilterSitfcr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterSitfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterSitfcr")
                .field("sitfen", &self.sitfen())
                .field("scksrc", &self.scksrc())
                .field("sitfmod", &self.sitfmod())
                .field("sth", &self.sth())
                .field("sitfactive", &self.sitfactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterSitfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterSitfcr {{ sitfen: {=bool:?}, scksrc: {=u8:?}, sitfmod: {=u8:?}, sth: {=u8:?}, sitfactive: {=bool:?} }}",
                self.sitfen(),
                self.scksrc(),
                self.sitfmod(),
                self.sth(),
                self.sitfactive()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdfFilterSnpsdr(pub u32);
    impl MdfFilterSnpsdr {
        #[must_use]
        #[inline(always)]
        pub const fn mcicdc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mcicdc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn extsdr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_extsdr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sdr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_sdr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MdfFilterSnpsdr {
        #[inline(always)]
        fn default() -> MdfFilterSnpsdr {
            MdfFilterSnpsdr(0)
        }
    }
    impl core::fmt::Debug for MdfFilterSnpsdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdfFilterSnpsdr")
                .field("mcicdc", &self.mcicdc())
                .field("extsdr", &self.extsdr())
                .field("sdr", &self.sdr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdfFilterSnpsdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdfFilterSnpsdr {{ mcicdc: {=u16:?}, extsdr: {=u8:?}, sdr: {=u16:?} }}",
                self.mcicdc(),
                self.extsdr(),
                self.sdr()
            )
        }
    }
}
