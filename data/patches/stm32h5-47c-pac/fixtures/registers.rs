#![no_std]
#[cfg(test)]
mod tests {
 use stm32_metapac as p;
 #[test] fn exact_bases_and_two_plls() {
  assert_eq!(p::FLASH.as_ptr() as usize, 0x40022000);
  assert_eq!(p::RCC.pllcfgr(1).as_ptr() as usize, p::RCC.as_ptr() as usize+44);
 }
 #[test] #[should_panic] fn absent_third_pll_is_rejected() { let _=p::RCC.pllcfgr(2); }
 #[test] fn exact_fdcan_ram_instances_and_last_words() {
  assert_eq!(p::FDCANRAM1.as_ptr() as usize,0x4000ac00);
  assert_eq!(p::FDCANRAM2.as_ptr() as usize,0x4000af50);
  let r=p::FDCANRAM1;let base=r.as_ptr() as usize;
  assert_eq!(r.flssa(27).as_ptr() as usize,base+108);
  assert_eq!(r.flesa(15).as_ptr() as usize,base+172);
  assert_eq!(r.rxfifo0(53).as_ptr() as usize,base+388);
  assert_eq!(r.rxfifo1(53).as_ptr() as usize,base+604);
  assert_eq!(r.txefifo(5).as_ptr() as usize,base+628);
  assert_eq!(r.txbuf(53).as_ptr() as usize,base+844);
 }
 #[test] #[should_panic] fn fdcan_ram_end_is_bounded() {let _=p::FDCANRAM2.txbuf(54);}
 #[test] fn exact_crs_and_rtc_api_values() {
  assert_eq!(p::CRS.as_ptr() as usize,0x40006000);
  let mut cf=p::crs::regs::Cfgr(0);cf.set_syncsrc(p::crs::vals::Syncsrc::Usb);assert_eq!(cf.0,2<<28);
  assert_eq!(p::rcc::vals::Rtcsel::Disable.to_bits(),0);
  assert_eq!(p::rcc::vals::Rtcsel::HseDivRtcpre.to_bits(),3);
 }
 #[test] fn flash_snb_and_protection_masks() {
  let mut cr=p::flash::regs::Nscr(0);cr.set_snb(255);assert_eq!(cr.0,0xFC0);
  let mut wm=p::flash::regs::Secwm(0);wm.set_secwm_strt(255);wm.set_secwm_end(255);assert_eq!(wm.0,0x3F003F);
  let mut wrp=p::flash::regs::Wrp(0);wrp.set_wrpsg(0xFFFF);assert_eq!(wrp.0,0xFFFF);
 }
 #[test] fn sram_power_47c_bits() {
  let mut pm=p::pwr::regs::Pmcr(0);pm.set_sram2_48so(p::pwr::vals::ShutOff::from_bits(1));pm.set_sram1so(p::pwr::vals::ShutOff::from_bits(1));assert_eq!(pm.0,0x0C000000);
 }
 #[test] fn rng_full_config_and_new_registers() {
  let mut cr=p::rng::regs::Cr(0);cr.set_rng_config1(255);assert_eq!(cr.rng_config1(),255);
  assert_eq!(p::RNG.htsr(1).as_ptr() as usize,p::RNG.as_ptr() as usize+36);
  assert_eq!(p::RNG.nsmr().as_ptr() as usize,p::RNG.as_ptr() as usize+48);
 }
 #[test] fn corrected_i3c_masks() {
  let mut t=p::i3c::regs::Timingr2(0);t.set_stalls(true);t.set_stalll(true);assert_eq!(t.0,0x60);
 }
 #[test] fn unknown_access_is_in_metadata() {
  let rng=p::metadata::METADATA.peripherals.iter().find(|x|x.name=="RNG").unwrap().registers.as_ref().unwrap();
  let block=rng.ir.blocks.iter().find(|x|x.name=="Rng").unwrap();
  let item=block.items.iter().find(|x|x.name=="nsmr").unwrap();
  let p::metadata::ir::BlockItemInner::Register(ref reg)=item.inner else {panic!()};
  assert!(matches!(reg.access,p::metadata::ir::Access::Raw));
 }
}
