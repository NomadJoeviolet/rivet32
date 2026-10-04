#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pssi {
    ptr: *mut u8,
}
unsafe impl Send for Pssi {}
unsafe impl Sync for Pssi {}
impl Pssi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::PssiCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::PssiSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn ris(self) -> crate::common::Reg<regs::PssiRis, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::PssiIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn mis(self) -> crate::common::Reg<regs::PssiMis, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::PssiIcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::PssiDr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PssiCr(pub u32);
    impl PssiCr {
        #[must_use]
        #[inline(always)]
        pub const fn ckpol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ckpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn depol(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_depol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdypol(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rdypol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn edm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_edm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn derdycfg(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_derdycfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn outen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_outen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PssiCr {
        #[inline(always)]
        fn default() -> PssiCr {
            PssiCr(0)
        }
    }
    impl core::fmt::Debug for PssiCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PssiCr")
                .field("ckpol", &self.ckpol())
                .field("depol", &self.depol())
                .field("rdypol", &self.rdypol())
                .field("edm", &self.edm())
                .field("enable", &self.enable())
                .field("derdycfg", &self.derdycfg())
                .field("dmaen", &self.dmaen())
                .field("outen", &self.outen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PssiCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PssiCr {{ ckpol: {=bool:?}, depol: {=bool:?}, rdypol: {=bool:?}, edm: {=u8:?}, enable: {=bool:?}, derdycfg: {=u8:?}, dmaen: {=bool:?}, outen: {=bool:?} }}",
                self.ckpol(),
                self.depol(),
                self.rdypol(),
                self.edm(),
                self.enable(),
                self.derdycfg(),
                self.dmaen(),
                self.outen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PssiDr(pub u32);
    impl PssiDr {
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PssiDr {
        #[inline(always)]
        fn default() -> PssiDr {
            PssiDr(0)
        }
    }
    impl core::fmt::Debug for PssiDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PssiDr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PssiDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PssiDr {{ dr: {=u32:?} }}", self.dr())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PssiIcr(pub u32);
    impl PssiIcr {
        #[must_use]
        #[inline(always)]
        pub const fn ovr_isc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ovr_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PssiIcr {
        #[inline(always)]
        fn default() -> PssiIcr {
            PssiIcr(0)
        }
    }
    impl core::fmt::Debug for PssiIcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PssiIcr")
                .field("ovr_isc", &self.ovr_isc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PssiIcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PssiIcr {{ ovr_isc: {=bool:?} }}", self.ovr_isc())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PssiIer(pub u32);
    impl PssiIer {
        #[must_use]
        #[inline(always)]
        pub const fn ovr_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ovr_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PssiIer {
        #[inline(always)]
        fn default() -> PssiIer {
            PssiIer(0)
        }
    }
    impl core::fmt::Debug for PssiIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PssiIer")
                .field("ovr_ie", &self.ovr_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PssiIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PssiIer {{ ovr_ie: {=bool:?} }}", self.ovr_ie())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PssiMis(pub u32);
    impl PssiMis {
        #[must_use]
        #[inline(always)]
        pub const fn ovr_mis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ovr_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PssiMis {
        #[inline(always)]
        fn default() -> PssiMis {
            PssiMis(0)
        }
    }
    impl core::fmt::Debug for PssiMis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PssiMis")
                .field("ovr_mis", &self.ovr_mis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PssiMis {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PssiMis {{ ovr_mis: {=bool:?} }}", self.ovr_mis())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PssiRis(pub u32);
    impl PssiRis {
        #[must_use]
        #[inline(always)]
        pub const fn ovr_ris(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ovr_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PssiRis {
        #[inline(always)]
        fn default() -> PssiRis {
            PssiRis(0)
        }
    }
    impl core::fmt::Debug for PssiRis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PssiRis")
                .field("ovr_ris", &self.ovr_ris())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PssiRis {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PssiRis {{ ovr_ris: {=bool:?} }}", self.ovr_ris())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PssiSr(pub u32);
    impl PssiSr {
        #[must_use]
        #[inline(always)]
        pub const fn rtt4b(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rtt4b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rtt1b(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rtt1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PssiSr {
        #[inline(always)]
        fn default() -> PssiSr {
            PssiSr(0)
        }
    }
    impl core::fmt::Debug for PssiSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PssiSr")
                .field("rtt4b", &self.rtt4b())
                .field("rtt1b", &self.rtt1b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PssiSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PssiSr {{ rtt4b: {=bool:?}, rtt1b: {=bool:?} }}",
                self.rtt4b(),
                self.rtt1b()
            )
        }
    }
}
