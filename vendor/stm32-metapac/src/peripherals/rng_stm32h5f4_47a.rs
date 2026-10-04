#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng {
    ptr: *mut u8,
}
unsafe impl Send for Rng {}
unsafe impl Sync for Rng {}
impl Rng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::RngCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::RngSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::RngNscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn htcr(self, n: usize) -> crate::common::Reg<regs::RngHtcr, crate::common::Raw> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn htsr(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn htsr0(self) -> crate::common::Reg<regs::RngHtsr0, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn htsr1(self) -> crate::common::Reg<regs::RngHtsr1, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn nsmr(self) -> crate::common::Reg<regs::RngNsmr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RngCr(pub u32);
    impl RngCr {
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ced(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ced(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ardis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ardis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rng_config3(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rng_config3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nistc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nistc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rng_config2(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rng_config2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rng_config1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rng_config1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn condrst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_condrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn configlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_configlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RngCr {
        #[inline(always)]
        fn default() -> RngCr {
            RngCr(0)
        }
    }
    impl core::fmt::Debug for RngCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RngCr")
                .field("rngen", &self.rngen())
                .field("ie", &self.ie())
                .field("ced", &self.ced())
                .field("ardis", &self.ardis())
                .field("rng_config3", &self.rng_config3())
                .field("nistc", &self.nistc())
                .field("rng_config2", &self.rng_config2())
                .field("clkdiv", &self.clkdiv())
                .field("rng_config1", &self.rng_config1())
                .field("condrst", &self.condrst())
                .field("configlock", &self.configlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RngCr {{ rngen: {=bool:?}, ie: {=bool:?}, ced: {=bool:?}, ardis: {=bool:?}, rng_config3: {=u8:?}, nistc: {=bool:?}, rng_config2: {=u8:?}, clkdiv: {=u8:?}, rng_config1: {=u8:?}, condrst: {=bool:?}, configlock: {=bool:?} }}",
                self.rngen(),
                self.ie(),
                self.ced(),
                self.ardis(),
                self.rng_config3(),
                self.nistc(),
                self.rng_config2(),
                self.clkdiv(),
                self.rng_config1(),
                self.condrst(),
                self.configlock()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RngHtcr(pub u32);
    impl RngHtcr {
        #[must_use]
        #[inline(always)]
        pub const fn htcfg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_htcfg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RngHtcr {
        #[inline(always)]
        fn default() -> RngHtcr {
            RngHtcr(0)
        }
    }
    impl core::fmt::Debug for RngHtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RngHtcr")
                .field("htcfg", &self.htcfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngHtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RngHtcr {{ htcfg: {=u32:?} }}", self.htcfg())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RngHtsr0(pub u32);
    impl RngHtsr0 {
        #[must_use]
        #[inline(always)]
        pub const fn rperrx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rperr9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rperr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for RngHtsr0 {
        #[inline(always)]
        fn default() -> RngHtsr0 {
            RngHtsr0(0)
        }
    }
    impl core::fmt::Debug for RngHtsr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RngHtsr0")
                .field("rperrx", &self.rperrx())
                .field("rperr1", &self.rperr1())
                .field("rperr2", &self.rperr2())
                .field("rperr3", &self.rperr3())
                .field("rperr4", &self.rperr4())
                .field("rperr5", &self.rperr5())
                .field("rperr6", &self.rperr6())
                .field("rperr7", &self.rperr7())
                .field("rperr8", &self.rperr8())
                .field("rperr9", &self.rperr9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngHtsr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RngHtsr0 {{ rperrx: {=bool:?}, rperr1: {=bool:?}, rperr2: {=bool:?}, rperr3: {=bool:?}, rperr4: {=bool:?}, rperr5: {=bool:?}, rperr6: {=bool:?}, rperr7: {=bool:?}, rperr8: {=bool:?}, rperr9: {=bool:?} }}",
                self.rperrx(),
                self.rperr1(),
                self.rperr2(),
                self.rperr3(),
                self.rperr4(),
                self.rperr5(),
                self.rperr6(),
                self.rperr7(),
                self.rperr8(),
                self.rperr9()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RngHtsr1(pub u32);
    impl RngHtsr1 {
        #[must_use]
        #[inline(always)]
        pub const fn aderrx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aderr9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aderr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for RngHtsr1 {
        #[inline(always)]
        fn default() -> RngHtsr1 {
            RngHtsr1(0)
        }
    }
    impl core::fmt::Debug for RngHtsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RngHtsr1")
                .field("aderrx", &self.aderrx())
                .field("aderr1", &self.aderr1())
                .field("aderr2", &self.aderr2())
                .field("aderr3", &self.aderr3())
                .field("aderr4", &self.aderr4())
                .field("aderr5", &self.aderr5())
                .field("aderr6", &self.aderr6())
                .field("aderr7", &self.aderr7())
                .field("aderr8", &self.aderr8())
                .field("aderr9", &self.aderr9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngHtsr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RngHtsr1 {{ aderrx: {=bool:?}, aderr1: {=bool:?}, aderr2: {=bool:?}, aderr3: {=bool:?}, aderr4: {=bool:?}, aderr5: {=bool:?}, aderr6: {=bool:?}, aderr7: {=bool:?}, aderr8: {=bool:?}, aderr9: {=bool:?} }}",
                self.aderrx(),
                self.aderr1(),
                self.aderr2(),
                self.aderr3(),
                self.aderr4(),
                self.aderr5(),
                self.aderr6(),
                self.aderr7(),
                self.aderr8(),
                self.aderr9()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RngNscr(pub u32);
    impl RngNscr {
        #[must_use]
        #[inline(always)]
        pub const fn en_osc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_en_osc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn en_osc2(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_en_osc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn en_osc3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_en_osc3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
    }
    impl Default for RngNscr {
        #[inline(always)]
        fn default() -> RngNscr {
            RngNscr(0)
        }
    }
    impl core::fmt::Debug for RngNscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RngNscr")
                .field("en_osc1", &self.en_osc1())
                .field("en_osc2", &self.en_osc2())
                .field("en_osc3", &self.en_osc3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngNscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RngNscr {{ en_osc1: {=u8:?}, en_osc2: {=u8:?}, en_osc3: {=u8:?} }}",
                self.en_osc1(),
                self.en_osc2(),
                self.en_osc3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RngNsmr(pub u32);
    impl RngNsmr {
        #[must_use]
        #[inline(always)]
        pub const fn mosc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mosc2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mosc3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mosc4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mosc5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mosc6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mosc7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mosc8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mosc9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mosc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for RngNsmr {
        #[inline(always)]
        fn default() -> RngNsmr {
            RngNsmr(0)
        }
    }
    impl core::fmt::Debug for RngNsmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RngNsmr")
                .field("mosc1", &self.mosc1())
                .field("mosc2", &self.mosc2())
                .field("mosc3", &self.mosc3())
                .field("mosc4", &self.mosc4())
                .field("mosc5", &self.mosc5())
                .field("mosc6", &self.mosc6())
                .field("mosc7", &self.mosc7())
                .field("mosc8", &self.mosc8())
                .field("mosc9", &self.mosc9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngNsmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RngNsmr {{ mosc1: {=bool:?}, mosc2: {=bool:?}, mosc3: {=bool:?}, mosc4: {=bool:?}, mosc5: {=bool:?}, mosc6: {=bool:?}, mosc7: {=bool:?}, mosc8: {=bool:?}, mosc9: {=bool:?} }}",
                self.mosc1(),
                self.mosc2(),
                self.mosc3(),
                self.mosc4(),
                self.mosc5(),
                self.mosc6(),
                self.mosc7(),
                self.mosc8(),
                self.mosc9()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RngSr(pub u32);
    impl RngSr {
        #[must_use]
        #[inline(always)]
        pub const fn drdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_drdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cecs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn secs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_secs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ceis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ceis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn seis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_seis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for RngSr {
        #[inline(always)]
        fn default() -> RngSr {
            RngSr(0)
        }
    }
    impl core::fmt::Debug for RngSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RngSr")
                .field("drdy", &self.drdy())
                .field("cecs", &self.cecs())
                .field("secs", &self.secs())
                .field("busy", &self.busy())
                .field("ceis", &self.ceis())
                .field("seis", &self.seis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RngSr {{ drdy: {=bool:?}, cecs: {=bool:?}, secs: {=bool:?}, busy: {=bool:?}, ceis: {=bool:?}, seis: {=bool:?} }}",
                self.drdy(),
                self.cecs(),
                self.secs(),
                self.busy(),
                self.ceis(),
                self.seis()
            )
        }
    }
}
