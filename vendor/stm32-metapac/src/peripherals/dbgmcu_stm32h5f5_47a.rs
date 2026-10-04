#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgmcu {
    ptr: *mut u8,
}
unsafe impl Send for Dbgmcu {}
unsafe impl Sync for Dbgmcu {}
impl Dbgmcu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::DbgmcuIdcode, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::DbgmcuCr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn apb1fzr1(self) -> crate::common::Reg<regs::DbgmcuApb1fzr1, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn apb1fzr2(self) -> crate::common::Reg<regs::DbgmcuApb1fzr2, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn apb2fzr(self) -> crate::common::Reg<regs::DbgmcuApb2fzr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn apb3fzr(self) -> crate::common::Reg<regs::DbgmcuApb3fzr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn ahb1fzr(self) -> crate::common::Reg<regs::DbgmcuAhb1fzr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::DbgmcuSr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn dbg_auth_host(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn dbg_auth_dev(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn dbg_auth_ack(
        self,
    ) -> crate::common::Reg<regs::DbgmcuDbgAuthAck, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn pidr4(self) -> crate::common::Reg<regs::DbgmcuPidr4, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[inline(always)]
    pub const fn pidr5(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[inline(always)]
    pub const fn pidr6(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd8usize) as _) }
    }
    #[inline(always)]
    pub const fn pidr7(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fdcusize) as _) }
    }
    #[inline(always)]
    pub const fn pidr0(self) -> crate::common::Reg<regs::DbgmcuPidr0, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[inline(always)]
    pub const fn pidr1(self) -> crate::common::Reg<regs::DbgmcuPidr1, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[inline(always)]
    pub const fn pidr2(self) -> crate::common::Reg<regs::DbgmcuPidr2, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[inline(always)]
    pub const fn pidr3(self) -> crate::common::Reg<regs::DbgmcuPidr3, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[inline(always)]
    pub const fn cidr0(self) -> crate::common::Reg<regs::DbgmcuCidr0, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[inline(always)]
    pub const fn cidr1(self) -> crate::common::Reg<regs::DbgmcuCidr1, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[inline(always)]
    pub const fn cidr2(self) -> crate::common::Reg<regs::DbgmcuCidr2, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[inline(always)]
    pub const fn cidr3(self) -> crate::common::Reg<regs::DbgmcuCidr3, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuAhb1fzr(pub u32);
    impl DbgmcuAhb1fzr {
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch0_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch1_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch2_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch3_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch4_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch5_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch6_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch7_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch8_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch9_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch10_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch11_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma1_ch11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch0_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch1_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch2_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch3_stop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch4_stop(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch5_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch6_stop(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch7_stop(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch8_stop(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch9_stop(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch10_stop(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_gpdma2_ch11_stop(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_gpdma2_ch11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for DbgmcuAhb1fzr {
        #[inline(always)]
        fn default() -> DbgmcuAhb1fzr {
            DbgmcuAhb1fzr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuAhb1fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuAhb1fzr")
                .field("dbg_gpdma1_ch0_stop", &self.dbg_gpdma1_ch0_stop())
                .field("dbg_gpdma1_ch1_stop", &self.dbg_gpdma1_ch1_stop())
                .field("dbg_gpdma1_ch2_stop", &self.dbg_gpdma1_ch2_stop())
                .field("dbg_gpdma1_ch3_stop", &self.dbg_gpdma1_ch3_stop())
                .field("dbg_gpdma1_ch4_stop", &self.dbg_gpdma1_ch4_stop())
                .field("dbg_gpdma1_ch5_stop", &self.dbg_gpdma1_ch5_stop())
                .field("dbg_gpdma1_ch6_stop", &self.dbg_gpdma1_ch6_stop())
                .field("dbg_gpdma1_ch7_stop", &self.dbg_gpdma1_ch7_stop())
                .field("dbg_gpdma1_ch8_stop", &self.dbg_gpdma1_ch8_stop())
                .field("dbg_gpdma1_ch9_stop", &self.dbg_gpdma1_ch9_stop())
                .field("dbg_gpdma1_ch10_stop", &self.dbg_gpdma1_ch10_stop())
                .field("dbg_gpdma1_ch11_stop", &self.dbg_gpdma1_ch11_stop())
                .field("dbg_gpdma2_ch0_stop", &self.dbg_gpdma2_ch0_stop())
                .field("dbg_gpdma2_ch1_stop", &self.dbg_gpdma2_ch1_stop())
                .field("dbg_gpdma2_ch2_stop", &self.dbg_gpdma2_ch2_stop())
                .field("dbg_gpdma2_ch3_stop", &self.dbg_gpdma2_ch3_stop())
                .field("dbg_gpdma2_ch4_stop", &self.dbg_gpdma2_ch4_stop())
                .field("dbg_gpdma2_ch5_stop", &self.dbg_gpdma2_ch5_stop())
                .field("dbg_gpdma2_ch6_stop", &self.dbg_gpdma2_ch6_stop())
                .field("dbg_gpdma2_ch7_stop", &self.dbg_gpdma2_ch7_stop())
                .field("dbg_gpdma2_ch8_stop", &self.dbg_gpdma2_ch8_stop())
                .field("dbg_gpdma2_ch9_stop", &self.dbg_gpdma2_ch9_stop())
                .field("dbg_gpdma2_ch10_stop", &self.dbg_gpdma2_ch10_stop())
                .field("dbg_gpdma2_ch11_stop", &self.dbg_gpdma2_ch11_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuAhb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuAhb1fzr {{ dbg_gpdma1_ch0_stop: {=bool:?}, dbg_gpdma1_ch1_stop: {=bool:?}, dbg_gpdma1_ch2_stop: {=bool:?}, dbg_gpdma1_ch3_stop: {=bool:?}, dbg_gpdma1_ch4_stop: {=bool:?}, dbg_gpdma1_ch5_stop: {=bool:?}, dbg_gpdma1_ch6_stop: {=bool:?}, dbg_gpdma1_ch7_stop: {=bool:?}, dbg_gpdma1_ch8_stop: {=bool:?}, dbg_gpdma1_ch9_stop: {=bool:?}, dbg_gpdma1_ch10_stop: {=bool:?}, dbg_gpdma1_ch11_stop: {=bool:?}, dbg_gpdma2_ch0_stop: {=bool:?}, dbg_gpdma2_ch1_stop: {=bool:?}, dbg_gpdma2_ch2_stop: {=bool:?}, dbg_gpdma2_ch3_stop: {=bool:?}, dbg_gpdma2_ch4_stop: {=bool:?}, dbg_gpdma2_ch5_stop: {=bool:?}, dbg_gpdma2_ch6_stop: {=bool:?}, dbg_gpdma2_ch7_stop: {=bool:?}, dbg_gpdma2_ch8_stop: {=bool:?}, dbg_gpdma2_ch9_stop: {=bool:?}, dbg_gpdma2_ch10_stop: {=bool:?}, dbg_gpdma2_ch11_stop: {=bool:?} }}",
                self.dbg_gpdma1_ch0_stop(),
                self.dbg_gpdma1_ch1_stop(),
                self.dbg_gpdma1_ch2_stop(),
                self.dbg_gpdma1_ch3_stop(),
                self.dbg_gpdma1_ch4_stop(),
                self.dbg_gpdma1_ch5_stop(),
                self.dbg_gpdma1_ch6_stop(),
                self.dbg_gpdma1_ch7_stop(),
                self.dbg_gpdma1_ch8_stop(),
                self.dbg_gpdma1_ch9_stop(),
                self.dbg_gpdma1_ch10_stop(),
                self.dbg_gpdma1_ch11_stop(),
                self.dbg_gpdma2_ch0_stop(),
                self.dbg_gpdma2_ch1_stop(),
                self.dbg_gpdma2_ch2_stop(),
                self.dbg_gpdma2_ch3_stop(),
                self.dbg_gpdma2_ch4_stop(),
                self.dbg_gpdma2_ch5_stop(),
                self.dbg_gpdma2_ch6_stop(),
                self.dbg_gpdma2_ch7_stop(),
                self.dbg_gpdma2_ch8_stop(),
                self.dbg_gpdma2_ch9_stop(),
                self.dbg_gpdma2_ch10_stop(),
                self.dbg_gpdma2_ch11_stop()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuApb1fzr1(pub u32);
    impl DbgmcuApb1fzr1 {
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim2_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim3_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim4_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim5_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim6_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim7_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim12_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim12_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim13_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim13_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim14_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim14_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_wwdg_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_i2c1_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_i2c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_i2c2_stop(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_i2c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_i3c1_stop(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_i3c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for DbgmcuApb1fzr1 {
        #[inline(always)]
        fn default() -> DbgmcuApb1fzr1 {
            DbgmcuApb1fzr1(0)
        }
    }
    impl core::fmt::Debug for DbgmcuApb1fzr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuApb1fzr1")
                .field("dbg_tim2_stop", &self.dbg_tim2_stop())
                .field("dbg_tim3_stop", &self.dbg_tim3_stop())
                .field("dbg_tim4_stop", &self.dbg_tim4_stop())
                .field("dbg_tim5_stop", &self.dbg_tim5_stop())
                .field("dbg_tim6_stop", &self.dbg_tim6_stop())
                .field("dbg_tim7_stop", &self.dbg_tim7_stop())
                .field("dbg_tim12_stop", &self.dbg_tim12_stop())
                .field("dbg_tim13_stop", &self.dbg_tim13_stop())
                .field("dbg_tim14_stop", &self.dbg_tim14_stop())
                .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
                .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
                .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
                .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
                .field("dbg_i3c1_stop", &self.dbg_i3c1_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuApb1fzr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuApb1fzr1 {{ dbg_tim2_stop: {=bool:?}, dbg_tim3_stop: {=bool:?}, dbg_tim4_stop: {=bool:?}, dbg_tim5_stop: {=bool:?}, dbg_tim6_stop: {=bool:?}, dbg_tim7_stop: {=bool:?}, dbg_tim12_stop: {=bool:?}, dbg_tim13_stop: {=bool:?}, dbg_tim14_stop: {=bool:?}, dbg_wwdg_stop: {=bool:?}, dbg_iwdg_stop: {=bool:?}, dbg_i2c1_stop: {=bool:?}, dbg_i2c2_stop: {=bool:?}, dbg_i3c1_stop: {=bool:?} }}",
                self.dbg_tim2_stop(),
                self.dbg_tim3_stop(),
                self.dbg_tim4_stop(),
                self.dbg_tim5_stop(),
                self.dbg_tim6_stop(),
                self.dbg_tim7_stop(),
                self.dbg_tim12_stop(),
                self.dbg_tim13_stop(),
                self.dbg_tim14_stop(),
                self.dbg_wwdg_stop(),
                self.dbg_iwdg_stop(),
                self.dbg_i2c1_stop(),
                self.dbg_i2c2_stop(),
                self.dbg_i3c1_stop()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuApb1fzr2(pub u32);
    impl DbgmcuApb1fzr2 {
        #[must_use]
        #[inline(always)]
        pub const fn dbg_lptim2_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_lptim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for DbgmcuApb1fzr2 {
        #[inline(always)]
        fn default() -> DbgmcuApb1fzr2 {
            DbgmcuApb1fzr2(0)
        }
    }
    impl core::fmt::Debug for DbgmcuApb1fzr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuApb1fzr2")
                .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuApb1fzr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuApb1fzr2 {{ dbg_lptim2_stop: {=bool:?} }}",
                self.dbg_lptim2_stop()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuApb2fzr(pub u32);
    impl DbgmcuApb2fzr {
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim1_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim8_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim15_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim16_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim16_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_tim17_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_tim17_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for DbgmcuApb2fzr {
        #[inline(always)]
        fn default() -> DbgmcuApb2fzr {
            DbgmcuApb2fzr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuApb2fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuApb2fzr")
                .field("dbg_tim1_stop", &self.dbg_tim1_stop())
                .field("dbg_tim8_stop", &self.dbg_tim8_stop())
                .field("dbg_tim15_stop", &self.dbg_tim15_stop())
                .field("dbg_tim16_stop", &self.dbg_tim16_stop())
                .field("dbg_tim17_stop", &self.dbg_tim17_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuApb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuApb2fzr {{ dbg_tim1_stop: {=bool:?}, dbg_tim8_stop: {=bool:?}, dbg_tim15_stop: {=bool:?}, dbg_tim16_stop: {=bool:?}, dbg_tim17_stop: {=bool:?} }}",
                self.dbg_tim1_stop(),
                self.dbg_tim8_stop(),
                self.dbg_tim15_stop(),
                self.dbg_tim16_stop(),
                self.dbg_tim17_stop()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuApb3fzr(pub u32);
    impl DbgmcuApb3fzr {
        #[must_use]
        #[inline(always)]
        pub const fn dbg_i2c3_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_i2c3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_i2c4_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_i2c4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_i3c2_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_i3c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_lptim1_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_lptim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_lptim3_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_lptim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_lptim4_stop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_lptim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_lptim5_stop(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_lptim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_lptim6_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_lptim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_rtc_stop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_rtc_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for DbgmcuApb3fzr {
        #[inline(always)]
        fn default() -> DbgmcuApb3fzr {
            DbgmcuApb3fzr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuApb3fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuApb3fzr")
                .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
                .field("dbg_i2c4_stop", &self.dbg_i2c4_stop())
                .field("dbg_i3c2_stop", &self.dbg_i3c2_stop())
                .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
                .field("dbg_lptim3_stop", &self.dbg_lptim3_stop())
                .field("dbg_lptim4_stop", &self.dbg_lptim4_stop())
                .field("dbg_lptim5_stop", &self.dbg_lptim5_stop())
                .field("dbg_lptim6_stop", &self.dbg_lptim6_stop())
                .field("dbg_rtc_stop", &self.dbg_rtc_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuApb3fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuApb3fzr {{ dbg_i2c3_stop: {=bool:?}, dbg_i2c4_stop: {=bool:?}, dbg_i3c2_stop: {=bool:?}, dbg_lptim1_stop: {=bool:?}, dbg_lptim3_stop: {=bool:?}, dbg_lptim4_stop: {=bool:?}, dbg_lptim5_stop: {=bool:?}, dbg_lptim6_stop: {=bool:?}, dbg_rtc_stop: {=bool:?} }}",
                self.dbg_i2c3_stop(),
                self.dbg_i2c4_stop(),
                self.dbg_i3c2_stop(),
                self.dbg_lptim1_stop(),
                self.dbg_lptim3_stop(),
                self.dbg_lptim4_stop(),
                self.dbg_lptim5_stop(),
                self.dbg_lptim6_stop(),
                self.dbg_rtc_stop()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCidr0(pub u32);
    impl DbgmcuCidr0 {
        #[must_use]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DbgmcuCidr0 {
        #[inline(always)]
        fn default() -> DbgmcuCidr0 {
            DbgmcuCidr0(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCidr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCidr0")
                .field("preamble", &self.preamble())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCidr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuCidr0 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCidr1(pub u32);
    impl DbgmcuCidr1 {
        #[must_use]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn class(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_class(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for DbgmcuCidr1 {
        #[inline(always)]
        fn default() -> DbgmcuCidr1 {
            DbgmcuCidr1(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCidr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCidr1")
                .field("preamble", &self.preamble())
                .field("class", &self.class())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCidr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuCidr1 {{ preamble: {=u8:?}, class: {=u8:?} }}",
                self.preamble(),
                self.class()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCidr2(pub u32);
    impl DbgmcuCidr2 {
        #[must_use]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DbgmcuCidr2 {
        #[inline(always)]
        fn default() -> DbgmcuCidr2 {
            DbgmcuCidr2(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCidr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCidr2")
                .field("preamble", &self.preamble())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCidr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuCidr2 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCidr3(pub u32);
    impl DbgmcuCidr3 {
        #[must_use]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DbgmcuCidr3 {
        #[inline(always)]
        fn default() -> DbgmcuCidr3 {
            DbgmcuCidr3(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCidr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCidr3")
                .field("preamble", &self.preamble())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCidr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuCidr3 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCr(pub u32);
    impl DbgmcuCr {
        #[must_use]
        #[inline(always)]
        pub const fn dbg_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trace_ioen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_trace_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trace_clken(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_trace_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trace_mode(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trace_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcrt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for DbgmcuCr {
        #[inline(always)]
        fn default() -> DbgmcuCr {
            DbgmcuCr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCr")
                .field("dbg_stop", &self.dbg_stop())
                .field("dbg_standby", &self.dbg_standby())
                .field("trace_ioen", &self.trace_ioen())
                .field("trace_clken", &self.trace_clken())
                .field("trace_mode", &self.trace_mode())
                .field("dcrt", &self.dcrt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuCr {{ dbg_stop: {=bool:?}, dbg_standby: {=bool:?}, trace_ioen: {=bool:?}, trace_clken: {=bool:?}, trace_mode: {=u8:?}, dcrt: {=bool:?} }}",
                self.dbg_stop(),
                self.dbg_standby(),
                self.trace_ioen(),
                self.trace_clken(),
                self.trace_mode(),
                self.dcrt()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgAuthAck(pub u32);
    impl DbgmcuDbgAuthAck {
        #[must_use]
        #[inline(always)]
        pub const fn host(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_host(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dev(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DbgmcuDbgAuthAck {
        #[inline(always)]
        fn default() -> DbgmcuDbgAuthAck {
            DbgmcuDbgAuthAck(0)
        }
    }
    impl core::fmt::Debug for DbgmcuDbgAuthAck {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuDbgAuthAck")
                .field("host", &self.host())
                .field("dev", &self.dev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuDbgAuthAck {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuDbgAuthAck {{ host: {=bool:?}, dev: {=bool:?} }}",
                self.host(),
                self.dev()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuIdcode(pub u32);
    impl DbgmcuIdcode {
        #[must_use]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DbgmcuIdcode {
        #[inline(always)]
        fn default() -> DbgmcuIdcode {
            DbgmcuIdcode(0)
        }
    }
    impl core::fmt::Debug for DbgmcuIdcode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuIdcode")
                .field("dev_id", &self.dev_id())
                .field("rev_id", &self.rev_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuIdcode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuIdcode {{ dev_id: {=u16:?}, rev_id: {=u16:?} }}",
                self.dev_id(),
                self.rev_id()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr0(pub u32);
    impl DbgmcuPidr0 {
        #[must_use]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DbgmcuPidr0 {
        #[inline(always)]
        fn default() -> DbgmcuPidr0 {
            DbgmcuPidr0(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr0")
                .field("partnum", &self.partnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuPidr0 {{ partnum: {=u8:?} }}", self.partnum())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr1(pub u32);
    impl DbgmcuPidr1 {
        #[must_use]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for DbgmcuPidr1 {
        #[inline(always)]
        fn default() -> DbgmcuPidr1 {
            DbgmcuPidr1(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr1")
                .field("partnum", &self.partnum())
                .field("jep106id", &self.jep106id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuPidr1 {{ partnum: {=u8:?}, jep106id: {=u8:?} }}",
                self.partnum(),
                self.jep106id()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr2(pub u32);
    impl DbgmcuPidr2 {
        #[must_use]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn jedec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_jedec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn revision(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_revision(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for DbgmcuPidr2 {
        #[inline(always)]
        fn default() -> DbgmcuPidr2 {
            DbgmcuPidr2(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr2")
                .field("jep106id", &self.jep106id())
                .field("jedec", &self.jedec())
                .field("revision", &self.revision())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuPidr2 {{ jep106id: {=u8:?}, jedec: {=bool:?}, revision: {=u8:?} }}",
                self.jep106id(),
                self.jedec(),
                self.revision()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr3(pub u32);
    impl DbgmcuPidr3 {
        #[must_use]
        #[inline(always)]
        pub const fn cmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn revand(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_revand(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for DbgmcuPidr3 {
        #[inline(always)]
        fn default() -> DbgmcuPidr3 {
            DbgmcuPidr3(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr3")
                .field("cmod", &self.cmod())
                .field("revand", &self.revand())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuPidr3 {{ cmod: {=u8:?}, revand: {=u8:?} }}",
                self.cmod(),
                self.revand()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr4(pub u32);
    impl DbgmcuPidr4 {
        #[must_use]
        #[inline(always)]
        pub const fn jep106con(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_jep106con(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn _4kcount(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set__4kcount(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for DbgmcuPidr4 {
        #[inline(always)]
        fn default() -> DbgmcuPidr4 {
            DbgmcuPidr4(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr4")
                .field("jep106con", &self.jep106con())
                .field("_4kcount", &self._4kcount())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuPidr4 {{ jep106con: {=u8:?}, _4kcount: {=u8:?} }}",
                self.jep106con(),
                self._4kcount()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuSr(pub u32);
    impl DbgmcuSr {
        #[must_use]
        #[inline(always)]
        pub const fn acc_port_pres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_acc_port_pres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn acc_port_enbl(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_acc_port_enbl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DbgmcuSr {
        #[inline(always)]
        fn default() -> DbgmcuSr {
            DbgmcuSr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuSr")
                .field("acc_port_pres", &self.acc_port_pres())
                .field("acc_port_enbl", &self.acc_port_enbl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuSr {{ acc_port_pres: {=u16:?}, acc_port_enbl: {=u16:?} }}",
                self.acc_port_pres(),
                self.acc_port_enbl()
            )
        }
    }
}
