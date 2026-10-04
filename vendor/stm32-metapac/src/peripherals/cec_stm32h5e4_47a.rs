#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cec {
    ptr: *mut u8,
}
unsafe impl Send for Cec {}
unsafe impl Sync for Cec {}
impl Cec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::CecCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::CecCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn txdr(self) -> crate::common::Reg<regs::CecTxdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn rxdr(self) -> crate::common::Reg<regs::CecRxdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::CecIsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::CecIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CecCfgr(pub u32);
    impl CecCfgr {
        #[must_use]
        #[inline(always)]
        pub const fn sft(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxtol(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxtol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn brestp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_brestp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bregen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lbpegen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lbpegen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn brdnogen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_brdnogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sftopt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sftopt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn oar(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_oar(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lstn(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lstn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CecCfgr {
        #[inline(always)]
        fn default() -> CecCfgr {
            CecCfgr(0)
        }
    }
    impl core::fmt::Debug for CecCfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CecCfgr")
                .field("sft", &self.sft())
                .field("rxtol", &self.rxtol())
                .field("brestp", &self.brestp())
                .field("bregen", &self.bregen())
                .field("lbpegen", &self.lbpegen())
                .field("brdnogen", &self.brdnogen())
                .field("sftopt", &self.sftopt())
                .field("oar", &self.oar())
                .field("lstn", &self.lstn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CecCfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CecCfgr {{ sft: {=u8:?}, rxtol: {=bool:?}, brestp: {=bool:?}, bregen: {=bool:?}, lbpegen: {=bool:?}, brdnogen: {=bool:?}, sftopt: {=bool:?}, oar: {=u16:?}, lstn: {=bool:?} }}",
                self.sft(),
                self.rxtol(),
                self.brestp(),
                self.bregen(),
                self.lbpegen(),
                self.brdnogen(),
                self.sftopt(),
                self.oar(),
                self.lstn()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CecCr(pub u32);
    impl CecCr {
        #[must_use]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txsom(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txsom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txeom(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txeom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for CecCr {
        #[inline(always)]
        fn default() -> CecCr {
            CecCr(0)
        }
    }
    impl core::fmt::Debug for CecCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CecCr")
                .field("cecen", &self.cecen())
                .field("txsom", &self.txsom())
                .field("txeom", &self.txeom())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CecCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CecCr {{ cecen: {=bool:?}, txsom: {=bool:?}, txeom: {=bool:?} }}",
                self.cecen(),
                self.txsom(),
                self.txeom()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CecIer(pub u32);
    impl CecIer {
        #[must_use]
        #[inline(always)]
        pub const fn rxbrie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxbrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxendie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxovrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn breie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_breie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbpeie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lbpeie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lbpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxackeie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxackeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn arblstie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_arblstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txbrie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txbrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txendie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txudrie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txudrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txerrie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txackeie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txackeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for CecIer {
        #[inline(always)]
        fn default() -> CecIer {
            CecIer(0)
        }
    }
    impl core::fmt::Debug for CecIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CecIer")
                .field("rxbrie", &self.rxbrie())
                .field("rxendie", &self.rxendie())
                .field("rxovrie", &self.rxovrie())
                .field("breie", &self.breie())
                .field("sbpeie", &self.sbpeie())
                .field("lbpeie", &self.lbpeie())
                .field("rxackeie", &self.rxackeie())
                .field("arblstie", &self.arblstie())
                .field("txbrie", &self.txbrie())
                .field("txendie", &self.txendie())
                .field("txudrie", &self.txudrie())
                .field("txerrie", &self.txerrie())
                .field("txackeie", &self.txackeie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CecIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CecIer {{ rxbrie: {=bool:?}, rxendie: {=bool:?}, rxovrie: {=bool:?}, breie: {=bool:?}, sbpeie: {=bool:?}, lbpeie: {=bool:?}, rxackeie: {=bool:?}, arblstie: {=bool:?}, txbrie: {=bool:?}, txendie: {=bool:?}, txudrie: {=bool:?}, txerrie: {=bool:?}, txackeie: {=bool:?} }}",
                self.rxbrie(),
                self.rxendie(),
                self.rxovrie(),
                self.breie(),
                self.sbpeie(),
                self.lbpeie(),
                self.rxackeie(),
                self.arblstie(),
                self.txbrie(),
                self.txendie(),
                self.txudrie(),
                self.txerrie(),
                self.txackeie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CecIsr(pub u32);
    impl CecIsr {
        #[must_use]
        #[inline(always)]
        pub const fn rxbr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxbr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxovr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bre(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lbpe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxacke(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxacke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn arblst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_arblst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txbr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txbr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txend(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txudr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txudr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txacke(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txacke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for CecIsr {
        #[inline(always)]
        fn default() -> CecIsr {
            CecIsr(0)
        }
    }
    impl core::fmt::Debug for CecIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CecIsr")
                .field("rxbr", &self.rxbr())
                .field("rxend", &self.rxend())
                .field("rxovr", &self.rxovr())
                .field("bre", &self.bre())
                .field("sbpe", &self.sbpe())
                .field("lbpe", &self.lbpe())
                .field("rxacke", &self.rxacke())
                .field("arblst", &self.arblst())
                .field("txbr", &self.txbr())
                .field("txend", &self.txend())
                .field("txudr", &self.txudr())
                .field("txerr", &self.txerr())
                .field("txacke", &self.txacke())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CecIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CecIsr {{ rxbr: {=bool:?}, rxend: {=bool:?}, rxovr: {=bool:?}, bre: {=bool:?}, sbpe: {=bool:?}, lbpe: {=bool:?}, rxacke: {=bool:?}, arblst: {=bool:?}, txbr: {=bool:?}, txend: {=bool:?}, txudr: {=bool:?}, txerr: {=bool:?}, txacke: {=bool:?} }}",
                self.rxbr(),
                self.rxend(),
                self.rxovr(),
                self.bre(),
                self.sbpe(),
                self.lbpe(),
                self.rxacke(),
                self.arblst(),
                self.txbr(),
                self.txend(),
                self.txudr(),
                self.txerr(),
                self.txacke()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CecRxdr(pub u32);
    impl CecRxdr {
        #[must_use]
        #[inline(always)]
        pub const fn rxd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CecRxdr {
        #[inline(always)]
        fn default() -> CecRxdr {
            CecRxdr(0)
        }
    }
    impl core::fmt::Debug for CecRxdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CecRxdr").field("rxd", &self.rxd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CecRxdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CecRxdr {{ rxd: {=u8:?} }}", self.rxd())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CecTxdr(pub u32);
    impl CecTxdr {
        #[must_use]
        #[inline(always)]
        pub const fn txd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_txd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CecTxdr {
        #[inline(always)]
        fn default() -> CecTxdr {
            CecTxdr(0)
        }
    }
    impl core::fmt::Debug for CecTxdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CecTxdr").field("txd", &self.txd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CecTxdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CecTxdr {{ txd: {=u8:?} }}", self.txd())
        }
    }
}
