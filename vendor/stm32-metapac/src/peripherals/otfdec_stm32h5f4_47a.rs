#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otfdec {
    ptr: *mut u8,
}
unsafe impl Send for Otfdec {}
unsafe impl Sync for Otfdec {}
impl Otfdec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::OtfdecCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::OtfdecPrivcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::OtfdecIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::OtfdecIcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::OtfdecIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtfdecCr(pub u32);
    impl OtfdecCr {
        #[must_use]
        #[inline(always)]
        pub const fn enc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_enc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for OtfdecCr {
        #[inline(always)]
        fn default() -> OtfdecCr {
            OtfdecCr(0)
        }
    }
    impl core::fmt::Debug for OtfdecCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtfdecCr")
                .field("enc", &self.enc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtfdecCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtfdecCr {{ enc: {=bool:?} }}", self.enc())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtfdecIcr(pub u32);
    impl OtfdecIcr {
        #[must_use]
        #[inline(always)]
        pub const fn seif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_seif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn xoneif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_xoneif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn keif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_keif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for OtfdecIcr {
        #[inline(always)]
        fn default() -> OtfdecIcr {
            OtfdecIcr(0)
        }
    }
    impl core::fmt::Debug for OtfdecIcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtfdecIcr")
                .field("seif", &self.seif())
                .field("xoneif", &self.xoneif())
                .field("keif", &self.keif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtfdecIcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OtfdecIcr {{ seif: {=bool:?}, xoneif: {=bool:?}, keif: {=bool:?} }}",
                self.seif(),
                self.xoneif(),
                self.keif()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtfdecIer(pub u32);
    impl OtfdecIer {
        #[must_use]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn xoneie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_xoneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn keie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_keie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for OtfdecIer {
        #[inline(always)]
        fn default() -> OtfdecIer {
            OtfdecIer(0)
        }
    }
    impl core::fmt::Debug for OtfdecIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtfdecIer")
                .field("seie", &self.seie())
                .field("xoneie", &self.xoneie())
                .field("keie", &self.keie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtfdecIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OtfdecIer {{ seie: {=bool:?}, xoneie: {=bool:?}, keie: {=bool:?} }}",
                self.seie(),
                self.xoneie(),
                self.keie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtfdecIsr(pub u32);
    impl OtfdecIsr {
        #[must_use]
        #[inline(always)]
        pub const fn seif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_seif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn xoneif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_xoneif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn keif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_keif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for OtfdecIsr {
        #[inline(always)]
        fn default() -> OtfdecIsr {
            OtfdecIsr(0)
        }
    }
    impl core::fmt::Debug for OtfdecIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtfdecIsr")
                .field("seif", &self.seif())
                .field("xoneif", &self.xoneif())
                .field("keif", &self.keif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtfdecIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OtfdecIsr {{ seif: {=bool:?}, xoneif: {=bool:?}, keif: {=bool:?} }}",
                self.seif(),
                self.xoneif(),
                self.keif()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtfdecPrivcfgr(pub u32);
    impl OtfdecPrivcfgr {
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for OtfdecPrivcfgr {
        #[inline(always)]
        fn default() -> OtfdecPrivcfgr {
            OtfdecPrivcfgr(0)
        }
    }
    impl core::fmt::Debug for OtfdecPrivcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtfdecPrivcfgr")
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtfdecPrivcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtfdecPrivcfgr {{ priv_: {=bool:?} }}", self.priv_())
        }
    }
}
