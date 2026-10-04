#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2d {
    ptr: *mut u8,
}
unsafe impl Send for Dma2d {}
unsafe impl Sync for Dma2d {}
impl Dma2d {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Dma2dCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Dma2dIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Dma2dIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Dma2dIfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn fgmar(self) -> crate::common::Reg<regs::Dma2dFgmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn fgor(self) -> crate::common::Reg<regs::Dma2dFgor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn fgmsr(self) -> crate::common::Reg<regs::Dma2dFgmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn fgpfccr(self) -> crate::common::Reg<regs::Dma2dFgpfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn fgcolr(self) -> crate::common::Reg<regs::Dma2dFgcolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn fgcmar(self) -> crate::common::Reg<regs::Dma2dFgcmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn bgmar(self) -> crate::common::Reg<regs::Dma2dBgmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn bgor(self) -> crate::common::Reg<regs::Dma2dBgor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn bgmsr(self) -> crate::common::Reg<regs::Dma2dBgmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn bgpfccr(self) -> crate::common::Reg<regs::Dma2dBgpfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn bgcolr(self) -> crate::common::Reg<regs::Dma2dBgcolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn bgcmar(self) -> crate::common::Reg<regs::Dma2dBgcmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn opfccr(self) -> crate::common::Reg<regs::Dma2dOpfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn ocolr(self) -> crate::common::Reg<regs::Dma2dOcolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn omar(self) -> crate::common::Reg<regs::Dma2dOmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn oor(self) -> crate::common::Reg<regs::Dma2dOor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn nlr(self) -> crate::common::Reg<regs::Dma2dNlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn lwr(self) -> crate::common::Reg<regs::Dma2dLwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn amtcr(self) -> crate::common::Reg<regs::Dma2dAmtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn sbcr(self) -> crate::common::Reg<regs::Dma2dSbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn sbmar(self) -> crate::common::Reg<regs::Dma2dSbmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn sbor(self) -> crate::common::Reg<regs::Dma2dSbor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn sbmsr(self) -> crate::common::Reg<regs::Dma2dSbmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn tbcr(self) -> crate::common::Reg<regs::Dma2dTbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Dma2dScr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn snlr(self) -> crate::common::Reg<regs::Dma2dSnlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Dma2dSsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn spr(self) -> crate::common::Reg<regs::Dma2dSpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[inline(always)]
    pub const fn gpfcr(self) -> crate::common::Reg<regs::Dma2dGpfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn gpfr(self) -> crate::common::Reg<regs::Dma2dGpfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn gpfsr(self) -> crate::common::Reg<regs::Dma2dGpfsr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[inline(always)]
    pub const fn gpfrr(self) -> crate::common::Reg<regs::Dma2dGpfrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn clcr(self) -> crate::common::Reg<regs::Dma2dClcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn clsr(self) -> crate::common::Reg<regs::Dma2dClsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[inline(always)]
    pub const fn rbbar(self) -> crate::common::Reg<regs::Dma2dRbbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[inline(always)]
    pub const fn rbhpr(self) -> crate::common::Reg<regs::Dma2dRbhpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[inline(always)]
    pub const fn rbwpr(self) -> crate::common::Reg<regs::Dma2dRbwpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[inline(always)]
    pub const fn lbcbar(self) -> crate::common::Reg<regs::Dma2dLbcbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn lbcsr(self) -> crate::common::Reg<regs::Dma2dLbcsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[inline(always)]
    pub const fn lbcar(self) -> crate::common::Reg<regs::Dma2dLbcar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[inline(always)]
    pub const fn verr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[inline(always)]
    pub const fn ipidr(self) -> crate::common::Reg<regs::Dma2dIpidr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[inline(always)]
    pub const fn sidr(self) -> crate::common::Reg<regs::Dma2dSidr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[inline(always)]
    pub const fn fgclut(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn bgclut(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dma2dBgclut, crate::common::Raw> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 4usize) as _)
        }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dAmtcr(pub u32);
    impl Dma2dAmtcr {
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Dma2dAmtcr {
        #[inline(always)]
        fn default() -> Dma2dAmtcr {
            Dma2dAmtcr(0)
        }
    }
    impl core::fmt::Debug for Dma2dAmtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dAmtcr")
                .field("en", &self.en())
                .field("dt", &self.dt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dAmtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dAmtcr {{ en: {=bool:?}, dt: {=u8:?} }}",
                self.en(),
                self.dt()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dBgclut(pub u32);
    impl Dma2dBgclut {
        #[must_use]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_alpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dma2dBgclut {
        #[inline(always)]
        fn default() -> Dma2dBgclut {
            Dma2dBgclut(0)
        }
    }
    impl core::fmt::Debug for Dma2dBgclut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dBgclut")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .field("alpha", &self.alpha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dBgclut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dBgclut {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?}, alpha: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red(),
                self.alpha()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dBgcmar(pub u32);
    impl Dma2dBgcmar {
        #[must_use]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dBgcmar {
        #[inline(always)]
        fn default() -> Dma2dBgcmar {
            Dma2dBgcmar(0)
        }
    }
    impl core::fmt::Debug for Dma2dBgcmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dBgcmar")
                .field("ma", &self.ma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dBgcmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dBgcmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dBgcolr(pub u32);
    impl Dma2dBgcolr {
        #[must_use]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Dma2dBgcolr {
        #[inline(always)]
        fn default() -> Dma2dBgcolr {
            Dma2dBgcolr(0)
        }
    }
    impl core::fmt::Debug for Dma2dBgcolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dBgcolr")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dBgcolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dBgcolr {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dBgmar(pub u32);
    impl Dma2dBgmar {
        #[must_use]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dBgmar {
        #[inline(always)]
        fn default() -> Dma2dBgmar {
            Dma2dBgmar(0)
        }
    }
    impl core::fmt::Debug for Dma2dBgmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dBgmar")
                .field("ma", &self.ma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dBgmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dBgmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dBgmsr(pub u32);
    impl Dma2dBgmsr {
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn htrail(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_htrail(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vpre(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vtrail(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vtrail(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Dma2dBgmsr {
        #[inline(always)]
        fn default() -> Dma2dBgmsr {
            Dma2dBgmsr(0)
        }
    }
    impl core::fmt::Debug for Dma2dBgmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dBgmsr")
                .field("hpre", &self.hpre())
                .field("htrail", &self.htrail())
                .field("vpre", &self.vpre())
                .field("vtrail", &self.vtrail())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dBgmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dBgmsr {{ hpre: {=u8:?}, htrail: {=u8:?}, vpre: {=u8:?}, vtrail: {=u8:?} }}",
                self.hpre(),
                self.htrail(),
                self.vpre(),
                self.vtrail()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dBgor(pub u32);
    impl Dma2dBgor {
        #[must_use]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dma2dBgor {
        #[inline(always)]
        fn default() -> Dma2dBgor {
            Dma2dBgor(0)
        }
    }
    impl core::fmt::Debug for Dma2dBgor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dBgor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dBgor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dBgor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dBgpfccr(pub u32);
    impl Dma2dBgpfccr {
        #[must_use]
        #[inline(always)]
        pub const fn cm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
        pub const fn cs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn am(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_am(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn css(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_css(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ai(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ai(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbs(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apos(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_apos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_alpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dma2dBgpfccr {
        #[inline(always)]
        fn default() -> Dma2dBgpfccr {
            Dma2dBgpfccr(0)
        }
    }
    impl core::fmt::Debug for Dma2dBgpfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dBgpfccr")
                .field("cm", &self.cm())
                .field("start", &self.start())
                .field("cs", &self.cs())
                .field("am", &self.am())
                .field("css", &self.css())
                .field("ai", &self.ai())
                .field("rbs", &self.rbs())
                .field("apos", &self.apos())
                .field("alpha", &self.alpha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dBgpfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dBgpfccr {{ cm: {=u8:?}, start: {=bool:?}, cs: {=u8:?}, am: {=u8:?}, css: {=u8:?}, ai: {=bool:?}, rbs: {=bool:?}, apos: {=bool:?}, alpha: {=u8:?} }}",
                self.cm(),
                self.start(),
                self.cs(),
                self.am(),
                self.css(),
                self.ai(),
                self.rbs(),
                self.apos(),
                self.alpha()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dClcr(pub u32);
    impl Dma2dClcr {
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abort(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_abort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for Dma2dClcr {
        #[inline(always)]
        fn default() -> Dma2dClcr {
            Dma2dClcr(0)
        }
    }
    impl core::fmt::Debug for Dma2dClcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dClcr")
                .field("start", &self.start())
                .field("susp", &self.susp())
                .field("abort", &self.abort())
                .field("rbs", &self.rbs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dClcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dClcr {{ start: {=bool:?}, susp: {=bool:?}, abort: {=bool:?}, rbs: {=u8:?} }}",
                self.start(),
                self.susp(),
                self.abort(),
                self.rbs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dClsr(pub u32);
    impl Dma2dClsr {
        #[must_use]
        #[inline(always)]
        pub const fn susps(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_susps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf0s(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf0s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf1s(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf2s(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf2s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf3s(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf3s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lclre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lclre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lclie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lclie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lclmse(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lclmse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Dma2dClsr {
        #[inline(always)]
        fn default() -> Dma2dClsr {
            Dma2dClsr(0)
        }
    }
    impl core::fmt::Debug for Dma2dClsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dClsr")
                .field("susps", &self.susps())
                .field("gpf0s", &self.gpf0s())
                .field("gpf1s", &self.gpf1s())
                .field("gpf2s", &self.gpf2s())
                .field("gpf3s", &self.gpf3s())
                .field("lclre", &self.lclre())
                .field("lclie", &self.lclie())
                .field("lclmse", &self.lclmse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dClsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dClsr {{ susps: {=bool:?}, gpf0s: {=bool:?}, gpf1s: {=bool:?}, gpf2s: {=bool:?}, gpf3s: {=bool:?}, lclre: {=bool:?}, lclie: {=bool:?}, lclmse: {=bool:?} }}",
                self.susps(),
                self.gpf0s(),
                self.gpf1s(),
                self.gpf2s(),
                self.gpf3s(),
                self.lclre(),
                self.lclie(),
                self.lclmse()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dCr(pub u32);
    impl Dma2dCr {
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn abort(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_abort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Dma2dCr {
        #[inline(always)]
        fn default() -> Dma2dCr {
            Dma2dCr(0)
        }
    }
    impl core::fmt::Debug for Dma2dCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dCr")
                .field("start", &self.start())
                .field("susp", &self.susp())
                .field("abort", &self.abort())
                .field("lom", &self.lom())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dCr {{ start: {=bool:?}, susp: {=bool:?}, abort: {=bool:?}, lom: {=bool:?}, mode: {=u8:?} }}",
                self.start(),
                self.susp(),
                self.abort(),
                self.lom(),
                self.mode()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dFgcmar(pub u32);
    impl Dma2dFgcmar {
        #[must_use]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dFgcmar {
        #[inline(always)]
        fn default() -> Dma2dFgcmar {
            Dma2dFgcmar(0)
        }
    }
    impl core::fmt::Debug for Dma2dFgcmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dFgcmar")
                .field("ma", &self.ma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dFgcmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dFgcmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dFgcolr(pub u32);
    impl Dma2dFgcolr {
        #[must_use]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Dma2dFgcolr {
        #[inline(always)]
        fn default() -> Dma2dFgcolr {
            Dma2dFgcolr(0)
        }
    }
    impl core::fmt::Debug for Dma2dFgcolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dFgcolr")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dFgcolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dFgcolr {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dFgmar(pub u32);
    impl Dma2dFgmar {
        #[must_use]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dFgmar {
        #[inline(always)]
        fn default() -> Dma2dFgmar {
            Dma2dFgmar(0)
        }
    }
    impl core::fmt::Debug for Dma2dFgmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dFgmar")
                .field("ma", &self.ma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dFgmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dFgmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dFgmsr(pub u32);
    impl Dma2dFgmsr {
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn htrail(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_htrail(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vpre(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vtrail(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vtrail(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Dma2dFgmsr {
        #[inline(always)]
        fn default() -> Dma2dFgmsr {
            Dma2dFgmsr(0)
        }
    }
    impl core::fmt::Debug for Dma2dFgmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dFgmsr")
                .field("hpre", &self.hpre())
                .field("htrail", &self.htrail())
                .field("vpre", &self.vpre())
                .field("vtrail", &self.vtrail())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dFgmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dFgmsr {{ hpre: {=u8:?}, htrail: {=u8:?}, vpre: {=u8:?}, vtrail: {=u8:?} }}",
                self.hpre(),
                self.htrail(),
                self.vpre(),
                self.vtrail()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dFgor(pub u32);
    impl Dma2dFgor {
        #[must_use]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dma2dFgor {
        #[inline(always)]
        fn default() -> Dma2dFgor {
            Dma2dFgor(0)
        }
    }
    impl core::fmt::Debug for Dma2dFgor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dFgor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dFgor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dFgor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dFgpfccr(pub u32);
    impl Dma2dFgpfccr {
        #[must_use]
        #[inline(always)]
        pub const fn cm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
        pub const fn cs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn am(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_am(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn css(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_css(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ai(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ai(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbs(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apos(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_apos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_alpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dma2dFgpfccr {
        #[inline(always)]
        fn default() -> Dma2dFgpfccr {
            Dma2dFgpfccr(0)
        }
    }
    impl core::fmt::Debug for Dma2dFgpfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dFgpfccr")
                .field("cm", &self.cm())
                .field("start", &self.start())
                .field("cs", &self.cs())
                .field("am", &self.am())
                .field("css", &self.css())
                .field("ai", &self.ai())
                .field("rbs", &self.rbs())
                .field("apos", &self.apos())
                .field("alpha", &self.alpha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dFgpfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dFgpfccr {{ cm: {=u8:?}, start: {=bool:?}, cs: {=u8:?}, am: {=u8:?}, css: {=u8:?}, ai: {=bool:?}, rbs: {=bool:?}, apos: {=bool:?}, alpha: {=u8:?} }}",
                self.cm(),
                self.start(),
                self.cs(),
                self.am(),
                self.css(),
                self.ai(),
                self.rbs(),
                self.apos(),
                self.alpha()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dGpfcr(pub u32);
    impl Dma2dGpfcr {
        #[must_use]
        #[inline(always)]
        pub const fn gpf0ic(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_gpf0ic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf1ic(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_gpf1ic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf2ic(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_gpf2ic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf3ic(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_gpf3ic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf0sc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf0sc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf1sc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf1sc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf2sc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf2sc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf3sc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf3sc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Dma2dGpfcr {
        #[inline(always)]
        fn default() -> Dma2dGpfcr {
            Dma2dGpfcr(0)
        }
    }
    impl core::fmt::Debug for Dma2dGpfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dGpfcr")
                .field("gpf0ic", &self.gpf0ic())
                .field("gpf1ic", &self.gpf1ic())
                .field("gpf2ic", &self.gpf2ic())
                .field("gpf3ic", &self.gpf3ic())
                .field("gpf0sc", &self.gpf0sc())
                .field("gpf1sc", &self.gpf1sc())
                .field("gpf2sc", &self.gpf2sc())
                .field("gpf3sc", &self.gpf3sc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dGpfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dGpfcr {{ gpf0ic: {=u8:?}, gpf1ic: {=u8:?}, gpf2ic: {=u8:?}, gpf3ic: {=u8:?}, gpf0sc: {=bool:?}, gpf1sc: {=bool:?}, gpf2sc: {=bool:?}, gpf3sc: {=bool:?} }}",
                self.gpf0ic(),
                self.gpf1ic(),
                self.gpf2ic(),
                self.gpf3ic(),
                self.gpf0sc(),
                self.gpf1sc(),
                self.gpf2sc(),
                self.gpf3sc()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dGpfr(pub u32);
    impl Dma2dGpfr {
        #[must_use]
        #[inline(always)]
        pub const fn gpf0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Dma2dGpfr {
        #[inline(always)]
        fn default() -> Dma2dGpfr {
            Dma2dGpfr(0)
        }
    }
    impl core::fmt::Debug for Dma2dGpfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dGpfr")
                .field("gpf0", &self.gpf0())
                .field("gpf1", &self.gpf1())
                .field("gpf2", &self.gpf2())
                .field("gpf3", &self.gpf3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dGpfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dGpfr {{ gpf0: {=bool:?}, gpf1: {=bool:?}, gpf2: {=bool:?}, gpf3: {=bool:?} }}",
                self.gpf0(),
                self.gpf1(),
                self.gpf2(),
                self.gpf3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dGpfrr(pub u32);
    impl Dma2dGpfrr {
        #[must_use]
        #[inline(always)]
        pub const fn rgpf0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rgpf0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rgpf1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rgpf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rgpf2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rgpf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rgpf3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rgpf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Dma2dGpfrr {
        #[inline(always)]
        fn default() -> Dma2dGpfrr {
            Dma2dGpfrr(0)
        }
    }
    impl core::fmt::Debug for Dma2dGpfrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dGpfrr")
                .field("rgpf0", &self.rgpf0())
                .field("rgpf1", &self.rgpf1())
                .field("rgpf2", &self.rgpf2())
                .field("rgpf3", &self.rgpf3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dGpfrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dGpfrr {{ rgpf0: {=bool:?}, rgpf1: {=bool:?}, rgpf2: {=bool:?}, rgpf3: {=bool:?} }}",
                self.rgpf0(),
                self.rgpf1(),
                self.rgpf2(),
                self.rgpf3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dGpfsr(pub u32);
    impl Dma2dGpfsr {
        #[must_use]
        #[inline(always)]
        pub const fn sgpf0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sgpf0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sgpf1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sgpf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sgpf2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sgpf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sgpf3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sgpf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Dma2dGpfsr {
        #[inline(always)]
        fn default() -> Dma2dGpfsr {
            Dma2dGpfsr(0)
        }
    }
    impl core::fmt::Debug for Dma2dGpfsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dGpfsr")
                .field("sgpf0", &self.sgpf0())
                .field("sgpf1", &self.sgpf1())
                .field("sgpf2", &self.sgpf2())
                .field("sgpf3", &self.sgpf3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dGpfsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dGpfsr {{ sgpf0: {=bool:?}, sgpf1: {=bool:?}, sgpf2: {=bool:?}, sgpf3: {=bool:?} }}",
                self.sgpf0(),
                self.sgpf1(),
                self.sgpf2(),
                self.sgpf3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dIer(pub u32);
    impl Dma2dIer {
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn twie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_twie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn caeie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_caeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctcie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ceie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ceie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbcie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbeie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clsie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cleie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cleie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf0ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf1ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf2ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf3ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Dma2dIer {
        #[inline(always)]
        fn default() -> Dma2dIer {
            Dma2dIer(0)
        }
    }
    impl core::fmt::Debug for Dma2dIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dIer")
                .field("teie", &self.teie())
                .field("tcie", &self.tcie())
                .field("twie", &self.twie())
                .field("caeie", &self.caeie())
                .field("ctcie", &self.ctcie())
                .field("ceie", &self.ceie())
                .field("rbcie", &self.rbcie())
                .field("rbeie", &self.rbeie())
                .field("clsie", &self.clsie())
                .field("cleie", &self.cleie())
                .field("gpf0ie", &self.gpf0ie())
                .field("gpf1ie", &self.gpf1ie())
                .field("gpf2ie", &self.gpf2ie())
                .field("gpf3ie", &self.gpf3ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dIer {{ teie: {=bool:?}, tcie: {=bool:?}, twie: {=bool:?}, caeie: {=bool:?}, ctcie: {=bool:?}, ceie: {=bool:?}, rbcie: {=bool:?}, rbeie: {=bool:?}, clsie: {=bool:?}, cleie: {=bool:?}, gpf0ie: {=bool:?}, gpf1ie: {=bool:?}, gpf2ie: {=bool:?}, gpf3ie: {=bool:?} }}",
                self.teie(),
                self.tcie(),
                self.twie(),
                self.caeie(),
                self.ctcie(),
                self.ceie(),
                self.rbcie(),
                self.rbeie(),
                self.clsie(),
                self.cleie(),
                self.gpf0ie(),
                self.gpf1ie(),
                self.gpf2ie(),
                self.gpf3ie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dIfcr(pub u32);
    impl Dma2dIfcr {
        #[must_use]
        #[inline(always)]
        pub const fn cteif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cteif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctcif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctwif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctwif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn caecif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_caecif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cctcif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cctcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cceif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cceif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbcie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbeie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clsie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cleie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cleie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cgpf0if(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cgpf0if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cgpf1if(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cgpf1if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cgpf2if(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cgpf2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cgpf3if(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cgpf3if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Dma2dIfcr {
        #[inline(always)]
        fn default() -> Dma2dIfcr {
            Dma2dIfcr(0)
        }
    }
    impl core::fmt::Debug for Dma2dIfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dIfcr")
                .field("cteif", &self.cteif())
                .field("ctcif", &self.ctcif())
                .field("ctwif", &self.ctwif())
                .field("caecif", &self.caecif())
                .field("cctcif", &self.cctcif())
                .field("cceif", &self.cceif())
                .field("rbcie", &self.rbcie())
                .field("rbeie", &self.rbeie())
                .field("clsie", &self.clsie())
                .field("cleie", &self.cleie())
                .field("cgpf0if", &self.cgpf0if())
                .field("cgpf1if", &self.cgpf1if())
                .field("cgpf2if", &self.cgpf2if())
                .field("cgpf3if", &self.cgpf3if())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dIfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dIfcr {{ cteif: {=bool:?}, ctcif: {=bool:?}, ctwif: {=bool:?}, caecif: {=bool:?}, cctcif: {=bool:?}, cceif: {=bool:?}, rbcie: {=bool:?}, rbeie: {=bool:?}, clsie: {=bool:?}, cleie: {=bool:?}, cgpf0if: {=bool:?}, cgpf1if: {=bool:?}, cgpf2if: {=bool:?}, cgpf3if: {=bool:?} }}",
                self.cteif(),
                self.ctcif(),
                self.ctwif(),
                self.caecif(),
                self.cctcif(),
                self.cceif(),
                self.rbcie(),
                self.rbeie(),
                self.clsie(),
                self.cleie(),
                self.cgpf0if(),
                self.cgpf1if(),
                self.cgpf2if(),
                self.cgpf3if()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dIpidr(pub u32);
    impl Dma2dIpidr {
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dIpidr {
        #[inline(always)]
        fn default() -> Dma2dIpidr {
            Dma2dIpidr(0)
        }
    }
    impl core::fmt::Debug for Dma2dIpidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dIpidr")
                .field("id", &self.id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dIpidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dIpidr {{ id: {=u32:?} }}", self.id())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dIsr(pub u32);
    impl Dma2dIsr {
        #[must_use]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tcif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn twif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_twif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn caeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_caeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctcif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ceif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ceif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbcif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbeif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn clsif(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clsif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cleif(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cleif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf0if(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf0if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf1if(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf1if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf2if(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpf3if(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpf3if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Dma2dIsr {
        #[inline(always)]
        fn default() -> Dma2dIsr {
            Dma2dIsr(0)
        }
    }
    impl core::fmt::Debug for Dma2dIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dIsr")
                .field("teif", &self.teif())
                .field("tcif", &self.tcif())
                .field("twif", &self.twif())
                .field("caeif", &self.caeif())
                .field("ctcif", &self.ctcif())
                .field("ceif", &self.ceif())
                .field("rbcif", &self.rbcif())
                .field("rbeif", &self.rbeif())
                .field("clsif", &self.clsif())
                .field("cleif", &self.cleif())
                .field("gpf0if", &self.gpf0if())
                .field("gpf1if", &self.gpf1if())
                .field("gpf2if", &self.gpf2if())
                .field("gpf3if", &self.gpf3if())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dIsr {{ teif: {=bool:?}, tcif: {=bool:?}, twif: {=bool:?}, caeif: {=bool:?}, ctcif: {=bool:?}, ceif: {=bool:?}, rbcif: {=bool:?}, rbeif: {=bool:?}, clsif: {=bool:?}, cleif: {=bool:?}, gpf0if: {=bool:?}, gpf1if: {=bool:?}, gpf2if: {=bool:?}, gpf3if: {=bool:?} }}",
                self.teif(),
                self.tcif(),
                self.twif(),
                self.caeif(),
                self.ctcif(),
                self.ceif(),
                self.rbcif(),
                self.rbeif(),
                self.clsif(),
                self.cleif(),
                self.gpf0if(),
                self.gpf1if(),
                self.gpf2if(),
                self.gpf3if()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dLbcar(pub u32);
    impl Dma2dLbcar {
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dLbcar {
        #[inline(always)]
        fn default() -> Dma2dLbcar {
            Dma2dLbcar(0)
        }
    }
    impl core::fmt::Debug for Dma2dLbcar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dLbcar")
                .field("address", &self.address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dLbcar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dLbcar {{ address: {=u32:?} }}", self.address())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dLbcbar(pub u32);
    impl Dma2dLbcbar {
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dLbcbar {
        #[inline(always)]
        fn default() -> Dma2dLbcbar {
            Dma2dLbcbar(0)
        }
    }
    impl core::fmt::Debug for Dma2dLbcbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dLbcbar")
                .field("address", &self.address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dLbcbar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dLbcbar {{ address: {=u32:?} }}", self.address())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dLbcsr(pub u32);
    impl Dma2dLbcsr {
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dma2dLbcsr {
        #[inline(always)]
        fn default() -> Dma2dLbcsr {
            Dma2dLbcsr(0)
        }
    }
    impl core::fmt::Debug for Dma2dLbcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dLbcsr")
                .field("size", &self.size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dLbcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dLbcsr {{ size: {=u16:?} }}", self.size())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dLwr(pub u32);
    impl Dma2dLwr {
        #[must_use]
        #[inline(always)]
        pub const fn lw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_lw(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dma2dLwr {
        #[inline(always)]
        fn default() -> Dma2dLwr {
            Dma2dLwr(0)
        }
    }
    impl core::fmt::Debug for Dma2dLwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dLwr").field("lw", &self.lw()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dLwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dLwr {{ lw: {=u16:?} }}", self.lw())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dNlr(pub u32);
    impl Dma2dNlr {
        #[must_use]
        #[inline(always)]
        pub const fn nl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_nl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pl(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_pl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Dma2dNlr {
        #[inline(always)]
        fn default() -> Dma2dNlr {
            Dma2dNlr(0)
        }
    }
    impl core::fmt::Debug for Dma2dNlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dNlr")
                .field("nl", &self.nl())
                .field("pl", &self.pl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dNlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dNlr {{ nl: {=u16:?}, pl: {=u16:?} }}",
                self.nl(),
                self.pl()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dOcolr(pub u32);
    impl Dma2dOcolr {
        #[must_use]
        #[inline(always)]
        pub const fn blue_1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_blue_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn blue_2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_blue_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn blue_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_blue_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn blue_4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_blue_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn green_4(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_green_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn green_1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_green_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn red_4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_red_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alpha_4(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_alpha_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alpha_3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_alpha_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn red_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_red_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn alpha_1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_alpha_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dma2dOcolr {
        #[inline(always)]
        fn default() -> Dma2dOcolr {
            Dma2dOcolr(0)
        }
    }
    impl core::fmt::Debug for Dma2dOcolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dOcolr")
                .field("blue_1", &self.blue_1())
                .field("blue_2", &self.blue_2())
                .field("blue_3", &self.blue_3())
                .field("blue_4", &self.blue_4())
                .field("green_4", &self.green_4())
                .field("green_1", &self.green_1())
                .field("red_4", &self.red_4())
                .field("alpha_4", &self.alpha_4())
                .field("alpha_3", &self.alpha_3())
                .field("red_1", &self.red_1())
                .field("alpha_1", &self.alpha_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dOcolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dOcolr {{ blue_1: {=u8:?}, blue_2: {=u8:?}, blue_3: {=u8:?}, blue_4: {=u8:?}, green_4: {=u8:?}, green_1: {=u8:?}, red_4: {=u8:?}, alpha_4: {=u8:?}, alpha_3: {=bool:?}, red_1: {=u8:?}, alpha_1: {=u8:?} }}",
                self.blue_1(),
                self.blue_2(),
                self.blue_3(),
                self.blue_4(),
                self.green_4(),
                self.green_1(),
                self.red_4(),
                self.alpha_4(),
                self.alpha_3(),
                self.red_1(),
                self.alpha_1()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dOmar(pub u32);
    impl Dma2dOmar {
        #[must_use]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dOmar {
        #[inline(always)]
        fn default() -> Dma2dOmar {
            Dma2dOmar(0)
        }
    }
    impl core::fmt::Debug for Dma2dOmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dOmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dOmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dOmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dOor(pub u32);
    impl Dma2dOor {
        #[must_use]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dma2dOor {
        #[inline(always)]
        fn default() -> Dma2dOor {
            Dma2dOor(0)
        }
    }
    impl core::fmt::Debug for Dma2dOor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dOor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dOor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dOor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dOpfccr(pub u32);
    impl Dma2dOpfccr {
        #[must_use]
        #[inline(always)]
        pub const fn cm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sb(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ai(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ai(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbs(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apos(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_apos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Dma2dOpfccr {
        #[inline(always)]
        fn default() -> Dma2dOpfccr {
            Dma2dOpfccr(0)
        }
    }
    impl core::fmt::Debug for Dma2dOpfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dOpfccr")
                .field("cm", &self.cm())
                .field("sb", &self.sb())
                .field("ai", &self.ai())
                .field("rbs", &self.rbs())
                .field("apos", &self.apos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dOpfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dOpfccr {{ cm: {=u8:?}, sb: {=bool:?}, ai: {=bool:?}, rbs: {=bool:?}, apos: {=bool:?} }}",
                self.cm(),
                self.sb(),
                self.ai(),
                self.rbs(),
                self.apos()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dRbbar(pub u32);
    impl Dma2dRbbar {
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dRbbar {
        #[inline(always)]
        fn default() -> Dma2dRbbar {
            Dma2dRbbar(0)
        }
    }
    impl core::fmt::Debug for Dma2dRbbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dRbbar")
                .field("address", &self.address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dRbbar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dRbbar {{ address: {=u32:?} }}", self.address())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dRbhpr(pub u32);
    impl Dma2dRbhpr {
        #[must_use]
        #[inline(always)]
        pub const fn hp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_hp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Dma2dRbhpr {
        #[inline(always)]
        fn default() -> Dma2dRbhpr {
            Dma2dRbhpr(0)
        }
    }
    impl core::fmt::Debug for Dma2dRbhpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dRbhpr")
                .field("hp", &self.hp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dRbhpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dRbhpr {{ hp: {=u16:?} }}", self.hp())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dRbwpr(pub u32);
    impl Dma2dRbwpr {
        #[must_use]
        #[inline(always)]
        pub const fn wp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_wp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Dma2dRbwpr {
        #[inline(always)]
        fn default() -> Dma2dRbwpr {
            Dma2dRbwpr(0)
        }
    }
    impl core::fmt::Debug for Dma2dRbwpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dRbwpr")
                .field("wp", &self.wp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dRbwpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dRbwpr {{ wp: {=u16:?} }}", self.wp())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dSbcr(pub u32);
    impl Dma2dSbcr {
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ai(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ai(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Dma2dSbcr {
        #[inline(always)]
        fn default() -> Dma2dSbcr {
            Dma2dSbcr(0)
        }
    }
    impl core::fmt::Debug for Dma2dSbcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dSbcr")
                .field("src", &self.src())
                .field("mode", &self.mode())
                .field("ai", &self.ai())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dSbcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dSbcr {{ src: {=u8:?}, mode: {=u8:?}, ai: {=bool:?} }}",
                self.src(),
                self.mode(),
                self.ai()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dSbmar(pub u32);
    impl Dma2dSbmar {
        #[must_use]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dSbmar {
        #[inline(always)]
        fn default() -> Dma2dSbmar {
            Dma2dSbmar(0)
        }
    }
    impl core::fmt::Debug for Dma2dSbmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dSbmar")
                .field("ma", &self.ma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dSbmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dSbmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dSbmsr(pub u32);
    impl Dma2dSbmsr {
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn htrail(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_htrail(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Dma2dSbmsr {
        #[inline(always)]
        fn default() -> Dma2dSbmsr {
            Dma2dSbmsr(0)
        }
    }
    impl core::fmt::Debug for Dma2dSbmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dSbmsr")
                .field("hpre", &self.hpre())
                .field("htrail", &self.htrail())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dSbmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dSbmsr {{ hpre: {=u8:?}, htrail: {=u8:?} }}",
                self.hpre(),
                self.htrail()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dSbor(pub u32);
    impl Dma2dSbor {
        #[must_use]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dma2dSbor {
        #[inline(always)]
        fn default() -> Dma2dSbor {
            Dma2dSbor(0)
        }
    }
    impl core::fmt::Debug for Dma2dSbor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dSbor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dSbor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dSbor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dScr(pub u32);
    impl Dma2dScr {
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Dma2dScr {
        #[inline(always)]
        fn default() -> Dma2dScr {
            Dma2dScr(0)
        }
    }
    impl core::fmt::Debug for Dma2dScr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dScr")
                .field("src", &self.src())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dScr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dScr {{ src: {=u8:?} }}", self.src())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dSidr(pub u32);
    impl Dma2dSidr {
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dma2dSidr {
        #[inline(always)]
        fn default() -> Dma2dSidr {
            Dma2dSidr(0)
        }
    }
    impl core::fmt::Debug for Dma2dSidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dSidr")
                .field("sid", &self.sid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dSidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dma2dSidr {{ sid: {=u32:?} }}", self.sid())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dSnlr(pub u32);
    impl Dma2dSnlr {
        #[must_use]
        #[inline(always)]
        pub const fn nl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_nl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pl(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_pl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Dma2dSnlr {
        #[inline(always)]
        fn default() -> Dma2dSnlr {
            Dma2dSnlr(0)
        }
    }
    impl core::fmt::Debug for Dma2dSnlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dSnlr")
                .field("nl", &self.nl())
                .field("pl", &self.pl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dSnlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dSnlr {{ nl: {=u16:?}, pl: {=u16:?} }}",
                self.nl(),
                self.pl()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dSpr(pub u32);
    impl Dma2dSpr {
        #[must_use]
        #[inline(always)]
        pub const fn hphase(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_hphase(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vphase(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vphase(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Dma2dSpr {
        #[inline(always)]
        fn default() -> Dma2dSpr {
            Dma2dSpr(0)
        }
    }
    impl core::fmt::Debug for Dma2dSpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dSpr")
                .field("hphase", &self.hphase())
                .field("vphase", &self.vphase())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dSpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dSpr {{ hphase: {=u16:?}, vphase: {=u16:?} }}",
                self.hphase(),
                self.vphase()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dSsr(pub u32);
    impl Dma2dSsr {
        #[must_use]
        #[inline(always)]
        pub const fn hstep(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_hstep(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vstep(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vstep(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Dma2dSsr {
        #[inline(always)]
        fn default() -> Dma2dSsr {
            Dma2dSsr(0)
        }
    }
    impl core::fmt::Debug for Dma2dSsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dSsr")
                .field("hstep", &self.hstep())
                .field("vstep", &self.vstep())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dSsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dSsr {{ hstep: {=u16:?}, vstep: {=u16:?} }}",
                self.hstep(),
                self.vstep()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma2dTbcr(pub u32);
    impl Dma2dTbcr {
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn xmen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_xmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ymen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ymen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn xysen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_xysen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Dma2dTbcr {
        #[inline(always)]
        fn default() -> Dma2dTbcr {
            Dma2dTbcr(0)
        }
    }
    impl core::fmt::Debug for Dma2dTbcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma2dTbcr")
                .field("src", &self.src())
                .field("xmen", &self.xmen())
                .field("ymen", &self.ymen())
                .field("xysen", &self.xysen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma2dTbcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma2dTbcr {{ src: {=u8:?}, xmen: {=bool:?}, ymen: {=bool:?}, xysen: {=bool:?} }}",
                self.src(),
                self.xmen(),
                self.ymen(),
                self.xysen()
            )
        }
    }
}
