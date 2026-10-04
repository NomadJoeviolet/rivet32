#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eth {
    ptr: *mut u8,
}
unsafe impl Send for Eth {}
unsafe impl Sync for Eth {}
impl Eth {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn maccr(self) -> crate::common::Reg<regs::EthMaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn macecr(self) -> crate::common::Reg<regs::EthMacecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn macpfr(self) -> crate::common::Reg<regs::EthMacpfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn macwjbtr(self) -> crate::common::Reg<regs::EthMacwjbtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn macht0r(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn macht1r(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn macvtr(self) -> crate::common::Reg<regs::EthMacvtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn macvhtr(self) -> crate::common::Reg<regs::EthMacvhtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn macvir(self) -> crate::common::Reg<regs::EthMacvir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn macivir(self) -> crate::common::Reg<regs::EthMacivir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn mactfcr(self) -> crate::common::Reg<regs::EthMactfcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn macrfcr(self) -> crate::common::Reg<regs::EthMacrfcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn macisr(self) -> crate::common::Reg<regs::EthMacisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn macier(self) -> crate::common::Reg<regs::EthMacier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn macrxtxsr(self) -> crate::common::Reg<regs::EthMacrxtxsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn macpcsr(self) -> crate::common::Reg<regs::EthMacpcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn macrwkpfr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn maclcsr(self) -> crate::common::Reg<regs::EthMaclcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn macltcr(self) -> crate::common::Reg<regs::EthMacltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn macletr(self) -> crate::common::Reg<regs::EthMacletr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn mac1ustcr(self) -> crate::common::Reg<regs::EthMac1ustcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn macvr(self) -> crate::common::Reg<regs::EthMacvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn macdr(self) -> crate::common::Reg<regs::EthMacdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn machwf0r(self) -> crate::common::Reg<regs::EthMachwf0r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn machwf1r(self) -> crate::common::Reg<regs::EthMachwf1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn machwf2r(self) -> crate::common::Reg<regs::EthMachwf2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn machwf3r(self) -> crate::common::Reg<regs::EthMachwf3r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn macmdioar(self) -> crate::common::Reg<regs::EthMacmdioar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn macmdiodr(self) -> crate::common::Reg<regs::EthMacmdiodr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn macarpar(self) -> crate::common::Reg<regs::EthMacarpar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn maccsrswcr(self) -> crate::common::Reg<regs::EthMaccsrswcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[inline(always)]
    pub const fn macfpecsr(self) -> crate::common::Reg<regs::EthMacfpecsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[inline(always)]
    pub const fn macprstimr(self) -> crate::common::Reg<regs::EthMacprstimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn macprstimur(self) -> crate::common::Reg<regs::EthMacprstimur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[inline(always)]
    pub const fn maca0hr(self) -> crate::common::Reg<regs::EthMaca0hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn maca0lr(self) -> crate::common::Reg<regs::EthMaca0lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[inline(always)]
    pub const fn maca1hr(self) -> crate::common::Reg<regs::EthMaca1hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[inline(always)]
    pub const fn maca1lr(self) -> crate::common::Reg<regs::EthMaca1lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[inline(always)]
    pub const fn maca2hr(self) -> crate::common::Reg<regs::EthMaca2hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[inline(always)]
    pub const fn maca2lr(self) -> crate::common::Reg<regs::EthMaca2lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[inline(always)]
    pub const fn maca3hr(self) -> crate::common::Reg<regs::EthMaca3hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[inline(always)]
    pub const fn maca3lr(self) -> crate::common::Reg<regs::EthMaca3lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[inline(always)]
    pub const fn mmccr(self) -> crate::common::Reg<regs::EthMmccr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrir(self) -> crate::common::Reg<regs::EthMmcrir, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[inline(always)]
    pub const fn mmctir(self) -> crate::common::Reg<regs::EthMmctir, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrimr(self) -> crate::common::Reg<regs::EthMmcrimr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[inline(always)]
    pub const fn mmctimr(self) -> crate::common::Reg<regs::EthMmctimr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[inline(always)]
    pub const fn mmctscgpr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x074cusize) as _) }
    }
    #[inline(always)]
    pub const fn mmctmcgpr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0750usize) as _) }
    }
    #[inline(always)]
    pub const fn mmctpcgr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrcrcepr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0794usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcraepr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0798usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrupgr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c4usize) as _) }
    }
    #[inline(always)]
    pub const fn mmctlpimstr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07ecusize) as _) }
    }
    #[inline(always)]
    pub const fn mmctlpitcr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrlpimstr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f4usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrlpitcr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcfpetisr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a0usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcfpetimr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a4usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcfpetfcr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a8usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcthrcr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08acusize) as _) }
    }
    #[inline(always)]
    pub const fn mmcfperisr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c0usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcfperimr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c4usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrpaer(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c8usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrpsmder(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08ccusize) as _) }
    }
    #[inline(always)]
    pub const fn mmcrpaokr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d0usize) as _) }
    }
    #[inline(always)]
    pub const fn mmcfperfcr(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d4usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3l4c0r(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[inline(always)]
    pub const fn macl4a0r(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0904usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3a0r0r(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0910usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3a1r0r(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0914usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3a2r0r(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0918usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3a3r0r(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x091cusize) as _) }
    }
    #[inline(always)]
    pub const fn macl3l4c1r(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0930usize) as _) }
    }
    #[inline(always)]
    pub const fn macl4a1r(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0934usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3a0r1r(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3a1r1r(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0944usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3a2r1r(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0948usize) as _) }
    }
    #[inline(always)]
    pub const fn macl3a3r1r(self) -> crate::common::Reg<u32, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x094cusize) as _) }
    }
    #[inline(always)]
    pub const fn maciacr(self) -> crate::common::Reg<regs::EthMaciacr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a70usize) as _) }
    }
    #[inline(always)]
    pub const fn mactmrqr(self) -> crate::common::Reg<regs::EthMactmrqr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a74usize) as _) }
    }
    #[inline(always)]
    pub const fn mactscr(self) -> crate::common::Reg<regs::EthMactscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[inline(always)]
    pub const fn macssir(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b04usize) as _) }
    }
    #[inline(always)]
    pub const fn macstsr(self) -> crate::common::Reg<regs::EthMacstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[inline(always)]
    pub const fn macstnr(self) -> crate::common::Reg<regs::EthMacstnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b0cusize) as _) }
    }
    #[inline(always)]
    pub const fn macstsur(self) -> crate::common::Reg<regs::EthMacstsur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[inline(always)]
    pub const fn macstnur(self) -> crate::common::Reg<regs::EthMacstnur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[inline(always)]
    pub const fn mactsar(self) -> crate::common::Reg<regs::EthMactsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b18usize) as _) }
    }
    #[inline(always)]
    pub const fn mactssr(self) -> crate::common::Reg<regs::EthMactssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[inline(always)]
    pub const fn macrxdti(self) -> crate::common::Reg<regs::EthMacrxdti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b24usize) as _) }
    }
    #[inline(always)]
    pub const fn mactxdti(self) -> crate::common::Reg<regs::EthMactxdti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b28usize) as _) }
    }
    #[inline(always)]
    pub const fn macttssnr(self) -> crate::common::Reg<regs::EthMacttssnr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[inline(always)]
    pub const fn macttsssr(self) -> crate::common::Reg<regs::EthMacttsssr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[inline(always)]
    pub const fn macacr(self) -> crate::common::Reg<regs::EthMacacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
    #[inline(always)]
    pub const fn macatsnr(self) -> crate::common::Reg<regs::EthMacatsnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b48usize) as _) }
    }
    #[inline(always)]
    pub const fn macatssr(self) -> crate::common::Reg<regs::EthMacatssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b4cusize) as _) }
    }
    #[inline(always)]
    pub const fn mactsiacr(self) -> crate::common::Reg<regs::EthMactsiacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b50usize) as _) }
    }
    #[inline(always)]
    pub const fn mactseacr(self) -> crate::common::Reg<regs::EthMactseacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b54usize) as _) }
    }
    #[inline(always)]
    pub const fn mactsicnr(self) -> crate::common::Reg<regs::EthMactsicnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b58usize) as _) }
    }
    #[inline(always)]
    pub const fn mactsecnr(self) -> crate::common::Reg<regs::EthMactsecnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b5cusize) as _) }
    }
    #[inline(always)]
    pub const fn mactsilr(self) -> crate::common::Reg<regs::EthMactsilr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[inline(always)]
    pub const fn mactselr(self) -> crate::common::Reg<regs::EthMactselr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[inline(always)]
    pub const fn macppscr(self) -> crate::common::Reg<regs::EthMacppscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b70usize) as _) }
    }
    #[inline(always)]
    pub const fn macppsttsr(self) -> crate::common::Reg<regs::EthMacppsttsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b80usize) as _) }
    }
    #[inline(always)]
    pub const fn macppsttnr(self) -> crate::common::Reg<regs::EthMacppsttnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b84usize) as _) }
    }
    #[inline(always)]
    pub const fn macppsir(self) -> crate::common::Reg<regs::EthMacppsir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b88usize) as _) }
    }
    #[inline(always)]
    pub const fn macppswr(self) -> crate::common::Reg<regs::EthMacppswr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b8cusize) as _) }
    }
    #[inline(always)]
    pub const fn macpocr(self) -> crate::common::Reg<regs::EthMacpocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc0usize) as _) }
    }
    #[inline(always)]
    pub const fn macspi0r(self) -> crate::common::Reg<regs::EthMacspi0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc4usize) as _) }
    }
    #[inline(always)]
    pub const fn macspi1r(self) -> crate::common::Reg<regs::EthMacspi1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc8usize) as _) }
    }
    #[inline(always)]
    pub const fn macspi2r(self) -> crate::common::Reg<regs::EthMacspi2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bccusize) as _) }
    }
    #[inline(always)]
    pub const fn maclmir(self) -> crate::common::Reg<regs::EthMaclmir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bd0usize) as _) }
    }
    #[inline(always)]
    pub const fn mtlomr(self) -> crate::common::Reg<regs::EthMtlomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[inline(always)]
    pub const fn mtlisr(self) -> crate::common::Reg<regs::EthMtlisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c20usize) as _) }
    }
    #[inline(always)]
    pub const fn mtltqomr(self) -> crate::common::Reg<regs::EthMtltqomr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d00usize) as _) }
    }
    #[inline(always)]
    pub const fn mtltqur(self) -> crate::common::Reg<regs::EthMtltqur, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d04usize) as _) }
    }
    #[inline(always)]
    pub const fn mtltqdr(self) -> crate::common::Reg<regs::EthMtltqdr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d08usize) as _) }
    }
    #[inline(always)]
    pub const fn mtlqicsr(self) -> crate::common::Reg<regs::EthMtlqicsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d2cusize) as _) }
    }
    #[inline(always)]
    pub const fn mtlrqomr(self) -> crate::common::Reg<regs::EthMtlrqomr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d30usize) as _) }
    }
    #[inline(always)]
    pub const fn mtlrqmpocr(self) -> crate::common::Reg<regs::EthMtlrqmpocr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d34usize) as _) }
    }
    #[inline(always)]
    pub const fn mtlrqdr(self) -> crate::common::Reg<regs::EthMtlrqdr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d38usize) as _) }
    }
    #[inline(always)]
    pub const fn dmamr(self) -> crate::common::Reg<regs::EthDmamr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[inline(always)]
    pub const fn dmasbmr(self) -> crate::common::Reg<regs::EthDmasbmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[inline(always)]
    pub const fn dmaisr(self) -> crate::common::Reg<regs::EthDmaisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[inline(always)]
    pub const fn dmadsr(self) -> crate::common::Reg<regs::EthDmadsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmaccr(self) -> crate::common::Reg<regs::EthDmaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[inline(always)]
    pub const fn dmactcr(self) -> crate::common::Reg<regs::EthDmactcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[inline(always)]
    pub const fn dmacrcr(self) -> crate::common::Reg<regs::EthDmacrcr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[inline(always)]
    pub const fn dmactdlar(self) -> crate::common::Reg<regs::EthDmactdlar, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[inline(always)]
    pub const fn dmacrdlar(self) -> crate::common::Reg<regs::EthDmacrdlar, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmactdtpr(self) -> crate::common::Reg<regs::EthDmactdtpr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[inline(always)]
    pub const fn dmacrdtpr(self) -> crate::common::Reg<regs::EthDmacrdtpr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[inline(always)]
    pub const fn dmactdrlr(self) -> crate::common::Reg<regs::EthDmactdrlr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmacrdrlr(self) -> crate::common::Reg<regs::EthDmacrdrlr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[inline(always)]
    pub const fn dmacier(self) -> crate::common::Reg<regs::EthDmacier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[inline(always)]
    pub const fn dmacriwtr(self) -> crate::common::Reg<regs::EthDmacriwtr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[inline(always)]
    pub const fn dmaccatdr(self) -> crate::common::Reg<regs::EthDmaccatdr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1144usize) as _) }
    }
    #[inline(always)]
    pub const fn dmaccardr(self) -> crate::common::Reg<regs::EthDmaccardr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x114cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmaccatbr(self) -> crate::common::Reg<regs::EthDmaccatbr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[inline(always)]
    pub const fn dmaccarbr(self) -> crate::common::Reg<regs::EthDmaccarbr, crate::common::Raw> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmacsr(self) -> crate::common::Reg<regs::EthDmacsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[inline(always)]
    pub const fn dmacmfcr(self) -> crate::common::Reg<regs::EthDmacmfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmaccarbr(pub u32);
    impl EthDmaccarbr {
        #[must_use]
        #[inline(always)]
        pub const fn currbufaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_currbufaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthDmaccarbr {
        #[inline(always)]
        fn default() -> EthDmaccarbr {
            EthDmaccarbr(0)
        }
    }
    impl core::fmt::Debug for EthDmaccarbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmaccarbr")
                .field("currbufaptr", &self.currbufaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmaccarbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmaccarbr {{ currbufaptr: {=u32:?} }}",
                self.currbufaptr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmaccardr(pub u32);
    impl EthDmaccardr {
        #[must_use]
        #[inline(always)]
        pub const fn currdesaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_currdesaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthDmaccardr {
        #[inline(always)]
        fn default() -> EthDmaccardr {
            EthDmaccardr(0)
        }
    }
    impl core::fmt::Debug for EthDmaccardr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmaccardr")
                .field("currdesaptr", &self.currdesaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmaccardr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmaccardr {{ currdesaptr: {=u32:?} }}",
                self.currdesaptr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmaccatbr(pub u32);
    impl EthDmaccatbr {
        #[must_use]
        #[inline(always)]
        pub const fn curtbufaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_curtbufaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthDmaccatbr {
        #[inline(always)]
        fn default() -> EthDmaccatbr {
            EthDmaccatbr(0)
        }
    }
    impl core::fmt::Debug for EthDmaccatbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmaccatbr")
                .field("curtbufaptr", &self.curtbufaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmaccatbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmaccatbr {{ curtbufaptr: {=u32:?} }}",
                self.curtbufaptr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmaccatdr(pub u32);
    impl EthDmaccatdr {
        #[must_use]
        #[inline(always)]
        pub const fn curtdesaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_curtdesaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthDmaccatdr {
        #[inline(always)]
        fn default() -> EthDmaccatdr {
            EthDmaccatdr(0)
        }
    }
    impl core::fmt::Debug for EthDmaccatdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmaccatdr")
                .field("curtdesaptr", &self.curtdesaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmaccatdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmaccatdr {{ curtdesaptr: {=u32:?} }}",
                self.curtdesaptr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmaccr(pub u32);
    impl EthDmaccr {
        #[must_use]
        #[inline(always)]
        pub const fn mss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mss(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dro(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dro(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for EthDmaccr {
        #[inline(always)]
        fn default() -> EthDmaccr {
            EthDmaccr(0)
        }
    }
    impl core::fmt::Debug for EthDmaccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmaccr")
                .field("mss", &self.mss())
                .field("dsl", &self.dsl())
                .field("dro", &self.dro())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmaccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmaccr {{ mss: {=u16:?}, dsl: {=u8:?}, dro: {=bool:?} }}",
                self.mss(),
                self.dsl(),
                self.dro()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmacier(pub u32);
    impl EthDmacier {
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tbue(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwte(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn etie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_etie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn erie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_erie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fbee(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fbee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cdee(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cdee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for EthDmacier {
        #[inline(always)]
        fn default() -> EthDmacier {
            EthDmacier(0)
        }
    }
    impl core::fmt::Debug for EthDmacier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmacier")
                .field("tie", &self.tie())
                .field("txse", &self.txse())
                .field("tbue", &self.tbue())
                .field("rie", &self.rie())
                .field("rbue", &self.rbue())
                .field("rse", &self.rse())
                .field("rwte", &self.rwte())
                .field("etie", &self.etie())
                .field("erie", &self.erie())
                .field("fbee", &self.fbee())
                .field("cdee", &self.cdee())
                .field("aie", &self.aie())
                .field("nie", &self.nie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmacier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmacier {{ tie: {=bool:?}, txse: {=bool:?}, tbue: {=bool:?}, rie: {=bool:?}, rbue: {=bool:?}, rse: {=bool:?}, rwte: {=bool:?}, etie: {=bool:?}, erie: {=bool:?}, fbee: {=bool:?}, cdee: {=bool:?}, aie: {=bool:?}, nie: {=bool:?} }}",
                self.tie(),
                self.txse(),
                self.tbue(),
                self.rie(),
                self.rbue(),
                self.rse(),
                self.rwte(),
                self.etie(),
                self.erie(),
                self.fbee(),
                self.cdee(),
                self.aie(),
                self.nie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmacmfcr(pub u32);
    impl EthDmacmfcr {
        #[must_use]
        #[inline(always)]
        pub const fn mfc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mfc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mfco(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mfco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for EthDmacmfcr {
        #[inline(always)]
        fn default() -> EthDmacmfcr {
            EthDmacmfcr(0)
        }
    }
    impl core::fmt::Debug for EthDmacmfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmacmfcr")
                .field("mfc", &self.mfc())
                .field("mfco", &self.mfco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmacmfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmacmfcr {{ mfc: {=u16:?}, mfco: {=bool:?} }}",
                self.mfc(),
                self.mfco()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmacrcr(pub u32);
    impl EthDmacrcr {
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbsz(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rbsz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u32) & 0x3fff) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rpbl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthDmacrcr {
        #[inline(always)]
        fn default() -> EthDmacrcr {
            EthDmacrcr(0)
        }
    }
    impl core::fmt::Debug for EthDmacrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmacrcr")
                .field("sr", &self.sr())
                .field("rbsz", &self.rbsz())
                .field("rpbl", &self.rpbl())
                .field("rpf", &self.rpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmacrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmacrcr {{ sr: {=bool:?}, rbsz: {=u16:?}, rpbl: {=u8:?}, rpf: {=bool:?} }}",
                self.sr(),
                self.rbsz(),
                self.rpbl(),
                self.rpf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmacrdlar(pub u32);
    impl EthDmacrdlar {
        #[must_use]
        #[inline(always)]
        pub const fn rdesla(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rdesla(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthDmacrdlar {
        #[inline(always)]
        fn default() -> EthDmacrdlar {
            EthDmacrdlar(0)
        }
    }
    impl core::fmt::Debug for EthDmacrdlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmacrdlar")
                .field("rdesla", &self.rdesla())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmacrdlar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthDmacrdlar {{ rdesla: {=u32:?} }}", self.rdesla())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmacrdrlr(pub u32);
    impl EthDmacrdrlr {
        #[must_use]
        #[inline(always)]
        pub const fn rdrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rdrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn arbs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_arbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for EthDmacrdrlr {
        #[inline(always)]
        fn default() -> EthDmacrdrlr {
            EthDmacrdrlr(0)
        }
    }
    impl core::fmt::Debug for EthDmacrdrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmacrdrlr")
                .field("rdrl", &self.rdrl())
                .field("arbs", &self.arbs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmacrdrlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmacrdrlr {{ rdrl: {=u16:?}, arbs: {=u8:?} }}",
                self.rdrl(),
                self.arbs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmacrdtpr(pub u32);
    impl EthDmacrdtpr {
        #[must_use]
        #[inline(always)]
        pub const fn rdt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rdt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthDmacrdtpr {
        #[inline(always)]
        fn default() -> EthDmacrdtpr {
            EthDmacrdtpr(0)
        }
    }
    impl core::fmt::Debug for EthDmacrdtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmacrdtpr")
                .field("rdt", &self.rdt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmacrdtpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthDmacrdtpr {{ rdt: {=u32:?} }}", self.rdt())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmacriwtr(pub u32);
    impl EthDmacriwtr {
        #[must_use]
        #[inline(always)]
        pub const fn rwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_itw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwtu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rwtu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwtu_2048(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rwtu_2048(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwtu_512(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwtu_512(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwtu_1024(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwtu_1024(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itcu(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_itcu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itcu_2048(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_itcu_2048(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itcu_512(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_itcu_512(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itcu_1024(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_itcu_1024(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbct(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rbct(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn psel(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_psel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthDmacriwtr {
        #[inline(always)]
        fn default() -> EthDmacriwtr {
            EthDmacriwtr(0)
        }
    }
    impl core::fmt::Debug for EthDmacriwtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmacriwtr")
                .field("rwt", &self.rwt())
                .field("itw", &self.itw())
                .field("rwtu", &self.rwtu())
                .field("rwtu_2048", &self.rwtu_2048())
                .field("rwtu_512", &self.rwtu_512())
                .field("rwtu_1024", &self.rwtu_1024())
                .field("itcu", &self.itcu())
                .field("itcu_2048", &self.itcu_2048())
                .field("itcu_512", &self.itcu_512())
                .field("itcu_1024", &self.itcu_1024())
                .field("rbct", &self.rbct())
                .field("psel", &self.psel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmacriwtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmacriwtr {{ rwt: {=u8:?}, itw: {=u8:?}, rwtu: {=u8:?}, rwtu_2048: {=u8:?}, rwtu_512: {=bool:?}, rwtu_1024: {=bool:?}, itcu: {=u8:?}, itcu_2048: {=u8:?}, itcu_512: {=bool:?}, itcu_1024: {=bool:?}, rbct: {=u16:?}, psel: {=bool:?} }}",
                self.rwt(),
                self.itw(),
                self.rwtu(),
                self.rwtu_2048(),
                self.rwtu_512(),
                self.rwtu_1024(),
                self.itcu(),
                self.itcu_2048(),
                self.itcu_512(),
                self.itcu_1024(),
                self.rbct(),
                self.psel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmacsr(pub u32);
    impl EthDmacsr {
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tps(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tbu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ri(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rbu(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rps(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eti(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eri(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fbe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn teb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_teb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn reb(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_reb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
    }
    impl Default for EthDmacsr {
        #[inline(always)]
        fn default() -> EthDmacsr {
            EthDmacsr(0)
        }
    }
    impl core::fmt::Debug for EthDmacsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmacsr")
                .field("ti", &self.ti())
                .field("tps", &self.tps())
                .field("tbu", &self.tbu())
                .field("ri", &self.ri())
                .field("rbu", &self.rbu())
                .field("rps", &self.rps())
                .field("rwt", &self.rwt())
                .field("eti", &self.eti())
                .field("eri", &self.eri())
                .field("fbe", &self.fbe())
                .field("cde", &self.cde())
                .field("ais", &self.ais())
                .field("nis", &self.nis())
                .field("teb", &self.teb())
                .field("reb", &self.reb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmacsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmacsr {{ ti: {=bool:?}, tps: {=bool:?}, tbu: {=bool:?}, ri: {=bool:?}, rbu: {=bool:?}, rps: {=bool:?}, rwt: {=bool:?}, eti: {=bool:?}, eri: {=bool:?}, fbe: {=bool:?}, cde: {=bool:?}, ais: {=bool:?}, nis: {=bool:?}, teb: {=u8:?}, reb: {=u8:?} }}",
                self.ti(),
                self.tps(),
                self.tbu(),
                self.ri(),
                self.rbu(),
                self.rps(),
                self.rwt(),
                self.eti(),
                self.eri(),
                self.fbe(),
                self.cde(),
                self.ais(),
                self.nis(),
                self.teb(),
                self.reb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmactcr(pub u32);
    impl EthDmactcr {
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn osp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_osp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tpbl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for EthDmactcr {
        #[inline(always)]
        fn default() -> EthDmactcr {
            EthDmactcr(0)
        }
    }
    impl core::fmt::Debug for EthDmactcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmactcr")
                .field("st", &self.st())
                .field("osp", &self.osp())
                .field("tse", &self.tse())
                .field("tpbl", &self.tpbl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmactcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmactcr {{ st: {=bool:?}, osp: {=bool:?}, tse: {=bool:?}, tpbl: {=u8:?} }}",
                self.st(),
                self.osp(),
                self.tse(),
                self.tpbl()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmactdlar(pub u32);
    impl EthDmactdlar {
        #[must_use]
        #[inline(always)]
        pub const fn tdesla(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tdesla(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthDmactdlar {
        #[inline(always)]
        fn default() -> EthDmactdlar {
            EthDmactdlar(0)
        }
    }
    impl core::fmt::Debug for EthDmactdlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmactdlar")
                .field("tdesla", &self.tdesla())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmactdlar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthDmactdlar {{ tdesla: {=u32:?} }}", self.tdesla())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmactdrlr(pub u32);
    impl EthDmactdrlr {
        #[must_use]
        #[inline(always)]
        pub const fn tdrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_tdrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for EthDmactdrlr {
        #[inline(always)]
        fn default() -> EthDmactdrlr {
            EthDmactdrlr(0)
        }
    }
    impl core::fmt::Debug for EthDmactdrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmactdrlr")
                .field("tdrl", &self.tdrl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmactdrlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthDmactdrlr {{ tdrl: {=u16:?} }}", self.tdrl())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmactdtpr(pub u32);
    impl EthDmactdtpr {
        #[must_use]
        #[inline(always)]
        pub const fn tdt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tdt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthDmactdtpr {
        #[inline(always)]
        fn default() -> EthDmactdtpr {
            EthDmactdtpr(0)
        }
    }
    impl core::fmt::Debug for EthDmactdtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmactdtpr")
                .field("tdt", &self.tdt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmactdtpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthDmactdtpr {{ tdt: {=u32:?} }}", self.tdt())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmadsr(pub u32);
    impl EthDmadsr {
        #[must_use]
        #[inline(always)]
        pub const fn axwhsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_axwhsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rps(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rps_fetching(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rps_fetching(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rps_transferring(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rps_transferring(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rps_waiting(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rps_waiting(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rps_timestamp_wr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rps_timestamp_wr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rps_suspended(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rps_suspended(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tps(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tps_closing(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tps_closing(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tps_fetching(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tps_fetching(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tps_reading(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tps_reading(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tps_suspended(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tps_suspended(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tps_waiting(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tps_waiting(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tps_timestamp_wr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tps_timestamp_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for EthDmadsr {
        #[inline(always)]
        fn default() -> EthDmadsr {
            EthDmadsr(0)
        }
    }
    impl core::fmt::Debug for EthDmadsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmadsr")
                .field("axwhsts", &self.axwhsts())
                .field("rps", &self.rps())
                .field("rps_fetching", &self.rps_fetching())
                .field("rps_transferring", &self.rps_transferring())
                .field("rps_waiting", &self.rps_waiting())
                .field("rps_timestamp_wr", &self.rps_timestamp_wr())
                .field("rps_suspended", &self.rps_suspended())
                .field("tps", &self.tps())
                .field("tps_closing", &self.tps_closing())
                .field("tps_fetching", &self.tps_fetching())
                .field("tps_reading", &self.tps_reading())
                .field("tps_suspended", &self.tps_suspended())
                .field("tps_waiting", &self.tps_waiting())
                .field("tps_timestamp_wr", &self.tps_timestamp_wr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmadsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmadsr {{ axwhsts: {=bool:?}, rps: {=u8:?}, rps_fetching: {=bool:?}, rps_transferring: {=u8:?}, rps_waiting: {=u8:?}, rps_timestamp_wr: {=u8:?}, rps_suspended: {=bool:?}, tps: {=u8:?}, tps_closing: {=u8:?}, tps_fetching: {=bool:?}, tps_reading: {=u8:?}, tps_suspended: {=u8:?}, tps_waiting: {=bool:?}, tps_timestamp_wr: {=bool:?} }}",
                self.axwhsts(),
                self.rps(),
                self.rps_fetching(),
                self.rps_transferring(),
                self.rps_waiting(),
                self.rps_timestamp_wr(),
                self.rps_suspended(),
                self.tps(),
                self.tps_closing(),
                self.tps_fetching(),
                self.tps_reading(),
                self.tps_suspended(),
                self.tps_waiting(),
                self.tps_timestamp_wr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmaisr(pub u32);
    impl EthDmaisr {
        #[must_use]
        #[inline(always)]
        pub const fn dmacis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dmacis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mtlis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mtlis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn macis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_macis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for EthDmaisr {
        #[inline(always)]
        fn default() -> EthDmaisr {
            EthDmaisr(0)
        }
    }
    impl core::fmt::Debug for EthDmaisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmaisr")
                .field("dmacis", &self.dmacis())
                .field("mtlis", &self.mtlis())
                .field("macis", &self.macis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmaisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmaisr {{ dmacis: {=bool:?}, mtlis: {=bool:?}, macis: {=bool:?} }}",
                self.dmacis(),
                self.mtlis(),
                self.macis()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmamr(pub u32);
    impl EthDmamr {
        #[must_use]
        #[inline(always)]
        pub const fn swr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_swr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn da(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_da(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txpr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn intm(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_intm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for EthDmamr {
        #[inline(always)]
        fn default() -> EthDmamr {
            EthDmamr(0)
        }
    }
    impl core::fmt::Debug for EthDmamr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmamr")
                .field("swr", &self.swr())
                .field("da", &self.da())
                .field("txpr", &self.txpr())
                .field("pr", &self.pr())
                .field("intm", &self.intm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmamr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmamr {{ swr: {=bool:?}, da: {=bool:?}, txpr: {=bool:?}, pr: {=u8:?}, intm: {=u8:?} }}",
                self.swr(),
                self.da(),
                self.txpr(),
                self.pr(),
                self.intm()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthDmasbmr(pub u32);
    impl EthDmasbmr {
        #[must_use]
        #[inline(always)]
        pub const fn fb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aal(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mb(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for EthDmasbmr {
        #[inline(always)]
        fn default() -> EthDmasbmr {
            EthDmasbmr(0)
        }
    }
    impl core::fmt::Debug for EthDmasbmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthDmasbmr")
                .field("fb", &self.fb())
                .field("aal", &self.aal())
                .field("mb", &self.mb())
                .field("rb", &self.rb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthDmasbmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthDmasbmr {{ fb: {=bool:?}, aal: {=bool:?}, mb: {=bool:?}, rb: {=bool:?} }}",
                self.fb(),
                self.aal(),
                self.mb(),
                self.rb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMac1ustcr(pub u32);
    impl EthMac1ustcr {
        #[must_use]
        #[inline(always)]
        pub const fn tic1uscntr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_tic1uscntr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for EthMac1ustcr {
        #[inline(always)]
        fn default() -> EthMac1ustcr {
            EthMac1ustcr(0)
        }
    }
    impl core::fmt::Debug for EthMac1ustcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMac1ustcr")
                .field("tic1uscntr", &self.tic1uscntr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMac1ustcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMac1ustcr {{ tic1uscntr: {=u16:?} }}",
                self.tic1uscntr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaca0hr(pub u32);
    impl EthMaca0hr {
        #[must_use]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMaca0hr {
        #[inline(always)]
        fn default() -> EthMaca0hr {
            EthMaca0hr(0)
        }
    }
    impl core::fmt::Debug for EthMaca0hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaca0hr")
                .field("addrhi", &self.addrhi())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaca0hr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaca0hr {{ addrhi: {=u16:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.ae()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaca0lr(pub u32);
    impl EthMaca0lr {
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMaca0lr {
        #[inline(always)]
        fn default() -> EthMaca0lr {
            EthMaca0lr(0)
        }
    }
    impl core::fmt::Debug for EthMaca0lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaca0lr")
                .field("addrlo", &self.addrlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaca0lr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMaca0lr {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaca1hr(pub u32);
    impl EthMaca1hr {
        #[must_use]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMaca1hr {
        #[inline(always)]
        fn default() -> EthMaca1hr {
            EthMaca1hr(0)
        }
    }
    impl core::fmt::Debug for EthMaca1hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaca1hr")
                .field("addrhi", &self.addrhi())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaca1hr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaca1hr {{ addrhi: {=u16:?}, mbc: {=u8:?}, sa: {=bool:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.mbc(),
                self.sa(),
                self.ae()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaca1lr(pub u32);
    impl EthMaca1lr {
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMaca1lr {
        #[inline(always)]
        fn default() -> EthMaca1lr {
            EthMaca1lr(0)
        }
    }
    impl core::fmt::Debug for EthMaca1lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaca1lr")
                .field("addrlo", &self.addrlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaca1lr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMaca1lr {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaca2hr(pub u32);
    impl EthMaca2hr {
        #[must_use]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMaca2hr {
        #[inline(always)]
        fn default() -> EthMaca2hr {
            EthMaca2hr(0)
        }
    }
    impl core::fmt::Debug for EthMaca2hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaca2hr")
                .field("addrhi", &self.addrhi())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaca2hr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaca2hr {{ addrhi: {=u16:?}, mbc: {=u8:?}, sa: {=bool:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.mbc(),
                self.sa(),
                self.ae()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaca2lr(pub u32);
    impl EthMaca2lr {
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMaca2lr {
        #[inline(always)]
        fn default() -> EthMaca2lr {
            EthMaca2lr(0)
        }
    }
    impl core::fmt::Debug for EthMaca2lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaca2lr")
                .field("addrlo", &self.addrlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaca2lr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMaca2lr {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaca3hr(pub u32);
    impl EthMaca3hr {
        #[must_use]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMaca3hr {
        #[inline(always)]
        fn default() -> EthMaca3hr {
            EthMaca3hr(0)
        }
    }
    impl core::fmt::Debug for EthMaca3hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaca3hr")
                .field("addrhi", &self.addrhi())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaca3hr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaca3hr {{ addrhi: {=u16:?}, mbc: {=u8:?}, sa: {=bool:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.mbc(),
                self.sa(),
                self.ae()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaca3lr(pub u32);
    impl EthMaca3lr {
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMaca3lr {
        #[inline(always)]
        fn default() -> EthMaca3lr {
            EthMaca3lr(0)
        }
    }
    impl core::fmt::Debug for EthMaca3lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaca3lr")
                .field("addrlo", &self.addrlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaca3lr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMaca3lr {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacacr(pub u32);
    impl EthMacacr {
        #[must_use]
        #[inline(always)]
        pub const fn atsfc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_atsfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atsen0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_atsen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atsen1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_atsen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atsen2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_atsen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atsen3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_atsen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for EthMacacr {
        #[inline(always)]
        fn default() -> EthMacacr {
            EthMacacr(0)
        }
    }
    impl core::fmt::Debug for EthMacacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacacr")
                .field("atsfc", &self.atsfc())
                .field("atsen0", &self.atsen0())
                .field("atsen1", &self.atsen1())
                .field("atsen2", &self.atsen2())
                .field("atsen3", &self.atsen3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacacr {{ atsfc: {=bool:?}, atsen0: {=bool:?}, atsen1: {=bool:?}, atsen2: {=bool:?}, atsen3: {=bool:?} }}",
                self.atsfc(),
                self.atsen0(),
                self.atsen1(),
                self.atsen2(),
                self.atsen3()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacarpar(pub u32);
    impl EthMacarpar {
        #[must_use]
        #[inline(always)]
        pub const fn arppa(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_arppa(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacarpar {
        #[inline(always)]
        fn default() -> EthMacarpar {
            EthMacarpar(0)
        }
    }
    impl core::fmt::Debug for EthMacarpar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacarpar")
                .field("arppa", &self.arppa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacarpar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacarpar {{ arppa: {=u32:?} }}", self.arppa())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacatsnr(pub u32);
    impl EthMacatsnr {
        #[must_use]
        #[inline(always)]
        pub const fn auxtslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_auxtslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for EthMacatsnr {
        #[inline(always)]
        fn default() -> EthMacatsnr {
            EthMacatsnr(0)
        }
    }
    impl core::fmt::Debug for EthMacatsnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacatsnr")
                .field("auxtslo", &self.auxtslo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacatsnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacatsnr {{ auxtslo: {=u32:?} }}", self.auxtslo())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacatssr(pub u32);
    impl EthMacatssr {
        #[must_use]
        #[inline(always)]
        pub const fn auxtshi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_auxtshi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacatssr {
        #[inline(always)]
        fn default() -> EthMacatssr {
            EthMacatssr(0)
        }
    }
    impl core::fmt::Debug for EthMacatssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacatssr")
                .field("auxtshi", &self.auxtshi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacatssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacatssr {{ auxtshi: {=u32:?} }}", self.auxtshi())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaccr(pub u32);
    impl EthMaccr {
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn prelen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_prelen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn do_(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_do_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ecrsfd(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ecrsfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dm(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fes(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn je(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_je(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn jd(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_jd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn wd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn acs(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_acs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn s2kp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_s2kp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gpslce(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gpslce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ipg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ipg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ipc(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ipc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sarc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sarc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sarc_repaddr0(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sarc_repaddr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sarc_repaddr1(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sarc_repaddr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sarc_insaddr0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sarc_insaddr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sarc_insaddr1(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sarc_insaddr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn arp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_arp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMaccr {
        #[inline(always)]
        fn default() -> EthMaccr {
            EthMaccr(0)
        }
    }
    impl core::fmt::Debug for EthMaccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaccr")
                .field("re", &self.re())
                .field("te", &self.te())
                .field("prelen", &self.prelen())
                .field("dc", &self.dc())
                .field("bl", &self.bl())
                .field("dr", &self.dr())
                .field("dcrs", &self.dcrs())
                .field("do_", &self.do_())
                .field("ecrsfd", &self.ecrsfd())
                .field("lm", &self.lm())
                .field("dm", &self.dm())
                .field("fes", &self.fes())
                .field("je", &self.je())
                .field("jd", &self.jd())
                .field("wd", &self.wd())
                .field("acs", &self.acs())
                .field("cst", &self.cst())
                .field("s2kp", &self.s2kp())
                .field("gpslce", &self.gpslce())
                .field("ipg", &self.ipg())
                .field("ipc", &self.ipc())
                .field("sarc", &self.sarc())
                .field("sarc_repaddr0", &self.sarc_repaddr0())
                .field("sarc_repaddr1", &self.sarc_repaddr1())
                .field("sarc_insaddr0", &self.sarc_insaddr0())
                .field("sarc_insaddr1", &self.sarc_insaddr1())
                .field("arp", &self.arp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaccr {{ re: {=bool:?}, te: {=bool:?}, prelen: {=u8:?}, dc: {=bool:?}, bl: {=u8:?}, dr: {=bool:?}, dcrs: {=bool:?}, do_: {=bool:?}, ecrsfd: {=bool:?}, lm: {=bool:?}, dm: {=bool:?}, fes: {=bool:?}, je: {=bool:?}, jd: {=bool:?}, wd: {=bool:?}, acs: {=bool:?}, cst: {=bool:?}, s2kp: {=bool:?}, gpslce: {=bool:?}, ipg: {=u8:?}, ipc: {=bool:?}, sarc: {=u8:?}, sarc_repaddr0: {=u8:?}, sarc_repaddr1: {=u8:?}, sarc_insaddr0: {=bool:?}, sarc_insaddr1: {=u8:?}, arp: {=bool:?} }}",
                self.re(),
                self.te(),
                self.prelen(),
                self.dc(),
                self.bl(),
                self.dr(),
                self.dcrs(),
                self.do_(),
                self.ecrsfd(),
                self.lm(),
                self.dm(),
                self.fes(),
                self.je(),
                self.jd(),
                self.wd(),
                self.acs(),
                self.cst(),
                self.s2kp(),
                self.gpslce(),
                self.ipg(),
                self.ipc(),
                self.sarc(),
                self.sarc_repaddr0(),
                self.sarc_repaddr1(),
                self.sarc_insaddr0(),
                self.sarc_insaddr1(),
                self.arp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaccsrswcr(pub u32);
    impl EthMaccsrswcr {
        #[must_use]
        #[inline(always)]
        pub const fn rcwe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn seen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_seen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for EthMaccsrswcr {
        #[inline(always)]
        fn default() -> EthMaccsrswcr {
            EthMaccsrswcr(0)
        }
    }
    impl core::fmt::Debug for EthMaccsrswcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaccsrswcr")
                .field("rcwe", &self.rcwe())
                .field("seen", &self.seen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaccsrswcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaccsrswcr {{ rcwe: {=bool:?}, seen: {=bool:?} }}",
                self.rcwe(),
                self.seen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacdr(pub u32);
    impl EthMacdr {
        #[must_use]
        #[inline(always)]
        pub const fn rpests(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rpests(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfcfcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rfcfcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tpests(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tpests(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfcsts(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tfcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfcsts_trasferip(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tfcsts_trasferip(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfcsts_wait(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tfcsts_wait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfcsts_generatepcp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tfcsts_generatepcp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for EthMacdr {
        #[inline(always)]
        fn default() -> EthMacdr {
            EthMacdr(0)
        }
    }
    impl core::fmt::Debug for EthMacdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacdr")
                .field("rpests", &self.rpests())
                .field("rfcfcsts", &self.rfcfcsts())
                .field("tpests", &self.tpests())
                .field("tfcsts", &self.tfcsts())
                .field("tfcsts_trasferip", &self.tfcsts_trasferip())
                .field("tfcsts_wait", &self.tfcsts_wait())
                .field("tfcsts_generatepcp", &self.tfcsts_generatepcp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacdr {{ rpests: {=bool:?}, rfcfcsts: {=u8:?}, tpests: {=bool:?}, tfcsts: {=u8:?}, tfcsts_trasferip: {=u8:?}, tfcsts_wait: {=bool:?}, tfcsts_generatepcp: {=bool:?} }}",
                self.rpests(),
                self.rfcfcsts(),
                self.tpests(),
                self.tfcsts(),
                self.tfcsts_trasferip(),
                self.tfcsts_wait(),
                self.tfcsts_generatepcp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacecr(pub u32);
    impl EthMacecr {
        #[must_use]
        #[inline(always)]
        pub const fn gpsl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_gpsl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcrcc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcrcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn spen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_spen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eipgen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eipgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eipg(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eipg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apdim(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_apdim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for EthMacecr {
        #[inline(always)]
        fn default() -> EthMacecr {
            EthMacecr(0)
        }
    }
    impl core::fmt::Debug for EthMacecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacecr")
                .field("gpsl", &self.gpsl())
                .field("dcrcc", &self.dcrcc())
                .field("spen", &self.spen())
                .field("usp", &self.usp())
                .field("eipgen", &self.eipgen())
                .field("eipg", &self.eipg())
                .field("apdim", &self.apdim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacecr {{ gpsl: {=u16:?}, dcrcc: {=bool:?}, spen: {=bool:?}, usp: {=bool:?}, eipgen: {=bool:?}, eipg: {=u8:?}, apdim: {=bool:?} }}",
                self.gpsl(),
                self.dcrcc(),
                self.spen(),
                self.usp(),
                self.eipgen(),
                self.eipg(),
                self.apdim()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacfpecsr(pub u32);
    impl EthMacfpecsr {
        #[must_use]
        #[inline(always)]
        pub const fn efpe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_efpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sver(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sver(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn srsp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_srsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn arv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_arv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rver(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rver(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrsp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rrsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tver(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tver(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trsp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_trsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for EthMacfpecsr {
        #[inline(always)]
        fn default() -> EthMacfpecsr {
            EthMacfpecsr(0)
        }
    }
    impl core::fmt::Debug for EthMacfpecsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacfpecsr")
                .field("efpe", &self.efpe())
                .field("sver", &self.sver())
                .field("srsp", &self.srsp())
                .field("arv", &self.arv())
                .field("rver", &self.rver())
                .field("rrsp", &self.rrsp())
                .field("tver", &self.tver())
                .field("trsp", &self.trsp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacfpecsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacfpecsr {{ efpe: {=bool:?}, sver: {=bool:?}, srsp: {=bool:?}, arv: {=bool:?}, rver: {=bool:?}, rrsp: {=bool:?}, tver: {=bool:?}, trsp: {=bool:?} }}",
                self.efpe(),
                self.sver(),
                self.srsp(),
                self.arv(),
                self.rver(),
                self.rrsp(),
                self.tver(),
                self.trsp()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMachwf0r(pub u32);
    impl EthMachwf0r {
        #[must_use]
        #[inline(always)]
        pub const fn miisel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_miisel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn gmiisel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_gmiisel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pcssel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pcssel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlhash(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlhash(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn smasel(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_smasel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwksel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mgksel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mgksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mmcsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mmcsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn arpoffsel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_arpoffsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tssel(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tssel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eeesel(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eeesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txcoesel(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txcoesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxcoesel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxcoesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn addmacadrsel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_addmacadrsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn macadr32sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_macadr32sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn macadr64sel(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_macadr64sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsstssel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tsstssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsstssel_both(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tsstssel_both(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsstssel_internal(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsstssel_internal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsstssel_external(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsstssel_external(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn savlanins(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_savlanins(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn actphysel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_actphysel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn actphysel_rgmii(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_actphysel_rgmii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn actphysel_tbi(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_actphysel_tbi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn actphysel_sgmii(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_actphysel_sgmii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn actphysel_smii(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_actphysel_smii(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn actphysel_rmii(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_actphysel_rmii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for EthMachwf0r {
        #[inline(always)]
        fn default() -> EthMachwf0r {
            EthMachwf0r(0)
        }
    }
    impl core::fmt::Debug for EthMachwf0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMachwf0r")
                .field("miisel", &self.miisel())
                .field("gmiisel", &self.gmiisel())
                .field("hdsel", &self.hdsel())
                .field("pcssel", &self.pcssel())
                .field("vlhash", &self.vlhash())
                .field("smasel", &self.smasel())
                .field("rwksel", &self.rwksel())
                .field("mgksel", &self.mgksel())
                .field("mmcsel", &self.mmcsel())
                .field("arpoffsel", &self.arpoffsel())
                .field("tssel", &self.tssel())
                .field("eeesel", &self.eeesel())
                .field("txcoesel", &self.txcoesel())
                .field("rxcoesel", &self.rxcoesel())
                .field("addmacadrsel", &self.addmacadrsel())
                .field("macadr32sel", &self.macadr32sel())
                .field("macadr64sel", &self.macadr64sel())
                .field("tsstssel", &self.tsstssel())
                .field("tsstssel_both", &self.tsstssel_both())
                .field("tsstssel_internal", &self.tsstssel_internal())
                .field("tsstssel_external", &self.tsstssel_external())
                .field("savlanins", &self.savlanins())
                .field("actphysel", &self.actphysel())
                .field("actphysel_rgmii", &self.actphysel_rgmii())
                .field("actphysel_tbi", &self.actphysel_tbi())
                .field("actphysel_sgmii", &self.actphysel_sgmii())
                .field("actphysel_smii", &self.actphysel_smii())
                .field("actphysel_rmii", &self.actphysel_rmii())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMachwf0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMachwf0r {{ miisel: {=bool:?}, gmiisel: {=bool:?}, hdsel: {=bool:?}, pcssel: {=bool:?}, vlhash: {=bool:?}, smasel: {=bool:?}, rwksel: {=bool:?}, mgksel: {=bool:?}, mmcsel: {=bool:?}, arpoffsel: {=bool:?}, tssel: {=bool:?}, eeesel: {=bool:?}, txcoesel: {=bool:?}, rxcoesel: {=bool:?}, addmacadrsel: {=u8:?}, macadr32sel: {=bool:?}, macadr64sel: {=bool:?}, tsstssel: {=u8:?}, tsstssel_both: {=u8:?}, tsstssel_internal: {=bool:?}, tsstssel_external: {=bool:?}, savlanins: {=bool:?}, actphysel: {=u8:?}, actphysel_rgmii: {=bool:?}, actphysel_tbi: {=u8:?}, actphysel_sgmii: {=bool:?}, actphysel_smii: {=u8:?}, actphysel_rmii: {=bool:?} }}",
                self.miisel(),
                self.gmiisel(),
                self.hdsel(),
                self.pcssel(),
                self.vlhash(),
                self.smasel(),
                self.rwksel(),
                self.mgksel(),
                self.mmcsel(),
                self.arpoffsel(),
                self.tssel(),
                self.eeesel(),
                self.txcoesel(),
                self.rxcoesel(),
                self.addmacadrsel(),
                self.macadr32sel(),
                self.macadr64sel(),
                self.tsstssel(),
                self.tsstssel_both(),
                self.tsstssel_internal(),
                self.tsstssel_external(),
                self.savlanins(),
                self.actphysel(),
                self.actphysel_rgmii(),
                self.actphysel_tbi(),
                self.actphysel_sgmii(),
                self.actphysel_smii(),
                self.actphysel_rmii()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMachwf1r(pub u32);
    impl EthMachwf1r {
        #[must_use]
        #[inline(always)]
        pub const fn rxfifosize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxfifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn spram(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_spram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txfifosize(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_txfifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn osten(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_osten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ptoen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ptoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn advthword(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_advthword(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn addr64(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_addr64(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dcben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dcben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sphen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sphen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsoen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbgmema(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbgmema(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn avsel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_avsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ravsel(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ravsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pouost(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pouost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hashtblsz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hashtblsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hashtblsz_256(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_hashtblsz_256(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hashtblsz_64(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hashtblsz_64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hashtblsz_128(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hashtblsz_128(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn l3l4fnum(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_l3l4fnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
    }
    impl Default for EthMachwf1r {
        #[inline(always)]
        fn default() -> EthMachwf1r {
            EthMachwf1r(0)
        }
    }
    impl core::fmt::Debug for EthMachwf1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMachwf1r")
                .field("rxfifosize", &self.rxfifosize())
                .field("spram", &self.spram())
                .field("txfifosize", &self.txfifosize())
                .field("osten", &self.osten())
                .field("ptoen", &self.ptoen())
                .field("advthword", &self.advthword())
                .field("addr64", &self.addr64())
                .field("dcben", &self.dcben())
                .field("sphen", &self.sphen())
                .field("tsoen", &self.tsoen())
                .field("dbgmema", &self.dbgmema())
                .field("avsel", &self.avsel())
                .field("ravsel", &self.ravsel())
                .field("pouost", &self.pouost())
                .field("hashtblsz", &self.hashtblsz())
                .field("hashtblsz_256", &self.hashtblsz_256())
                .field("hashtblsz_64", &self.hashtblsz_64())
                .field("hashtblsz_128", &self.hashtblsz_128())
                .field("l3l4fnum", &self.l3l4fnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMachwf1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMachwf1r {{ rxfifosize: {=u8:?}, spram: {=bool:?}, txfifosize: {=u8:?}, osten: {=bool:?}, ptoen: {=bool:?}, advthword: {=bool:?}, addr64: {=u8:?}, dcben: {=bool:?}, sphen: {=bool:?}, tsoen: {=bool:?}, dbgmema: {=bool:?}, avsel: {=bool:?}, ravsel: {=bool:?}, pouost: {=bool:?}, hashtblsz: {=u8:?}, hashtblsz_256: {=u8:?}, hashtblsz_64: {=bool:?}, hashtblsz_128: {=bool:?}, l3l4fnum: {=u8:?} }}",
                self.rxfifosize(),
                self.spram(),
                self.txfifosize(),
                self.osten(),
                self.ptoen(),
                self.advthword(),
                self.addr64(),
                self.dcben(),
                self.sphen(),
                self.tsoen(),
                self.dbgmema(),
                self.avsel(),
                self.ravsel(),
                self.pouost(),
                self.hashtblsz(),
                self.hashtblsz_256(),
                self.hashtblsz_64(),
                self.hashtblsz_128(),
                self.l3l4fnum()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMachwf2r(pub u32);
    impl EthMachwf2r {
        #[must_use]
        #[inline(always)]
        pub const fn rxqcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxqcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxqcnt_2mtlrxq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxqcnt_2mtlrxq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txqcnt(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_txqcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txqcnt_2mtltxq(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txqcnt_2mtltxq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxchcnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxchcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxchcnt_2dmarxch(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxchcnt_2dmarxch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rdcsz(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rdcsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txchcnt(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_txchcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txchcnt_2dmatxch(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txchcnt_2dmatxch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tdcsz(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tdcsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ppsoutnum(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ppsoutnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn auxsnapnum(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_auxsnapnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for EthMachwf2r {
        #[inline(always)]
        fn default() -> EthMachwf2r {
            EthMachwf2r(0)
        }
    }
    impl core::fmt::Debug for EthMachwf2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMachwf2r")
                .field("rxqcnt", &self.rxqcnt())
                .field("rxqcnt_2mtlrxq", &self.rxqcnt_2mtlrxq())
                .field("txqcnt", &self.txqcnt())
                .field("txqcnt_2mtltxq", &self.txqcnt_2mtltxq())
                .field("rxchcnt", &self.rxchcnt())
                .field("rxchcnt_2dmarxch", &self.rxchcnt_2dmarxch())
                .field("rdcsz", &self.rdcsz())
                .field("txchcnt", &self.txchcnt())
                .field("txchcnt_2dmatxch", &self.txchcnt_2dmatxch())
                .field("tdcsz", &self.tdcsz())
                .field("ppsoutnum", &self.ppsoutnum())
                .field("auxsnapnum", &self.auxsnapnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMachwf2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMachwf2r {{ rxqcnt: {=u8:?}, rxqcnt_2mtlrxq: {=bool:?}, txqcnt: {=u8:?}, txqcnt_2mtltxq: {=bool:?}, rxchcnt: {=u8:?}, rxchcnt_2dmarxch: {=bool:?}, rdcsz: {=u8:?}, txchcnt: {=u8:?}, txchcnt_2dmatxch: {=bool:?}, tdcsz: {=u8:?}, ppsoutnum: {=u8:?}, auxsnapnum: {=u8:?} }}",
                self.rxqcnt(),
                self.rxqcnt_2mtlrxq(),
                self.txqcnt(),
                self.txqcnt_2mtltxq(),
                self.rxchcnt(),
                self.rxchcnt_2dmarxch(),
                self.rdcsz(),
                self.txchcnt(),
                self.txchcnt_2dmatxch(),
                self.tdcsz(),
                self.ppsoutnum(),
                self.auxsnapnum()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMachwf3r(pub u32);
    impl EthMachwf3r {
        #[must_use]
        #[inline(always)]
        pub const fn nrvf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nrvf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nrvf_16exrxvlanfltr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_nrvf_16exrxvlanfltr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nrvf_4exrxvlanfltr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nrvf_4exrxvlanfltr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nrvf_8exrxvlanfltr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nrvf_8exrxvlanfltr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nrvf_24exrxvlanfltr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nrvf_24exrxvlanfltr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cbtisel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cbtisel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dvlan(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dvlan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pdupsel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pdupsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frpsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frpsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frpbs(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_frpbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frpbs_128(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frpbs_128(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frpbs_256(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frpbs_256(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frpes(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_frpes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frpes_128(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frpes_128(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn frpes_256(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_frpes_256(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estsel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_estsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estdep(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_estdep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estdep_256(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_estdep_256(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estdep_64(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_estdep_64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estdep_128(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_estdep_128(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estdep_512(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_estdep_512(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estwid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_estwid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estwid_16(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_estwid_16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estwid_24(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_estwid_24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn estwid_20(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_estwid_20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fpesel(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fpesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tbssel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tbssel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asp(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_asp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asp_asppe(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_asp_asppe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asp_ecc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_asp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asp_asnppe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_asp_asnppe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for EthMachwf3r {
        #[inline(always)]
        fn default() -> EthMachwf3r {
            EthMachwf3r(0)
        }
    }
    impl core::fmt::Debug for EthMachwf3r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMachwf3r")
                .field("nrvf", &self.nrvf())
                .field("nrvf_16exrxvlanfltr", &self.nrvf_16exrxvlanfltr())
                .field("nrvf_4exrxvlanfltr", &self.nrvf_4exrxvlanfltr())
                .field("nrvf_8exrxvlanfltr", &self.nrvf_8exrxvlanfltr())
                .field("nrvf_24exrxvlanfltr", &self.nrvf_24exrxvlanfltr())
                .field("cbtisel", &self.cbtisel())
                .field("dvlan", &self.dvlan())
                .field("pdupsel", &self.pdupsel())
                .field("frpsel", &self.frpsel())
                .field("frpbs", &self.frpbs())
                .field("frpbs_128", &self.frpbs_128())
                .field("frpbs_256", &self.frpbs_256())
                .field("frpes", &self.frpes())
                .field("frpes_128", &self.frpes_128())
                .field("frpes_256", &self.frpes_256())
                .field("estsel", &self.estsel())
                .field("estdep", &self.estdep())
                .field("estdep_256", &self.estdep_256())
                .field("estdep_64", &self.estdep_64())
                .field("estdep_128", &self.estdep_128())
                .field("estdep_512", &self.estdep_512())
                .field("estwid", &self.estwid())
                .field("estwid_16", &self.estwid_16())
                .field("estwid_24", &self.estwid_24())
                .field("estwid_20", &self.estwid_20())
                .field("fpesel", &self.fpesel())
                .field("tbssel", &self.tbssel())
                .field("asp", &self.asp())
                .field("asp_asppe", &self.asp_asppe())
                .field("asp_ecc", &self.asp_ecc())
                .field("asp_asnppe", &self.asp_asnppe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMachwf3r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMachwf3r {{ nrvf: {=u8:?}, nrvf_16exrxvlanfltr: {=u8:?}, nrvf_4exrxvlanfltr: {=bool:?}, nrvf_8exrxvlanfltr: {=bool:?}, nrvf_24exrxvlanfltr: {=bool:?}, cbtisel: {=bool:?}, dvlan: {=bool:?}, pdupsel: {=bool:?}, frpsel: {=bool:?}, frpbs: {=u8:?}, frpbs_128: {=bool:?}, frpbs_256: {=bool:?}, frpes: {=u8:?}, frpes_128: {=bool:?}, frpes_256: {=bool:?}, estsel: {=bool:?}, estdep: {=u8:?}, estdep_256: {=u8:?}, estdep_64: {=bool:?}, estdep_128: {=bool:?}, estdep_512: {=bool:?}, estwid: {=u8:?}, estwid_16: {=bool:?}, estwid_24: {=u8:?}, estwid_20: {=bool:?}, fpesel: {=bool:?}, tbssel: {=bool:?}, asp: {=u8:?}, asp_asppe: {=u8:?}, asp_ecc: {=bool:?}, asp_asnppe: {=bool:?} }}",
                self.nrvf(),
                self.nrvf_16exrxvlanfltr(),
                self.nrvf_4exrxvlanfltr(),
                self.nrvf_8exrxvlanfltr(),
                self.nrvf_24exrxvlanfltr(),
                self.cbtisel(),
                self.dvlan(),
                self.pdupsel(),
                self.frpsel(),
                self.frpbs(),
                self.frpbs_128(),
                self.frpbs_256(),
                self.frpes(),
                self.frpes_128(),
                self.frpes_256(),
                self.estsel(),
                self.estdep(),
                self.estdep_256(),
                self.estdep_64(),
                self.estdep_128(),
                self.estdep_512(),
                self.estwid(),
                self.estwid_16(),
                self.estwid_24(),
                self.estwid_20(),
                self.fpesel(),
                self.tbssel(),
                self.asp(),
                self.asp_asppe(),
                self.asp_ecc(),
                self.asp_asnppe()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaciacr(pub u32);
    impl EthMaciacr {
        #[must_use]
        #[inline(always)]
        pub const fn ob(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ob(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn com(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_com(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn auto(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aoff(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_aoff(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aoff_ind_reg1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aoff_ind_reg1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aoff_ind_reg3(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_aoff_ind_reg3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aoff_ind_reg7(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_aoff_ind_reg7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aoff_ind_reg2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aoff_ind_reg2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aoff_ind_reg6(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_aoff_ind_reg6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aoff_ind_reg4(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aoff_ind_reg4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_msel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for EthMaciacr {
        #[inline(always)]
        fn default() -> EthMaciacr {
            EthMaciacr(0)
        }
    }
    impl core::fmt::Debug for EthMaciacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaciacr")
                .field("ob", &self.ob())
                .field("com", &self.com())
                .field("auto", &self.auto())
                .field("aoff", &self.aoff())
                .field("aoff_ind_reg1", &self.aoff_ind_reg1())
                .field("aoff_ind_reg3", &self.aoff_ind_reg3())
                .field("aoff_ind_reg7", &self.aoff_ind_reg7())
                .field("aoff_ind_reg2", &self.aoff_ind_reg2())
                .field("aoff_ind_reg6", &self.aoff_ind_reg6())
                .field("aoff_ind_reg4", &self.aoff_ind_reg4())
                .field("msel", &self.msel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaciacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaciacr {{ ob: {=bool:?}, com: {=bool:?}, auto: {=bool:?}, aoff: {=u8:?}, aoff_ind_reg1: {=bool:?}, aoff_ind_reg3: {=u8:?}, aoff_ind_reg7: {=u8:?}, aoff_ind_reg2: {=bool:?}, aoff_ind_reg6: {=u8:?}, aoff_ind_reg4: {=bool:?}, msel: {=u8:?} }}",
                self.ob(),
                self.com(),
                self.auto(),
                self.aoff(),
                self.aoff_ind_reg1(),
                self.aoff_ind_reg3(),
                self.aoff_ind_reg7(),
                self.aoff_ind_reg2(),
                self.aoff_ind_reg6(),
                self.aoff_ind_reg4(),
                self.msel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacier(pub u32);
    impl EthMacier {
        #[must_use]
        #[inline(always)]
        pub const fn phyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_phyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pmtie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pmtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpiie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txstsie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txstsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxstsie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxstsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fpeie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mdioie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mdioie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for EthMacier {
        #[inline(always)]
        fn default() -> EthMacier {
            EthMacier(0)
        }
    }
    impl core::fmt::Debug for EthMacier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacier")
                .field("phyie", &self.phyie())
                .field("pmtie", &self.pmtie())
                .field("lpiie", &self.lpiie())
                .field("tsie", &self.tsie())
                .field("txstsie", &self.txstsie())
                .field("rxstsie", &self.rxstsie())
                .field("fpeie", &self.fpeie())
                .field("mdioie", &self.mdioie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacier {{ phyie: {=bool:?}, pmtie: {=bool:?}, lpiie: {=bool:?}, tsie: {=bool:?}, txstsie: {=bool:?}, rxstsie: {=bool:?}, fpeie: {=bool:?}, mdioie: {=bool:?} }}",
                self.phyie(),
                self.pmtie(),
                self.lpiie(),
                self.tsie(),
                self.txstsie(),
                self.rxstsie(),
                self.fpeie(),
                self.mdioie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacisr(pub u32);
    impl EthMacisr {
        #[must_use]
        #[inline(always)]
        pub const fn phyis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_phyis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pmtis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pmtis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpiis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpiis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mmcis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mmcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mmcrxis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mmcrxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mmctxis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mmctxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txstsis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txstsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxstsis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxstsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fpeie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mdioie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mdioie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mftis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mftis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mfris(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mfris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for EthMacisr {
        #[inline(always)]
        fn default() -> EthMacisr {
            EthMacisr(0)
        }
    }
    impl core::fmt::Debug for EthMacisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacisr")
                .field("phyis", &self.phyis())
                .field("pmtis", &self.pmtis())
                .field("lpiis", &self.lpiis())
                .field("mmcis", &self.mmcis())
                .field("mmcrxis", &self.mmcrxis())
                .field("mmctxis", &self.mmctxis())
                .field("tsis", &self.tsis())
                .field("txstsis", &self.txstsis())
                .field("rxstsis", &self.rxstsis())
                .field("fpeie", &self.fpeie())
                .field("mdioie", &self.mdioie())
                .field("mftis", &self.mftis())
                .field("mfris", &self.mfris())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacisr {{ phyis: {=bool:?}, pmtis: {=bool:?}, lpiis: {=bool:?}, mmcis: {=bool:?}, mmcrxis: {=bool:?}, mmctxis: {=bool:?}, tsis: {=bool:?}, txstsis: {=bool:?}, rxstsis: {=bool:?}, fpeie: {=bool:?}, mdioie: {=bool:?}, mftis: {=bool:?}, mfris: {=bool:?} }}",
                self.phyis(),
                self.pmtis(),
                self.lpiis(),
                self.mmcis(),
                self.mmcrxis(),
                self.mmctxis(),
                self.tsis(),
                self.txstsis(),
                self.rxstsis(),
                self.fpeie(),
                self.mdioie(),
                self.mftis(),
                self.mfris()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacivir(pub u32);
    impl EthMacivir {
        #[must_use]
        #[inline(always)]
        pub const fn vlt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vlt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlt_vid(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vlt_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlt_cfidei(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlt_cfidei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlt_up(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vlt_up(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlc_vlantagdelete(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlc_vlantagdelete(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlc_vlantagreplace(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vlc_vlantagreplace(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlc_vlantaginsert(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlc_vlantaginsert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn csvl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_csvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlti(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for EthMacivir {
        #[inline(always)]
        fn default() -> EthMacivir {
            EthMacivir(0)
        }
    }
    impl core::fmt::Debug for EthMacivir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacivir")
                .field("vlt", &self.vlt())
                .field("vlt_vid", &self.vlt_vid())
                .field("vlt_cfidei", &self.vlt_cfidei())
                .field("vlt_up", &self.vlt_up())
                .field("vlc", &self.vlc())
                .field("vlc_vlantagdelete", &self.vlc_vlantagdelete())
                .field("vlc_vlantagreplace", &self.vlc_vlantagreplace())
                .field("vlc_vlantaginsert", &self.vlc_vlantaginsert())
                .field("vlp", &self.vlp())
                .field("csvl", &self.csvl())
                .field("vlti", &self.vlti())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacivir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacivir {{ vlt: {=u16:?}, vlt_vid: {=u16:?}, vlt_cfidei: {=bool:?}, vlt_up: {=u8:?}, vlc: {=u8:?}, vlc_vlantagdelete: {=bool:?}, vlc_vlantagreplace: {=u8:?}, vlc_vlantaginsert: {=bool:?}, vlp: {=bool:?}, csvl: {=bool:?}, vlti: {=bool:?} }}",
                self.vlt(),
                self.vlt_vid(),
                self.vlt_cfidei(),
                self.vlt_up(),
                self.vlc(),
                self.vlc_vlantagdelete(),
                self.vlc_vlantagreplace(),
                self.vlc_vlantaginsert(),
                self.vlp(),
                self.csvl(),
                self.vlti()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaclcsr(pub u32);
    impl EthMaclcsr {
        #[must_use]
        #[inline(always)]
        pub const fn tlpien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tlpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tlpiex(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tlpiex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rlpien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rlpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rlpiex(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rlpiex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tlpist(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tlpist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rlpist(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rlpist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpitxa(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpitxa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpite(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpitcse(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpitcse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for EthMaclcsr {
        #[inline(always)]
        fn default() -> EthMaclcsr {
            EthMaclcsr(0)
        }
    }
    impl core::fmt::Debug for EthMaclcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaclcsr")
                .field("tlpien", &self.tlpien())
                .field("tlpiex", &self.tlpiex())
                .field("rlpien", &self.rlpien())
                .field("rlpiex", &self.rlpiex())
                .field("tlpist", &self.tlpist())
                .field("rlpist", &self.rlpist())
                .field("lpien", &self.lpien())
                .field("pls", &self.pls())
                .field("lpitxa", &self.lpitxa())
                .field("lpite", &self.lpite())
                .field("lpitcse", &self.lpitcse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaclcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaclcsr {{ tlpien: {=bool:?}, tlpiex: {=bool:?}, rlpien: {=bool:?}, rlpiex: {=bool:?}, tlpist: {=bool:?}, rlpist: {=bool:?}, lpien: {=bool:?}, pls: {=bool:?}, lpitxa: {=bool:?}, lpite: {=bool:?}, lpitcse: {=bool:?} }}",
                self.tlpien(),
                self.tlpiex(),
                self.rlpien(),
                self.rlpiex(),
                self.tlpist(),
                self.rlpist(),
                self.lpien(),
                self.pls(),
                self.lpitxa(),
                self.lpite(),
                self.lpitcse()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacletr(pub u32);
    impl EthMacletr {
        #[must_use]
        #[inline(always)]
        pub const fn lpiet(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_lpiet(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for EthMacletr {
        #[inline(always)]
        fn default() -> EthMacletr {
            EthMacletr(0)
        }
    }
    impl core::fmt::Debug for EthMacletr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacletr")
                .field("lpiet", &self.lpiet())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacletr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacletr {{ lpiet: {=u32:?} }}", self.lpiet())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMaclmir(pub u32);
    impl EthMaclmir {
        #[must_use]
        #[inline(always)]
        pub const fn lsi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lsi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn drsyncr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_drsyncr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lmpdri(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lmpdri(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for EthMaclmir {
        #[inline(always)]
        fn default() -> EthMaclmir {
            EthMaclmir(0)
        }
    }
    impl core::fmt::Debug for EthMaclmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMaclmir")
                .field("lsi", &self.lsi())
                .field("drsyncr", &self.drsyncr())
                .field("lmpdri", &self.lmpdri())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMaclmir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMaclmir {{ lsi: {=u8:?}, drsyncr: {=u8:?}, lmpdri: {=u8:?} }}",
                self.lsi(),
                self.drsyncr(),
                self.lmpdri()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacltcr(pub u32);
    impl EthMacltcr {
        #[must_use]
        #[inline(always)]
        pub const fn twt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_twt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lst(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_lst(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for EthMacltcr {
        #[inline(always)]
        fn default() -> EthMacltcr {
            EthMacltcr(0)
        }
    }
    impl core::fmt::Debug for EthMacltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacltcr")
                .field("twt", &self.twt())
                .field("lst", &self.lst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacltcr {{ twt: {=u16:?}, lst: {=u16:?} }}",
                self.twt(),
                self.lst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacmdioar(pub u32);
    impl EthMacmdioar {
        #[must_use]
        #[inline(always)]
        pub const fn mb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn c45e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_c45e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn moc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_moc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn moc_rd(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_moc_rd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn moc_wr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_moc_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn moc_prdia(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_moc_prdia(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn skap(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_skap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr_div18ar(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cr_div18ar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr_div26(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cr_div26(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr_div62(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cr_div62(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr_div16(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cr_div16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr_div16ar(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cr_div16ar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr_div102(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cr_div102(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr_div12ar(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cr_div12ar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cr_div4ar(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cr_div4ar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ntc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ntc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rda(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda_pcs_jbtmr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rda_pcs_jbtmr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda_plca_nodecr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rda_plca_nodecr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda_plca_tmr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rda_plca_tmr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda_pcs_diag2(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rda_pcs_diag2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda_plca_sr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rda_plca_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda_pcs_sr(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rda_pcs_sr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda_plca_buf_depth(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rda_plca_buf_depth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rda_pma_tmcr(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rda_pma_tmcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn btb(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_btb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pse(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for EthMacmdioar {
        #[inline(always)]
        fn default() -> EthMacmdioar {
            EthMacmdioar(0)
        }
    }
    impl core::fmt::Debug for EthMacmdioar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacmdioar")
                .field("mb", &self.mb())
                .field("c45e", &self.c45e())
                .field("moc", &self.moc())
                .field("moc_rd", &self.moc_rd())
                .field("moc_wr", &self.moc_wr())
                .field("moc_prdia", &self.moc_prdia())
                .field("skap", &self.skap())
                .field("cr", &self.cr())
                .field("cr_div18ar", &self.cr_div18ar())
                .field("cr_div26", &self.cr_div26())
                .field("cr_div62", &self.cr_div62())
                .field("cr_div16", &self.cr_div16())
                .field("cr_div16ar", &self.cr_div16ar())
                .field("cr_div102", &self.cr_div102())
                .field("cr_div12ar", &self.cr_div12ar())
                .field("cr_div4ar", &self.cr_div4ar())
                .field("ntc", &self.ntc())
                .field("rda", &self.rda())
                .field("rda_pcs_jbtmr", &self.rda_pcs_jbtmr())
                .field("rda_plca_nodecr", &self.rda_plca_nodecr())
                .field("rda_plca_tmr", &self.rda_plca_tmr())
                .field("rda_pcs_diag2", &self.rda_pcs_diag2())
                .field("rda_plca_sr", &self.rda_plca_sr())
                .field("rda_pcs_sr", &self.rda_pcs_sr())
                .field("rda_plca_buf_depth", &self.rda_plca_buf_depth())
                .field("rda_pma_tmcr", &self.rda_pma_tmcr())
                .field("pa", &self.pa())
                .field("btb", &self.btb())
                .field("pse", &self.pse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacmdioar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacmdioar {{ mb: {=bool:?}, c45e: {=bool:?}, moc: {=u8:?}, moc_rd: {=u8:?}, moc_wr: {=bool:?}, moc_prdia: {=bool:?}, skap: {=bool:?}, cr: {=u8:?}, cr_div18ar: {=u8:?}, cr_div26: {=u8:?}, cr_div62: {=bool:?}, cr_div16: {=bool:?}, cr_div16ar: {=u8:?}, cr_div102: {=bool:?}, cr_div12ar: {=u8:?}, cr_div4ar: {=bool:?}, ntc: {=u8:?}, rda: {=u8:?}, rda_pcs_jbtmr: {=u8:?}, rda_plca_nodecr: {=bool:?}, rda_plca_tmr: {=u8:?}, rda_pcs_diag2: {=u8:?}, rda_plca_sr: {=bool:?}, rda_pcs_sr: {=u8:?}, rda_plca_buf_depth: {=bool:?}, rda_pma_tmcr: {=u8:?}, pa: {=u8:?}, btb: {=bool:?}, pse: {=bool:?} }}",
                self.mb(),
                self.c45e(),
                self.moc(),
                self.moc_rd(),
                self.moc_wr(),
                self.moc_prdia(),
                self.skap(),
                self.cr(),
                self.cr_div18ar(),
                self.cr_div26(),
                self.cr_div62(),
                self.cr_div16(),
                self.cr_div16ar(),
                self.cr_div102(),
                self.cr_div12ar(),
                self.cr_div4ar(),
                self.ntc(),
                self.rda(),
                self.rda_pcs_jbtmr(),
                self.rda_plca_nodecr(),
                self.rda_plca_tmr(),
                self.rda_pcs_diag2(),
                self.rda_plca_sr(),
                self.rda_pcs_sr(),
                self.rda_plca_buf_depth(),
                self.rda_pma_tmcr(),
                self.pa(),
                self.btb(),
                self.pse()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacmdiodr(pub u32);
    impl EthMacmdiodr {
        #[must_use]
        #[inline(always)]
        pub const fn md(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_md(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ra(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for EthMacmdiodr {
        #[inline(always)]
        fn default() -> EthMacmdiodr {
            EthMacmdiodr(0)
        }
    }
    impl core::fmt::Debug for EthMacmdiodr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacmdiodr")
                .field("md", &self.md())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacmdiodr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacmdiodr {{ md: {=u16:?}, ra: {=u16:?} }}",
                self.md(),
                self.ra()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacpcsr(pub u32);
    impl EthMacpcsr {
        #[must_use]
        #[inline(always)]
        pub const fn pwrdwn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pwrdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mgkpkten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mgkpkten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwkpkten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwkpkten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mgkprcvd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mgkprcvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwkprcvd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwkprcvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn glblucast(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_glblucast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwkpfe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwkpfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwkptr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rwkptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwkfiltrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwkfiltrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMacpcsr {
        #[inline(always)]
        fn default() -> EthMacpcsr {
            EthMacpcsr(0)
        }
    }
    impl core::fmt::Debug for EthMacpcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacpcsr")
                .field("pwrdwn", &self.pwrdwn())
                .field("mgkpkten", &self.mgkpkten())
                .field("rwkpkten", &self.rwkpkten())
                .field("mgkprcvd", &self.mgkprcvd())
                .field("rwkprcvd", &self.rwkprcvd())
                .field("glblucast", &self.glblucast())
                .field("rwkpfe", &self.rwkpfe())
                .field("rwkptr", &self.rwkptr())
                .field("rwkfiltrst", &self.rwkfiltrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacpcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacpcsr {{ pwrdwn: {=bool:?}, mgkpkten: {=bool:?}, rwkpkten: {=bool:?}, mgkprcvd: {=bool:?}, rwkprcvd: {=bool:?}, glblucast: {=bool:?}, rwkpfe: {=bool:?}, rwkptr: {=u8:?}, rwkfiltrst: {=bool:?} }}",
                self.pwrdwn(),
                self.mgkpkten(),
                self.rwkpkten(),
                self.mgkprcvd(),
                self.rwkprcvd(),
                self.glblucast(),
                self.rwkpfe(),
                self.rwkptr(),
                self.rwkfiltrst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacpfr(pub u32);
    impl EthMacpfr {
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn huc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_huc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hmc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hmc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn daif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_daif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pcf(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pcf_forwardallexceptpa(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pcf_forwardallexceptpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pcf_forwardpassedaddrfilter(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pcf_forwardpassedaddrfilter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pcf_forwardall(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pcf_forwardall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn saif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_saif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hpf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vtfe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vtfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ipfe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ipfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dntu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dntu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMacpfr {
        #[inline(always)]
        fn default() -> EthMacpfr {
            EthMacpfr(0)
        }
    }
    impl core::fmt::Debug for EthMacpfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacpfr")
                .field("pr", &self.pr())
                .field("huc", &self.huc())
                .field("hmc", &self.hmc())
                .field("daif", &self.daif())
                .field("pm", &self.pm())
                .field("dbf", &self.dbf())
                .field("pcf", &self.pcf())
                .field("pcf_forwardallexceptpa", &self.pcf_forwardallexceptpa())
                .field(
                    "pcf_forwardpassedaddrfilter",
                    &self.pcf_forwardpassedaddrfilter(),
                )
                .field("pcf_forwardall", &self.pcf_forwardall())
                .field("saif", &self.saif())
                .field("saf", &self.saf())
                .field("hpf", &self.hpf())
                .field("vtfe", &self.vtfe())
                .field("ipfe", &self.ipfe())
                .field("dntu", &self.dntu())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacpfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacpfr {{ pr: {=bool:?}, huc: {=bool:?}, hmc: {=bool:?}, daif: {=bool:?}, pm: {=bool:?}, dbf: {=bool:?}, pcf: {=u8:?}, pcf_forwardallexceptpa: {=bool:?}, pcf_forwardpassedaddrfilter: {=u8:?}, pcf_forwardall: {=bool:?}, saif: {=bool:?}, saf: {=bool:?}, hpf: {=bool:?}, vtfe: {=bool:?}, ipfe: {=bool:?}, dntu: {=bool:?}, ra: {=bool:?} }}",
                self.pr(),
                self.huc(),
                self.hmc(),
                self.daif(),
                self.pm(),
                self.dbf(),
                self.pcf(),
                self.pcf_forwardallexceptpa(),
                self.pcf_forwardpassedaddrfilter(),
                self.pcf_forwardall(),
                self.saif(),
                self.saf(),
                self.hpf(),
                self.vtfe(),
                self.ipfe(),
                self.dntu(),
                self.ra()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacpocr(pub u32);
    impl EthMacpocr {
        #[must_use]
        #[inline(always)]
        pub const fn ptoen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ptoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asyncen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_asyncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apdreqen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_apdreqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn asynctrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_asynctrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn apdreqtrig(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_apdreqtrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn drrdis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_drrdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pdrdis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pdrdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for EthMacpocr {
        #[inline(always)]
        fn default() -> EthMacpocr {
            EthMacpocr(0)
        }
    }
    impl core::fmt::Debug for EthMacpocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacpocr")
                .field("ptoen", &self.ptoen())
                .field("asyncen", &self.asyncen())
                .field("apdreqen", &self.apdreqen())
                .field("asynctrig", &self.asynctrig())
                .field("apdreqtrig", &self.apdreqtrig())
                .field("drrdis", &self.drrdis())
                .field("pdrdis", &self.pdrdis())
                .field("dn", &self.dn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacpocr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacpocr {{ ptoen: {=bool:?}, asyncen: {=bool:?}, apdreqen: {=bool:?}, asynctrig: {=bool:?}, apdreqtrig: {=bool:?}, drrdis: {=bool:?}, pdrdis: {=bool:?}, dn: {=u8:?} }}",
                self.ptoen(),
                self.asyncen(),
                self.apdreqen(),
                self.asynctrig(),
                self.apdreqtrig(),
                self.drrdis(),
                self.pdrdis(),
                self.dn()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacppscr(pub u32);
    impl EthMacppscr {
        #[must_use]
        #[inline(always)]
        pub const fn ppsctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ppsctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ppsen0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ppsen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trgtmodsel0(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trgtmodsel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn timesel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_timesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for EthMacppscr {
        #[inline(always)]
        fn default() -> EthMacppscr {
            EthMacppscr(0)
        }
    }
    impl core::fmt::Debug for EthMacppscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacppscr")
                .field("ppsctrl", &self.ppsctrl())
                .field("ppsen0", &self.ppsen0())
                .field("trgtmodsel0", &self.trgtmodsel0())
                .field("timesel", &self.timesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacppscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacppscr {{ ppsctrl: {=u8:?}, ppsen0: {=bool:?}, trgtmodsel0: {=u8:?}, timesel: {=bool:?} }}",
                self.ppsctrl(),
                self.ppsen0(),
                self.trgtmodsel0(),
                self.timesel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacppsir(pub u32);
    impl EthMacppsir {
        #[must_use]
        #[inline(always)]
        pub const fn ppsint0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ppsint0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacppsir {
        #[inline(always)]
        fn default() -> EthMacppsir {
            EthMacppsir(0)
        }
    }
    impl core::fmt::Debug for EthMacppsir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacppsir")
                .field("ppsint0", &self.ppsint0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacppsir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacppsir {{ ppsint0: {=u32:?} }}", self.ppsint0())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacppsttnr(pub u32);
    impl EthMacppsttnr {
        #[must_use]
        #[inline(always)]
        pub const fn ttsl0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ttsl0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trgtbusy0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_trgtbusy0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMacppsttnr {
        #[inline(always)]
        fn default() -> EthMacppsttnr {
            EthMacppsttnr(0)
        }
    }
    impl core::fmt::Debug for EthMacppsttnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacppsttnr")
                .field("ttsl0", &self.ttsl0())
                .field("trgtbusy0", &self.trgtbusy0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacppsttnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacppsttnr {{ ttsl0: {=u32:?}, trgtbusy0: {=bool:?} }}",
                self.ttsl0(),
                self.trgtbusy0()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacppsttsr(pub u32);
    impl EthMacppsttsr {
        #[must_use]
        #[inline(always)]
        pub const fn tstrh0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tstrh0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacppsttsr {
        #[inline(always)]
        fn default() -> EthMacppsttsr {
            EthMacppsttsr(0)
        }
    }
    impl core::fmt::Debug for EthMacppsttsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacppsttsr")
                .field("tstrh0", &self.tstrh0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacppsttsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacppsttsr {{ tstrh0: {=u32:?} }}", self.tstrh0())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacppswr(pub u32);
    impl EthMacppswr {
        #[must_use]
        #[inline(always)]
        pub const fn ppswidth0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ppswidth0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacppswr {
        #[inline(always)]
        fn default() -> EthMacppswr {
            EthMacppswr(0)
        }
    }
    impl core::fmt::Debug for EthMacppswr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacppswr")
                .field("ppswidth0", &self.ppswidth0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacppswr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacppswr {{ ppswidth0: {=u32:?} }}", self.ppswidth0())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacprstimr(pub u32);
    impl EthMacprstimr {
        #[must_use]
        #[inline(always)]
        pub const fn mptn(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_mptn(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacprstimr {
        #[inline(always)]
        fn default() -> EthMacprstimr {
            EthMacprstimr(0)
        }
    }
    impl core::fmt::Debug for EthMacprstimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacprstimr")
                .field("mptn", &self.mptn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacprstimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacprstimr {{ mptn: {=u32:?} }}", self.mptn())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacprstimur(pub u32);
    impl EthMacprstimur {
        #[must_use]
        #[inline(always)]
        pub const fn mptu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_mptu(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacprstimur {
        #[inline(always)]
        fn default() -> EthMacprstimur {
            EthMacprstimur(0)
        }
    }
    impl core::fmt::Debug for EthMacprstimur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacprstimur")
                .field("mptu", &self.mptu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacprstimur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacprstimur {{ mptu: {=u32:?} }}", self.mptu())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacrfcr(pub u32);
    impl EthMacrfcr {
        #[must_use]
        #[inline(always)]
        pub const fn rfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for EthMacrfcr {
        #[inline(always)]
        fn default() -> EthMacrfcr {
            EthMacrfcr(0)
        }
    }
    impl core::fmt::Debug for EthMacrfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacrfcr")
                .field("rfe", &self.rfe())
                .field("up", &self.up())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacrfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacrfcr {{ rfe: {=bool:?}, up: {=bool:?} }}",
                self.rfe(),
                self.up()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacrxdti(pub u32);
    impl EthMacrxdti {
        #[must_use]
        #[inline(always)]
        pub const fn rxns(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rxns(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for EthMacrxdti {
        #[inline(always)]
        fn default() -> EthMacrxdti {
            EthMacrxdti(0)
        }
    }
    impl core::fmt::Debug for EthMacrxdti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacrxdti")
                .field("rxns", &self.rxns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacrxdti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacrxdti {{ rxns: {=u16:?} }}", self.rxns())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacrxtxsr(pub u32);
    impl EthMacrxtxsr {
        #[must_use]
        #[inline(always)]
        pub const fn tjt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tjt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ncarr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ncarr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lcarr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lcarr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exdef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exdef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lcol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lcol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn excol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_excol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rwt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for EthMacrxtxsr {
        #[inline(always)]
        fn default() -> EthMacrxtxsr {
            EthMacrxtxsr(0)
        }
    }
    impl core::fmt::Debug for EthMacrxtxsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacrxtxsr")
                .field("tjt", &self.tjt())
                .field("ncarr", &self.ncarr())
                .field("lcarr", &self.lcarr())
                .field("exdef", &self.exdef())
                .field("lcol", &self.lcol())
                .field("excol", &self.excol())
                .field("rwt", &self.rwt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacrxtxsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacrxtxsr {{ tjt: {=bool:?}, ncarr: {=bool:?}, lcarr: {=bool:?}, exdef: {=bool:?}, lcol: {=bool:?}, excol: {=bool:?}, rwt: {=bool:?} }}",
                self.tjt(),
                self.ncarr(),
                self.lcarr(),
                self.exdef(),
                self.lcol(),
                self.excol(),
                self.rwt()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacspi0r(pub u32);
    impl EthMacspi0r {
        #[must_use]
        #[inline(always)]
        pub const fn spi0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_spi0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacspi0r {
        #[inline(always)]
        fn default() -> EthMacspi0r {
            EthMacspi0r(0)
        }
    }
    impl core::fmt::Debug for EthMacspi0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacspi0r")
                .field("spi0", &self.spi0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacspi0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacspi0r {{ spi0: {=u32:?} }}", self.spi0())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacspi1r(pub u32);
    impl EthMacspi1r {
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacspi1r {
        #[inline(always)]
        fn default() -> EthMacspi1r {
            EthMacspi1r(0)
        }
    }
    impl core::fmt::Debug for EthMacspi1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacspi1r")
                .field("spi1", &self.spi1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacspi1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacspi1r {{ spi1: {=u32:?} }}", self.spi1())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacspi2r(pub u32);
    impl EthMacspi2r {
        #[must_use]
        #[inline(always)]
        pub const fn spi2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_spi2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for EthMacspi2r {
        #[inline(always)]
        fn default() -> EthMacspi2r {
            EthMacspi2r(0)
        }
    }
    impl core::fmt::Debug for EthMacspi2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacspi2r")
                .field("spi2", &self.spi2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacspi2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacspi2r {{ spi2: {=u16:?} }}", self.spi2())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacstnr(pub u32);
    impl EthMacstnr {
        #[must_use]
        #[inline(always)]
        pub const fn tsss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tsss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for EthMacstnr {
        #[inline(always)]
        fn default() -> EthMacstnr {
            EthMacstnr(0)
        }
    }
    impl core::fmt::Debug for EthMacstnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacstnr")
                .field("tsss", &self.tsss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacstnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacstnr {{ tsss: {=u32:?} }}", self.tsss())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacstnur(pub u32);
    impl EthMacstnur {
        #[must_use]
        #[inline(always)]
        pub const fn tsss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tsss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn addsub(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_addsub(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMacstnur {
        #[inline(always)]
        fn default() -> EthMacstnur {
            EthMacstnur(0)
        }
    }
    impl core::fmt::Debug for EthMacstnur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacstnur")
                .field("tsss", &self.tsss())
                .field("addsub", &self.addsub())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacstnur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacstnur {{ tsss: {=u32:?}, addsub: {=bool:?} }}",
                self.tsss(),
                self.addsub()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacstsr(pub u32);
    impl EthMacstsr {
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacstsr {
        #[inline(always)]
        fn default() -> EthMacstsr {
            EthMacstsr(0)
        }
    }
    impl core::fmt::Debug for EthMacstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacstsr")
                .field("tss", &self.tss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacstsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacstsr {{ tss: {=u32:?} }}", self.tss())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacstsur(pub u32);
    impl EthMacstsur {
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacstsur {
        #[inline(always)]
        fn default() -> EthMacstsur {
            EthMacstsur(0)
        }
    }
    impl core::fmt::Debug for EthMacstsur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacstsur")
                .field("tss", &self.tss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacstsur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacstsur {{ tss: {=u32:?} }}", self.tss())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactfcr(pub u32);
    impl EthMactfcr {
        #[must_use]
        #[inline(always)]
        pub const fn fcb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn plt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_plt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn plt_minus144(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_plt_minus144(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn plt_minus28(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_plt_minus28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn plt_minus36(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_plt_minus36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn plt_minus256(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_plt_minus256(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dzpq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dzpq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_pt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for EthMactfcr {
        #[inline(always)]
        fn default() -> EthMactfcr {
            EthMactfcr(0)
        }
    }
    impl core::fmt::Debug for EthMactfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactfcr")
                .field("fcb", &self.fcb())
                .field("tfe", &self.tfe())
                .field("plt", &self.plt())
                .field("plt_minus144", &self.plt_minus144())
                .field("plt_minus28", &self.plt_minus28())
                .field("plt_minus36", &self.plt_minus36())
                .field("plt_minus256", &self.plt_minus256())
                .field("dzpq", &self.dzpq())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMactfcr {{ fcb: {=bool:?}, tfe: {=bool:?}, plt: {=u8:?}, plt_minus144: {=u8:?}, plt_minus28: {=bool:?}, plt_minus36: {=bool:?}, plt_minus256: {=bool:?}, dzpq: {=bool:?}, pt: {=u16:?} }}",
                self.fcb(),
                self.tfe(),
                self.plt(),
                self.plt_minus144(),
                self.plt_minus28(),
                self.plt_minus36(),
                self.plt_minus256(),
                self.dzpq(),
                self.pt()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactmrqr(pub u32);
    impl EthMactmrqr {
        #[must_use]
        #[inline(always)]
        pub const fn typ(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_typ(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tmrq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tmrq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pfex(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pfex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for EthMactmrqr {
        #[inline(always)]
        fn default() -> EthMactmrqr {
            EthMactmrqr(0)
        }
    }
    impl core::fmt::Debug for EthMactmrqr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactmrqr")
                .field("typ", &self.typ())
                .field("tmrq", &self.tmrq())
                .field("pfex", &self.pfex())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactmrqr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMactmrqr {{ typ: {=u16:?}, tmrq: {=u8:?}, pfex: {=bool:?} }}",
                self.typ(),
                self.tmrq(),
                self.pfex()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactsar(pub u32);
    impl EthMactsar {
        #[must_use]
        #[inline(always)]
        pub const fn tsar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tsar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMactsar {
        #[inline(always)]
        fn default() -> EthMactsar {
            EthMactsar(0)
        }
    }
    impl core::fmt::Debug for EthMactsar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactsar")
                .field("tsar", &self.tsar())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactsar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMactsar {{ tsar: {=u32:?} }}", self.tsar())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactscr(pub u32);
    impl EthMactscr {
        #[must_use]
        #[inline(always)]
        pub const fn tsena(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tscfupdt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tscfupdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsinit(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsupdt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsupdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsaddreg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsaddreg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ptge(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ptge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsenall(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsenall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsctrlssr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsctrlssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsver2ena(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsver2ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsipena(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsipena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsipv6ena(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsipv6ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsipv4ena(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsipv4ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsevntena(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsevntena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsmstrena(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsmstrena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn snaptypsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_snaptypsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsenmacaddr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsenmacaddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txtsstsm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txtsstsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn epcsl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_epcsl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ecpd(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ecpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lita(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lita(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn av8021asmen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_av8021asmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for EthMactscr {
        #[inline(always)]
        fn default() -> EthMactscr {
            EthMactscr(0)
        }
    }
    impl core::fmt::Debug for EthMactscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactscr")
                .field("tsena", &self.tsena())
                .field("tscfupdt", &self.tscfupdt())
                .field("tsinit", &self.tsinit())
                .field("tsupdt", &self.tsupdt())
                .field("tsaddreg", &self.tsaddreg())
                .field("ptge", &self.ptge())
                .field("tsenall", &self.tsenall())
                .field("tsctrlssr", &self.tsctrlssr())
                .field("tsver2ena", &self.tsver2ena())
                .field("tsipena", &self.tsipena())
                .field("tsipv6ena", &self.tsipv6ena())
                .field("tsipv4ena", &self.tsipv4ena())
                .field("tsevntena", &self.tsevntena())
                .field("tsmstrena", &self.tsmstrena())
                .field("snaptypsel", &self.snaptypsel())
                .field("tsenmacaddr", &self.tsenmacaddr())
                .field("txtsstsm", &self.txtsstsm())
                .field("epcsl", &self.epcsl())
                .field("ecpd", &self.ecpd())
                .field("lita", &self.lita())
                .field("av8021asmen", &self.av8021asmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMactscr {{ tsena: {=bool:?}, tscfupdt: {=bool:?}, tsinit: {=bool:?}, tsupdt: {=bool:?}, tsaddreg: {=bool:?}, ptge: {=bool:?}, tsenall: {=bool:?}, tsctrlssr: {=bool:?}, tsver2ena: {=bool:?}, tsipena: {=bool:?}, tsipv6ena: {=bool:?}, tsipv4ena: {=bool:?}, tsevntena: {=bool:?}, tsmstrena: {=bool:?}, snaptypsel: {=u8:?}, tsenmacaddr: {=bool:?}, txtsstsm: {=bool:?}, epcsl: {=bool:?}, ecpd: {=bool:?}, lita: {=bool:?}, av8021asmen: {=bool:?} }}",
                self.tsena(),
                self.tscfupdt(),
                self.tsinit(),
                self.tsupdt(),
                self.tsaddreg(),
                self.ptge(),
                self.tsenall(),
                self.tsctrlssr(),
                self.tsver2ena(),
                self.tsipena(),
                self.tsipv6ena(),
                self.tsipv4ena(),
                self.tsevntena(),
                self.tsmstrena(),
                self.snaptypsel(),
                self.tsenmacaddr(),
                self.txtsstsm(),
                self.epcsl(),
                self.ecpd(),
                self.lita(),
                self.av8021asmen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactseacr(pub u32);
    impl EthMactseacr {
        #[must_use]
        #[inline(always)]
        pub const fn osteac(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_osteac(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMactseacr {
        #[inline(always)]
        fn default() -> EthMactseacr {
            EthMactseacr(0)
        }
    }
    impl core::fmt::Debug for EthMactseacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactseacr")
                .field("osteac", &self.osteac())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactseacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMactseacr {{ osteac: {=u32:?} }}", self.osteac())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactsecnr(pub u32);
    impl EthMactsecnr {
        #[must_use]
        #[inline(always)]
        pub const fn tsec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tsec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMactsecnr {
        #[inline(always)]
        fn default() -> EthMactsecnr {
            EthMactsecnr(0)
        }
    }
    impl core::fmt::Debug for EthMactsecnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactsecnr")
                .field("tsec", &self.tsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactsecnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMactsecnr {{ tsec: {=u32:?} }}", self.tsec())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactselr(pub u32);
    impl EthMactselr {
        #[must_use]
        #[inline(always)]
        pub const fn etlsns(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_etlsns(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn etlns(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_etlns(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for EthMactselr {
        #[inline(always)]
        fn default() -> EthMactselr {
            EthMactselr(0)
        }
    }
    impl core::fmt::Debug for EthMactselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactselr")
                .field("etlsns", &self.etlsns())
                .field("etlns", &self.etlns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactselr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMactselr {{ etlsns: {=u8:?}, etlns: {=u16:?} }}",
                self.etlsns(),
                self.etlns()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactsiacr(pub u32);
    impl EthMactsiacr {
        #[must_use]
        #[inline(always)]
        pub const fn ostiac(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ostiac(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMactsiacr {
        #[inline(always)]
        fn default() -> EthMactsiacr {
            EthMactsiacr(0)
        }
    }
    impl core::fmt::Debug for EthMactsiacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactsiacr")
                .field("ostiac", &self.ostiac())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactsiacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMactsiacr {{ ostiac: {=u32:?} }}", self.ostiac())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactsicnr(pub u32);
    impl EthMactsicnr {
        #[must_use]
        #[inline(always)]
        pub const fn tsic(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_tsic(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMactsicnr {
        #[inline(always)]
        fn default() -> EthMactsicnr {
            EthMactsicnr(0)
        }
    }
    impl core::fmt::Debug for EthMactsicnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactsicnr")
                .field("tsic", &self.tsic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactsicnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMactsicnr {{ tsic: {=u32:?} }}", self.tsic())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactsilr(pub u32);
    impl EthMactsilr {
        #[must_use]
        #[inline(always)]
        pub const fn itlsns(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_itlsns(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn itlns(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_itlns(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for EthMactsilr {
        #[inline(always)]
        fn default() -> EthMactsilr {
            EthMactsilr(0)
        }
    }
    impl core::fmt::Debug for EthMactsilr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactsilr")
                .field("itlsns", &self.itlsns())
                .field("itlns", &self.itlns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactsilr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMactsilr {{ itlsns: {=u8:?}, itlns: {=u16:?} }}",
                self.itlsns(),
                self.itlns()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactssr(pub u32);
    impl EthMactssr {
        #[must_use]
        #[inline(always)]
        pub const fn tssovf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tssovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tstargt0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tstargt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn auxtstrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_auxtstrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tstrgterr0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tstrgterr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txtssis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txtssis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atsstn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_atsstn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atsstm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_atsstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn atsns(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_atsns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for EthMactssr {
        #[inline(always)]
        fn default() -> EthMactssr {
            EthMactssr(0)
        }
    }
    impl core::fmt::Debug for EthMactssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactssr")
                .field("tssovf", &self.tssovf())
                .field("tstargt0", &self.tstargt0())
                .field("auxtstrig", &self.auxtstrig())
                .field("tstrgterr0", &self.tstrgterr0())
                .field("txtssis", &self.txtssis())
                .field("atsstn", &self.atsstn())
                .field("atsstm", &self.atsstm())
                .field("atsns", &self.atsns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMactssr {{ tssovf: {=bool:?}, tstargt0: {=bool:?}, auxtstrig: {=bool:?}, tstrgterr0: {=bool:?}, txtssis: {=bool:?}, atsstn: {=u8:?}, atsstm: {=bool:?}, atsns: {=u8:?} }}",
                self.tssovf(),
                self.tstargt0(),
                self.auxtstrig(),
                self.tstrgterr0(),
                self.txtssis(),
                self.atsstn(),
                self.atsstm(),
                self.atsns()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacttssnr(pub u32);
    impl EthMacttssnr {
        #[must_use]
        #[inline(always)]
        pub const fn txtsslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_txtsslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txtssmis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txtssmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMacttssnr {
        #[inline(always)]
        fn default() -> EthMacttssnr {
            EthMacttssnr(0)
        }
    }
    impl core::fmt::Debug for EthMacttssnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacttssnr")
                .field("txtsslo", &self.txtsslo())
                .field("txtssmis", &self.txtssmis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacttssnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacttssnr {{ txtsslo: {=u32:?}, txtssmis: {=bool:?} }}",
                self.txtsslo(),
                self.txtssmis()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacttsssr(pub u32);
    impl EthMacttsssr {
        #[must_use]
        #[inline(always)]
        pub const fn txtsshi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_txtsshi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EthMacttsssr {
        #[inline(always)]
        fn default() -> EthMacttsssr {
            EthMacttsssr(0)
        }
    }
    impl core::fmt::Debug for EthMacttsssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacttsssr")
                .field("txtsshi", &self.txtsshi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacttsssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacttsssr {{ txtsshi: {=u32:?} }}", self.txtsshi())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMactxdti(pub u32);
    impl EthMactxdti {
        #[must_use]
        #[inline(always)]
        pub const fn txns(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_txns(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for EthMactxdti {
        #[inline(always)]
        fn default() -> EthMactxdti {
            EthMactxdti(0)
        }
    }
    impl core::fmt::Debug for EthMactxdti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMactxdti")
                .field("txns", &self.txns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMactxdti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMactxdti {{ txns: {=u16:?} }}", self.txns())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacvhtr(pub u32);
    impl EthMacvhtr {
        #[must_use]
        #[inline(always)]
        pub const fn vlht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vlht(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for EthMacvhtr {
        #[inline(always)]
        fn default() -> EthMacvhtr {
            EthMacvhtr(0)
        }
    }
    impl core::fmt::Debug for EthMacvhtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacvhtr")
                .field("vlht", &self.vlht())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacvhtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMacvhtr {{ vlht: {=u16:?} }}", self.vlht())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacvir(pub u32);
    impl EthMacvir {
        #[must_use]
        #[inline(always)]
        pub const fn vlt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vlt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlt_vid(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vlt_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlt_cfidei(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlt_cfidei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlt_up(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vlt_up(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlc_vlantagdelete(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlc_vlantagdelete(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlc_vlantagreplace(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vlc_vlantagreplace(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlc_vlantaginsert(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlc_vlantaginsert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn csvl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_csvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vlti(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vlti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for EthMacvir {
        #[inline(always)]
        fn default() -> EthMacvir {
            EthMacvir(0)
        }
    }
    impl core::fmt::Debug for EthMacvir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacvir")
                .field("vlt", &self.vlt())
                .field("vlt_vid", &self.vlt_vid())
                .field("vlt_cfidei", &self.vlt_cfidei())
                .field("vlt_up", &self.vlt_up())
                .field("vlc", &self.vlc())
                .field("vlc_vlantagdelete", &self.vlc_vlantagdelete())
                .field("vlc_vlantagreplace", &self.vlc_vlantagreplace())
                .field("vlc_vlantaginsert", &self.vlc_vlantaginsert())
                .field("vlp", &self.vlp())
                .field("csvl", &self.csvl())
                .field("vlti", &self.vlti())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacvir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacvir {{ vlt: {=u16:?}, vlt_vid: {=u16:?}, vlt_cfidei: {=bool:?}, vlt_up: {=u8:?}, vlc: {=u8:?}, vlc_vlantagdelete: {=bool:?}, vlc_vlantagreplace: {=u8:?}, vlc_vlantaginsert: {=bool:?}, vlp: {=bool:?}, csvl: {=bool:?}, vlti: {=bool:?} }}",
                self.vlt(),
                self.vlt_vid(),
                self.vlt_cfidei(),
                self.vlt_up(),
                self.vlc(),
                self.vlc_vlantagdelete(),
                self.vlc_vlantagreplace(),
                self.vlc_vlantaginsert(),
                self.vlp(),
                self.csvl(),
                self.vlti()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacvr(pub u32);
    impl EthMacvr {
        #[must_use]
        #[inline(always)]
        pub const fn snpsver(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_snpsver(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn userver(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_userver(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for EthMacvr {
        #[inline(always)]
        fn default() -> EthMacvr {
            EthMacvr(0)
        }
    }
    impl core::fmt::Debug for EthMacvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacvr")
                .field("snpsver", &self.snpsver())
                .field("userver", &self.userver())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacvr {{ snpsver: {=u8:?}, userver: {=u8:?} }}",
                self.snpsver(),
                self.userver()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacvtr(pub u32);
    impl EthMacvtr {
        #[must_use]
        #[inline(always)]
        pub const fn vl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vl_vid(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_vl_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vl_cfidei(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vl_cfidei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vl_up(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vl_up(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn etv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_etv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vtim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn esvl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_esvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ersvlm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ersvlm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dovltc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dovltc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn evls(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_evls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn evls_alwaysstrip(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_evls_alwaysstrip(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn evls_stripifpass(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_evls_stripifpass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn evls_stripiffails(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_evls_stripiffails(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn evlrxs(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_evlrxs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vthm(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vthm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn edvlp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_edvlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn erivlt(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_erivlt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eivls(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eivls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eivls_alwaysstrip(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eivls_alwaysstrip(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eivls_stripifpass(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eivls_stripifpass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eivls_stripiffails(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eivls_stripiffails(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eivlrxs(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_eivlrxs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EthMacvtr {
        #[inline(always)]
        fn default() -> EthMacvtr {
            EthMacvtr(0)
        }
    }
    impl core::fmt::Debug for EthMacvtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacvtr")
                .field("vl", &self.vl())
                .field("vl_vid", &self.vl_vid())
                .field("vl_cfidei", &self.vl_cfidei())
                .field("vl_up", &self.vl_up())
                .field("etv", &self.etv())
                .field("vtim", &self.vtim())
                .field("esvl", &self.esvl())
                .field("ersvlm", &self.ersvlm())
                .field("dovltc", &self.dovltc())
                .field("evls", &self.evls())
                .field("evls_alwaysstrip", &self.evls_alwaysstrip())
                .field("evls_stripifpass", &self.evls_stripifpass())
                .field("evls_stripiffails", &self.evls_stripiffails())
                .field("evlrxs", &self.evlrxs())
                .field("vthm", &self.vthm())
                .field("edvlp", &self.edvlp())
                .field("erivlt", &self.erivlt())
                .field("eivls", &self.eivls())
                .field("eivls_alwaysstrip", &self.eivls_alwaysstrip())
                .field("eivls_stripifpass", &self.eivls_stripifpass())
                .field("eivls_stripiffails", &self.eivls_stripiffails())
                .field("eivlrxs", &self.eivlrxs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacvtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacvtr {{ vl: {=u16:?}, vl_vid: {=u16:?}, vl_cfidei: {=bool:?}, vl_up: {=u8:?}, etv: {=bool:?}, vtim: {=bool:?}, esvl: {=bool:?}, ersvlm: {=bool:?}, dovltc: {=bool:?}, evls: {=u8:?}, evls_alwaysstrip: {=u8:?}, evls_stripifpass: {=bool:?}, evls_stripiffails: {=bool:?}, evlrxs: {=bool:?}, vthm: {=bool:?}, edvlp: {=bool:?}, erivlt: {=bool:?}, eivls: {=u8:?}, eivls_alwaysstrip: {=u8:?}, eivls_stripifpass: {=bool:?}, eivls_stripiffails: {=bool:?}, eivlrxs: {=bool:?} }}",
                self.vl(),
                self.vl_vid(),
                self.vl_cfidei(),
                self.vl_up(),
                self.etv(),
                self.vtim(),
                self.esvl(),
                self.ersvlm(),
                self.dovltc(),
                self.evls(),
                self.evls_alwaysstrip(),
                self.evls_stripifpass(),
                self.evls_stripiffails(),
                self.evlrxs(),
                self.vthm(),
                self.edvlp(),
                self.erivlt(),
                self.eivls(),
                self.eivls_alwaysstrip(),
                self.eivls_stripifpass(),
                self.eivls_stripiffails(),
                self.eivlrxs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMacwjbtr(pub u32);
    impl EthMacwjbtr {
        #[must_use]
        #[inline(always)]
        pub const fn wto(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_wto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pwe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn jto(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_jto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pje(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pje(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for EthMacwjbtr {
        #[inline(always)]
        fn default() -> EthMacwjbtr {
            EthMacwjbtr(0)
        }
    }
    impl core::fmt::Debug for EthMacwjbtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMacwjbtr")
                .field("wto", &self.wto())
                .field("pwe", &self.pwe())
                .field("jto", &self.jto())
                .field("pje", &self.pje())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMacwjbtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMacwjbtr {{ wto: {=u8:?}, pwe: {=bool:?}, jto: {=u8:?}, pje: {=bool:?} }}",
                self.wto(),
                self.pwe(),
                self.jto(),
                self.pje()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMmccr(pub u32);
    impl EthMmccr {
        #[must_use]
        #[inline(always)]
        pub const fn cntrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntstopro(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntstopro(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rstonrd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rstonrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntfreez(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntfreez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntprstlvl(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntprstlvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ucdbc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ucdbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for EthMmccr {
        #[inline(always)]
        fn default() -> EthMmccr {
            EthMmccr(0)
        }
    }
    impl core::fmt::Debug for EthMmccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMmccr")
                .field("cntrst", &self.cntrst())
                .field("cntstopro", &self.cntstopro())
                .field("rstonrd", &self.rstonrd())
                .field("cntfreez", &self.cntfreez())
                .field("cntprst", &self.cntprst())
                .field("cntprstlvl", &self.cntprstlvl())
                .field("ucdbc", &self.ucdbc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMmccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMmccr {{ cntrst: {=bool:?}, cntstopro: {=bool:?}, rstonrd: {=bool:?}, cntfreez: {=bool:?}, cntprst: {=bool:?}, cntprstlvl: {=bool:?}, ucdbc: {=bool:?} }}",
                self.cntrst(),
                self.cntstopro(),
                self.rstonrd(),
                self.cntfreez(),
                self.cntprst(),
                self.cntprstlvl(),
                self.ucdbc()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMmcrimr(pub u32);
    impl EthMmcrimr {
        #[must_use]
        #[inline(always)]
        pub const fn rxcrcerpim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxcrcerpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxalgnerpim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxalgnerpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxucgpim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxucgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxlpiuscim(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxlpiuscim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxlpitrcim(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxlpitrcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for EthMmcrimr {
        #[inline(always)]
        fn default() -> EthMmcrimr {
            EthMmcrimr(0)
        }
    }
    impl core::fmt::Debug for EthMmcrimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMmcrimr")
                .field("rxcrcerpim", &self.rxcrcerpim())
                .field("rxalgnerpim", &self.rxalgnerpim())
                .field("rxucgpim", &self.rxucgpim())
                .field("rxlpiuscim", &self.rxlpiuscim())
                .field("rxlpitrcim", &self.rxlpitrcim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMmcrimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMmcrimr {{ rxcrcerpim: {=bool:?}, rxalgnerpim: {=bool:?}, rxucgpim: {=bool:?}, rxlpiuscim: {=bool:?}, rxlpitrcim: {=bool:?} }}",
                self.rxcrcerpim(),
                self.rxalgnerpim(),
                self.rxucgpim(),
                self.rxlpiuscim(),
                self.rxlpitrcim()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMmcrir(pub u32);
    impl EthMmcrir {
        #[must_use]
        #[inline(always)]
        pub const fn rxcrcerpis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxcrcerpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxalgnerpis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxalgnerpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxucgpis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxucgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxlpiuscis(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxlpiuscis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxlpitrcis(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxlpitrcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for EthMmcrir {
        #[inline(always)]
        fn default() -> EthMmcrir {
            EthMmcrir(0)
        }
    }
    impl core::fmt::Debug for EthMmcrir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMmcrir")
                .field("rxcrcerpis", &self.rxcrcerpis())
                .field("rxalgnerpis", &self.rxalgnerpis())
                .field("rxucgpis", &self.rxucgpis())
                .field("rxlpiuscis", &self.rxlpiuscis())
                .field("rxlpitrcis", &self.rxlpitrcis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMmcrir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMmcrir {{ rxcrcerpis: {=bool:?}, rxalgnerpis: {=bool:?}, rxucgpis: {=bool:?}, rxlpiuscis: {=bool:?}, rxlpitrcis: {=bool:?} }}",
                self.rxcrcerpis(),
                self.rxalgnerpis(),
                self.rxucgpis(),
                self.rxlpiuscis(),
                self.rxlpitrcis()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMmctimr(pub u32);
    impl EthMmctimr {
        #[must_use]
        #[inline(always)]
        pub const fn txscolgpim(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txscolgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmcolgpim(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmcolgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txgpktim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txgpktim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txlpiuscim(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txlpiuscim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txlpitrcim(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txlpitrcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for EthMmctimr {
        #[inline(always)]
        fn default() -> EthMmctimr {
            EthMmctimr(0)
        }
    }
    impl core::fmt::Debug for EthMmctimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMmctimr")
                .field("txscolgpim", &self.txscolgpim())
                .field("txmcolgpim", &self.txmcolgpim())
                .field("txgpktim", &self.txgpktim())
                .field("txlpiuscim", &self.txlpiuscim())
                .field("txlpitrcim", &self.txlpitrcim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMmctimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMmctimr {{ txscolgpim: {=bool:?}, txmcolgpim: {=bool:?}, txgpktim: {=bool:?}, txlpiuscim: {=bool:?}, txlpitrcim: {=bool:?} }}",
                self.txscolgpim(),
                self.txmcolgpim(),
                self.txgpktim(),
                self.txlpiuscim(),
                self.txlpitrcim()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMmctir(pub u32);
    impl EthMmctir {
        #[must_use]
        #[inline(always)]
        pub const fn txscolgpis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txscolgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txmcolgpis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txmcolgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txgpktis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txgpktis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txlpiuscis(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txlpiuscis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txlpitrcis(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txlpitrcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for EthMmctir {
        #[inline(always)]
        fn default() -> EthMmctir {
            EthMmctir(0)
        }
    }
    impl core::fmt::Debug for EthMmctir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMmctir")
                .field("txscolgpis", &self.txscolgpis())
                .field("txmcolgpis", &self.txmcolgpis())
                .field("txgpktis", &self.txgpktis())
                .field("txlpiuscis", &self.txlpiuscis())
                .field("txlpitrcis", &self.txlpitrcis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMmctir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMmctir {{ txscolgpis: {=bool:?}, txmcolgpis: {=bool:?}, txgpktis: {=bool:?}, txlpiuscis: {=bool:?}, txlpitrcis: {=bool:?} }}",
                self.txscolgpis(),
                self.txmcolgpis(),
                self.txgpktis(),
                self.txlpiuscis(),
                self.txlpitrcis()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtlisr(pub u32);
    impl EthMtlisr {
        #[must_use]
        #[inline(always)]
        pub const fn q0is(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_q0is(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EthMtlisr {
        #[inline(always)]
        fn default() -> EthMtlisr {
            EthMtlisr(0)
        }
    }
    impl core::fmt::Debug for EthMtlisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtlisr")
                .field("q0is", &self.q0is())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtlisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EthMtlisr {{ q0is: {=bool:?} }}", self.q0is())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtlomr(pub u32);
    impl EthMtlomr {
        #[must_use]
        #[inline(always)]
        pub const fn dtxsts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dtxsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntprst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cntclr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_cntclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for EthMtlomr {
        #[inline(always)]
        fn default() -> EthMtlomr {
            EthMtlomr(0)
        }
    }
    impl core::fmt::Debug for EthMtlomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtlomr")
                .field("dtxsts", &self.dtxsts())
                .field("cntprst", &self.cntprst())
                .field("cntclr", &self.cntclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtlomr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMtlomr {{ dtxsts: {=bool:?}, cntprst: {=bool:?}, cntclr: {=bool:?} }}",
                self.dtxsts(),
                self.cntprst(),
                self.cntclr()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtlqicsr(pub u32);
    impl EthMtlqicsr {
        #[must_use]
        #[inline(always)]
        pub const fn txunfis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txunfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txuie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxovfis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxovfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxoie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for EthMtlqicsr {
        #[inline(always)]
        fn default() -> EthMtlqicsr {
            EthMtlqicsr(0)
        }
    }
    impl core::fmt::Debug for EthMtlqicsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtlqicsr")
                .field("txunfis", &self.txunfis())
                .field("txuie", &self.txuie())
                .field("rxovfis", &self.rxovfis())
                .field("rxoie", &self.rxoie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtlqicsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMtlqicsr {{ txunfis: {=bool:?}, txuie: {=bool:?}, rxovfis: {=bool:?}, rxoie: {=bool:?} }}",
                self.txunfis(),
                self.txuie(),
                self.rxovfis(),
                self.rxoie()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtlrqdr(pub u32);
    impl EthMtlrqdr {
        #[must_use]
        #[inline(always)]
        pub const fn rwcsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rwcsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rrcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrcsts_flushing(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rrcsts_flushing(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrcsts_readingdata(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rrcsts_readingdata(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rrcsts_readingstatus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rrcsts_readingstatus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxqsts(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxqsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxqsts_belowthreshold(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxqsts_belowthreshold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxqsts_full(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rxqsts_full(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rxqsts_abovethreshold(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rxqsts_abovethreshold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn prxq(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_prxq(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for EthMtlrqdr {
        #[inline(always)]
        fn default() -> EthMtlrqdr {
            EthMtlrqdr(0)
        }
    }
    impl core::fmt::Debug for EthMtlrqdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtlrqdr")
                .field("rwcsts", &self.rwcsts())
                .field("rrcsts", &self.rrcsts())
                .field("rrcsts_flushing", &self.rrcsts_flushing())
                .field("rrcsts_readingdata", &self.rrcsts_readingdata())
                .field("rrcsts_readingstatus", &self.rrcsts_readingstatus())
                .field("rxqsts", &self.rxqsts())
                .field("rxqsts_belowthreshold", &self.rxqsts_belowthreshold())
                .field("rxqsts_full", &self.rxqsts_full())
                .field("rxqsts_abovethreshold", &self.rxqsts_abovethreshold())
                .field("prxq", &self.prxq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtlrqdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMtlrqdr {{ rwcsts: {=bool:?}, rrcsts: {=u8:?}, rrcsts_flushing: {=u8:?}, rrcsts_readingdata: {=bool:?}, rrcsts_readingstatus: {=bool:?}, rxqsts: {=u8:?}, rxqsts_belowthreshold: {=bool:?}, rxqsts_full: {=u8:?}, rxqsts_abovethreshold: {=bool:?}, prxq: {=u16:?} }}",
                self.rwcsts(),
                self.rrcsts(),
                self.rrcsts_flushing(),
                self.rrcsts_readingdata(),
                self.rrcsts_readingstatus(),
                self.rxqsts(),
                self.rxqsts_belowthreshold(),
                self.rxqsts_full(),
                self.rxqsts_abovethreshold(),
                self.prxq()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtlrqmpocr(pub u32);
    impl EthMtlrqmpocr {
        #[must_use]
        #[inline(always)]
        pub const fn ovfpktcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ovfpktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ovfcntovf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ovfcntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mispktcnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_mispktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn miscntovf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_miscntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for EthMtlrqmpocr {
        #[inline(always)]
        fn default() -> EthMtlrqmpocr {
            EthMtlrqmpocr(0)
        }
    }
    impl core::fmt::Debug for EthMtlrqmpocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtlrqmpocr")
                .field("ovfpktcnt", &self.ovfpktcnt())
                .field("ovfcntovf", &self.ovfcntovf())
                .field("mispktcnt", &self.mispktcnt())
                .field("miscntovf", &self.miscntovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtlrqmpocr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMtlrqmpocr {{ ovfpktcnt: {=u16:?}, ovfcntovf: {=bool:?}, mispktcnt: {=u16:?}, miscntovf: {=bool:?} }}",
                self.ovfpktcnt(),
                self.ovfcntovf(),
                self.mispktcnt(),
                self.miscntovf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtlrqomr(pub u32);
    impl EthMtlrqomr {
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn fep(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_fep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn distcpef(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_distcpef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rqs(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for EthMtlrqomr {
        #[inline(always)]
        fn default() -> EthMtlrqomr {
            EthMtlrqomr(0)
        }
    }
    impl core::fmt::Debug for EthMtlrqomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtlrqomr")
                .field("rtc", &self.rtc())
                .field("fup", &self.fup())
                .field("fep", &self.fep())
                .field("rsf", &self.rsf())
                .field("distcpef", &self.distcpef())
                .field("rqs", &self.rqs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtlrqomr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMtlrqomr {{ rtc: {=u8:?}, fup: {=bool:?}, fep: {=bool:?}, rsf: {=bool:?}, distcpef: {=bool:?}, rqs: {=u8:?} }}",
                self.rtc(),
                self.fup(),
                self.fep(),
                self.rsf(),
                self.distcpef(),
                self.rqs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtltqdr(pub u32);
    impl EthMtltqdr {
        #[must_use]
        #[inline(always)]
        pub const fn txqpaused(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txqpaused(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn trcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_trcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn twcsts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_twcsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txqsts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txqsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txstsfsts(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txstsfsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ptxq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ptxq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn stxstsf(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_stxstsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for EthMtltqdr {
        #[inline(always)]
        fn default() -> EthMtltqdr {
            EthMtltqdr(0)
        }
    }
    impl core::fmt::Debug for EthMtltqdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtltqdr")
                .field("txqpaused", &self.txqpaused())
                .field("trcsts", &self.trcsts())
                .field("twcsts", &self.twcsts())
                .field("txqsts", &self.txqsts())
                .field("txstsfsts", &self.txstsfsts())
                .field("ptxq", &self.ptxq())
                .field("stxstsf", &self.stxstsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtltqdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMtltqdr {{ txqpaused: {=bool:?}, trcsts: {=u8:?}, twcsts: {=bool:?}, txqsts: {=bool:?}, txstsfsts: {=bool:?}, ptxq: {=u8:?}, stxstsf: {=u8:?} }}",
                self.txqpaused(),
                self.trcsts(),
                self.twcsts(),
                self.txqsts(),
                self.txstsfsts(),
                self.ptxq(),
                self.stxstsf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtltqomr(pub u32);
    impl EthMtltqomr {
        #[must_use]
        #[inline(always)]
        pub const fn ftq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ftq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txqen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_txqen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txqen_avmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txqen_avmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn txqen_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_txqen_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ttc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ttc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn tqs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_tqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for EthMtltqomr {
        #[inline(always)]
        fn default() -> EthMtltqomr {
            EthMtltqomr(0)
        }
    }
    impl core::fmt::Debug for EthMtltqomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtltqomr")
                .field("ftq", &self.ftq())
                .field("tsf", &self.tsf())
                .field("txqen", &self.txqen())
                .field("txqen_avmode", &self.txqen_avmode())
                .field("txqen_en", &self.txqen_en())
                .field("ttc", &self.ttc())
                .field("tqs", &self.tqs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtltqomr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMtltqomr {{ ftq: {=bool:?}, tsf: {=bool:?}, txqen: {=u8:?}, txqen_avmode: {=bool:?}, txqen_en: {=bool:?}, ttc: {=u8:?}, tqs: {=u8:?} }}",
                self.ftq(),
                self.tsf(),
                self.txqen(),
                self.txqen_avmode(),
                self.txqen_en(),
                self.ttc(),
                self.tqs()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EthMtltqur(pub u32);
    impl EthMtltqur {
        #[must_use]
        #[inline(always)]
        pub const fn ufpktcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ufpktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ufcntovf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ufcntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for EthMtltqur {
        #[inline(always)]
        fn default() -> EthMtltqur {
            EthMtltqur(0)
        }
    }
    impl core::fmt::Debug for EthMtltqur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EthMtltqur")
                .field("ufpktcnt", &self.ufpktcnt())
                .field("ufcntovf", &self.ufcntovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EthMtltqur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EthMtltqur {{ ufpktcnt: {=u16:?}, ufcntovf: {=bool:?} }}",
                self.ufpktcnt(),
                self.ufcntovf()
            )
        }
    }
}
