#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ucpd {
    ptr: *mut u8,
}
unsafe impl Send for Ucpd {}
unsafe impl Sync for Ucpd {}
impl Ucpd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn cfg3(self) -> crate::common::Reg<regs::Cfg3, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ordset(self) -> crate::common::Reg<regs::TxOrdset, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_paysz(self) -> crate::common::Reg<regs::TxPaysz, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn txdr(self) -> crate::common::Reg<regs::Txdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ordset(self) -> crate::common::Reg<regs::RxOrdset, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_paysz(self) -> crate::common::Reg<regs::RxPaysz, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn rxdr(self) -> crate::common::Reg<regs::Rxdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ordext1(self) -> crate::common::Reg<regs::RxOrdext1, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ordext2(self) -> crate::common::Reg<regs::RxOrdext2, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[must_use]
        #[inline(always)]
        pub const fn hbitclkdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hbitclkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ifrgap(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ifrgap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn transwin(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_transwin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn psc_ucpdclk(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_psc_ucpdclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxordseten(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rxordseten(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val as u32) & 0x01ff) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ucpden(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ucpden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    impl core::fmt::Debug for Cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg1")
                .field("hbitclkdiv", &self.hbitclkdiv())
                .field("ifrgap", &self.ifrgap())
                .field("transwin", &self.transwin())
                .field("psc_ucpdclk", &self.psc_ucpdclk())
                .field("rxordseten", &self.rxordseten())
                .field("txdmaen", &self.txdmaen())
                .field("rxdmaen", &self.rxdmaen())
                .field("ucpden", &self.ucpden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg1 {{ hbitclkdiv: {=u8:?}, ifrgap: {=u8:?}, transwin: {=u8:?}, psc_ucpdclk: {=u8:?}, rxordseten: {=u16:?}, txdmaen: {=bool:?}, rxdmaen: {=bool:?}, ucpden: {=bool:?} }}",
                self.hbitclkdiv(),
                self.ifrgap(),
                self.transwin(),
                self.psc_ucpdclk(),
                self.rxordseten(),
                self.txdmaen(),
                self.rxdmaen(),
                self.ucpden()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[must_use]
        #[inline(always)]
        pub const fn rxfiltdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfiltdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxfilt2n3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxfilt2n3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn forceclk(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_forceclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wupen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxafilten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxafilten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    impl core::fmt::Debug for Cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg2")
                .field("rxfiltdis", &self.rxfiltdis())
                .field("rxfilt2n3", &self.rxfilt2n3())
                .field("forceclk", &self.forceclk())
                .field("wupen", &self.wupen())
                .field("rxafilten", &self.rxafilten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg2 {{ rxfiltdis: {=bool:?}, rxfilt2n3: {=bool:?}, forceclk: {=bool:?}, wupen: {=bool:?}, rxafilten: {=bool:?} }}",
                self.rxfiltdis(),
                self.rxfilt2n3(),
                self.forceclk(),
                self.wupen(),
                self.rxafilten()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg3(pub u32);
    impl Cfg3 {
        #[must_use]
        #[inline(always)]
        pub const fn trim_cc1_rd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trim_cc1_rd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trim_cc1_rp(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trim_cc1_rp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trim_cc2_rd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trim_cc2_rd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trim_cc2_rp(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trim_cc2_rp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
        }
    }
    impl Default for Cfg3 {
        #[inline(always)]
        fn default() -> Cfg3 {
            Cfg3(0)
        }
    }
    impl core::fmt::Debug for Cfg3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg3")
                .field("trim_cc1_rd", &self.trim_cc1_rd())
                .field("trim_cc1_rp", &self.trim_cc1_rp())
                .field("trim_cc2_rd", &self.trim_cc2_rd())
                .field("trim_cc2_rp", &self.trim_cc2_rp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg3 {{ trim_cc1_rd: {=u8:?}, trim_cc1_rp: {=u8:?}, trim_cc2_rd: {=u8:?}, trim_cc2_rp: {=u8:?} }}",
                self.trim_cc1_rd(),
                self.trim_cc1_rp(),
                self.trim_cc2_rd(),
                self.trim_cc2_rp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[must_use]
        #[inline(always)]
        pub const fn txmode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_txmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txsend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txsend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txhrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txhrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxmode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn phyrxen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_phyrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn phyccsel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_phyccsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn anasubmode(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_anasubmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn anamode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_anamode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ccenable(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ccenable(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frsrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frsrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frstx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdch(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rdch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cc1tcdis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cc1tcdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cc2tcdis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cc2tcdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("txmode", &self.txmode())
                .field("txsend", &self.txsend())
                .field("txhrst", &self.txhrst())
                .field("rxmode", &self.rxmode())
                .field("phyrxen", &self.phyrxen())
                .field("phyccsel", &self.phyccsel())
                .field("anasubmode", &self.anasubmode())
                .field("anamode", &self.anamode())
                .field("ccenable", &self.ccenable())
                .field("frsrxen", &self.frsrxen())
                .field("frstx", &self.frstx())
                .field("rdch", &self.rdch())
                .field("cc1tcdis", &self.cc1tcdis())
                .field("cc2tcdis", &self.cc2tcdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ txmode: {=u8:?}, txsend: {=bool:?}, txhrst: {=bool:?}, rxmode: {=bool:?}, phyrxen: {=bool:?}, phyccsel: {=bool:?}, anasubmode: {=u8:?}, anamode: {=bool:?}, ccenable: {=u8:?}, frsrxen: {=bool:?}, frstx: {=bool:?}, rdch: {=bool:?}, cc1tcdis: {=bool:?}, cc2tcdis: {=bool:?} }}",
                self.txmode(),
                self.txsend(),
                self.txhrst(),
                self.rxmode(),
                self.phyrxen(),
                self.phyccsel(),
                self.anasubmode(),
                self.anamode(),
                self.ccenable(),
                self.frsrxen(),
                self.frstx(),
                self.rdch(),
                self.cc1tcdis(),
                self.cc2tcdis()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[must_use]
        #[inline(always)]
        pub const fn txmsgdisccf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgdisccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmsgsentcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgsentcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmsgabtcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgabtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hrstdisccf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hrstdisccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hrstsentcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hrstsentcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txundcf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txundcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxorddetcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxorddetcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxhrstdetcf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxhrstdetcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxovrcf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxovrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxmsgendcf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxmsgendcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typecevt1cf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_typecevt1cf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typecevt2cf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_typecevt2cf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frsevtcf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frsevtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("txmsgdisccf", &self.txmsgdisccf())
                .field("txmsgsentcf", &self.txmsgsentcf())
                .field("txmsgabtcf", &self.txmsgabtcf())
                .field("hrstdisccf", &self.hrstdisccf())
                .field("hrstsentcf", &self.hrstsentcf())
                .field("txundcf", &self.txundcf())
                .field("rxorddetcf", &self.rxorddetcf())
                .field("rxhrstdetcf", &self.rxhrstdetcf())
                .field("rxovrcf", &self.rxovrcf())
                .field("rxmsgendcf", &self.rxmsgendcf())
                .field("typecevt1cf", &self.typecevt1cf())
                .field("typecevt2cf", &self.typecevt2cf())
                .field("frsevtcf", &self.frsevtcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ txmsgdisccf: {=bool:?}, txmsgsentcf: {=bool:?}, txmsgabtcf: {=bool:?}, hrstdisccf: {=bool:?}, hrstsentcf: {=bool:?}, txundcf: {=bool:?}, rxorddetcf: {=bool:?}, rxhrstdetcf: {=bool:?}, rxovrcf: {=bool:?}, rxmsgendcf: {=bool:?}, typecevt1cf: {=bool:?}, typecevt2cf: {=bool:?}, frsevtcf: {=bool:?} }}",
                self.txmsgdisccf(),
                self.txmsgsentcf(),
                self.txmsgabtcf(),
                self.hrstdisccf(),
                self.hrstsentcf(),
                self.txundcf(),
                self.rxorddetcf(),
                self.rxhrstdetcf(),
                self.rxovrcf(),
                self.rxmsgendcf(),
                self.typecevt1cf(),
                self.typecevt2cf(),
                self.frsevtcf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr(pub u32);
    impl Imr {
        #[must_use]
        #[inline(always)]
        pub const fn txisie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txisie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmsgdiscie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgdiscie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmsgsentie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmsgabtie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgabtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hrstdiscie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hrstdiscie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hrstsentie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hrstsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txundie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txundie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxorddetie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxorddetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxhrstdetie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxhrstdetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxovrie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxmsgendie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxmsgendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typecevt1ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_typecevt1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typecevt2ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_typecevt2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frsevtie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frsevtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Imr {
        #[inline(always)]
        fn default() -> Imr {
            Imr(0)
        }
    }
    impl core::fmt::Debug for Imr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imr")
                .field("txisie", &self.txisie())
                .field("txmsgdiscie", &self.txmsgdiscie())
                .field("txmsgsentie", &self.txmsgsentie())
                .field("txmsgabtie", &self.txmsgabtie())
                .field("hrstdiscie", &self.hrstdiscie())
                .field("hrstsentie", &self.hrstsentie())
                .field("txundie", &self.txundie())
                .field("rxneie", &self.rxneie())
                .field("rxorddetie", &self.rxorddetie())
                .field("rxhrstdetie", &self.rxhrstdetie())
                .field("rxovrie", &self.rxovrie())
                .field("rxmsgendie", &self.rxmsgendie())
                .field("typecevt1ie", &self.typecevt1ie())
                .field("typecevt2ie", &self.typecevt2ie())
                .field("frsevtie", &self.frsevtie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Imr {{ txisie: {=bool:?}, txmsgdiscie: {=bool:?}, txmsgsentie: {=bool:?}, txmsgabtie: {=bool:?}, hrstdiscie: {=bool:?}, hrstsentie: {=bool:?}, txundie: {=bool:?}, rxneie: {=bool:?}, rxorddetie: {=bool:?}, rxhrstdetie: {=bool:?}, rxovrie: {=bool:?}, rxmsgendie: {=bool:?}, typecevt1ie: {=bool:?}, typecevt2ie: {=bool:?}, frsevtie: {=bool:?} }}",
                self.txisie(),
                self.txmsgdiscie(),
                self.txmsgsentie(),
                self.txmsgabtie(),
                self.hrstdiscie(),
                self.hrstsentie(),
                self.txundie(),
                self.rxneie(),
                self.rxorddetie(),
                self.rxhrstdetie(),
                self.rxovrie(),
                self.rxmsgendie(),
                self.typecevt1ie(),
                self.typecevt2ie(),
                self.frsevtie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxOrdext1(pub u32);
    impl RxOrdext1 {
        #[must_use]
        #[inline(always)]
        pub const fn rxsopx1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rxsopx1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for RxOrdext1 {
        #[inline(always)]
        fn default() -> RxOrdext1 {
            RxOrdext1(0)
        }
    }
    impl core::fmt::Debug for RxOrdext1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxOrdext1")
                .field("rxsopx1", &self.rxsopx1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxOrdext1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxOrdext1 {{ rxsopx1: {=u32:?} }}", self.rxsopx1())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxOrdext2(pub u32);
    impl RxOrdext2 {
        #[must_use]
        #[inline(always)]
        pub const fn rxsopx2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rxsopx2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for RxOrdext2 {
        #[inline(always)]
        fn default() -> RxOrdext2 {
            RxOrdext2(0)
        }
    }
    impl core::fmt::Debug for RxOrdext2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxOrdext2")
                .field("rxsopx2", &self.rxsopx2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxOrdext2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxOrdext2 {{ rxsopx2: {=u32:?} }}", self.rxsopx2())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxOrdset(pub u32);
    impl RxOrdset {
        #[must_use]
        #[inline(always)]
        pub const fn rxordset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxordset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxsop3of4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxsop3of4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxsopkinvalid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxsopkinvalid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for RxOrdset {
        #[inline(always)]
        fn default() -> RxOrdset {
            RxOrdset(0)
        }
    }
    impl core::fmt::Debug for RxOrdset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxOrdset")
                .field("rxordset", &self.rxordset())
                .field("rxsop3of4", &self.rxsop3of4())
                .field("rxsopkinvalid", &self.rxsopkinvalid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxOrdset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RxOrdset {{ rxordset: {=u8:?}, rxsop3of4: {=bool:?}, rxsopkinvalid: {=u8:?} }}",
                self.rxordset(),
                self.rxsop3of4(),
                self.rxsopkinvalid()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxPaysz(pub u32);
    impl RxPaysz {
        #[must_use]
        #[inline(always)]
        pub const fn rxpaysz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rxpaysz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for RxPaysz {
        #[inline(always)]
        fn default() -> RxPaysz {
            RxPaysz(0)
        }
    }
    impl core::fmt::Debug for RxPaysz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxPaysz")
                .field("rxpaysz", &self.rxpaysz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxPaysz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxPaysz {{ rxpaysz: {=u16:?} }}", self.rxpaysz())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdr(pub u32);
    impl Rxdr {
        #[must_use]
        #[inline(always)]
        pub const fn rxdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rxdr {
        #[inline(always)]
        fn default() -> Rxdr {
            Rxdr(0)
        }
    }
    impl core::fmt::Debug for Rxdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxdr")
                .field("rxdata", &self.rxdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxdr {{ rxdata: {=u8:?} }}", self.rxdata())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[must_use]
        #[inline(always)]
        pub const fn txis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmsgdisc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgdisc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmsgsent(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmsgabt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmsgabt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hrstdisc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hrstdisc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hrstsent(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hrstsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txund(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txund(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxorddet(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxorddet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxhrstdet(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxhrstdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxovr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxmsgend(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxmsgend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typecevt1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_typecevt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typecevt2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_typecevt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typec_vstate_cc1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_typec_vstate_cc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typec_vstate_cc2(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_typec_vstate_cc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frsevt(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frsevt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("txis", &self.txis())
                .field("txmsgdisc", &self.txmsgdisc())
                .field("txmsgsent", &self.txmsgsent())
                .field("txmsgabt", &self.txmsgabt())
                .field("hrstdisc", &self.hrstdisc())
                .field("hrstsent", &self.hrstsent())
                .field("txund", &self.txund())
                .field("rxne", &self.rxne())
                .field("rxorddet", &self.rxorddet())
                .field("rxhrstdet", &self.rxhrstdet())
                .field("rxovr", &self.rxovr())
                .field("rxmsgend", &self.rxmsgend())
                .field("rxerr", &self.rxerr())
                .field("typecevt1", &self.typecevt1())
                .field("typecevt2", &self.typecevt2())
                .field("typec_vstate_cc1", &self.typec_vstate_cc1())
                .field("typec_vstate_cc2", &self.typec_vstate_cc2())
                .field("frsevt", &self.frsevt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ txis: {=bool:?}, txmsgdisc: {=bool:?}, txmsgsent: {=bool:?}, txmsgabt: {=bool:?}, hrstdisc: {=bool:?}, hrstsent: {=bool:?}, txund: {=bool:?}, rxne: {=bool:?}, rxorddet: {=bool:?}, rxhrstdet: {=bool:?}, rxovr: {=bool:?}, rxmsgend: {=bool:?}, rxerr: {=bool:?}, typecevt1: {=bool:?}, typecevt2: {=bool:?}, typec_vstate_cc1: {=u8:?}, typec_vstate_cc2: {=u8:?}, frsevt: {=bool:?} }}",
                self.txis(),
                self.txmsgdisc(),
                self.txmsgsent(),
                self.txmsgabt(),
                self.hrstdisc(),
                self.hrstsent(),
                self.txund(),
                self.rxne(),
                self.rxorddet(),
                self.rxhrstdet(),
                self.rxovr(),
                self.rxmsgend(),
                self.rxerr(),
                self.typecevt1(),
                self.typecevt2(),
                self.typec_vstate_cc1(),
                self.typec_vstate_cc2(),
                self.frsevt()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxOrdset(pub u32);
    impl TxOrdset {
        #[must_use]
        #[inline(always)]
        pub const fn txordset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_txordset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for TxOrdset {
        #[inline(always)]
        fn default() -> TxOrdset {
            TxOrdset(0)
        }
    }
    impl core::fmt::Debug for TxOrdset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxOrdset")
                .field("txordset", &self.txordset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxOrdset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxOrdset {{ txordset: {=u32:?} }}", self.txordset())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxPaysz(pub u32);
    impl TxPaysz {
        #[must_use]
        #[inline(always)]
        pub const fn txpaysz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_txpaysz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for TxPaysz {
        #[inline(always)]
        fn default() -> TxPaysz {
            TxPaysz(0)
        }
    }
    impl core::fmt::Debug for TxPaysz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxPaysz")
                .field("txpaysz", &self.txpaysz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxPaysz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxPaysz {{ txpaysz: {=u16:?} }}", self.txpaysz())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txdr(pub u32);
    impl Txdr {
        #[must_use]
        #[inline(always)]
        pub const fn txdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_txdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Txdr {
        #[inline(always)]
        fn default() -> Txdr {
            Txdr(0)
        }
    }
    impl core::fmt::Debug for Txdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txdr")
                .field("txdata", &self.txdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txdr {{ txdata: {=u8:?} }}", self.txdata())
        }
    }
}
