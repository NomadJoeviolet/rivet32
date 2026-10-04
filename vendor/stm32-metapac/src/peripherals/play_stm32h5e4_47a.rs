#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Play {
    ptr: *mut u8,
}
unsafe impl Send for Play {}
unsafe impl Sync for Play {}
impl Play {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cfgcr(self) -> crate::common::Reg<regs::PlayCfgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::PlaySeccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::PlayPrivcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved1(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn filtcfg(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn lecfg1(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn lecfg2(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved2(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn outcfg(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn reserved3(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 48usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn swin(self) -> crate::common::Reg<regs::PlaySwin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn swinset(self) -> crate::common::Reg<regs::PlaySwinset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn swinclr(self) -> crate::common::Reg<regs::PlaySwinclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved4(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn osr(self) -> crate::common::Reg<regs::PlayOsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved5(self, n: usize) -> crate::common::Reg<u32, crate::common::Raw> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn flstat(self) -> crate::common::Reg<regs::PlayFlstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn flset(self) -> crate::common::Reg<regs::PlayFlset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[inline(always)]
    pub const fn flclr(self) -> crate::common::Reg<regs::PlayFlclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[inline(always)]
    pub const fn flier(self) -> crate::common::Reg<regs::PlayFlier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[inline(always)]
    pub const fn flctl(self) -> crate::common::Reg<regs::PlayFlctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::PlayMsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::PlayIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::PlayIcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::PlayIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayCfgcr(pub u32);
    impl PlayCfgcr {
        #[must_use]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PlayCfgcr {
        #[inline(always)]
        fn default() -> PlayCfgcr {
            PlayCfgcr(0)
        }
    }
    impl core::fmt::Debug for PlayCfgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayCfgcr")
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayCfgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PlayCfgcr {{ unlock: {=bool:?} }}", self.unlock())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayFlclr(pub u32);
    impl PlayFlclr {
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_clr15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_clr15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PlayFlclr {
        #[inline(always)]
        fn default() -> PlayFlclr {
            PlayFlclr(0)
        }
    }
    impl core::fmt::Debug for PlayFlclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayFlclr")
                .field("flagd_clr0", &self.flagd_clr0())
                .field("flagd_clr1", &self.flagd_clr1())
                .field("flagd_clr2", &self.flagd_clr2())
                .field("flagd_clr3", &self.flagd_clr3())
                .field("flagd_clr4", &self.flagd_clr4())
                .field("flagd_clr5", &self.flagd_clr5())
                .field("flagd_clr6", &self.flagd_clr6())
                .field("flagd_clr7", &self.flagd_clr7())
                .field("flagd_clr8", &self.flagd_clr8())
                .field("flagd_clr9", &self.flagd_clr9())
                .field("flagd_clr10", &self.flagd_clr10())
                .field("flagd_clr11", &self.flagd_clr11())
                .field("flagd_clr12", &self.flagd_clr12())
                .field("flagd_clr13", &self.flagd_clr13())
                .field("flagd_clr14", &self.flagd_clr14())
                .field("flagd_clr15", &self.flagd_clr15())
                .field("flagr_clr0", &self.flagr_clr0())
                .field("flagr_clr1", &self.flagr_clr1())
                .field("flagr_clr2", &self.flagr_clr2())
                .field("flagr_clr3", &self.flagr_clr3())
                .field("flagr_clr4", &self.flagr_clr4())
                .field("flagr_clr5", &self.flagr_clr5())
                .field("flagr_clr6", &self.flagr_clr6())
                .field("flagr_clr7", &self.flagr_clr7())
                .field("flagr_clr8", &self.flagr_clr8())
                .field("flagr_clr9", &self.flagr_clr9())
                .field("flagr_clr10", &self.flagr_clr10())
                .field("flagr_clr11", &self.flagr_clr11())
                .field("flagr_clr12", &self.flagr_clr12())
                .field("flagr_clr13", &self.flagr_clr13())
                .field("flagr_clr14", &self.flagr_clr14())
                .field("flagr_clr15", &self.flagr_clr15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayFlclr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayFlclr {{ flagd_clr0: {=bool:?}, flagd_clr1: {=bool:?}, flagd_clr2: {=bool:?}, flagd_clr3: {=bool:?}, flagd_clr4: {=bool:?}, flagd_clr5: {=bool:?}, flagd_clr6: {=bool:?}, flagd_clr7: {=bool:?}, flagd_clr8: {=bool:?}, flagd_clr9: {=bool:?}, flagd_clr10: {=bool:?}, flagd_clr11: {=bool:?}, flagd_clr12: {=bool:?}, flagd_clr13: {=bool:?}, flagd_clr14: {=bool:?}, flagd_clr15: {=bool:?}, flagr_clr0: {=bool:?}, flagr_clr1: {=bool:?}, flagr_clr2: {=bool:?}, flagr_clr3: {=bool:?}, flagr_clr4: {=bool:?}, flagr_clr5: {=bool:?}, flagr_clr6: {=bool:?}, flagr_clr7: {=bool:?}, flagr_clr8: {=bool:?}, flagr_clr9: {=bool:?}, flagr_clr10: {=bool:?}, flagr_clr11: {=bool:?}, flagr_clr12: {=bool:?}, flagr_clr13: {=bool:?}, flagr_clr14: {=bool:?}, flagr_clr15: {=bool:?} }}",
                self.flagd_clr0(),
                self.flagd_clr1(),
                self.flagd_clr2(),
                self.flagd_clr3(),
                self.flagd_clr4(),
                self.flagd_clr5(),
                self.flagd_clr6(),
                self.flagd_clr7(),
                self.flagd_clr8(),
                self.flagd_clr9(),
                self.flagd_clr10(),
                self.flagd_clr11(),
                self.flagd_clr12(),
                self.flagd_clr13(),
                self.flagd_clr14(),
                self.flagd_clr15(),
                self.flagr_clr0(),
                self.flagr_clr1(),
                self.flagr_clr2(),
                self.flagr_clr3(),
                self.flagr_clr4(),
                self.flagr_clr5(),
                self.flagr_clr6(),
                self.flagr_clr7(),
                self.flagr_clr8(),
                self.flagr_clr9(),
                self.flagr_clr10(),
                self.flagr_clr11(),
                self.flagr_clr12(),
                self.flagr_clr13(),
                self.flagr_clr14(),
                self.flagr_clr15()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayFlctl(pub u32);
    impl PlayFlctl {
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_edge15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_edge15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PlayFlctl {
        #[inline(always)]
        fn default() -> PlayFlctl {
            PlayFlctl(0)
        }
    }
    impl core::fmt::Debug for PlayFlctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayFlctl")
                .field("flagd_edge0", &self.flagd_edge0())
                .field("flagd_edge1", &self.flagd_edge1())
                .field("flagd_edge2", &self.flagd_edge2())
                .field("flagd_edge3", &self.flagd_edge3())
                .field("flagd_edge4", &self.flagd_edge4())
                .field("flagd_edge5", &self.flagd_edge5())
                .field("flagd_edge6", &self.flagd_edge6())
                .field("flagd_edge7", &self.flagd_edge7())
                .field("flagd_edge8", &self.flagd_edge8())
                .field("flagd_edge9", &self.flagd_edge9())
                .field("flagd_edge10", &self.flagd_edge10())
                .field("flagd_edge11", &self.flagd_edge11())
                .field("flagd_edge12", &self.flagd_edge12())
                .field("flagd_edge13", &self.flagd_edge13())
                .field("flagd_edge14", &self.flagd_edge14())
                .field("flagd_edge15", &self.flagd_edge15())
                .field("flagr_edge0", &self.flagr_edge0())
                .field("flagr_edge1", &self.flagr_edge1())
                .field("flagr_edge2", &self.flagr_edge2())
                .field("flagr_edge3", &self.flagr_edge3())
                .field("flagr_edge4", &self.flagr_edge4())
                .field("flagr_edge5", &self.flagr_edge5())
                .field("flagr_edge6", &self.flagr_edge6())
                .field("flagr_edge7", &self.flagr_edge7())
                .field("flagr_edge8", &self.flagr_edge8())
                .field("flagr_edge9", &self.flagr_edge9())
                .field("flagr_edge10", &self.flagr_edge10())
                .field("flagr_edge11", &self.flagr_edge11())
                .field("flagr_edge12", &self.flagr_edge12())
                .field("flagr_edge13", &self.flagr_edge13())
                .field("flagr_edge14", &self.flagr_edge14())
                .field("flagr_edge15", &self.flagr_edge15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayFlctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayFlctl {{ flagd_edge0: {=bool:?}, flagd_edge1: {=bool:?}, flagd_edge2: {=bool:?}, flagd_edge3: {=bool:?}, flagd_edge4: {=bool:?}, flagd_edge5: {=bool:?}, flagd_edge6: {=bool:?}, flagd_edge7: {=bool:?}, flagd_edge8: {=bool:?}, flagd_edge9: {=bool:?}, flagd_edge10: {=bool:?}, flagd_edge11: {=bool:?}, flagd_edge12: {=bool:?}, flagd_edge13: {=bool:?}, flagd_edge14: {=bool:?}, flagd_edge15: {=bool:?}, flagr_edge0: {=bool:?}, flagr_edge1: {=bool:?}, flagr_edge2: {=bool:?}, flagr_edge3: {=bool:?}, flagr_edge4: {=bool:?}, flagr_edge5: {=bool:?}, flagr_edge6: {=bool:?}, flagr_edge7: {=bool:?}, flagr_edge8: {=bool:?}, flagr_edge9: {=bool:?}, flagr_edge10: {=bool:?}, flagr_edge11: {=bool:?}, flagr_edge12: {=bool:?}, flagr_edge13: {=bool:?}, flagr_edge14: {=bool:?}, flagr_edge15: {=bool:?} }}",
                self.flagd_edge0(),
                self.flagd_edge1(),
                self.flagd_edge2(),
                self.flagd_edge3(),
                self.flagd_edge4(),
                self.flagd_edge5(),
                self.flagd_edge6(),
                self.flagd_edge7(),
                self.flagd_edge8(),
                self.flagd_edge9(),
                self.flagd_edge10(),
                self.flagd_edge11(),
                self.flagd_edge12(),
                self.flagd_edge13(),
                self.flagd_edge14(),
                self.flagd_edge15(),
                self.flagr_edge0(),
                self.flagr_edge1(),
                self.flagr_edge2(),
                self.flagr_edge3(),
                self.flagr_edge4(),
                self.flagr_edge5(),
                self.flagr_edge6(),
                self.flagr_edge7(),
                self.flagr_edge8(),
                self.flagr_edge9(),
                self.flagr_edge10(),
                self.flagr_edge11(),
                self.flagr_edge12(),
                self.flagr_edge13(),
                self.flagr_edge14(),
                self.flagr_edge15()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayFlier(pub u32);
    impl PlayFlier {
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_ien15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_ien15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PlayFlier {
        #[inline(always)]
        fn default() -> PlayFlier {
            PlayFlier(0)
        }
    }
    impl core::fmt::Debug for PlayFlier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayFlier")
                .field("flagd_ien0", &self.flagd_ien0())
                .field("flagd_ien1", &self.flagd_ien1())
                .field("flagd_ien2", &self.flagd_ien2())
                .field("flagd_ien3", &self.flagd_ien3())
                .field("flagd_ien4", &self.flagd_ien4())
                .field("flagd_ien5", &self.flagd_ien5())
                .field("flagd_ien6", &self.flagd_ien6())
                .field("flagd_ien7", &self.flagd_ien7())
                .field("flagd_ien8", &self.flagd_ien8())
                .field("flagd_ien9", &self.flagd_ien9())
                .field("flagd_ien10", &self.flagd_ien10())
                .field("flagd_ien11", &self.flagd_ien11())
                .field("flagd_ien12", &self.flagd_ien12())
                .field("flagd_ien13", &self.flagd_ien13())
                .field("flagd_ien14", &self.flagd_ien14())
                .field("flagd_ien15", &self.flagd_ien15())
                .field("flagr_ien0", &self.flagr_ien0())
                .field("flagr_ien1", &self.flagr_ien1())
                .field("flagr_ien2", &self.flagr_ien2())
                .field("flagr_ien3", &self.flagr_ien3())
                .field("flagr_ien4", &self.flagr_ien4())
                .field("flagr_ien5", &self.flagr_ien5())
                .field("flagr_ien6", &self.flagr_ien6())
                .field("flagr_ien7", &self.flagr_ien7())
                .field("flagr_ien8", &self.flagr_ien8())
                .field("flagr_ien9", &self.flagr_ien9())
                .field("flagr_ien10", &self.flagr_ien10())
                .field("flagr_ien11", &self.flagr_ien11())
                .field("flagr_ien12", &self.flagr_ien12())
                .field("flagr_ien13", &self.flagr_ien13())
                .field("flagr_ien14", &self.flagr_ien14())
                .field("flagr_ien15", &self.flagr_ien15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayFlier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayFlier {{ flagd_ien0: {=bool:?}, flagd_ien1: {=bool:?}, flagd_ien2: {=bool:?}, flagd_ien3: {=bool:?}, flagd_ien4: {=bool:?}, flagd_ien5: {=bool:?}, flagd_ien6: {=bool:?}, flagd_ien7: {=bool:?}, flagd_ien8: {=bool:?}, flagd_ien9: {=bool:?}, flagd_ien10: {=bool:?}, flagd_ien11: {=bool:?}, flagd_ien12: {=bool:?}, flagd_ien13: {=bool:?}, flagd_ien14: {=bool:?}, flagd_ien15: {=bool:?}, flagr_ien0: {=bool:?}, flagr_ien1: {=bool:?}, flagr_ien2: {=bool:?}, flagr_ien3: {=bool:?}, flagr_ien4: {=bool:?}, flagr_ien5: {=bool:?}, flagr_ien6: {=bool:?}, flagr_ien7: {=bool:?}, flagr_ien8: {=bool:?}, flagr_ien9: {=bool:?}, flagr_ien10: {=bool:?}, flagr_ien11: {=bool:?}, flagr_ien12: {=bool:?}, flagr_ien13: {=bool:?}, flagr_ien14: {=bool:?}, flagr_ien15: {=bool:?} }}",
                self.flagd_ien0(),
                self.flagd_ien1(),
                self.flagd_ien2(),
                self.flagd_ien3(),
                self.flagd_ien4(),
                self.flagd_ien5(),
                self.flagd_ien6(),
                self.flagd_ien7(),
                self.flagd_ien8(),
                self.flagd_ien9(),
                self.flagd_ien10(),
                self.flagd_ien11(),
                self.flagd_ien12(),
                self.flagd_ien13(),
                self.flagd_ien14(),
                self.flagd_ien15(),
                self.flagr_ien0(),
                self.flagr_ien1(),
                self.flagr_ien2(),
                self.flagr_ien3(),
                self.flagr_ien4(),
                self.flagr_ien5(),
                self.flagr_ien6(),
                self.flagr_ien7(),
                self.flagr_ien8(),
                self.flagr_ien9(),
                self.flagr_ien10(),
                self.flagr_ien11(),
                self.flagr_ien12(),
                self.flagr_ien13(),
                self.flagr_ien14(),
                self.flagr_ien15()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayFlset(pub u32);
    impl PlayFlset {
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd_set15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr_set15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PlayFlset {
        #[inline(always)]
        fn default() -> PlayFlset {
            PlayFlset(0)
        }
    }
    impl core::fmt::Debug for PlayFlset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayFlset")
                .field("flagd_set0", &self.flagd_set0())
                .field("flagd_set1", &self.flagd_set1())
                .field("flagd_set2", &self.flagd_set2())
                .field("flagd_set3", &self.flagd_set3())
                .field("flagd_set4", &self.flagd_set4())
                .field("flagd_set5", &self.flagd_set5())
                .field("flagd_set6", &self.flagd_set6())
                .field("flagd_set7", &self.flagd_set7())
                .field("flagd_set8", &self.flagd_set8())
                .field("flagd_set9", &self.flagd_set9())
                .field("flagd_set10", &self.flagd_set10())
                .field("flagd_set11", &self.flagd_set11())
                .field("flagd_set12", &self.flagd_set12())
                .field("flagd_set13", &self.flagd_set13())
                .field("flagd_set14", &self.flagd_set14())
                .field("flagd_set15", &self.flagd_set15())
                .field("flagr_set0", &self.flagr_set0())
                .field("flagr_set1", &self.flagr_set1())
                .field("flagr_set2", &self.flagr_set2())
                .field("flagr_set3", &self.flagr_set3())
                .field("flagr_set4", &self.flagr_set4())
                .field("flagr_set5", &self.flagr_set5())
                .field("flagr_set6", &self.flagr_set6())
                .field("flagr_set7", &self.flagr_set7())
                .field("flagr_set8", &self.flagr_set8())
                .field("flagr_set9", &self.flagr_set9())
                .field("flagr_set10", &self.flagr_set10())
                .field("flagr_set11", &self.flagr_set11())
                .field("flagr_set12", &self.flagr_set12())
                .field("flagr_set13", &self.flagr_set13())
                .field("flagr_set14", &self.flagr_set14())
                .field("flagr_set15", &self.flagr_set15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayFlset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayFlset {{ flagd_set0: {=bool:?}, flagd_set1: {=bool:?}, flagd_set2: {=bool:?}, flagd_set3: {=bool:?}, flagd_set4: {=bool:?}, flagd_set5: {=bool:?}, flagd_set6: {=bool:?}, flagd_set7: {=bool:?}, flagd_set8: {=bool:?}, flagd_set9: {=bool:?}, flagd_set10: {=bool:?}, flagd_set11: {=bool:?}, flagd_set12: {=bool:?}, flagd_set13: {=bool:?}, flagd_set14: {=bool:?}, flagd_set15: {=bool:?}, flagr_set0: {=bool:?}, flagr_set1: {=bool:?}, flagr_set2: {=bool:?}, flagr_set3: {=bool:?}, flagr_set4: {=bool:?}, flagr_set5: {=bool:?}, flagr_set6: {=bool:?}, flagr_set7: {=bool:?}, flagr_set8: {=bool:?}, flagr_set9: {=bool:?}, flagr_set10: {=bool:?}, flagr_set11: {=bool:?}, flagr_set12: {=bool:?}, flagr_set13: {=bool:?}, flagr_set14: {=bool:?}, flagr_set15: {=bool:?} }}",
                self.flagd_set0(),
                self.flagd_set1(),
                self.flagd_set2(),
                self.flagd_set3(),
                self.flagd_set4(),
                self.flagd_set5(),
                self.flagd_set6(),
                self.flagd_set7(),
                self.flagd_set8(),
                self.flagd_set9(),
                self.flagd_set10(),
                self.flagd_set11(),
                self.flagd_set12(),
                self.flagd_set13(),
                self.flagd_set14(),
                self.flagd_set15(),
                self.flagr_set0(),
                self.flagr_set1(),
                self.flagr_set2(),
                self.flagr_set3(),
                self.flagr_set4(),
                self.flagr_set5(),
                self.flagr_set6(),
                self.flagr_set7(),
                self.flagr_set8(),
                self.flagr_set9(),
                self.flagr_set10(),
                self.flagr_set11(),
                self.flagr_set12(),
                self.flagr_set13(),
                self.flagr_set14(),
                self.flagr_set15()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayFlstat(pub u32);
    impl PlayFlstat {
        #[must_use]
        #[inline(always)]
        pub const fn flagd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flagr15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flagr15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PlayFlstat {
        #[inline(always)]
        fn default() -> PlayFlstat {
            PlayFlstat(0)
        }
    }
    impl core::fmt::Debug for PlayFlstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayFlstat")
                .field("flagd0", &self.flagd0())
                .field("flagd1", &self.flagd1())
                .field("flagd2", &self.flagd2())
                .field("flagd3", &self.flagd3())
                .field("flagd4", &self.flagd4())
                .field("flagd5", &self.flagd5())
                .field("flagd6", &self.flagd6())
                .field("flagd7", &self.flagd7())
                .field("flagd8", &self.flagd8())
                .field("flagd9", &self.flagd9())
                .field("flagd10", &self.flagd10())
                .field("flagd11", &self.flagd11())
                .field("flagd12", &self.flagd12())
                .field("flagd13", &self.flagd13())
                .field("flagd14", &self.flagd14())
                .field("flagd15", &self.flagd15())
                .field("flagr0", &self.flagr0())
                .field("flagr1", &self.flagr1())
                .field("flagr2", &self.flagr2())
                .field("flagr3", &self.flagr3())
                .field("flagr4", &self.flagr4())
                .field("flagr5", &self.flagr5())
                .field("flagr6", &self.flagr6())
                .field("flagr7", &self.flagr7())
                .field("flagr8", &self.flagr8())
                .field("flagr9", &self.flagr9())
                .field("flagr10", &self.flagr10())
                .field("flagr11", &self.flagr11())
                .field("flagr12", &self.flagr12())
                .field("flagr13", &self.flagr13())
                .field("flagr14", &self.flagr14())
                .field("flagr15", &self.flagr15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayFlstat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayFlstat {{ flagd0: {=bool:?}, flagd1: {=bool:?}, flagd2: {=bool:?}, flagd3: {=bool:?}, flagd4: {=bool:?}, flagd5: {=bool:?}, flagd6: {=bool:?}, flagd7: {=bool:?}, flagd8: {=bool:?}, flagd9: {=bool:?}, flagd10: {=bool:?}, flagd11: {=bool:?}, flagd12: {=bool:?}, flagd13: {=bool:?}, flagd14: {=bool:?}, flagd15: {=bool:?}, flagr0: {=bool:?}, flagr1: {=bool:?}, flagr2: {=bool:?}, flagr3: {=bool:?}, flagr4: {=bool:?}, flagr5: {=bool:?}, flagr6: {=bool:?}, flagr7: {=bool:?}, flagr8: {=bool:?}, flagr9: {=bool:?}, flagr10: {=bool:?}, flagr11: {=bool:?}, flagr12: {=bool:?}, flagr13: {=bool:?}, flagr14: {=bool:?}, flagr15: {=bool:?} }}",
                self.flagd0(),
                self.flagd1(),
                self.flagd2(),
                self.flagd3(),
                self.flagd4(),
                self.flagd5(),
                self.flagd6(),
                self.flagd7(),
                self.flagd8(),
                self.flagd9(),
                self.flagd10(),
                self.flagd11(),
                self.flagd12(),
                self.flagd13(),
                self.flagd14(),
                self.flagd15(),
                self.flagr0(),
                self.flagr1(),
                self.flagr2(),
                self.flagr3(),
                self.flagr4(),
                self.flagr5(),
                self.flagr6(),
                self.flagr7(),
                self.flagr8(),
                self.flagr9(),
                self.flagr10(),
                self.flagr11(),
                self.flagr12(),
                self.flagr13(),
                self.flagr14(),
                self.flagr15()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayIcr(pub u32);
    impl PlayIcr {
        #[must_use]
        #[inline(always)]
        pub const fn swinwc_clr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swinwc_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flctlwc_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flctlwc_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PlayIcr {
        #[inline(always)]
        fn default() -> PlayIcr {
            PlayIcr(0)
        }
    }
    impl core::fmt::Debug for PlayIcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayIcr")
                .field("swinwc_clr", &self.swinwc_clr())
                .field("flctlwc_clr", &self.flctlwc_clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayIcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayIcr {{ swinwc_clr: {=bool:?}, flctlwc_clr: {=bool:?} }}",
                self.swinwc_clr(),
                self.flctlwc_clr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayIer(pub u32);
    impl PlayIer {
        #[must_use]
        #[inline(always)]
        pub const fn swinwc_ien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swinwc_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flctlwc_ien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flctlwc_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PlayIer {
        #[inline(always)]
        fn default() -> PlayIer {
            PlayIer(0)
        }
    }
    impl core::fmt::Debug for PlayIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayIer")
                .field("swinwc_ien", &self.swinwc_ien())
                .field("flctlwc_ien", &self.flctlwc_ien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayIer {{ swinwc_ien: {=bool:?}, flctlwc_ien: {=bool:?} }}",
                self.swinwc_ien(),
                self.flctlwc_ien()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayIsr(pub u32);
    impl PlayIsr {
        #[must_use]
        #[inline(always)]
        pub const fn swinwc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swinwc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flctlwc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flctlwc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flags(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flags(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for PlayIsr {
        #[inline(always)]
        fn default() -> PlayIsr {
            PlayIsr(0)
        }
    }
    impl core::fmt::Debug for PlayIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayIsr")
                .field("swinwc", &self.swinwc())
                .field("flctlwc", &self.flctlwc())
                .field("flags", &self.flags())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayIsr {{ swinwc: {=bool:?}, flctlwc: {=bool:?}, flags: {=bool:?} }}",
                self.swinwc(),
                self.flctlwc(),
                self.flags()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayMsr(pub u32);
    impl PlayMsr {
        #[must_use]
        #[inline(always)]
        pub const fn swinwbfs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swinwbfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flctlwbfs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_flctlwbfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PlayMsr {
        #[inline(always)]
        fn default() -> PlayMsr {
            PlayMsr(0)
        }
    }
    impl core::fmt::Debug for PlayMsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayMsr")
                .field("swinwbfs", &self.swinwbfs())
                .field("flctlwbfs", &self.flctlwbfs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayMsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayMsr {{ swinwbfs: {=bool:?}, flctlwbfs: {=bool:?} }}",
                self.swinwbfs(),
                self.flctlwbfs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayOsr(pub u32);
    impl PlayOsr {
        #[must_use]
        #[inline(always)]
        pub const fn leoutd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn leoutr15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_leoutr15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PlayOsr {
        #[inline(always)]
        fn default() -> PlayOsr {
            PlayOsr(0)
        }
    }
    impl core::fmt::Debug for PlayOsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayOsr")
                .field("leoutd0", &self.leoutd0())
                .field("leoutd1", &self.leoutd1())
                .field("leoutd2", &self.leoutd2())
                .field("leoutd3", &self.leoutd3())
                .field("leoutd4", &self.leoutd4())
                .field("leoutd5", &self.leoutd5())
                .field("leoutd6", &self.leoutd6())
                .field("leoutd7", &self.leoutd7())
                .field("leoutd8", &self.leoutd8())
                .field("leoutd9", &self.leoutd9())
                .field("leoutd10", &self.leoutd10())
                .field("leoutd11", &self.leoutd11())
                .field("leoutd12", &self.leoutd12())
                .field("leoutd13", &self.leoutd13())
                .field("leoutd14", &self.leoutd14())
                .field("leoutd15", &self.leoutd15())
                .field("leoutr0", &self.leoutr0())
                .field("leoutr1", &self.leoutr1())
                .field("leoutr2", &self.leoutr2())
                .field("leoutr3", &self.leoutr3())
                .field("leoutr4", &self.leoutr4())
                .field("leoutr5", &self.leoutr5())
                .field("leoutr6", &self.leoutr6())
                .field("leoutr7", &self.leoutr7())
                .field("leoutr8", &self.leoutr8())
                .field("leoutr9", &self.leoutr9())
                .field("leoutr10", &self.leoutr10())
                .field("leoutr11", &self.leoutr11())
                .field("leoutr12", &self.leoutr12())
                .field("leoutr13", &self.leoutr13())
                .field("leoutr14", &self.leoutr14())
                .field("leoutr15", &self.leoutr15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayOsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlayOsr {{ leoutd0: {=bool:?}, leoutd1: {=bool:?}, leoutd2: {=bool:?}, leoutd3: {=bool:?}, leoutd4: {=bool:?}, leoutd5: {=bool:?}, leoutd6: {=bool:?}, leoutd7: {=bool:?}, leoutd8: {=bool:?}, leoutd9: {=bool:?}, leoutd10: {=bool:?}, leoutd11: {=bool:?}, leoutd12: {=bool:?}, leoutd13: {=bool:?}, leoutd14: {=bool:?}, leoutd15: {=bool:?}, leoutr0: {=bool:?}, leoutr1: {=bool:?}, leoutr2: {=bool:?}, leoutr3: {=bool:?}, leoutr4: {=bool:?}, leoutr5: {=bool:?}, leoutr6: {=bool:?}, leoutr7: {=bool:?}, leoutr8: {=bool:?}, leoutr9: {=bool:?}, leoutr10: {=bool:?}, leoutr11: {=bool:?}, leoutr12: {=bool:?}, leoutr13: {=bool:?}, leoutr14: {=bool:?}, leoutr15: {=bool:?} }}",
                self.leoutd0(),
                self.leoutd1(),
                self.leoutd2(),
                self.leoutd3(),
                self.leoutd4(),
                self.leoutd5(),
                self.leoutd6(),
                self.leoutd7(),
                self.leoutd8(),
                self.leoutd9(),
                self.leoutd10(),
                self.leoutd11(),
                self.leoutd12(),
                self.leoutd13(),
                self.leoutd14(),
                self.leoutd15(),
                self.leoutr0(),
                self.leoutr1(),
                self.leoutr2(),
                self.leoutr3(),
                self.leoutr4(),
                self.leoutr5(),
                self.leoutr6(),
                self.leoutr7(),
                self.leoutr8(),
                self.leoutr9(),
                self.leoutr10(),
                self.leoutr11(),
                self.leoutr12(),
                self.leoutr13(),
                self.leoutr14(),
                self.leoutr15()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlayPrivcfgr(pub u32);
    impl PlayPrivcfgr {
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_priv_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for PlayPrivcfgr {
        #[inline(always)]
        fn default() -> PlayPrivcfgr {
            PlayPrivcfgr(0)
        }
    }
    impl core::fmt::Debug for PlayPrivcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlayPrivcfgr")
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlayPrivcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PlayPrivcfgr {{ priv_: {=u8:?} }}", self.priv_())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlaySeccfgr(pub u32);
    impl PlaySeccfgr {
        #[must_use]
        #[inline(always)]
        pub const fn sec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for PlaySeccfgr {
        #[inline(always)]
        fn default() -> PlaySeccfgr {
            PlaySeccfgr(0)
        }
    }
    impl core::fmt::Debug for PlaySeccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlaySeccfgr")
                .field("sec", &self.sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlaySeccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PlaySeccfgr {{ sec: {=u8:?} }}", self.sec())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlaySwin(pub u32);
    impl PlaySwin {
        #[must_use]
        #[inline(always)]
        pub const fn swin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for PlaySwin {
        #[inline(always)]
        fn default() -> PlaySwin {
            PlaySwin(0)
        }
    }
    impl core::fmt::Debug for PlaySwin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlaySwin")
                .field("swin0", &self.swin0())
                .field("swin1", &self.swin1())
                .field("swin2", &self.swin2())
                .field("swin3", &self.swin3())
                .field("swin4", &self.swin4())
                .field("swin5", &self.swin5())
                .field("swin6", &self.swin6())
                .field("swin7", &self.swin7())
                .field("swin8", &self.swin8())
                .field("swin9", &self.swin9())
                .field("swin10", &self.swin10())
                .field("swin11", &self.swin11())
                .field("swin12", &self.swin12())
                .field("swin13", &self.swin13())
                .field("swin14", &self.swin14())
                .field("swin15", &self.swin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlaySwin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlaySwin {{ swin0: {=bool:?}, swin1: {=bool:?}, swin2: {=bool:?}, swin3: {=bool:?}, swin4: {=bool:?}, swin5: {=bool:?}, swin6: {=bool:?}, swin7: {=bool:?}, swin8: {=bool:?}, swin9: {=bool:?}, swin10: {=bool:?}, swin11: {=bool:?}, swin12: {=bool:?}, swin13: {=bool:?}, swin14: {=bool:?}, swin15: {=bool:?} }}",
                self.swin0(),
                self.swin1(),
                self.swin2(),
                self.swin3(),
                self.swin4(),
                self.swin5(),
                self.swin6(),
                self.swin7(),
                self.swin8(),
                self.swin9(),
                self.swin10(),
                self.swin11(),
                self.swin12(),
                self.swin13(),
                self.swin14(),
                self.swin15()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlaySwinclr(pub u32);
    impl PlaySwinclr {
        #[must_use]
        #[inline(always)]
        pub const fn swin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for PlaySwinclr {
        #[inline(always)]
        fn default() -> PlaySwinclr {
            PlaySwinclr(0)
        }
    }
    impl core::fmt::Debug for PlaySwinclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlaySwinclr")
                .field("swin0", &self.swin0())
                .field("swin1", &self.swin1())
                .field("swin2", &self.swin2())
                .field("swin3", &self.swin3())
                .field("swin4", &self.swin4())
                .field("swin5", &self.swin5())
                .field("swin6", &self.swin6())
                .field("swin7", &self.swin7())
                .field("swin8", &self.swin8())
                .field("swin9", &self.swin9())
                .field("swin10", &self.swin10())
                .field("swin11", &self.swin11())
                .field("swin12", &self.swin12())
                .field("swin13", &self.swin13())
                .field("swin14", &self.swin14())
                .field("swin15", &self.swin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlaySwinclr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlaySwinclr {{ swin0: {=bool:?}, swin1: {=bool:?}, swin2: {=bool:?}, swin3: {=bool:?}, swin4: {=bool:?}, swin5: {=bool:?}, swin6: {=bool:?}, swin7: {=bool:?}, swin8: {=bool:?}, swin9: {=bool:?}, swin10: {=bool:?}, swin11: {=bool:?}, swin12: {=bool:?}, swin13: {=bool:?}, swin14: {=bool:?}, swin15: {=bool:?} }}",
                self.swin0(),
                self.swin1(),
                self.swin2(),
                self.swin3(),
                self.swin4(),
                self.swin5(),
                self.swin6(),
                self.swin7(),
                self.swin8(),
                self.swin9(),
                self.swin10(),
                self.swin11(),
                self.swin12(),
                self.swin13(),
                self.swin14(),
                self.swin15()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlaySwinset(pub u32);
    impl PlaySwinset {
        #[must_use]
        #[inline(always)]
        pub const fn swin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn swin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for PlaySwinset {
        #[inline(always)]
        fn default() -> PlaySwinset {
            PlaySwinset(0)
        }
    }
    impl core::fmt::Debug for PlaySwinset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PlaySwinset")
                .field("swin0", &self.swin0())
                .field("swin1", &self.swin1())
                .field("swin2", &self.swin2())
                .field("swin3", &self.swin3())
                .field("swin4", &self.swin4())
                .field("swin5", &self.swin5())
                .field("swin6", &self.swin6())
                .field("swin7", &self.swin7())
                .field("swin8", &self.swin8())
                .field("swin9", &self.swin9())
                .field("swin10", &self.swin10())
                .field("swin11", &self.swin11())
                .field("swin12", &self.swin12())
                .field("swin13", &self.swin13())
                .field("swin14", &self.swin14())
                .field("swin15", &self.swin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PlaySwinset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PlaySwinset {{ swin0: {=bool:?}, swin1: {=bool:?}, swin2: {=bool:?}, swin3: {=bool:?}, swin4: {=bool:?}, swin5: {=bool:?}, swin6: {=bool:?}, swin7: {=bool:?}, swin8: {=bool:?}, swin9: {=bool:?}, swin10: {=bool:?}, swin11: {=bool:?}, swin12: {=bool:?}, swin13: {=bool:?}, swin14: {=bool:?}, swin15: {=bool:?} }}",
                self.swin0(),
                self.swin1(),
                self.swin2(),
                self.swin3(),
                self.swin4(),
                self.swin5(),
                self.swin6(),
                self.swin7(),
                self.swin8(),
                self.swin9(),
                self.swin10(),
                self.swin11(),
                self.swin12(),
                self.swin13(),
                self.swin14(),
                self.swin15()
            )
        }
    }
}
