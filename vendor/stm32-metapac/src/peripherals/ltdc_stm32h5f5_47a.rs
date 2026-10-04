#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ltdc {
    ptr: *mut u8,
}
unsafe impl Send for Ltdc {}
unsafe impl Sync for Ltdc {}
impl Ltdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn sscr(self) -> crate::common::Reg<regs::LtdcSscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn bpcr(self) -> crate::common::Reg<regs::LtdcBpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn awcr(self) -> crate::common::Reg<regs::LtdcAwcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn twcr(self) -> crate::common::Reg<regs::LtdcTwcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::LtdcGcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn srcr(self) -> crate::common::Reg<regs::LtdcSrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn bccr(self) -> crate::common::Reg<regs::LtdcBccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::LtdcIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::LtdcIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::LtdcIcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn lipcr(self) -> crate::common::Reg<regs::LtdcLipcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn cpsr(self) -> crate::common::Reg<regs::LtdcCpsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn cdsr(self) -> crate::common::Reg<regs::LtdcCdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcAwcr(pub u32);
    impl LtdcAwcr {
        #[must_use]
        #[inline(always)]
        pub const fn aah(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_aah(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aaw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_aaw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for LtdcAwcr {
        #[inline(always)]
        fn default() -> LtdcAwcr {
            LtdcAwcr(0)
        }
    }
    impl core::fmt::Debug for LtdcAwcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcAwcr")
                .field("aah", &self.aah())
                .field("aaw", &self.aaw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcAwcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcAwcr {{ aah: {=u16:?}, aaw: {=u16:?} }}",
                self.aah(),
                self.aaw()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcBccr(pub u32);
    impl LtdcBccr {
        #[must_use]
        #[inline(always)]
        pub const fn bcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bcred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for LtdcBccr {
        #[inline(always)]
        fn default() -> LtdcBccr {
            LtdcBccr(0)
        }
    }
    impl core::fmt::Debug for LtdcBccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcBccr")
                .field("bcblue", &self.bcblue())
                .field("bcgreen", &self.bcgreen())
                .field("bcred", &self.bcred())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcBccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcBccr {{ bcblue: {=u8:?}, bcgreen: {=u8:?}, bcred: {=u8:?} }}",
                self.bcblue(),
                self.bcgreen(),
                self.bcred()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcBpcr(pub u32);
    impl LtdcBpcr {
        #[must_use]
        #[inline(always)]
        pub const fn avbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_avbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ahbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ahbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for LtdcBpcr {
        #[inline(always)]
        fn default() -> LtdcBpcr {
            LtdcBpcr(0)
        }
    }
    impl core::fmt::Debug for LtdcBpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcBpcr")
                .field("avbp", &self.avbp())
                .field("ahbp", &self.ahbp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcBpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcBpcr {{ avbp: {=u16:?}, ahbp: {=u16:?} }}",
                self.avbp(),
                self.ahbp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcCdsr(pub u32);
    impl LtdcCdsr {
        #[must_use]
        #[inline(always)]
        pub const fn vdes(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hdes(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vsyncs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hsyncs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for LtdcCdsr {
        #[inline(always)]
        fn default() -> LtdcCdsr {
            LtdcCdsr(0)
        }
    }
    impl core::fmt::Debug for LtdcCdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcCdsr")
                .field("vdes", &self.vdes())
                .field("hdes", &self.hdes())
                .field("vsyncs", &self.vsyncs())
                .field("hsyncs", &self.hsyncs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcCdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcCdsr {{ vdes: {=bool:?}, hdes: {=bool:?}, vsyncs: {=bool:?}, hsyncs: {=bool:?} }}",
                self.vdes(),
                self.hdes(),
                self.vsyncs(),
                self.hsyncs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcCpsr(pub u32);
    impl LtdcCpsr {
        #[must_use]
        #[inline(always)]
        pub const fn cypos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_cypos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cxpos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_cxpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for LtdcCpsr {
        #[inline(always)]
        fn default() -> LtdcCpsr {
            LtdcCpsr(0)
        }
    }
    impl core::fmt::Debug for LtdcCpsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcCpsr")
                .field("cypos", &self.cypos())
                .field("cxpos", &self.cxpos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcCpsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcCpsr {{ cypos: {=u16:?}, cxpos: {=u16:?} }}",
                self.cypos(),
                self.cxpos()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcGcr(pub u32);
    impl LtdcGcr {
        #[must_use]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbw(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dbw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dgw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dgw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn drw(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_drw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pcpol(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pcpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn depol(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_depol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vspol(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hspol(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LtdcGcr {
        #[inline(always)]
        fn default() -> LtdcGcr {
            LtdcGcr(0)
        }
    }
    impl core::fmt::Debug for LtdcGcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcGcr")
                .field("ltdcen", &self.ltdcen())
                .field("dbw", &self.dbw())
                .field("dgw", &self.dgw())
                .field("drw", &self.drw())
                .field("den", &self.den())
                .field("pcpol", &self.pcpol())
                .field("depol", &self.depol())
                .field("vspol", &self.vspol())
                .field("hspol", &self.hspol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcGcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcGcr {{ ltdcen: {=bool:?}, dbw: {=u8:?}, dgw: {=u8:?}, drw: {=u8:?}, den: {=bool:?}, pcpol: {=bool:?}, depol: {=bool:?}, vspol: {=bool:?}, hspol: {=bool:?} }}",
                self.ltdcen(),
                self.dbw(),
                self.dgw(),
                self.drw(),
                self.den(),
                self.pcpol(),
                self.depol(),
                self.vspol(),
                self.hspol()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcIcr(pub u32);
    impl LtdcIcr {
        #[must_use]
        #[inline(always)]
        pub const fn clif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cfuif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cfuif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cterrif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cterrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn crrif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_crrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for LtdcIcr {
        #[inline(always)]
        fn default() -> LtdcIcr {
            LtdcIcr(0)
        }
    }
    impl core::fmt::Debug for LtdcIcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcIcr")
                .field("clif", &self.clif())
                .field("cfuif", &self.cfuif())
                .field("cterrif", &self.cterrif())
                .field("crrif", &self.crrif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcIcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcIcr {{ clif: {=bool:?}, cfuif: {=bool:?}, cterrif: {=bool:?}, crrif: {=bool:?} }}",
                self.clif(),
                self.cfuif(),
                self.cterrif(),
                self.crrif()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcIer(pub u32);
    impl LtdcIer {
        #[must_use]
        #[inline(always)]
        pub const fn lie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fuie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn terrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_terrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for LtdcIer {
        #[inline(always)]
        fn default() -> LtdcIer {
            LtdcIer(0)
        }
    }
    impl core::fmt::Debug for LtdcIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcIer")
                .field("lie", &self.lie())
                .field("fuie", &self.fuie())
                .field("terrie", &self.terrie())
                .field("rrie", &self.rrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcIer {{ lie: {=bool:?}, fuie: {=bool:?}, terrie: {=bool:?}, rrie: {=bool:?} }}",
                self.lie(),
                self.fuie(),
                self.terrie(),
                self.rrie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcIsr(pub u32);
    impl LtdcIsr {
        #[must_use]
        #[inline(always)]
        pub const fn lif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fuif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fuif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn terrif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_terrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for LtdcIsr {
        #[inline(always)]
        fn default() -> LtdcIsr {
            LtdcIsr(0)
        }
    }
    impl core::fmt::Debug for LtdcIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcIsr")
                .field("lif", &self.lif())
                .field("fuif", &self.fuif())
                .field("terrif", &self.terrif())
                .field("rrif", &self.rrif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcIsr {{ lif: {=bool:?}, fuif: {=bool:?}, terrif: {=bool:?}, rrif: {=bool:?} }}",
                self.lif(),
                self.fuif(),
                self.terrif(),
                self.rrif()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcLipcr(pub u32);
    impl LtdcLipcr {
        #[must_use]
        #[inline(always)]
        pub const fn lipos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_lipos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for LtdcLipcr {
        #[inline(always)]
        fn default() -> LtdcLipcr {
            LtdcLipcr(0)
        }
    }
    impl core::fmt::Debug for LtdcLipcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcLipcr")
                .field("lipos", &self.lipos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcLipcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LtdcLipcr {{ lipos: {=u16:?} }}", self.lipos())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcSrcr(pub u32);
    impl LtdcSrcr {
        #[must_use]
        #[inline(always)]
        pub const fn imr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_imr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vbr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vbr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for LtdcSrcr {
        #[inline(always)]
        fn default() -> LtdcSrcr {
            LtdcSrcr(0)
        }
    }
    impl core::fmt::Debug for LtdcSrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcSrcr")
                .field("imr", &self.imr())
                .field("vbr", &self.vbr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcSrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcSrcr {{ imr: {=bool:?}, vbr: {=bool:?} }}",
                self.imr(),
                self.vbr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcSscr(pub u32);
    impl LtdcSscr {
        #[must_use]
        #[inline(always)]
        pub const fn vsh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vsh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hsw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_hsw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for LtdcSscr {
        #[inline(always)]
        fn default() -> LtdcSscr {
            LtdcSscr(0)
        }
    }
    impl core::fmt::Debug for LtdcSscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcSscr")
                .field("vsh", &self.vsh())
                .field("hsw", &self.hsw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcSscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcSscr {{ vsh: {=u16:?}, hsw: {=u16:?} }}",
                self.vsh(),
                self.hsw()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LtdcTwcr(pub u32);
    impl LtdcTwcr {
        #[must_use]
        #[inline(always)]
        pub const fn totalh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_totalh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn totalw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_totalw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for LtdcTwcr {
        #[inline(always)]
        fn default() -> LtdcTwcr {
            LtdcTwcr(0)
        }
    }
    impl core::fmt::Debug for LtdcTwcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LtdcTwcr")
                .field("totalh", &self.totalh())
                .field("totalw", &self.totalw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LtdcTwcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LtdcTwcr {{ totalh: {=u16:?}, totalw: {=u16:?} }}",
                self.totalh(),
                self.totalw()
            )
        }
    }
}
