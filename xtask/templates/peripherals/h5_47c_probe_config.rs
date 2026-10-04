// Exact H543/H553 constructor clocks; board timing and USB enumeration need HIL.
// HSI64 / 40 * 120 / 4 = PLL1Q 48 MHz; reference 1.6 MHz, medium VCO 192 MHz.
fn probe_config() -> hal::Config {
    use hal::rcc::mux::{Fdcan12sel, Persel, Spi1sel, Usbsel};
    use hal::rcc::{HSIPrescaler, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk};
    let mut config = hal::Config::default();
    config.rcc.hsi = Some(HSIPrescaler::Div1);
    config.rcc.sys = Sysclk::Hsi;
    config.rcc.pll1 = Some(Pll {
        source: PllSource::Hsi,
        prediv: PllPreDiv::Div40,
        mul: PllMul::Mul120,
        divp: None,
        divq: Some(PllDiv::Div4),
        divr: None,
    });
    config.rcc.mux.fdcan12sel = Fdcan12sel::Pll1Q;
    config.rcc.mux.persel = Persel::Hsi;
    config.rcc.mux.spi1sel = Spi1sel::Per;
    // USB FS calibrates HSI48 from host SOF packets.
    config.rcc.hsi48 = Some(hal::rcc::Hsi48Config { sync_from_usb: true });
    config.rcc.mux.usbsel = Usbsel::Hsi48;
    config
}
