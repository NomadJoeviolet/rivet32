#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp {
    ptr: *mut u8,
}
unsafe impl Send for Opamp {}
unsafe impl Sync for Opamp {}
impl Opamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::OpampCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn otr(self) -> crate::common::Reg<regs::OpampOtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn hsotr(self) -> crate::common::Reg<regs::OpampHsotr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpampCsr(pub u32);
    impl OpampCsr {
        #[must_use]
        #[inline(always)]
        pub const fn opam_px_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_opam_px_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn forcevp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_forcevp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vpsel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vmsel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn opahsm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_opahsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calon(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_calsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pggain(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pggain(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usertrim(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usertrim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tstref(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tstref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn calout(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_calout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for OpampCsr {
        #[inline(always)]
        fn default() -> OpampCsr {
            OpampCsr(0)
        }
    }
    impl core::fmt::Debug for OpampCsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpampCsr")
                .field("opam_px_en", &self.opam_px_en())
                .field("forcevp", &self.forcevp())
                .field("vpsel", &self.vpsel())
                .field("vmsel", &self.vmsel())
                .field("opahsm", &self.opahsm())
                .field("calon", &self.calon())
                .field("calsel", &self.calsel())
                .field("pggain", &self.pggain())
                .field("usertrim", &self.usertrim())
                .field("tstref", &self.tstref())
                .field("calout", &self.calout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpampCsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OpampCsr {{ opam_px_en: {=bool:?}, forcevp: {=bool:?}, vpsel: {=u8:?}, vmsel: {=u8:?}, opahsm: {=bool:?}, calon: {=bool:?}, calsel: {=u8:?}, pggain: {=u8:?}, usertrim: {=bool:?}, tstref: {=bool:?}, calout: {=bool:?} }}",
                self.opam_px_en(),
                self.forcevp(),
                self.vpsel(),
                self.vmsel(),
                self.opahsm(),
                self.calon(),
                self.calsel(),
                self.pggain(),
                self.usertrim(),
                self.tstref(),
                self.calout()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpampHsotr(pub u32);
    impl OpampHsotr {
        #[must_use]
        #[inline(always)]
        pub const fn trimhsoffsetn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trimhsoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trimhsoffsetp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trimhsoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for OpampHsotr {
        #[inline(always)]
        fn default() -> OpampHsotr {
            OpampHsotr(0)
        }
    }
    impl core::fmt::Debug for OpampHsotr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpampHsotr")
                .field("trimhsoffsetn", &self.trimhsoffsetn())
                .field("trimhsoffsetp", &self.trimhsoffsetp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpampHsotr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OpampHsotr {{ trimhsoffsetn: {=u8:?}, trimhsoffsetp: {=u8:?} }}",
                self.trimhsoffsetn(),
                self.trimhsoffsetp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpampOtr(pub u32);
    impl OpampOtr {
        #[must_use]
        #[inline(always)]
        pub const fn trimoffsetn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trimoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trimoffsetp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trimoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for OpampOtr {
        #[inline(always)]
        fn default() -> OpampOtr {
            OpampOtr(0)
        }
    }
    impl core::fmt::Debug for OpampOtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpampOtr")
                .field("trimoffsetn", &self.trimoffsetn())
                .field("trimoffsetp", &self.trimoffsetp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpampOtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OpampOtr {{ trimoffsetn: {=u8:?}, trimoffsetp: {=u8:?} }}",
                self.trimoffsetn(),
                self.trimoffsetp()
            )
        }
    }
}
