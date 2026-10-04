#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dts {
    ptr: *mut u8,
}
unsafe impl Send for Dts {}
unsafe impl Sync for Dts {}
impl Dts {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::DtsCfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn t0valr1(self) -> crate::common::Reg<regs::DtsT0valr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn rampvalr(self) -> crate::common::Reg<regs::DtsRampvalr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn itr1(self) -> crate::common::Reg<regs::DtsItr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::DtsDr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::DtsSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn itenr(self) -> crate::common::Reg<regs::DtsItenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn icifr(self) -> crate::common::Reg<regs::DtsIcifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DtsCfgr1(pub u32);
    impl DtsCfgr1 {
        #[must_use]
        #[inline(always)]
        pub const fn ts1_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_start(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_intrig_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ts1_intrig_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_smp_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ts1_smp_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn refclk_sel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_refclk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn q_meas_opt(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_q_meas_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hsref_clk_div(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hsref_clk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for DtsCfgr1 {
        #[inline(always)]
        fn default() -> DtsCfgr1 {
            DtsCfgr1(0)
        }
    }
    impl core::fmt::Debug for DtsCfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DtsCfgr1")
                .field("ts1_en", &self.ts1_en())
                .field("ts1_start", &self.ts1_start())
                .field("ts1_intrig_sel", &self.ts1_intrig_sel())
                .field("ts1_smp_time", &self.ts1_smp_time())
                .field("refclk_sel", &self.refclk_sel())
                .field("q_meas_opt", &self.q_meas_opt())
                .field("hsref_clk_div", &self.hsref_clk_div())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DtsCfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DtsCfgr1 {{ ts1_en: {=bool:?}, ts1_start: {=bool:?}, ts1_intrig_sel: {=u8:?}, ts1_smp_time: {=u8:?}, refclk_sel: {=bool:?}, q_meas_opt: {=bool:?}, hsref_clk_div: {=u8:?} }}",
                self.ts1_en(),
                self.ts1_start(),
                self.ts1_intrig_sel(),
                self.ts1_smp_time(),
                self.refclk_sel(),
                self.q_meas_opt(),
                self.hsref_clk_div()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DtsDr(pub u32);
    impl DtsDr {
        #[must_use]
        #[inline(always)]
        pub const fn ts1_mfreq(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ts1_mfreq(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DtsDr {
        #[inline(always)]
        fn default() -> DtsDr {
            DtsDr(0)
        }
    }
    impl core::fmt::Debug for DtsDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DtsDr")
                .field("ts1_mfreq", &self.ts1_mfreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DtsDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DtsDr {{ ts1_mfreq: {=u16:?} }}", self.ts1_mfreq())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DtsIcifr(pub u32);
    impl DtsIcifr {
        #[must_use]
        #[inline(always)]
        pub const fn ts1_citef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_citef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_citlf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_citlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_cithf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_cithf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_caitef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_caitef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_caitlf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_caitlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_caithf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_caithf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for DtsIcifr {
        #[inline(always)]
        fn default() -> DtsIcifr {
            DtsIcifr(0)
        }
    }
    impl core::fmt::Debug for DtsIcifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DtsIcifr")
                .field("ts1_citef", &self.ts1_citef())
                .field("ts1_citlf", &self.ts1_citlf())
                .field("ts1_cithf", &self.ts1_cithf())
                .field("ts1_caitef", &self.ts1_caitef())
                .field("ts1_caitlf", &self.ts1_caitlf())
                .field("ts1_caithf", &self.ts1_caithf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DtsIcifr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DtsIcifr {{ ts1_citef: {=bool:?}, ts1_citlf: {=bool:?}, ts1_cithf: {=bool:?}, ts1_caitef: {=bool:?}, ts1_caitlf: {=bool:?}, ts1_caithf: {=bool:?} }}",
                self.ts1_citef(),
                self.ts1_citlf(),
                self.ts1_cithf(),
                self.ts1_caitef(),
                self.ts1_caitlf(),
                self.ts1_caithf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DtsItenr(pub u32);
    impl DtsItenr {
        #[must_use]
        #[inline(always)]
        pub const fn ts1_iteen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_iteen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_itlen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_itlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_ithen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_ithen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_aiteen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_aiteen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_aitlen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_aitlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_aithen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_aithen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for DtsItenr {
        #[inline(always)]
        fn default() -> DtsItenr {
            DtsItenr(0)
        }
    }
    impl core::fmt::Debug for DtsItenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DtsItenr")
                .field("ts1_iteen", &self.ts1_iteen())
                .field("ts1_itlen", &self.ts1_itlen())
                .field("ts1_ithen", &self.ts1_ithen())
                .field("ts1_aiteen", &self.ts1_aiteen())
                .field("ts1_aitlen", &self.ts1_aitlen())
                .field("ts1_aithen", &self.ts1_aithen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DtsItenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DtsItenr {{ ts1_iteen: {=bool:?}, ts1_itlen: {=bool:?}, ts1_ithen: {=bool:?}, ts1_aiteen: {=bool:?}, ts1_aitlen: {=bool:?}, ts1_aithen: {=bool:?} }}",
                self.ts1_iteen(),
                self.ts1_itlen(),
                self.ts1_ithen(),
                self.ts1_aiteen(),
                self.ts1_aitlen(),
                self.ts1_aithen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DtsItr1(pub u32);
    impl DtsItr1 {
        #[must_use]
        #[inline(always)]
        pub const fn ts1_litthd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ts1_litthd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_hitthd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ts1_hitthd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DtsItr1 {
        #[inline(always)]
        fn default() -> DtsItr1 {
            DtsItr1(0)
        }
    }
    impl core::fmt::Debug for DtsItr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DtsItr1")
                .field("ts1_litthd", &self.ts1_litthd())
                .field("ts1_hitthd", &self.ts1_hitthd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DtsItr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DtsItr1 {{ ts1_litthd: {=u16:?}, ts1_hitthd: {=u16:?} }}",
                self.ts1_litthd(),
                self.ts1_hitthd()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DtsRampvalr(pub u32);
    impl DtsRampvalr {
        #[must_use]
        #[inline(always)]
        pub const fn ts1_ramp_coeff(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ts1_ramp_coeff(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DtsRampvalr {
        #[inline(always)]
        fn default() -> DtsRampvalr {
            DtsRampvalr(0)
        }
    }
    impl core::fmt::Debug for DtsRampvalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DtsRampvalr")
                .field("ts1_ramp_coeff", &self.ts1_ramp_coeff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DtsRampvalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DtsRampvalr {{ ts1_ramp_coeff: {=u16:?} }}",
                self.ts1_ramp_coeff()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DtsSr(pub u32);
    impl DtsSr {
        #[must_use]
        #[inline(always)]
        pub const fn ts1_itef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_itef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_itlf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_itlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_ithf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_ithf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_aitef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_aitef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_aitlf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_aitlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_aithf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_aithf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_rdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ts1_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for DtsSr {
        #[inline(always)]
        fn default() -> DtsSr {
            DtsSr(0)
        }
    }
    impl core::fmt::Debug for DtsSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DtsSr")
                .field("ts1_itef", &self.ts1_itef())
                .field("ts1_itlf", &self.ts1_itlf())
                .field("ts1_ithf", &self.ts1_ithf())
                .field("ts1_aitef", &self.ts1_aitef())
                .field("ts1_aitlf", &self.ts1_aitlf())
                .field("ts1_aithf", &self.ts1_aithf())
                .field("ts1_rdy", &self.ts1_rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DtsSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DtsSr {{ ts1_itef: {=bool:?}, ts1_itlf: {=bool:?}, ts1_ithf: {=bool:?}, ts1_aitef: {=bool:?}, ts1_aitlf: {=bool:?}, ts1_aithf: {=bool:?}, ts1_rdy: {=bool:?} }}",
                self.ts1_itef(),
                self.ts1_itlf(),
                self.ts1_ithf(),
                self.ts1_aitef(),
                self.ts1_aitlf(),
                self.ts1_aithf(),
                self.ts1_rdy()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DtsT0valr1(pub u32);
    impl DtsT0valr1 {
        #[must_use]
        #[inline(always)]
        pub const fn ts1_fmt0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ts1_fmt0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ts1_t0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ts1_t0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for DtsT0valr1 {
        #[inline(always)]
        fn default() -> DtsT0valr1 {
            DtsT0valr1(0)
        }
    }
    impl core::fmt::Debug for DtsT0valr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DtsT0valr1")
                .field("ts1_fmt0", &self.ts1_fmt0())
                .field("ts1_t0", &self.ts1_t0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DtsT0valr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DtsT0valr1 {{ ts1_fmt0: {=u16:?}, ts1_t0: {=u8:?} }}",
                self.ts1_fmt0(),
                self.ts1_t0()
            )
        }
    }
}
